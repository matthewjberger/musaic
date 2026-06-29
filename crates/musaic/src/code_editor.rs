use leptos::prelude::*;
use wasm_bindgen::JsCast;

pub type Highlighter = fn(&str) -> Vec<(&'static str, String)>;

#[component]
pub fn CodeEditor(
    value: RwSignal<String>,
    #[prop(optional)] highlighter: Option<Highlighter>,
    #[prop(into, optional, default = "240px".to_string())] height: String,
    #[prop(optional)] fill: bool,
) -> impl IntoView {
    let pre_ref = NodeRef::<leptos::html::Pre>::new();
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

    let on_scroll = move |event: web_sys::Event| {
        let area = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok());
        if let (Some(area), Some(pre)) = (area, pre_ref.get()) {
            pre.set_scroll_top(area.scroll_top());
            pre.set_scroll_left(area.scroll_left());
        }
    };

    view! {
        <div class=class style=style>
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
    }
}
