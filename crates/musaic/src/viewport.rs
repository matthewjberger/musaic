use std::collections::HashMap;

use leptos::html;
use leptos::prelude::*;
use musaic_protocol::{CANVAS_KEY, MESSAGE_KEY, TouchPhase};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    HtmlCanvasElement, MessageEvent, MouseEvent, OffscreenCanvas, PointerEvent, ResizeObserver,
    WheelEvent, Worker, WorkerOptions, WorkerType,
};

const CLICK_DRAG_THRESHOLD: f32 = 5.0;
const MAX_RENDER_DPR: f64 = 2.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewportEvent {
    Resize {
        width: f32,
        height: f32,
    },
    PointerMove {
        x: f32,
        y: f32,
    },
    PointerButton {
        button: u8,
        pressed: bool,
    },
    Wheel {
        delta: f32,
    },
    Touch {
        id: u64,
        phase: TouchPhase,
        x: f32,
        y: f32,
    },
    Pick {
        x: f32,
        y: f32,
    },
}

#[derive(Clone)]
pub struct Bridge {
    worker: Worker,
}

impl Bridge {
    pub fn send<Message: Serialize>(&self, message: &Message) {
        let envelope = js_sys::Object::new();
        let value = serde_wasm_bindgen::to_value(message).unwrap_or(JsValue::NULL);
        let _ = js_sys::Reflect::set(&envelope, &JsValue::from_str(MESSAGE_KEY), &value);
        let _ = self.worker.post_message(&envelope);
    }

    pub fn send_with_canvas<Message: Serialize>(
        &self,
        message: &Message,
        canvas: &OffscreenCanvas,
    ) {
        let envelope = js_sys::Object::new();
        let value = serde_wasm_bindgen::to_value(message).unwrap_or(JsValue::NULL);
        let _ = js_sys::Reflect::set(&envelope, &JsValue::from_str(MESSAGE_KEY), &value);
        let _ = js_sys::Reflect::set(&envelope, &JsValue::from_str(CANVAS_KEY), canvas);
        let transfer = js_sys::Array::of1(canvas);
        let _ = self.worker.post_message_with_transfer(&envelope, &transfer);
    }
}

#[derive(Clone, Copy, Default)]
struct DragState {
    button: Option<u8>,
    last_x: f32,
    last_y: f32,
    moved: f32,
}

#[derive(Clone, Copy)]
struct TouchTrack {
    last_x: f32,
    last_y: f32,
    moved: f32,
}

pub fn webgpu_supported() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Ok(navigator) = js_sys::Reflect::get(window.as_ref(), &JsValue::from_str("navigator"))
    else {
        return false;
    };
    js_sys::Reflect::get(&navigator, &JsValue::from_str("gpu"))
        .map(|gpu| !gpu.is_undefined() && !gpu.is_null())
        .unwrap_or(false)
}

#[component]
pub fn WebGpuGate(children: ChildrenFn) -> impl IntoView {
    if webgpu_supported() {
        children().into_any()
    } else {
        view! {
            <div class="musaic-webgpu-gate">
                <div class="musaic-webgpu-gate-card">
                    <h1>"WebGPU not available"</h1>
                    <p>
                        "This app renders through WebGPU in a web worker. Open it in a browser with WebGPU and OffscreenCanvas-in-workers support (Chromium 113+, Firefox 141+)."
                    </p>
                </div>
            </div>
        }
        .into_any()
    }
}

