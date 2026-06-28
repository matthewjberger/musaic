use leptos::prelude::*;

#[component]
pub fn Menu(#[prop(into)] label: String, children: ChildrenFn) -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <div class="musaic-menu">
            <button class="musaic-button" on:click=move |_| open.update(|value| *value = !*value)>
                {label}
            </button>
            <Show when=move || open.get() fallback=|| ()>
                <div class="musaic-menu-list" on:click=move |_| open.set(false)>
                    {children()}
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn MenuItem(
    #[prop(into)] label: String,
    #[prop(into, optional)] shortcut: String,
    #[prop(optional)] on_select: Option<Callback<()>>,
) -> impl IntoView {
    let handle = move |_event: web_sys::MouseEvent| {
        if let Some(callback) = on_select {
            callback.run(());
        }
    };
    view! {
        <div class="musaic-menu-item" on:click=handle>
            <span>{label}</span>
            <span class="shortcut">{shortcut}</span>
        </div>
    }
}

#[component]
pub fn ContextMenu(
    open: RwSignal<bool>,
    #[prop(into)] x: Signal<i32>,
    #[prop(into)] y: Signal<i32>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Show when=move || open.get() fallback=|| ()>
            <div
                class="musaic-context-menu"
                style=move || format!("left:{}px;top:{}px", x.get(), y.get())
                on:click=move |_| open.set(false)
            >
                {children()}
            </div>
        </Show>
    }
}

#[component]
pub fn TabBar(tabs: Vec<(String, String)>, active: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="musaic-tab-bar">
            {tabs
                .into_iter()
                .map(|(id, label)| {
                    let id_for_class = id.clone();
                    view! {
                        <button
                            class=move || {
                                if active.get() == id_for_class {
                                    "musaic-tab active"
                                } else {
                                    "musaic-tab"
                                }
                            }
                            on:click=move |_| active.set(id.clone())
                        >
                            {label}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}
