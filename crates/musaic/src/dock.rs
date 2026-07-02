//! A resizable, optionally collapsible dock layout with a main area and edge panels.

use leptos::html;
use leptos::prelude::*;

/// Whether a `DockLayout` arranges its children horizontally or vertically.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SplitAxis {
    /// Lay children out left to right.
    Row,
    /// Lay children out top to bottom.
    Column,
}

/// Which edge a `DockPanel` sits against, which also flips its resize direction.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DockSide {
    /// The leading edge (left or top).
    Start,
    /// The trailing edge (right or bottom).
    End,
}

#[derive(Clone, Copy)]
struct DockAxis(SplitAxis);

/// The dock container. Arranges its `DockMain` and `DockPanel` children along
/// `axis` and provides that axis to them via context.
#[component]
pub fn DockLayout(
    #[prop(default = SplitAxis::Row)] axis: SplitAxis,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    provide_context(DockAxis(axis));
    let orientation = match axis {
        SplitAxis::Row => "row",
        SplitAxis::Column => "column",
    };
    view! {
        <div class=format!("musaic-dock-layout {orientation} {class}")>{children()}</div>
    }
}

/// The flexible central region of a `DockLayout` that fills the space left by the panels.
#[component]
pub fn DockMain(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("musaic-dock-main {class}")>{children()}</div> }
}

/// An edge panel with a draggable handle. `size` holds its pixel extent along the
/// layout axis and is updated as the handle is dragged, clamped between `min` and
/// `max`; `side` picks the edge and drag direction. With `collapsible` set it
/// shows a toggle in its header that drives the `collapsed` signal.
#[component]
pub fn DockPanel(
    #[prop(into, optional)] title: String,
    #[prop(default = DockSide::Start)] side: DockSide,
    size: RwSignal<f64>,
    #[prop(default = 140.0)] min: f64,
    #[prop(default = 680.0)] max: f64,
    #[prop(optional)] collapsible: bool,
    #[prop(optional)] collapsed: Option<RwSignal<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let axis = use_context::<DockAxis>()
        .map(|context| context.0)
        .unwrap_or(SplitAxis::Row);
    let horizontal = axis == SplitAxis::Row;
    let collapsed = collapsed.unwrap_or_else(|| RwSignal::new(false));
    let has_title = !title.is_empty();

    let handle_ref = NodeRef::<html::Div>::new();
    let drag = StoredValue::new(None::<(f64, f64)>);
    let invert = matches!(side, DockSide::End);

    let pointer_position = move |event: &web_sys::PointerEvent| {
        if horizontal {
            event.client_x() as f64
        } else {
            event.client_y() as f64
        }
    };

    let on_pointerdown = move |event: web_sys::PointerEvent| {
        event.prevent_default();
        drag.set_value(Some((pointer_position(&event), size.get_untracked())));
        if let Some(node) = handle_ref.get() {
            let _ = node.set_pointer_capture(event.pointer_id());
        }
    };
    let on_pointermove = move |event: web_sys::PointerEvent| {
        if let Some((start_pointer, start_size)) = drag.get_value() {
            let mut delta = pointer_position(&event) - start_pointer;
            if invert {
                delta = -delta;
            }
            size.set((start_size + delta).clamp(min, max));
        }
    };
    let on_pointerup = move |event: web_sys::PointerEvent| {
        drag.set_value(None);
        if let Some(node) = handle_ref.get() {
            let _ = node.release_pointer_capture(event.pointer_id());
        }
    };

    let axis_class = if horizontal { "row" } else { "column" };
    let side_class = match side {
        DockSide::Start => "start",
        DockSide::End => "end",
    };

    let style = move || {
        if collapsed.get() {
            "flex:0 0 auto".to_string()
        } else {
            format!("flex:0 0 {}px", size.get())
        }
    };

    let children = StoredValue::new(children);

    view! {
        <div
            class=format!("musaic-dock-panel {axis_class} {side_class}")
            class:collapsed=move || collapsed.get()
            style=style
        >
            {(has_title || collapsible)
                .then(|| {
                    view! {
                        <div class="musaic-dock-header">
                            <span class="musaic-dock-title">{title.clone()}</span>
                            {collapsible
                                .then(|| {
                                    view! {
                                        <button
                                            class="musaic-dock-collapse"
                                            aria-label="Toggle panel"
                                            aria-expanded=move || (!collapsed.get()).to_string()
                                            on:click=move |_| collapsed.update(|value| *value = !*value)
                                        >
                                            {move || if collapsed.get() { "＋" } else { "－" }}
                                        </button>
                                    }
                                })}
                        </div>
                    }
                })}
            <Show when=move || !collapsed.get() fallback=|| ()>
                <div class="musaic-dock-body">{children.with_value(|render| render())}</div>
            </Show>
            <Show when=move || !collapsed.get() fallback=|| ()>
                <div
                    node_ref=handle_ref
                    class=format!("musaic-dock-handle {axis_class} {side_class}")
                    on:pointerdown=on_pointerdown
                    on:pointermove=on_pointermove
                    on:pointerup=on_pointerup
                ></div>
            </Show>
        </div>
    }
}
