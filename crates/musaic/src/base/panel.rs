use leptos::prelude::*;

#[component]
pub fn Panel(
    #[prop(into, optional)] title: String,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let header = (!title.is_empty())
        .then(|| view! { <div class="musaic-panel-title">{title.clone()}</div> });
    view! {
        <div class=format!("musaic-panel {class}")>
            {header}
            <div class="musaic-panel-body">{children()}</div>
        </div>
    }
}