#[component]
pub fn Loader(
    ready: RwSignal<bool>,
    #[prop(into, optional, default = "Starting the renderer…".to_string())] message: String,
) -> impl IntoView {
    view! {
        <Show when=move || !ready.get() fallback=|| ()>
            <div class="musaic-loader-overlay">
                <div class="musaic-loader-card">
                    <span class="musaic-spinner"></span>
                    {message.clone()}
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn Viewport(
    #[prop(into)] worker_url: String,
    grabbing: RwSignal<bool>,
    on_input: Callback<ViewportEvent>,
    on_connect: Callback<(Bridge, OffscreenCanvas, f32, f32)>,
    on_message: Callback<JsValue>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<html::Canvas>::new();
    let drag = StoredValue::new(DragState::default());
    let touches = StoredValue::new(HashMap::<i32, TouchTrack>::new());
    let rect_offset = StoredValue::new((0.0_f64, 0.0_f64));
    let connected = StoredValue::new(false);
    let worker_url = StoredValue::new(worker_url);

    Effect::new(move |_| {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        if connected.get_value() {
            return;
        }
        let dpr = render_dpr() as f32;
        let rect = canvas.get_bounding_client_rect();
        let width = rect.width() as f32 * dpr;
        let height = rect.height() as f32 * dpr;
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        let offscreen = canvas
            .transfer_control_to_offscreen()
            .expect("failed to transfer canvas to offscreen");
        let bridge = spawn_worker(&worker_url.get_value(), on_message);
        attach_wheel(&canvas, on_input);
        observe_resize(canvas, on_input);
        connected.set_value(true);
        on_connect.run((bridge, offscreen, width, height));
    });

    let on_pointerdown = move |event: PointerEvent| {
        if let Some(canvas) = canvas_ref.get() {
            let rect = canvas.get_bounding_client_rect();
            rect_offset.set_value((rect.left(), rect.top()));
        }
        if event.pointer_type() == "touch" {
            let id = event.pointer_id();
            touches.update_value(|map| {
                map.insert(
                    id,
                    TouchTrack {
                        last_x: event.client_x() as f32,
                        last_y: event.client_y() as f32,
                        moved: 0.0,
                    },
                );
            });
            if let Some(canvas) = canvas_ref.get() {
                let _ = canvas.set_pointer_capture(id);
            }
            let (x, y) = physical(rect_offset.get_value(), event.client_x(), event.client_y());
            on_input.run(ViewportEvent::Touch {
                id: id as u64,
                phase: TouchPhase::Started,
                x,
                y,
            });
            grabbing.set(true);
            return;
        }
        let button = event.button().max(0) as u8;
        drag.update_value(|state| {
            state.button = Some(button);
            state.last_x = event.client_x() as f32;
            state.last_y = event.client_y() as f32;
            state.moved = 0.0;
        });
        if let Some(canvas) = canvas_ref.get() {
            let _ = canvas.set_pointer_capture(event.pointer_id());
            let (x, y) = physical(rect_offset.get_value(), event.client_x(), event.client_y());
            on_input.run(ViewportEvent::PointerMove { x, y });
            on_input.run(ViewportEvent::PointerButton {
                button,
                pressed: true,
            });
        }
        grabbing.set(true);
    };

    let on_pointermove = move |event: PointerEvent| {
        if event.pointer_type() == "touch" {
            let id = event.pointer_id();
            touches.update_value(|map| {
                if let Some(track) = map.get_mut(&id) {
                    let x = event.client_x() as f32;
                    let y = event.client_y() as f32;
                    track.moved += (x - track.last_x).abs() + (y - track.last_y).abs();
                    track.last_x = x;
                    track.last_y = y;
                }
            });
            let (x, y) = physical(rect_offset.get_value(), event.client_x(), event.client_y());
            on_input.run(ViewportEvent::Touch {
                id: id as u64,
                phase: TouchPhase::Moved,
                x,
                y,
            });
            return;
        }
        drag.update_value(|state| {
            let x = event.client_x() as f32;
            let y = event.client_y() as f32;
            state.moved += (x - state.last_x).abs() + (y - state.last_y).abs();
            state.last_x = x;
            state.last_y = y;
        });
        let (x, y) = physical(rect_offset.get_value(), event.client_x(), event.client_y());
        on_input.run(ViewportEvent::PointerMove { x, y });
    };

    let on_pointerup = move |event: PointerEvent| {
        if event.pointer_type() == "touch" {
            let id = event.pointer_id();
            let (moved, count) = touches.with_value(|map| {
                (
                    map.get(&id).map(|track| track.moved).unwrap_or(0.0),
                    map.len(),
                )
            });
            touches.update_value(|map| {
                map.remove(&id);
            });
            if let Some(canvas) = canvas_ref.get() {
                let _ = canvas.release_pointer_capture(id);
            }
            let (x, y) = physical(rect_offset.get_value(), event.client_x(), event.client_y());
            on_input.run(ViewportEvent::Touch {
                id: id as u64,
                phase: TouchPhase::Ended,
                x,
                y,
            });
            if count == 1 && moved < CLICK_DRAG_THRESHOLD {
                on_input.run(ViewportEvent::Pick { x, y });
            }
            if touches.with_value(HashMap::is_empty) {
                grabbing.set(false);
            }
            return;
        }
        let (button, moved) = drag.with_value(|state| (state.button, state.moved));
        drag.update_value(|state| state.button = None);
        grabbing.set(false);
        if let Some(canvas) = canvas_ref.get() {
            let _ = canvas.release_pointer_capture(event.pointer_id());
            let (x, y) = physical(rect_offset.get_value(), event.client_x(), event.client_y());
            on_input.run(ViewportEvent::PointerButton {
                button: event.button().max(0) as u8,
                pressed: false,
            });
            if button == Some(0) && moved < CLICK_DRAG_THRESHOLD {
                on_input.run(ViewportEvent::Pick { x, y });
            }
        }
    };

    let on_pointercancel = move |event: PointerEvent| {
        if event.pointer_type() != "touch" {
            return;
        }
        let id = event.pointer_id();
        touches.update_value(|map| {
            map.remove(&id);
        });
        if let Some(canvas) = canvas_ref.get() {
            let _ = canvas.release_pointer_capture(id);
        }
        let (x, y) = physical(rect_offset.get_value(), event.client_x(), event.client_y());
        on_input.run(ViewportEvent::Touch {
            id: id as u64,
            phase: TouchPhase::Cancelled,
            x,
            y,
        });
        if touches.with_value(HashMap::is_empty) {
            grabbing.set(false);
        }
    };

    let on_contextmenu = move |event: MouseEvent| event.prevent_default();

    let canvas_class = move || {
        if grabbing.get() {
            "musaic-viewport-canvas grabbing"
        } else {
            "musaic-viewport-canvas"
        }
    };

    view! {
        <div class="musaic-viewport">
            <canvas
                id="canvas"
                node_ref=canvas_ref
                class=canvas_class
                on:pointerdown=on_pointerdown
                on:pointermove=on_pointermove
                on:pointerup=on_pointerup
                on:pointercancel=on_pointercancel
                on:contextmenu=on_contextmenu
            ></canvas>
        </div>
    }
}

fn spawn_worker(url: &str, on_message: Callback<JsValue>) -> Bridge {
    let options = WorkerOptions::new();
    options.set_type(WorkerType::Module);
    let worker = Worker::new_with_options(url, &options).expect("failed to spawn worker");
    let handler = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        let data = event.data();
        if let Ok(payload) = js_sys::Reflect::get(&data, &JsValue::from_str(MESSAGE_KEY)) {
            on_message.run(payload);
        }
    });
    worker.set_onmessage(Some(handler.as_ref().unchecked_ref()));
    handler.forget();
    Bridge { worker }
}

