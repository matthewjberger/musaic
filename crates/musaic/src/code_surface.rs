//! Read-only, virtualized code viewer with brace-based code folding.

use std::collections::{HashMap, HashSet};

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::visible_range;

use crate::code_editor::Highlighter;

fn fold_regions(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut stack: Vec<usize> = Vec::new();
    let mut regions = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        for character in line.chars() {
            if character == '{' {
                stack.push(index);
            } else if character == '}'
                && let Some(start) = stack.pop()
                && index > start
            {
                regions.push((start, index));
            }
        }
    }
    regions
}

/// A read-only, virtualized code view of `value`: renders only the rows in the
/// visible window (plus `overscan`) at a fixed `line_height` within a scrollable
/// area of the given `height`, applies the optional `highlighter`, and lets
/// brace-delimited regions be folded from the gutter.
#[component]
pub fn CodeSurface(
    #[prop(into)] value: Signal<String>,
    #[prop(optional)] highlighter: Option<Highlighter>,
    #[prop(default = 20.0)] line_height: f64,
    #[prop(default = 420.0)] height: f64,
    #[prop(default = 20)] overscan: usize,
) -> impl IntoView {
    let folds = RwSignal::new(HashSet::<usize>::new());
    let scroll_top = RwSignal::new(0.0);
    let viewport_height = RwSignal::new(height);
    let wrap_ref = NodeRef::<html::Div>::new();

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

    let model = Memo::new(move |_| {
        let source = value.get();
        let lines: Vec<String> = source.split('\n').map(str::to_string).collect();
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        let regions = fold_regions(&refs);
        (lines, regions)
    });

    let body = move || {
        let folded = folds.get();
        let view_height = viewport_height.get().max(line_height);
        let scroll = scroll_top.get();
        model.with(|(lines, regions)| {
        let mut hidden = HashSet::new();
        for (start, end) in regions {
            if folded.contains(start) {
                for line in (start + 1)..=*end {
                    hidden.insert(line);
                }
            }
        }
        let headers: HashMap<usize, usize> = regions.iter().copied().collect();
        let visible: Vec<usize> = (0..lines.len())
            .filter(|line| !hidden.contains(line))
            .collect();
        let total = visible.len();
        let (start, end) = visible_range(scroll, view_height, line_height, overscan, total);
        let top_pad = start as f64 * line_height;
        let bottom_pad = total.saturating_sub(end) as f64 * line_height;

        let rows = visible[start..end]
            .iter()
            .map(|&line_index| {
                let text = lines[line_index].clone();
                let is_header = headers.contains_key(&line_index);
                let is_folded = folded.contains(&line_index);
                let spans = match highlighter {
                    Some(highlight) => highlight(&text),
                    None => vec![("tok-plain", text.clone())],
                };
                let toggle = move |_: web_sys::MouseEvent| {
                    folds.update(|set| {
                        if !set.remove(&line_index) {
                            set.insert(line_index);
                        }
                    });
                };
                view! {
                    <div class="musaic-surface-row" style=format!("height:{line_height}px")>
                        <span class="musaic-surface-num">{line_index + 1}</span>
                        {if is_header {
                            view! {
                                <span
                                    class="musaic-surface-fold"
                                    class:folded=is_folded
                                    on:click=toggle
                                >
                                    "\u{25be}"
                                </span>
                            }
                                .into_any()
                        } else {
                            view! { <span class="musaic-surface-fold-spacer"></span> }.into_any()
                        }}
                        <span class="musaic-surface-text">
                            {spans
                                .into_iter()
                                .map(|(class, text)| view! { <span class=class>{text}</span> })
                                .collect_view()}
                            {is_folded.then(|| view! { <span class="musaic-surface-ellipsis">"\u{2026}"</span> })}
                        </span>
                    </div>
                }
            })
            .collect_view();

        view! {
            <div style=format!("height:{top_pad}px")></div>
            {rows}
            <div style=format!("height:{bottom_pad}px")></div>
        }
        })
    };

    view! {
        <div
            class="musaic-code-surface-view"
            node_ref=wrap_ref
            style=format!("height:{height}px")
            on:scroll=on_scroll
        >
            {body}
        </div>
    }
}
