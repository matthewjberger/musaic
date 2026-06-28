use leptos::prelude::*;
use musaic::{
    AppShell, Bridge, Command, CommandPalette, Loader, MusaicStyles, ResizeAxis, ResizeHandle,
    SelectedEntity, THEMES, ThemeProvider, TouchPhase as MusaicTouchPhase, Viewport, ViewportEvent,
    WebGpuGate, use_theme,
};
use wasm_bindgen::{JsCast, JsValue};

use crate::components::dock::Dock;
use crate::components::sidebar::Sidebar;
use crate::components::toolbar::Toolbar;
use crate::state::DemoState;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <MusaicStyles />
        <ThemeProvider>
            <WebGpuGate>
                <Stage />
            </WebGpuGate>
        </ThemeProvider>
    }
}

#[component]
fn Stage() -> impl IntoView {
    let state = DemoState::new();
    let bridge = StoredValue::new_local(None::<Bridge>);
    let theme = use_theme();

    let send = Callback::new(move |message: protocol::ClientMessage| {
        if let Some(bridge) = bridge.get_value() {
            bridge.send(&message);
        }
    });

    let _ = window_event_listener(leptos::ev::keydown, move |event| {
        if (event.ctrl_key() || event.meta_key()) && event.key() == "k" {
            event.prevent_default();
            state.palette_open.update(|open| *open = !*open);
            return;
        }
        if typing_in_field(&event) {
            return;
        }
        if let Some(bridge) = bridge.get_value() {
            let text = (event.key().chars().count() == 1).then(|| event.key());
            bridge.send(&protocol::ClientMessage::Key {
                code: event.code(),
                pressed: true,
                text,
            });
        }
    });

    let _ = window_event_listener(leptos::ev::keyup, move |event| {
        if typing_in_field(&event) {
            return;
        }
        if let Some(bridge) = bridge.get_value() {
            bridge.send(&protocol::ClientMessage::Key {
                code: event.code(),
                pressed: false,
                text: None,
            });
        }
    });

    let on_input = Callback::new(move |event: ViewportEvent| {
        let message = match event {
            ViewportEvent::Resize { width, height } => {
                protocol::ClientMessage::Resize { width, height }
            }
            ViewportEvent::PointerMove { x, y } => protocol::ClientMessage::PointerMove { x, y },
            ViewportEvent::PointerButton { button, pressed } => {
                protocol::ClientMessage::PointerButton { button, pressed }
            }
            ViewportEvent::Wheel { delta } => protocol::ClientMessage::Wheel { delta },
            ViewportEvent::Touch { id, phase, x, y } => protocol::ClientMessage::Touch {
                id,
                phase: map_phase(phase),
                x,
                y,
            },
            ViewportEvent::Pick { x, y } => protocol::ClientMessage::Pick { x, y },
        };
        send.run(message);
    });

    let on_connect = Callback::new(
        move |(connected, canvas, width, height): (Bridge, web_sys::OffscreenCanvas, f32, f32)| {
            connected.send_with_canvas(&protocol::ClientMessage::Init { width, height }, &canvas);
            bridge.set_value(Some(connected));
        },
    );

    let on_message = Callback::new(move |payload: JsValue| {
        let Ok(message) = serde_wasm_bindgen::from_value::<protocol::WorkerMessage>(payload) else {
            return;
        };
        match message {
            protocol::WorkerMessage::Ready { adapter } => {
                state.adapter.set(adapter.clone());
                state.ready.set(true);
                state.log_line(format!("renderer ready ({adapter})"));
            }
            protocol::WorkerMessage::Stats { fps, entity_count } => {
                state.fps.set(fps);
                state.entity_count.set(entity_count);
            }
            protocol::WorkerMessage::Selected { detail } => match detail {
                Some(entity) => {
                    state.log_line(format!("selected {} (#{})", entity.name, entity.id));
                    state.selected.set(Some(SelectedEntity {
                        id: entity.id,
                        name: entity.name,
                    }));
                }
                None => {
                    state.selected.set(None);
                    state.log_line("cleared selection");
                }
            },
            protocol::WorkerMessage::CubeCount { count } => state.object_count.set(count),
        }
    });

    let commands = Signal::derive(move || {
        vec![
            Command::new(
                "spawn-cube",
                "Spawn cube",
                Callback::new(move |_| {
                    send.run(protocol::ClientMessage::SpawnCube);
                    state.log_line("spawned cube");
                }),
            )
            .with_hint("Space"),
            Command::new(
                "spawn-sphere",
                "Spawn sphere",
                Callback::new(move |_| {
                    send.run(protocol::ClientMessage::SpawnSphere);
                    state.log_line("spawned sphere");
                }),
            ),
            Command::new(
                "toggle-spin",
                "Toggle spin",
                Callback::new(move |_| {
                    let next = !state.spinning.get_untracked();
                    state.spinning.set(next);
                    send.run(protocol::ClientMessage::SetSpin { spinning: next });
                }),
            ),
            Command::new(
                "next-theme",
                "Cycle theme",
                Callback::new(move |_| {
                    let current = theme.get_untracked();
                    let index = THEMES
                        .iter()
                        .position(|(id, _)| *id == current)
                        .unwrap_or(0);
                    theme.set(THEMES[(index + 1) % THEMES.len()].0.to_string());
                }),
            ),
        ]
    });

    view! {
        <AppShell>
            <div
                class="ed-grid"
                style=move || {
                    format!("grid-template-rows: 48px minmax(0,1fr) 6px {}px", state.dock_height.get())
                }
            >
                <Toolbar state=state send=send />
                <div
                    class="ed-body"
                    style=move || {
                        format!(
                            "grid-template-columns: {}px 6px minmax(0,1fr)",
                            state.sidebar_width.get(),
                        )
                    }
                >
                    <Sidebar state=state send=send />
                    <ResizeHandle
                        value=state.sidebar_width
                        axis=ResizeAxis::Horizontal
                        min=200.0
                        max=560.0
                    />
                    <div class="ed-viewport-cell">
                        <Viewport
                            worker_url="runtime/worker.js"
                            grabbing=state.grabbing
                            on_input=on_input
                            on_connect=on_connect
                            on_message=on_message
                        />
                        <Loader ready=state.ready />
                    </div>
                </div>
                <ResizeHandle
                    value=state.dock_height
                    axis=ResizeAxis::Vertical
                    min=120.0
                    max=560.0
                    invert=true
                />
                <Dock state=state />
            </div>
        </AppShell>
        <CommandPalette open=state.palette_open commands=commands />
    }
}

fn map_phase(phase: MusaicTouchPhase) -> protocol::TouchPhase {
    match phase {
        MusaicTouchPhase::Started => protocol::TouchPhase::Started,
        MusaicTouchPhase::Moved => protocol::TouchPhase::Moved,
        MusaicTouchPhase::Ended => protocol::TouchPhase::Ended,
        MusaicTouchPhase::Cancelled => protocol::TouchPhase::Cancelled,
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
