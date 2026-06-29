use leptos::prelude::*;

#[component]
pub fn Badge(#[prop(into, optional)] variant: String, children: Children) -> impl IntoView {
    view! { <span class=format!("musaic-badge {variant}")>{children()}</span> }
}
