//! Pointer-based drag and drop that works inside webviews where HTML5 DnD does not fire.
//! `provide_drag()` at the root, then wrap items in `DragSource` and targets in `DropZone`,
//! and render one `DragLayer`.

use std::collections::HashMap;

use leptos::prelude::*;

/// What is being dragged: a `kind` used by drop zones to accept or reject it, an
/// `id` identifying the source, and a `label` shown in the drag preview.
#[derive(Clone)]
pub struct DragPayload {
    /// Category of the dragged item, matched against by drop zones.
    pub kind: String,
    /// Identifier of the dragged item.
    pub id: String,
    /// Text shown in the floating drag preview.
    pub label: String,
}

impl DragPayload {
    /// Creates a payload from a kind, id, and label.
    pub fn new(kind: impl Into<String>, id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            id: id.into(),
            label: label.into(),
        }
    }
}

const DRAG_THRESHOLD: f64 = 4.0;

/// The shared drag session, provided via context by `provide_drag`. Tracks the
/// active payload, the pointer position, the hovered drop zone, and the registry
/// of drop callbacks. `Copy`, so it can be captured in event handlers.
#[derive(Clone, Copy)]
pub struct DragState {
    payload: RwSignal<Option<DragPayload>>,
    position: RwSignal<(f64, f64)>,
    over: RwSignal<Option<String>>,
    zones: StoredValue<HashMap<String, Callback<DragPayload>>>,
    pending: StoredValue<Option<(DragPayload, f64, f64)>>,
}

impl DragState {
    /// Reactively reports whether a drag is currently in progress.
    pub fn active(&self) -> bool {
        self.payload.get().is_some()
    }

    /// Arms a potential drag at a start position; it begins once the pointer moves past the threshold.
    pub fn arm(&self, payload: DragPayload, x: f64, y: f64) {
        self.pending.set_value(Some((payload, x, y)));
    }

    /// Reactively reads the payload of the active drag, if any.
    pub fn payload(&self) -> Option<DragPayload> {
        self.payload.get()
    }

    /// Reactively reads the current pointer position.
    pub fn position(&self) -> (f64, f64) {
        self.position.get()
    }

    /// Reactively reads the id of the drop zone currently under the pointer, if any.
    pub fn over(&self) -> Option<String> {
        self.over.get()
    }

    /// Begins an active drag at the given position with the given payload.
    pub fn start(&self, payload: DragPayload, x: f64, y: f64) {
        self.position.set((x, y));
        self.payload.set(Some(payload));
    }

    /// Marks the drop zone `id` as hovered while a drag is active.
    pub fn set_over(&self, id: String) {
        if self.payload.get_untracked().is_some() {
            self.over.set(Some(id));
        }
    }

    /// Clears the hovered drop zone if it is currently `id`.
    pub fn clear_over(&self, id: &str) {
        if self.over.get_untracked().as_deref() == Some(id) {
            self.over.set(None);
        }
    }

    fn register(&self, id: String, on_drop: Callback<DragPayload>) {
        self.zones.update_value(|zones| {
            zones.insert(id, on_drop);
        });
    }

    fn unregister(&self, id: &str) {
        self.zones.update_value(|zones| {
            zones.remove(id);
        });
    }

    fn finish(&self) {
        let payload = self.payload.get_untracked();
        let over = self.over.get_untracked();
        if let (Some(payload), Some(over)) = (payload, over) {
            let callback = self.zones.with_value(|zones| zones.get(&over).copied());
            if let Some(callback) = callback {
                callback.run(payload);
            }
        }
        self.payload.set(None);
        self.over.set(None);
    }
}

/// Creates the drag session, provides it via context, and installs the window
/// pointer listeners that promote an armed drag to active, track the pointer, and
/// commit the drop on release. Call once near the root; returns the `DragState`.
pub fn provide_drag() -> DragState {
    let state = DragState {
        payload: RwSignal::new(None),
        position: RwSignal::new((0.0, 0.0)),
        over: RwSignal::new(None),
        zones: StoredValue::new(HashMap::new()),
        pending: StoredValue::new(None),
    };
    provide_context(state);
    let move_handle = window_event_listener(
        leptos::ev::pointermove,
        move |event: web_sys::PointerEvent| {
            let x = event.client_x() as f64;
            let y = event.client_y() as f64;
            if state.payload.get_untracked().is_some() {
                state.position.set((x, y));
            } else if let Some((payload, start_x, start_y)) = state.pending.get_value()
                && (x - start_x).hypot(y - start_y) > DRAG_THRESHOLD
            {
                state.pending.set_value(None);
                state.start(payload, x, y);
            }
        },
    );
    let up_handle = window_event_listener(leptos::ev::pointerup, move |_| {
        state.pending.set_value(None);
        if state.payload.get_untracked().is_some() {
            state.finish();
        }
    });
    on_cleanup(move || {
        move_handle.remove();
        up_handle.remove();
    });
    state
}

/// Reads the drag session from context. Returns a detached, no-op `DragState` if
/// `provide_drag` was never called.
pub fn use_drag() -> DragState {
    use_context::<DragState>().unwrap_or_else(|| DragState {
        payload: RwSignal::new(None),
        position: RwSignal::new((0.0, 0.0)),
        over: RwSignal::new(None),
        zones: StoredValue::new(HashMap::new()),
        pending: StoredValue::new(None),
    })
}

/// Wraps children as a draggable item. Arms a drag with a `DragPayload` built
/// from `kind`, `id`, and `label` on pointer down; the drag starts once the
/// pointer moves past the threshold.
#[component]
pub fn DragSource(
    #[prop(into)] kind: String,
    #[prop(into)] id: String,
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let drag = use_drag();
    let payload = StoredValue::new(DragPayload::new(kind, id, label));
    view! {
        <div
            class=format!("musaic-drag-source {class}")
            on:pointerdown=move |event: web_sys::PointerEvent| {
                drag.arm(payload.get_value(), event.client_x() as f64, event.client_y() as f64);
            }
        >
            {children()}
        </div>
    }
}

/// Wraps children as a drop target registered under `id`. Highlights while a
/// drag hovers it and runs `on_drop` with the dropped `DragPayload` when a drag
/// is released over it.
#[component]
pub fn DropZone(
    #[prop(into)] id: String,
    on_drop: Callback<DragPayload>,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let drag = use_drag();
    let id = StoredValue::new(id);
    drag.register(id.get_value(), on_drop);
    on_cleanup(move || drag.unregister(&id.get_value()));
    let is_over = move || drag.over().as_deref() == Some(id.get_value().as_str());
    view! {
        <div
            class=format!("musaic-drop-zone {class}")
            class:over=is_over
            on:pointerenter=move |_| drag.set_over(id.get_value())
            on:pointermove=move |_| drag.set_over(id.get_value())
            on:pointerleave=move |_| drag.clear_over(&id.get_value())
        >
            {children()}
        </div>
    }
}

/// Renders the floating drag preview that follows the pointer while a drag is
/// active, showing the payload's label. Render one of these near the root.
#[component]
pub fn DragLayer() -> impl IntoView {
    let drag = use_drag();
    view! {
        <Show when=move || drag.active() fallback=|| ()>
            <div
                class="musaic-drag-preview"
                style=move || {
                    let (x, y) = drag.position();
                    format!("left:{}px;top:{}px", x + 10.0, y + 10.0)
                }
            >
                {move || drag.payload().map(|payload| payload.label)}
            </div>
        </Show>
    }
}
