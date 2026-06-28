use leptos::html;
use leptos::prelude::*;

#[component]
pub fn AppShell(children: Children) -> impl IntoView {
    view! { <div class="musaic-app-shell">{children()}</div> }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResizeAxis {
    Horizontal,
    Vertical,
}

#[component]
pub fn ResizeHandle(
    value: RwSignal<f64>,
    axis: ResizeAxis,
    #[prop(default = 120.0)] min: f64,
    #[prop(default = 2000.0)] max: f64,
    #[prop(optional)] invert: bool,
) -> impl IntoView {
    let handle_ref = NodeRef::<html::Div>::new();
    let drag = StoredValue::new(None::<(f64, f64)>);

    let pointer_position = move |event: &web_sys::PointerEvent| match axis {
        ResizeAxis::Horizontal => event.client_x() as f64,
        ResizeAxis::Vertical => event.client_y() as f64,
    };

    let on_pointerdown = move |event: web_sys::PointerEvent| {
        event.prevent_default();
        drag.set_value(Some((pointer_position(&event), value.get_untracked())));
        if let Some(node) = handle_ref.get() {
            let _ = node.set_pointer_capture(event.pointer_id());
        }
    };

    let on_pointermove = move |event: web_sys::PointerEvent| {
        if let Some((start_pointer, start_value)) = drag.get_value() {
            let mut delta = pointer_position(&event) - start_pointer;
            if invert {
                delta = -delta;
            }
            value.set((start_value + delta).clamp(min, max));
        }
    };

    let on_pointerup = move |event: web_sys::PointerEvent| {
        drag.set_value(None);
        if let Some(node) = handle_ref.get() {
            let _ = node.release_pointer_capture(event.pointer_id());
        }
    };

    let class = match axis {
        ResizeAxis::Horizontal => "musaic-resize-handle horizontal",
        ResizeAxis::Vertical => "musaic-resize-handle vertical",
    };

    view! {
        <div
            node_ref=handle_ref
            class=class
            on:pointerdown=on_pointerdown
            on:pointermove=on_pointermove
            on:pointerup=on_pointerup
        ></div>
    }
}

#[component]
pub fn Row(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("musaic-row {class}")>{children()}</div> }
}

#[component]
pub fn Column(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("musaic-column {class}")>{children()}</div> }
}

#[component]
pub fn Grid(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("musaic-grid {class}")>{children()}</div> }
}
