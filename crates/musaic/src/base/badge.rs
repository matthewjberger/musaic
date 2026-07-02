use leptos::prelude::*;

/// A small inline label. Pass a `variant` class (such as `"success"` or
/// `"danger"`) to color it and put the label text in `children`.
#[component]
pub fn Badge(#[prop(into, optional)] variant: String, children: Children) -> impl IntoView {
    view! { <span class=format!("musaic-badge {variant}")>{children()}</span> }
}
