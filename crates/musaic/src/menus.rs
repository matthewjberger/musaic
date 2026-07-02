//! Menu, submenu, context-menu, and tab-bar components with keyboard roving focus.

use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

fn active_element() -> Option<HtmlElement> {
    web_sys::window()?
        .document()?
        .active_element()
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
}

fn same_element(left: &HtmlElement, right: &HtmlElement) -> bool {
    let left: &wasm_bindgen::JsValue = left.as_ref();
    let right: &wasm_bindgen::JsValue = right.as_ref();
    left == right
}

fn items_within(container: &HtmlElement) -> Vec<HtmlElement> {
    let Ok(list) = container.query_selector_all(".musaic-menu-item:not([aria-disabled='true'])")
    else {
        return Vec::new();
    };
    (0..list.length())
        .filter_map(|index| list.item(index))
        .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
        .collect()
}

fn roving_focus(container: &HtmlElement, event: &KeyboardEvent) -> bool {
    let items = items_within(container);
    if items.is_empty() {
        return false;
    }
    let current = active_element()
        .and_then(|active| items.iter().position(|item| same_element(item, &active)));
    match event.key().as_str() {
        "ArrowDown" => {
            event.prevent_default();
            let next = current.map(|index| (index + 1) % items.len()).unwrap_or(0);
            let _ = items[next].focus();
            true
        }
        "ArrowUp" => {
            event.prevent_default();
            let next = current
                .map(|index| (index + items.len() - 1) % items.len())
                .unwrap_or(items.len() - 1);
            let _ = items[next].focus();
            true
        }
        "Home" => {
            event.prevent_default();
            let _ = items[0].focus();
            true
        }
        "End" => {
            event.prevent_default();
            let _ = items[items.len() - 1].focus();
            true
        }
        _ => false,
    }
}

/// A `label` button that toggles a dropdown of its `children`, closing on outside click or Escape and moving focus with the arrow, Home, and End keys.
#[component]
pub fn Menu(#[prop(into)] label: String, children: ChildrenFn) -> impl IntoView {
    let open = RwSignal::new(false);
    let root = NodeRef::<html::Div>::new();
    let list_ref = NodeRef::<html::Div>::new();
    let children = StoredValue::new(children);

    let handle = window_event_listener(leptos::ev::mousedown, move |event| {
        if !open.get_untracked() {
            return;
        }
        let inside = root.get_untracked().is_some_and(|node| {
            event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
                .is_some_and(|target| node.contains(Some(&target)))
        });
        if !inside {
            open.set(false);
        }
    });
    on_cleanup(move || handle.remove());

    Effect::new(move |_| {
        if open.get()
            && let Some(list) = list_ref.get()
            && let Some(first) = items_within(&list).into_iter().next()
        {
            let _ = first.focus();
        }
    });

    let on_keydown = move |event: KeyboardEvent| {
        if event.key() == "Escape" {
            event.prevent_default();
            open.set(false);
            return;
        }
        if let Some(list) = list_ref.get() {
            roving_focus(&list, &event);
        }
    };

    view! {
        <div class="musaic-menu" node_ref=root>
            <button
                class="musaic-button"
                aria-haspopup="menu"
                aria-expanded=move || open.get().to_string()
                on:click=move |_| open.update(|value| *value = !*value)
            >
                {label}
            </button>
            <Show when=move || open.get() fallback=|| ()>
                <div
                    class="musaic-menu-list"
                    role="menu"
                    node_ref=list_ref
                    on:keydown=on_keydown
                    on:click=move |_| open.set(false)
                >
                    {children.with_value(|render| render())}
                </div>
            </Show>
        </div>
    }
}

/// A single selectable menu row that fires `on_select` on click or Enter/Space.
///
/// Passing a `checked` signal renders it as a checkbox item; `disabled` blocks
/// selection, and `shortcut` shows a trailing key hint.
#[component]
pub fn MenuItem(
    #[prop(into)] label: String,
    #[prop(into, optional)] shortcut: String,
    #[prop(optional)] checked: Option<Signal<bool>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] on_select: Option<Callback<()>>,
) -> impl IntoView {
    let role = if checked.is_some() {
        "menuitemcheckbox"
    } else {
        "menuitem"
    };
    let handle = move |_event: web_sys::MouseEvent| {
        if disabled.get_untracked() {
            return;
        }
        if let Some(callback) = on_select {
            callback.run(());
        }
    };
    let on_keydown = move |event: KeyboardEvent| {
        if matches!(event.key().as_str(), "Enter" | " ")
            && !disabled.get_untracked()
            && let Some(callback) = on_select
        {
            event.prevent_default();
            callback.run(());
        }
    };
    view! {
        <div
            class="musaic-menu-item"
            role=role
            tabindex="-1"
            aria-disabled=move || disabled.get().to_string()
            aria-checked=checked.map(|checked| move || checked.get().to_string())
            on:click=handle
            on:keydown=on_keydown
        >
            {checked
                .map(|checked| {
                    view! {
                        <span class="musaic-menu-check" class:on=move || checked.get()>
                            "\u{2713}"
                        </span>
                    }
                })}
            <span class="musaic-menu-label">{label}</span>
            <span class="shortcut">{shortcut}</span>
        </div>
    }
}

