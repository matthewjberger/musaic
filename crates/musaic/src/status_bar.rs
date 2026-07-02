//! A horizontal status strip built from small labelled cells, like an editor's bottom bar.

use leptos::prelude::*;

/// The status strip container. Renders a `role="status"` row and lays out its
/// `StatusItem` and `StatusSpacer` children left to right.
#[component]
pub fn StatusBar(children: Children) -> impl IntoView {
    view! { <div class="musaic-status-bar" role="status">{children()}</div> }
}

/// A single cell in the status bar. Shows an optional leading `icon` before the
/// children, and forwards an extra `class` for styling.
#[component]
pub fn StatusItem(
    #[prop(into, optional)] icon: String,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let has_icon = !icon.is_empty();
    view! {
        <div class=format!("musaic-status-item {class}")>
            {has_icon.then(|| view! { <span class="musaic-status-icon">{icon}</span> })}
            {children()}
        </div>
    }
}

/// A flexible gap that pushes the items after it to the far end of the bar.
#[component]
pub fn StatusSpacer() -> impl IntoView {
    view! { <div class="musaic-status-spacer"></div> }
}
