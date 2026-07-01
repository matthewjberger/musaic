use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlDivElement, HtmlElement, KeyboardEvent};

#[component]
pub fn Overlay(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);
    view! { <Portal>{move || children.with_value(|render| render())}</Portal> }
}

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

const FOCUSABLE_SELECTOR: &str = "a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex='-1'])";

#[component]
pub fn Modal(open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    let dialog_ref = NodeRef::<html::Div>::new();
    let previously_focused = StoredValue::new_local(None::<HtmlElement>);
    let children = StoredValue::new(children);

    Effect::new(move |_| {
        if open.get() {
            previously_focused.set_value(active_element());
            if let Some(dialog) = dialog_ref.get() {
                let _ = dialog.focus();
            }
        } else if let Some(element) = previously_focused.get_value() {
            let _ = element.focus();
            previously_focused.set_value(None);
        }
    });

    on_cleanup(move || {
        if let Some(element) = previously_focused.get_value() {
            let _ = element.focus();
        }
    });

    let on_keydown = move |event: KeyboardEvent| match event.key().as_str() {
        "Escape" => {
            event.prevent_default();
            open.set(false);
        }
        "Tab" => {
            if let Some(dialog) = dialog_ref.get() {
                trap_tab(&dialog, &event);
            }
        }
        _ => {}
    };

    view! {
        <Show when=move || open.get() fallback=|| ()>
            <Overlay>
                <div class="musaic-scrim" on:click=move |_| open.set(false)>
                    <div
                        node_ref=dialog_ref
                        class="musaic-modal"
                        role="dialog"
                        aria-modal="true"
                        tabindex="-1"
                        on:click=|event| event.stop_propagation()
                        on:keydown=on_keydown
                    >
                        {children.with_value(|render| render())}
                    </div>
                </div>
            </Overlay>
        </Show>
    }
}

fn active_element() -> Option<HtmlElement> {
    web_sys::window()?
        .document()?
        .active_element()
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
}

fn trap_tab(dialog: &HtmlDivElement, event: &KeyboardEvent) {
    let Ok(list) = dialog.query_selector_all(FOCUSABLE_SELECTOR) else {
        return;
    };
    let focusable = (0..list.length())
        .filter_map(|index| list.item(index))
        .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
        .collect::<Vec<_>>();
    let (Some(first), Some(last)) = (focusable.first(), focusable.last()) else {
        return;
    };
    let active = active_element();
    if event.shift_key() {
        if active
            .as_ref()
            .is_some_and(|element| same_element(element, first))
        {
            event.prevent_default();
            let _ = last.focus();
        }
    } else if active
        .as_ref()
        .is_some_and(|element| same_element(element, last))
    {
        event.prevent_default();
        let _ = first.focus();
    }
}

fn same_element(left: &HtmlElement, right: &HtmlElement) -> bool {
    let left: &JsValue = left.as_ref();
    let right: &JsValue = right.as_ref();
    left == right
}
