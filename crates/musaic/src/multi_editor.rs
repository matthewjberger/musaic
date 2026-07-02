use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::code_editor::Highlighter;

#[derive(Clone, Copy)]
struct Caret {
    anchor: usize,
    head: usize,
}

impl Caret {
    fn collapsed(position: usize) -> Self {
        Self {
            anchor: position,
            head: position,
        }
    }

    fn low(&self) -> usize {
        self.anchor.min(self.head)
    }

    fn high(&self) -> usize {
        self.anchor.max(self.head)
    }
}

fn line_col(chars: &[char], index: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;
    for &character in &chars[..index.min(chars.len())] {
        if character == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn line_start(chars: &[char], line: usize) -> usize {
    let mut current = 0;
    let mut index = 0;
    while index < chars.len() && current < line {
        if chars[index] == '\n' {
            current += 1;
        }
        index += 1;
    }
    index
}

fn line_len(chars: &[char], line: usize) -> usize {
    let start = line_start(chars, line);
    let mut end = start;
    while end < chars.len() && chars[end] != '\n' {
        end += 1;
    }
    end - start
}

fn total_lines(chars: &[char]) -> usize {
    chars.iter().filter(|&&character| character == '\n').count() + 1
}

fn offset_of(chars: &[char], line: usize, col: usize) -> usize {
    line_start(chars, line) + col.min(line_len(chars, line))
}

fn normalize(carets: &mut Vec<Caret>) {
    carets.sort_by_key(Caret::low);
    carets.dedup_by_key(|caret| (caret.anchor, caret.head));
}

fn insert_text(chars: &mut Vec<char>, carets: &mut [Caret], insert: &[char]) {
    carets.sort_by_key(Caret::low);
    let mut delta: isize = 0;
    for caret in carets.iter_mut() {
        let low = (caret.low() as isize + delta) as usize;
        let high = (caret.high() as isize + delta) as usize;
        chars.splice(low..high, insert.iter().copied());
        *caret = Caret::collapsed(low + insert.len());
        delta += insert.len() as isize - (high as isize - low as isize);
    }
}

fn delete_backward(chars: &mut Vec<char>, carets: &mut [Caret]) {
    carets.sort_by_key(Caret::low);
    let mut delta: isize = 0;
    for caret in carets.iter_mut() {
        let low = (caret.low() as isize + delta) as usize;
        let high = (caret.high() as isize + delta) as usize;
        let (from, to) = if high > low {
            (low, high)
        } else if low > 0 {
            (low - 1, low)
        } else {
            (low, low)
        };
        chars.drain(from..to);
        *caret = Caret::collapsed(from);
        delta -= (to - from) as isize;
    }
}

fn move_horizontal(chars: &[char], carets: &mut [Caret], direction: isize, extend: bool) {
    let len = chars.len() as isize;
    for caret in carets.iter_mut() {
        let head = (caret.head as isize + direction).clamp(0, len) as usize;
        caret.head = head;
        if !extend {
            caret.anchor = head;
        }
    }
}

fn move_vertical(chars: &[char], carets: &mut [Caret], direction: isize, extend: bool) {
    for caret in carets.iter_mut() {
        let (line, col) = line_col(chars, caret.head);
        let target = (line as isize + direction).max(0) as usize;
        if target < total_lines(chars) {
            caret.head = offset_of(chars, target, col);
            if !extend {
                caret.anchor = caret.head;
            }
        }
    }
}

fn word_bounds(chars: &[char], index: usize) -> (usize, usize) {
    let is_word = |character: char| character.is_alphanumeric() || character == '_';
    let mut start = index;
    while start > 0 && chars.get(start - 1).is_some_and(|&c| is_word(c)) {
        start -= 1;
    }
    let mut end = index;
    while end < chars.len() && chars.get(end).is_some_and(|&c| is_word(c)) {
        end += 1;
    }
    (start, end)
}

fn add_next_occurrence(chars: &[char], carets: &mut Vec<Caret>) {
    let Some(last) = carets.iter().max_by_key(|caret| caret.high()).copied() else {
        return;
    };
    let (low, high) = if last.high() > last.low() {
        (last.low(), last.high())
    } else {
        word_bounds(chars, last.head)
    };
    if high <= low {
        return;
    }
    let needle = &chars[low..high];
    let mut cursor = high;
    while cursor + needle.len() <= chars.len() {
        if &chars[cursor..cursor + needle.len()] == needle {
            carets.push(Caret {
                anchor: cursor,
                head: cursor + needle.len(),
            });
            return;
        }
        cursor += 1;
    }
}

#[component]
pub fn MultiEditor(
    value: RwSignal<String>,
    #[prop(optional)] highlighter: Option<Highlighter>,
    #[prop(default = 20.0)] line_height: f64,
    #[prop(default = 360.0)] height: f64,
    #[prop(default = 12)] overscan: usize,
) -> impl IntoView {
    let carets = RwSignal::new(vec![Caret::collapsed(0)]);
    let scroll_top = RwSignal::new(0.0);
    let viewport_height = RwSignal::new(height);
    let wrap_ref = NodeRef::<html::Div>::new();

    let edit = move |mutate: &dyn Fn(&mut Vec<char>, &mut Vec<Caret>)| {
        let mut chars: Vec<char> = value.get_untracked().chars().collect();
        let mut current = carets.get_untracked();
        mutate(&mut chars, &mut current);
        normalize(&mut current);
        value.set(chars.into_iter().collect());
        carets.set(current);
    };

    let on_key = move |event: web_sys::KeyboardEvent| {
        let key = event.key();
        let ctrl = event.ctrl_key() || event.meta_key();
        let alt = event.alt_key();
        let shift = event.shift_key();

        if ctrl && alt && key == "ArrowDown" {
            event.prevent_default();
            edit(&|chars, carets| {
                if let Some(last) = carets.iter().max_by_key(|caret| caret.high()).copied() {
                    let (line, col) = line_col(chars, last.head);
                    if line + 1 < total_lines(chars) {
                        carets.push(Caret::collapsed(offset_of(chars, line + 1, col)));
                    }
                }
            });
            return;
        }
        if ctrl && alt && key == "ArrowUp" {
            event.prevent_default();
            edit(&|chars, carets| {
                if let Some(first) = carets.iter().min_by_key(|caret| caret.low()).copied() {
                    let (line, col) = line_col(chars, first.head);
                    if line > 0 {
                        carets.push(Caret::collapsed(offset_of(chars, line - 1, col)));
                    }
                }
            });
            return;
        }
        if ctrl && key == "d" {
            event.prevent_default();
            edit(&|chars, carets| add_next_occurrence(chars, carets));
            return;
        }
        match key.as_str() {
            "ArrowLeft" => {
                event.prevent_default();
                edit(&move |chars, carets| move_horizontal(chars, carets, -1, shift));
            }
            "ArrowRight" => {
                event.prevent_default();
                edit(&move |chars, carets| move_horizontal(chars, carets, 1, shift));
            }
            "ArrowUp" => {
                event.prevent_default();
                edit(&move |chars, carets| move_vertical(chars, carets, -1, shift));
            }
            "ArrowDown" => {
                event.prevent_default();
                edit(&move |chars, carets| move_vertical(chars, carets, 1, shift));
            }
            "Backspace" => {
                event.prevent_default();
                edit(&|chars, carets| delete_backward(chars, carets));
            }
            "Enter" => {
                event.prevent_default();
                edit(&|chars, carets| insert_text(chars, carets, &['\n']));
            }
            "Tab" => {
                event.prevent_default();
                edit(&|chars, carets| insert_text(chars, carets, &[' ', ' ', ' ', ' ']));
            }
            "Escape" => {
                event.prevent_default();
                carets.update(|list| {
                    if let Some(first) = list.first().copied() {
                        *list = vec![Caret::collapsed(first.head)];
                    }
                });
            }
            text if text.chars().count() == 1 && !ctrl && !alt => {
                event.prevent_default();
                let glyph: Vec<char> = text.chars().collect();
                edit(&move |chars, carets| insert_text(chars, carets, &glyph));
            }
            _ => {}
        }
    };

    let on_scroll = move |event: web_sys::Event| {
        if let Some(element) = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
        {
            scroll_top.set(element.scroll_top() as f64);
            viewport_height.set(element.client_height() as f64);
        }
    };

    Effect::new(move |_| {
        if let Some(element) = wrap_ref.get() {
            viewport_height.set(element.client_height() as f64);
        }
    });

    let content = move || {
        let source = value.get();
        let chars: Vec<char> = source.chars().collect();
        let lines: Vec<String> = source.split('\n').map(str::to_string).collect();
        let total = lines.len();
        let view_height = viewport_height.get().max(line_height);
        let first = ((scroll_top.get() / line_height).floor() as usize).saturating_sub(overscan);
        let count = (view_height / line_height).ceil() as usize + overscan * 2 + 1;
        let start = first.min(total);
        let end = (start + count).min(total);

        let rendered_lines = (start..end)
            .map(|index| {
                let text = lines[index].clone();
                let spans = match highlighter {
                    Some(highlight) => highlight(&text),
                    None => vec![("tok-plain", text)],
                };
                view! {
                    <div
                        class="musaic-ml-line"
                        style=format!("top:{}px;height:{}px", index as f64 * line_height, line_height)
                    >
                        {spans
                            .into_iter()
                            .map(|(class, text)| view! { <span class=class>{text}</span> })
                            .collect_view()}
                    </div>
                }
            })
            .collect_view();

        let overlays = carets
            .get()
            .into_iter()
            .flat_map(|caret| {
                let mut nodes = Vec::new();
                if caret.high() > caret.low() {
                    let (start_line, start_col) = line_col(&chars, caret.low());
                    let (end_line, end_col) = line_col(&chars, caret.high());
                    for line in start_line..=end_line {
                        let from = if line == start_line { start_col } else { 0 };
                        let to = if line == end_line {
                            end_col
                        } else {
                            line_len(&chars, line) + 1
                        };
                        nodes.push(
                            view! {
                                <div
                                    class="musaic-ml-selection"
                                    style=format!(
                                        "top:{}px;height:{}px;left:calc(10px + {}ch);width:calc({}ch)",
                                        line as f64 * line_height,
                                        line_height,
                                        from,
                                        to.saturating_sub(from),
                                    )
                                ></div>
                            }
                            .into_any(),
                        );
                    }
                }
                let (line, col) = line_col(&chars, caret.head);
                nodes.push(
                    view! {
                        <div
                            class="musaic-ml-caret"
                            style=format!(
                                "top:{}px;height:{}px;left:calc(10px + {}ch)",
                                line as f64 * line_height,
                                line_height,
                                col,
                            )
                        ></div>
                    }
                    .into_any(),
                );
                nodes
            })
            .collect_view();

        view! {
            <div
                class="musaic-ml-inner"
                style=format!("height:{}px", total as f64 * line_height)
            >
                {overlays}
                {rendered_lines}
            </div>
        }
    };

    view! {
        <div
            class="musaic-ml"
            tabindex="0"
            node_ref=wrap_ref
            style=format!("height:{height}px")
            on:keydown=on_key
            on:scroll=on_scroll
        >
            {content}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{Caret, add_next_occurrence, delete_backward, insert_text};

    fn chars(text: &str) -> Vec<char> {
        text.chars().collect()
    }

    #[test]
    fn insert_at_multiple_carets_shifts_correctly() {
        let mut buffer = chars("ab\ncd");
        let mut carets = vec![Caret::collapsed(0), Caret::collapsed(3)];
        insert_text(&mut buffer, &mut carets, &['X']);
        assert_eq!(buffer.iter().collect::<String>(), "Xab\nXcd");
        assert_eq!(carets[0].head, 1);
        assert_eq!(carets[1].head, 5);
    }

    #[test]
    fn backspace_deletes_before_each_caret() {
        let mut buffer = chars("ab\ncd");
        let mut carets = vec![Caret::collapsed(2), Caret::collapsed(5)];
        delete_backward(&mut buffer, &mut carets);
        assert_eq!(buffer.iter().collect::<String>(), "a\nc");
    }

    #[test]
    fn add_next_occurrence_selects_following_match() {
        let buffer = chars("foo bar foo");
        let mut carets = vec![Caret { anchor: 0, head: 3 }];
        add_next_occurrence(&buffer, &mut carets);
        assert_eq!(carets.len(), 2);
        assert_eq!(carets[1].anchor, 8);
        assert_eq!(carets[1].head, 11);
    }
}
