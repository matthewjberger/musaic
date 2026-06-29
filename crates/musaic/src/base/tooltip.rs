use leptos::prelude::*;

#[component]
pub fn Tooltip(#[prop(into)] text: String, children: Children) -> impl IntoView {
    view! {
        <span class="musaic-tooltip" tabindex="0">
            {children()}
            <span class="musaic-tooltip-bubble" role="tooltip">
                {text}
            </span>
        </span>
    }
}
