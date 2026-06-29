use std::cell::RefCell;
use std::rc::Rc;

use leptos_musaic_protocol::{CANVAS_KEY, FromWorker, MESSAGE_KEY, SelectedEntity, ToWorker};
use nightshade::prelude::*;
use nightshade::render::wgpu::create_wgpu_renderer;
use nightshade_api::prelude::entity_under_cursor;
use serde::Serialize;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent, OffscreenCanvas};

type SetupFn<Scene> = Box<dyn FnOnce(&mut Scene, &mut World)>;
type TickFn<Scene> = Box<dyn FnMut(&mut Scene, &mut World)>;
type CustomFn<Scene> = Box<dyn FnMut(&mut Scene, &mut World, Option<Entity>, Value)>;

struct Driver<Scene> {
    scene: Scene,
    selected: Option<Entity>,
    setup: Option<SetupFn<Scene>>,
    tick: TickFn<Scene>,
}

impl<Scene> State for Driver<Scene> {
    fn initialize(&mut self, world: &mut World) {
        if let Some(setup) = self.setup.take() {
            setup(&mut self.scene, world);
        }
    }

    fn run_systems(&mut self, world: &mut World) {
        camera_controllers_system(world);
        (self.tick)(&mut self.scene, world);
    }
}

struct App<Scene> {
    world: World,
    renderer: WgpuRenderer,
    driver: Driver<Scene>,
    on_custom: CustomFn<Scene>,
}

type AppSlot<Scene> = Rc<RefCell<Option<App<Scene>>>>;
type Pending<Scene> = Rc<RefCell<Option<(Driver<Scene>, CustomFn<Scene>)>>>;
type PendingMessages = Rc<RefCell<Vec<JsValue>>>;

pub fn run_offscreen<Scene, Setup, Tick, OnCustom>(
    scene: Scene,
    setup: Setup,
    tick: Tick,
    on_custom: OnCustom,
) where
    Scene: 'static,
    Setup: FnOnce(&mut Scene, &mut World) + 'static,
    Tick: FnMut(&mut Scene, &mut World) + 'static,
    OnCustom: FnMut(&mut Scene, &mut World, Option<Entity>, Value) + 'static,
{
    console_error_panic_hook::set_once();

    let scope: DedicatedWorkerGlobalScope = js_sys::global().unchecked_into();
    let app_slot: AppSlot<Scene> = Rc::new(RefCell::new(None));
    let messages: PendingMessages = Rc::new(RefCell::new(Vec::new()));
    let pending: Pending<Scene> = Rc::new(RefCell::new(Some((
        Driver {
            scene,
            selected: None,
            setup: Some(Box::new(setup)),
            tick: Box::new(tick),
        },
        Box::new(on_custom) as CustomFn<Scene>,
    ))));

    let handler_scope = scope.clone();
    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        handle_data(&handler_scope, &app_slot, &messages, &pending, event.data());
    });
    scope.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
}

fn handle_data<Scene: 'static>(
    scope: &DedicatedWorkerGlobalScope,
    app_slot: &AppSlot<Scene>,
    messages: &PendingMessages,
    pending: &Pending<Scene>,
    data: JsValue,
) {
    let Ok(payload) = js_sys::Reflect::get(&data, &JsValue::from_str(MESSAGE_KEY)) else {
        return;
    };
    let Ok(message) = serde_wasm_bindgen::from_value::<ToWorker>(payload) else {
        return;
    };

    if !matches!(message, ToWorker::Init { .. }) && app_slot.borrow().is_none() {
        messages.borrow_mut().push(data);
        return;
    }

    match message {
        ToWorker::Init { width, height } => {
            let Some(canvas) = canvas_from(&data) else {
                return;
            };
            let Some((driver, on_custom)) = pending.borrow_mut().take() else {
                return;
            };
            let scope = scope.clone();
            let app_slot = app_slot.clone();
            let messages = messages.clone();
            let pending = pending.clone();
            spawn_local(async move {
                let app = create_app(canvas, width, height, driver, on_custom).await;
                *app_slot.borrow_mut() = Some(app);
                let queued = std::mem::take(&mut *messages.borrow_mut());
                for data in queued {
                    handle_data(&scope, &app_slot, &messages, &pending, data);
                }
                post(&FromWorker::Ready {
                    adapter: "WebGPU".to_string(),
                });
                start_render_loop(app_slot);
            });
        }
        ToWorker::Resize { width, height } => {
            if let Some(app) = app_slot.borrow_mut().as_mut() {
                let physical_width = (width as u32).max(1);
                let physical_height = (height as u32).max(1);
                resize_offscreen(
                    &mut app.world,
                    &mut app.renderer,
                    physical_width,
                    physical_height,
                );
                app.world.resources.window.active_viewport_rect =
                    Some(nightshade::ecs::window::resources::ViewportRect {
                        x: 0.0,
                        y: 0.0,
                        width: physical_width as f32,
                        height: physical_height as f32,
                    });
            }
        }
        other => {
            if let Some(app) = app_slot.borrow_mut().as_mut() {
                apply(app, other);
            }
        }
    }
}

