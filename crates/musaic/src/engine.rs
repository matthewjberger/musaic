//! A higher-level wrapper over `Viewport` that manages a render worker connection,
//! queues messages until it is ready, and exposes reactive render state.

use crate::protocol::{FromWorker, SelectedEntity, ToWorker};
use leptos::prelude::*;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};

use crate::viewport::{Bridge, Viewport, ViewportEvent};

/// Reactive signals reflecting the render worker's status, kept in sync from the
/// messages it sends back.
#[derive(Clone, Copy)]
pub struct EngineState {
    /// Whether the worker has reported that it is ready.
    pub ready: RwSignal<bool>,
    /// A human-readable name of the graphics adapter in use.
    pub adapter: RwSignal<String>,
    /// The most recent frames-per-second reading.
    pub fps: RwSignal<f32>,
    /// The current number of entities in the scene.
    pub entity_count: RwSignal<u32>,
    /// The currently selected entity, if any.
    pub selected: RwSignal<Option<SelectedEntity>>,
    /// Whether a pointer drag is in progress over the surface.
    pub grabbing: RwSignal<bool>,
}

impl EngineState {
    fn new() -> Self {
        Self {
            ready: RwSignal::new(false),
            adapter: RwSignal::new(String::new()),
            fps: RwSignal::new(0.0),
            entity_count: RwSignal::new(0),
            selected: RwSignal::new(None),
            grabbing: RwSignal::new(false),
        }
    }
}

/// A `Copy` handle to the render worker connection. Holds the reactive `state`,
/// buffers outgoing messages until the worker connects, and routes custom messages
/// to a registered handler.
#[derive(Clone, Copy)]
pub struct Engine {
    /// The reactive render state exposed to the UI.
    pub state: EngineState,
    bridge: StoredValue<Option<Bridge>, LocalStorage>,
    custom_handler: StoredValue<Option<Callback<serde_json::Value>>, LocalStorage>,
    queue: StoredValue<Vec<ToWorker>, LocalStorage>,
    worker_url: StoredValue<String>,
}

impl Engine {
    /// Sends an application-defined message to the worker as a custom payload,
    /// queuing it if the worker is not yet connected.
    pub fn send<Message: Serialize>(&self, message: &Message) {
        if let Ok(value) = serde_json::to_value(message) {
            self.dispatch(ToWorker::Custom(value));
        }
    }

    /// Registers the handler invoked for each custom message received from the worker.
    pub fn on_custom(&self, handler: Callback<serde_json::Value>) {
        self.custom_handler.set_value(Some(handler));
    }

    fn dispatch(&self, message: ToWorker) {
        if let Some(bridge) = self.bridge.get_value() {
            bridge.send(&message);
        } else {
            self.queue.update_value(|queue| queue.push(message));
        }
    }

    fn connect(&self, bridge: Bridge, canvas: &web_sys::OffscreenCanvas, width: f32, height: f32) {
        bridge.send_with_canvas(&ToWorker::Init { width, height }, canvas);
        for message in self.queue.get_value() {
            bridge.send(&message);
        }
        self.queue.set_value(Vec::new());
        self.bridge.set_value(Some(bridge));
    }
}

/// Creates an `Engine` for the worker at `worker_url` and installs global key
/// listeners that forward keystrokes to the worker, skipping modifier combos and
/// keys pressed while typing in a form field. Pair it with `EngineViewport`.
pub fn use_engine(worker_url: impl Into<String>) -> Engine {
    let engine = Engine {
        state: EngineState::new(),
        bridge: StoredValue::new_local(None),
        custom_handler: StoredValue::new_local(None),
        queue: StoredValue::new_local(Vec::new()),
        worker_url: StoredValue::new(worker_url.into()),
    };

    let _ = window_event_listener(leptos::ev::keydown, move |event| {
        if event.ctrl_key() || event.meta_key() || typing_in_field(&event) {
            return;
        }
        let text = (event.key().chars().count() == 1).then(|| event.key());
        engine.dispatch(ToWorker::Key {
            code: event.code(),
            pressed: true,
            text,
        });
    });
    let _ = window_event_listener(leptos::ev::keyup, move |event| {
        if event.ctrl_key() || event.meta_key() || typing_in_field(&event) {
            return;
        }
        engine.dispatch(ToWorker::Key {
            code: event.code(),
            pressed: false,
            text: None,
        });
    });

    engine
}

/// Renders the `Viewport` wired to `engine`: it translates `ViewportEvent`s into
/// worker messages, completes the connection on connect, and updates the engine
/// `state` (and any custom handler) from messages the worker sends back.
#[component]
pub fn EngineViewport(engine: Engine) -> impl IntoView {
    let state = engine.state;
    let custom_handler = engine.custom_handler;

    let on_input = Callback::new(move |event: ViewportEvent| {
        let message = match event {
            ViewportEvent::Resize { width, height } => ToWorker::Resize { width, height },
            ViewportEvent::PointerMove { x, y } => ToWorker::PointerMove { x, y },
            ViewportEvent::PointerButton { button, pressed } => {
                ToWorker::PointerButton { button, pressed }
            }
            ViewportEvent::Wheel { delta } => ToWorker::Wheel { delta },
            ViewportEvent::Touch { id, phase, x, y } => ToWorker::Touch { id, phase, x, y },
            ViewportEvent::Pick { x, y } => ToWorker::Pick { x, y },
        };
        engine.dispatch(message);
    });

    let on_connect = Callback::new(
        move |(connected, canvas, width, height): (Bridge, web_sys::OffscreenCanvas, f32, f32)| {
            engine.connect(connected, &canvas, width, height);
        },
    );

    let on_message = Callback::new(move |payload: JsValue| {
        let Ok(message) = serde_wasm_bindgen::from_value::<FromWorker>(payload) else {
            return;
        };
        match message {
            FromWorker::Ready { adapter } => {
                state.adapter.set(adapter);
                state.ready.set(true);
            }
            FromWorker::Stats { fps, entity_count } => {
                state.fps.set(fps);
                state.entity_count.set(entity_count);
            }
            FromWorker::Selected { detail } => state.selected.set(detail),
            FromWorker::Custom(value) => {
                if let Some(handler) = custom_handler.get_value() {
                    handler.run(value);
                }
            }
        }
    });

    view! {
        <Viewport
            worker_url=engine.worker_url.get_value()
            grabbing=state.grabbing
            on_input=on_input
            on_connect=on_connect
            on_message=on_message
        />
    }
}

fn typing_in_field(event: &web_sys::KeyboardEvent) -> bool {
    event
        .target()
        .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
        .map(|element| {
            let tag = element.tag_name();
            tag.eq_ignore_ascii_case("input")
                || tag.eq_ignore_ascii_case("textarea")
                || tag.eq_ignore_ascii_case("select")
                || element.is_content_editable()
        })
        .unwrap_or(false)
}
