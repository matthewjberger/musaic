use leptos::prelude::*;

#[component]
pub fn Button(
    #[prop(into, optional)] class: String,
    #[prop(optional)] on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let handle = move |event: web_sys::MouseEvent| {
        if let Some(callback) = on_click {
            callback.run(event);
        }
    };
    view! {
        <button class=format!("musaic-button {class}") on:click=handle>
            {children()}
        </button>
    }
}

#[component]
pub fn IconButton(
    #[prop(into, optional)] class: String,
    #[prop(optional)] on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let handle = move |event: web_sys::MouseEvent| {
        if let Some(callback) = on_click {
            callback.run(event);
        }
    };
    view! {
        <button class=format!("musaic-icon-button {class}") on:click=handle>
            {children()}
        </button>
    }
}