fn apply<Scene: 'static>(app: &mut App<Scene>, message: ToWorker) {
    let App {
        world,
        driver,
        on_custom,
        ..
    } = app;
    match message {
        ToWorker::PointerMove { x, y } => input_inject_cursor_moved(world, Vec2::new(x, y)),
        ToWorker::PointerButton { button, pressed } => {
            let state = if pressed {
                KeyState::Pressed
            } else {
                KeyState::Released
            };
            input_inject_mouse_button(world, mouse_button(button), state);
        }
        ToWorker::Wheel { delta } => {
            input_inject_mouse_wheel(world, Vec2::new(0.0, -delta / 100.0))
        }
        ToWorker::Touch { id, phase, x, y } => {
            input_inject_touch(world, id, touch_phase(phase), Vec2::new(x, y));
        }
        ToWorker::Key {
            code,
            pressed,
            text,
        } => {
            if let Some(key_code) = key_code_from_dom(&code) {
                let state = if pressed {
                    KeyState::Pressed
                } else {
                    KeyState::Released
                };
                input_inject_keyboard(world, key_code, state, text.as_deref());
            }
        }
        ToWorker::Pick { x, y } => {
            input_inject_cursor_moved(world, Vec2::new(x.max(0.0), y.max(0.0)));
            let entity = entity_under_cursor(world);
            driver.selected = entity;
            world
                .resources
                .editor_selection
                .bounding_volume_selected_entity = entity;
            world.resources.editor_selection.selected_entities = entity.into_iter().collect();
            let detail = entity.map(|entity| SelectedEntity {
                id: entity.id,
                name: world
                    .core
                    .get_name(entity)
                    .map(|name| name.0.clone())
                    .unwrap_or_default(),
            });
            post(&FromWorker::Selected { detail });
        }
        ToWorker::Custom(value) => {
            let selected = driver.selected;
            on_custom(&mut driver.scene, world, selected, value);
        }
        ToWorker::Init { .. } | ToWorker::Resize { .. } => {}
    }
}

async fn create_app<Scene: 'static>(
    canvas: OffscreenCanvas,
    width: f32,
    height: f32,
    mut driver: Driver<Scene>,
    on_custom: CustomFn<Scene>,
) -> App<Scene> {
    let physical_width = (width as u32).max(1);
    let physical_height = (height as u32).max(1);

    let surface_target = wgpu::SurfaceTarget::OffscreenCanvas(canvas);
    let mut renderer = create_wgpu_renderer(surface_target, physical_width, physical_height)
        .await
        .expect("failed to create renderer from offscreen canvas");

    let mut world = World::default();
    initialize_offscreen(
        &mut world,
        &mut driver,
        &mut renderer,
        (physical_width, physical_height),
        1.0,
    );
    world.resources.window.active_viewport_rect =
        Some(nightshade::ecs::window::resources::ViewportRect {
            x: 0.0,
            y: 0.0,
            width: physical_width as f32,
            height: physical_height as f32,
        });

    App {
        world,
        renderer,
        driver,
        on_custom,
    }
}

fn start_render_loop<Scene: 'static>(app_slot: AppSlot<Scene>) {
    let last_push = Rc::new(RefCell::new(0.0_f64));

    spawn_animation_frame_loop(move || {
        if let Some(app) = app_slot.borrow_mut().as_mut() {
            tick_offscreen(&mut app.world, &mut app.driver, &mut app.renderer);
            let scope: DedicatedWorkerGlobalScope = js_sys::global().unchecked_into();
            if let Some(performance) = scope.performance() {
                let now = performance.now();
                let mut last = last_push.borrow_mut();
                if now - *last > 500.0 {
                    *last = now;
                    let entity_count = app
                        .world
                        .core
                        .query_entities(
                            nightshade::ecs::world::LOCAL_TRANSFORM
                                | nightshade::ecs::world::GLOBAL_TRANSFORM,
                        )
                        .count() as u32;
                    post(&FromWorker::Stats {
                        fps: app.world.resources.window.timing.frames_per_second,
                        entity_count,
                    });
                }
            }
        }
    });
}

pub fn post_custom<Message: Serialize>(message: &Message) {
    if let Ok(value) = serde_json::to_value(message) {
        post(&FromWorker::Custom(value));
    }
}

