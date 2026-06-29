use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(into, optional)] title: String,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let header =
        (!title.is_empty()).then(|| view! { <div class="musaic-card-title">{title.clone()}</div> });
    view! {
        <div class=format!("musaic-card {class}")>
            {header}
            <div class="musaic-card-body">{children()}</div>
        </div>
    }
}
