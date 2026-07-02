use std::collections::HashSet;

use leptos::prelude::*;
use wasm_bindgen::JsCast;

pub type Highlighter = fn(&str) -> Vec<(&'static str, String)>;

pub fn highlight_code(
    source: &str,
    keywords: &[&'static str],
    commands: &[&'static str],
) -> Vec<(&'static str, String)> {
    let keyword_set: HashSet<&'static str> = keywords.iter().copied().collect();
    let command_set: HashSet<&'static str> = commands.iter().copied().collect();
    scan(source, &keyword_set, &command_set)
}

fn scan(
    source: &str,
    keywords: &HashSet<&'static str>,
    commands: &HashSet<&'static str>,
) -> Vec<(&'static str, String)> {
    let characters: Vec<char> = source.chars().collect();
    let count = characters.len();
    let mut runs: Vec<(&'static str, String)> = Vec::new();
    let mut index = 0;
    while index < count {
        let current = characters[index];
        if current == '/' && index + 1 < count && characters[index + 1] == '*' {
            let start = index;
            index += 2;
            while index < count
                && !(characters[index] == '*' && index + 1 < count && characters[index + 1] == '/')
            {
                index += 1;
            }
            index = (index + 2).min(count);
            runs.push(("tok-comment", characters[start..index].iter().collect()));
        } else if current == '/' && index + 1 < count && characters[index + 1] == '/' {
            let start = index;
            while index < count && characters[index] != '\n' {
                index += 1;
            }
            runs.push(("tok-comment", characters[start..index].iter().collect()));
        } else if current == '"' {
            let start = index;
            index += 1;
            while index < count {
                if characters[index] == '\\' && index + 1 < count {
                    index += 2;
                    continue;
                }
                let quote = characters[index] == '"';
                index += 1;
                if quote {
                    break;
                }
            }
            runs.push(("tok-string", characters[start..index].iter().collect()));
        } else if current.is_ascii_digit() {
            let start = index;
            while index < count
                && (characters[index].is_ascii_alphanumeric() || characters[index] == '.')
            {
                index += 1;
            }
            runs.push(("tok-number", characters[start..index].iter().collect()));
        } else if current.is_alphabetic() || current == '_' {
            let start = index;
            while index < count && (characters[index].is_alphanumeric() || characters[index] == '_')
            {
                index += 1;
            }
            let word: String = characters[start..index].iter().collect();
            let class = if keywords.contains(word.as_str()) {
                "tok-keyword"
            } else if commands.contains(word.as_str()) {
                "tok-command"
            } else {
                "tok-plain"
            };
            runs.push((class, word));
        } else {
            let start = index;
            index += 1;
            while index < count {
                let next = characters[index];
                let token_start = (next == '/'
                    && index + 1 < count
                    && (characters[index + 1] == '/' || characters[index + 1] == '*'))
                    || next == '"'
                    || next.is_ascii_digit()
                    || next.is_alphabetic()
                    || next == '_';
                if token_start {
                    break;
                }
                index += 1;
            }
            runs.push(("tok-plain", characters[start..index].iter().collect()));
        }
    }
    runs
}

#[component]
pub fn CodeEditor(
    value: RwSignal<String>,
    #[prop(optional)] highlighter: Option<Highlighter>,
    #[prop(into, optional, default = "240px".to_string())] height: String,
    #[prop(optional)] fill: bool,
    #[prop(optional)] gutter: bool,
    #[prop(into, optional)] diagnostics: Signal<Vec<usize>>,
) -> impl IntoView {
    let pre_ref = NodeRef::<leptos::html::Pre>::new();
    let gutter_ref = NodeRef::<leptos::html::Div>::new();
    let class = if fill {
        "musaic-code-editor fill"
    } else {
        "musaic-code-editor"
    };
    let style = (!fill).then(|| format!("height:{height}"));

    let spans = move || {
        let text = value.get();
        match highlighter {
            Some(highlight) => highlight(&text),
            None => vec![("tok-plain", text)],
        }
    };

    let line_count = move || value.get().lines().count().max(1);

    let on_scroll = move |event: web_sys::Event| {
        let area = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok());
        if let Some(area) = area {
            if let Some(pre) = pre_ref.get() {
                pre.set_scroll_top(area.scroll_top());
                pre.set_scroll_left(area.scroll_left());
            }
            if let Some(gutter) = gutter_ref.get() {
                gutter.set_scroll_top(area.scroll_top());
            }
        }
    };

    view! {
        <div class=class style=style>
            {gutter
                .then(|| {
                    view! {
                        <div class="musaic-code-gutter" node_ref=gutter_ref>
                            {move || {
                                let errors = diagnostics.get();
                                (1..=line_count())
                                    .map(|line| {
                                        let is_error = errors.contains(&line);
                                        view! {
                                            <div class="musaic-code-gutter-line" class:error=is_error>
                                                {line}
                                            </div>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </div>
                    }
                })}
            <div class="musaic-code-surface">
                <pre node_ref=pre_ref class="musaic-code-highlight" aria-hidden="true">
                    {move || {
                        spans()
                            .into_iter()
                            .map(|(class, text)| view! { <span class=class>{text}</span> })
                            .collect_view()
                    }}
                </pre>
                <textarea
                    class="musaic-code-textarea"
                    spellcheck="false"
                    prop:value=move || value.get()
                    on:input=move |event| value.set(event_target_value(&event))
                    on:scroll=on_scroll
                ></textarea>
            </div>
        </div>
    }
}
