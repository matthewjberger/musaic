use leptos::prelude::*;
use wasm_bindgen::JsCast;

pub type Highlighter = fn(&str) -> Vec<(&'static str, String)>;

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