fn post(message: &FromWorker) {
    let scope: DedicatedWorkerGlobalScope = js_sys::global().unchecked_into();
    if let Ok(value) = serde_wasm_bindgen::to_value(message) {
        let envelope = js_sys::Object::new();
        if js_sys::Reflect::set(&envelope, &JsValue::from_str(MESSAGE_KEY), &value).is_ok() {
            drop(scope.post_message(&envelope));
        }
    }
}

fn mouse_button(button: u8) -> MouseButton {
    match button {
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        _ => MouseButton::Left,
    }
}

fn touch_phase(phase: leptos_musaic_protocol::TouchPhase) -> TouchPhase {
    match phase {
        leptos_musaic_protocol::TouchPhase::Started => TouchPhase::Started,
        leptos_musaic_protocol::TouchPhase::Moved => TouchPhase::Moved,
        leptos_musaic_protocol::TouchPhase::Ended => TouchPhase::Ended,
        leptos_musaic_protocol::TouchPhase::Cancelled => TouchPhase::Cancelled,
    }
}

fn canvas_from(data: &JsValue) -> Option<OffscreenCanvas> {
    js_sys::Reflect::get(data, &JsValue::from_str(CANVAS_KEY))
        .ok()
        .and_then(|value| value.dyn_into::<OffscreenCanvas>().ok())
}

fn key_code_from_dom(code: &str) -> Option<KeyCode> {
    Some(match code {
        "KeyA" => KeyCode::KeyA,
        "KeyB" => KeyCode::KeyB,
        "KeyC" => KeyCode::KeyC,
        "KeyD" => KeyCode::KeyD,
        "KeyE" => KeyCode::KeyE,
        "KeyF" => KeyCode::KeyF,
        "KeyG" => KeyCode::KeyG,
        "KeyH" => KeyCode::KeyH,
        "KeyI" => KeyCode::KeyI,
        "KeyJ" => KeyCode::KeyJ,
        "KeyK" => KeyCode::KeyK,
        "KeyL" => KeyCode::KeyL,
        "KeyM" => KeyCode::KeyM,
        "KeyN" => KeyCode::KeyN,
        "KeyO" => KeyCode::KeyO,
        "KeyP" => KeyCode::KeyP,
        "KeyQ" => KeyCode::KeyQ,
        "KeyR" => KeyCode::KeyR,
        "KeyS" => KeyCode::KeyS,
        "KeyT" => KeyCode::KeyT,
        "KeyU" => KeyCode::KeyU,
        "KeyV" => KeyCode::KeyV,
        "KeyW" => KeyCode::KeyW,
        "KeyX" => KeyCode::KeyX,
        "KeyY" => KeyCode::KeyY,
        "KeyZ" => KeyCode::KeyZ,
        "Digit0" => KeyCode::Digit0,
        "Digit1" => KeyCode::Digit1,
        "Digit2" => KeyCode::Digit2,
        "Digit3" => KeyCode::Digit3,
        "Digit4" => KeyCode::Digit4,
        "Digit5" => KeyCode::Digit5,
        "Digit6" => KeyCode::Digit6,
        "Digit7" => KeyCode::Digit7,
        "Digit8" => KeyCode::Digit8,
        "Digit9" => KeyCode::Digit9,
        "Escape" => KeyCode::Escape,
        "Enter" => KeyCode::Enter,
        "NumpadEnter" => KeyCode::NumpadEnter,
        "Tab" => KeyCode::Tab,
        "Space" => KeyCode::Space,
        "Delete" => KeyCode::Delete,
        "Backspace" => KeyCode::Backspace,
        "Home" => KeyCode::Home,
        "End" => KeyCode::End,
        "ArrowLeft" => KeyCode::ArrowLeft,
        "ArrowRight" => KeyCode::ArrowRight,
        "ArrowUp" => KeyCode::ArrowUp,
        "ArrowDown" => KeyCode::ArrowDown,
        "ShiftLeft" => KeyCode::ShiftLeft,
        "ShiftRight" => KeyCode::ShiftRight,
        "ControlLeft" => KeyCode::ControlLeft,
        "ControlRight" => KeyCode::ControlRight,
        "AltLeft" => KeyCode::AltLeft,
        "AltRight" => KeyCode::AltRight,
        "F1" => KeyCode::F1,
        "F2" => KeyCode::F2,
        "F3" => KeyCode::F3,
        "F4" => KeyCode::F4,
        "F5" => KeyCode::F5,
        "F6" => KeyCode::F6,
        "F7" => KeyCode::F7,
        "F8" => KeyCode::F8,
        "F9" => KeyCode::F9,
        "F10" => KeyCode::F10,
        "F11" => KeyCode::F11,
        "F12" => KeyCode::F12,
        "Comma" => KeyCode::Comma,
        "Period" => KeyCode::Period,
        "Minus" => KeyCode::Minus,
        "Equal" => KeyCode::Equal,
        _ => return None,
    })
}
