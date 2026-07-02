use leptos::prelude::*;

#[component]
pub fn Toolbar(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("musaic-toolbar {class}") role="toolbar">{children()}</div> }
}

#[component]
pub fn ToolbarGroup(children: Children) -> impl IntoView {
    view! { <div class="musaic-toolbar-group">{children()}</div> }
}

#[component]
pub fn ToolbarSpacer() -> impl IntoView {
    view! { <div class="musaic-toolbar-spacer"></div> }
}

#[component]
pub fn ToolButton(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] active: Signal<bool>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] title: String,
    #[prop(optional)] on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let handle = move |event: web_sys::MouseEvent| {
        if !disabled.get_untracked()
            && let Some(callback) = on_click
        {
            callback.run(event);
        }
    };
    view! {
        <button
            class=format!("musaic-tool-button {class}")
            class:active=move || active.get()
            title=title
            aria-pressed=move || active.get().to_string()
            disabled=move || disabled.get()
            on:click=handle
        >
            {children()}
        </button>
    }
}

#[derive(Clone)]
pub struct ActivityItem {
    pub id: String,
    pub icon: String,
    pub label: String,
}

impl ActivityItem {
    pub fn new(id: impl Into<String>, icon: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            label: label.into(),
        }
    }
}

#[component]
pub fn ActivityBar(
    items: Vec<ActivityItem>,
    active: RwSignal<String>,
    #[prop(optional)] on_select: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div class="musaic-activity-bar" role="tablist">
            {items
                .into_iter()
                .map(|item| {
                    let id_click = item.id.clone();
                    let id_active = StoredValue::new(item.id.clone());
                    let is_active = move || active.get() == id_active.get_value();
                    view! {
                        <button
                            class="musaic-activity-item"
                            class:active=is_active
                            role="tab"
                            title=item.label
                            aria-selected=move || is_active().to_string()
                            on:click=move |_| {
                                active.set(id_click.clone());
                                if let Some(callback) = on_select {
                                    callback.run(id_click.clone());
                                }
                            }
                        >
                            {item.icon}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[derive(Clone, Copy)]
struct MenuBarContext(RwSignal<Option<String>>);

#[component]
pub fn MenuBar(children: Children) -> impl IntoView {
    let active = RwSignal::new(None::<String>);
    provide_context(MenuBarContext(active));
    view! {
        <div
            class="musaic-menu-bar"
            role="menubar"
            on:pointerleave=move |_| active.set(None)
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MenuBarMenu(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    children: ChildrenFn,
) -> impl IntoView {
    let active = use_context::<MenuBarContext>()
        .map(|context| context.0)
        .unwrap_or_else(|| RwSignal::new(None));
    let id = StoredValue::new(id);
    let children = StoredValue::new(children);
    let is_open = move || active.get().as_deref() == Some(id.get_value().as_str());
    let toggle = move |_: web_sys::MouseEvent| {
        let key = id.get_value();
        active.update(|current| {
            *current = if current.as_deref() == Some(key.as_str()) {
                None
            } else {
                Some(key)
            };
        });
    };
    let hover = move |_: web_sys::PointerEvent| {
        if active.get_untracked().is_some() {
            active.set(Some(id.get_value()));
        }
    };
    view! {
        <div class="musaic-menu-bar-item">
            <button
                class="musaic-menu-bar-trigger"
                class:active=is_open
                aria-haspopup="menu"
                aria-expanded=move || is_open().to_string()
                on:click=toggle
                on:pointerenter=hover
            >
                {label}
            </button>
            <Show when=is_open fallback=|| ()>
                <div class="musaic-menu-list" role="menu" on:click=move |_| active.set(None)>
                    {children.with_value(|render| render())}
                </div>
            </Show>
        </div>
    }
}
