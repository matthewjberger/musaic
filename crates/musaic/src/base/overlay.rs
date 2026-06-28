use leptos::prelude::*;

#[component]
pub fn Scrim(
    #[prop(optional)] on_dismiss: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let handle = move |_event: web_sys::MouseEvent| {
        if let Some(callback) = on_dismiss {
            callback.run(());
        }
    };
    view! {
        <div class="musaic-scrim" on:click=handle>
            {children()}
        </div>
    }
}

#[component]
pub fn Modal(open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    view! {
        <Show when=move || open.get() fallback=|| ()>
            <div class="musaic-scrim" on:click=move |_| open.set(false)>
                <div class="musaic-modal" on:click=|event| event.stop_propagation()>
                    {children()}
                </div>
            </div>
        </Show>
    }
}