fn render_dpr() -> f64 {
    web_sys::window()
        .map(|window| window.device_pixel_ratio().min(MAX_RENDER_DPR))
        .unwrap_or(1.0)
}

fn physical(offset: (f64, f64), client_x: i32, client_y: i32) -> (f32, f32) {
    let dpr = render_dpr();
    (
        ((client_x as f64 - offset.0) * dpr) as f32,
        ((client_y as f64 - offset.1) * dpr) as f32,
    )
}

fn attach_wheel(canvas: &HtmlCanvasElement, on_input: Callback<ViewportEvent>) {
    let handler = Closure::<dyn FnMut(WheelEvent)>::new(move |event: WheelEvent| {
        event.prevent_default();
        on_input.run(ViewportEvent::Wheel {
            delta: event.delta_y() as f32,
        });
    });
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas
        .add_event_listener_with_callback_and_add_event_listener_options(
            "wheel",
            handler.as_ref().unchecked_ref(),
            &options,
        )
        .expect("failed to add wheel listener");
    handler.forget();
}

fn observe_resize(canvas: HtmlCanvasElement, on_input: Callback<ViewportEvent>) {
    let resize_canvas = canvas.clone();
    let handler = Closure::<dyn FnMut()>::new(move || {
        let dpr = render_dpr() as f32;
        let rect = resize_canvas.get_bounding_client_rect();
        on_input.run(ViewportEvent::Resize {
            width: rect.width() as f32 * dpr,
            height: rect.height() as f32 * dpr,
        });
    });
    let observer = ResizeObserver::new(handler.as_ref().unchecked_ref())
        .expect("failed to create resize observer");
    observer.observe(&canvas);
    handler.forget();
    std::mem::forget(observer);
}
