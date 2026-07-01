use leptos::prelude::*;

#[component]
pub fn StatusBar(children: Children) -> impl IntoView {
    view! { <div class="musaic-status-bar" role="status">{children()}</div> }
}

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

#[component]
pub fn StatusSpacer() -> impl IntoView {
    view! { <div class="musaic-status-spacer"></div> }
}