/// A horizontal divider rendered between groups of menu items.
#[component]
pub fn MenuSeparator() -> impl IntoView {
    view! { <div class="musaic-menu-separator" role="separator"></div> }
}

/// A nested menu row that reveals its `children` in a flyout on hover or click.
#[component]
pub fn Submenu(#[prop(into)] label: String, children: ChildrenFn) -> impl IntoView {
    let open = RwSignal::new(false);
    let children = StoredValue::new(children);
    view! {
        <div
            class="musaic-submenu"
            on:mouseenter=move |_| open.set(true)
            on:mouseleave=move |_| open.set(false)
        >
            <div
                class="musaic-menu-item"
                role="menuitem"
                tabindex="-1"
                aria-haspopup="menu"
                aria-expanded=move || open.get().to_string()
                on:click=move |event| {
                    event.stop_propagation();
                    open.update(|value| *value = !*value);
                }
            >
                <span class="musaic-menu-label">{label}</span>
                <span class="musaic-menu-arrow">"\u{203a}"</span>
            </div>
            <Show when=move || open.get() fallback=|| ()>
                <div class="musaic-menu-list submenu" role="menu">
                    {children.with_value(|render| render())}
                </div>
            </Show>
        </div>
    }
}

/// A menu portaled to the document at position (`x`, `y`), shown while `open` is `true`.
///
/// Clicking the backdrop or pressing Escape closes it; arrow, Home, and End
/// keys move focus across its `children`.
#[component]
pub fn ContextMenu(
    open: RwSignal<bool>,
    #[prop(into)] x: Signal<i32>,
    #[prop(into)] y: Signal<i32>,
    children: ChildrenFn,
) -> impl IntoView {
    let list_ref = NodeRef::<html::Div>::new();
    let children = StoredValue::new(children);

    Effect::new(move |_| {
        if open.get()
            && let Some(list) = list_ref.get()
            && let Some(first) = items_within(&list).into_iter().next()
        {
            let _ = first.focus();
        }
    });

    let on_keydown = move |event: KeyboardEvent| {
        if event.key() == "Escape" {
            event.prevent_default();
            open.set(false);
            return;
        }
        if let Some(list) = list_ref.get() {
            roving_focus(&list, &event);
        }
    };

    view! {
        <Show when=move || open.get() fallback=|| ()>
            <Portal>
                <div class="musaic-context-scrim" on:mousedown=move |_| open.set(false)>
                    <div
                        class="musaic-context-menu"
                        role="menu"
                        node_ref=list_ref
                        style=move || format!("left:{}px;top:{}px", x.get(), y.get())
                        on:mousedown=|event| event.stop_propagation()
                        on:keydown=on_keydown
                        on:click=move |_| open.set(false)
                    >
                        {children.with_value(|render| render())}
                    </div>
                </div>
            </Portal>
        </Show>
    }
}

/// A horizontal row of tabs from `(id, label)` pairs, binding the selected id to `active` and cycling selection with the left and right arrow keys.
#[component]
pub fn TabBar(tabs: Vec<(String, String)>, active: RwSignal<String>) -> impl IntoView {
    let ids: Vec<String> = tabs.iter().map(|(id, _)| id.clone()).collect();
    let on_keydown = move |event: KeyboardEvent| {
        let current = ids.iter().position(|id| id == &active.get_untracked());
        let Some(current) = current else {
            return;
        };
        match event.key().as_str() {
            "ArrowRight" => {
                event.prevent_default();
                active.set(ids[(current + 1) % ids.len()].clone());
            }
            "ArrowLeft" => {
                event.prevent_default();
                active.set(ids[(current + ids.len() - 1) % ids.len()].clone());
            }
            _ => {}
        }
    };
    view! {
        <div class="musaic-tab-bar" role="tablist" on:keydown=on_keydown>
            {tabs
                .into_iter()
                .map(|(id, label)| {
                    let id_for_class = id.clone();
                    let id_for_aria = id.clone();
                    let id_for_tabindex = id.clone();
                    view! {
                        <button
                            role="tab"
                            aria-selected=move || (active.get() == id_for_aria).to_string()
                            tabindex=move || {
                                if active.get() == id_for_tabindex { "0" } else { "-1" }
                            }
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
