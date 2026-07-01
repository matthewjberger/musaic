use leptos::prelude::*;
use leptos_musaic::{
    AppShell, Command, CommandPalette, EngineViewport, KeymapProvider, Loader, MusaicStyles,
    ResizeAxis, ResizeHandle, THEMES, ThemeProvider, WebGpuGate, provide_command_registry,
    use_engine, use_theme,
};

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
    let engine = use_engine("runtime/worker.js");
    let state = DemoState::new();
    let theme = use_theme();

    engine.on_custom(Callback::new(move |value: serde_json::Value| {
        if let Ok(protocol::Event::ObjectCount { count }) = serde_json::from_value(value) {
            state.object_count.set(count);
        }
    }));

    let registry = provide_command_registry();
    registry.register_all([
        Command::new(
            "spawn-cube",
            "Spawn cube",
            Callback::new(move |_| {
                engine.send(&protocol::Command::SpawnCube);
                state.log_line("spawned cube");
            }),
        )
        .with_keybinding("c")
        .with_group("Scene"),
        Command::new(
            "spawn-sphere",
            "Spawn sphere",
            Callback::new(move |_| {
                engine.send(&protocol::Command::SpawnSphere);
                state.log_line("spawned sphere");
            }),
        )
        .with_group("Scene"),
        Command::new(
            "toggle-spin",
            "Toggle spin",
            Callback::new(move |_| {
                let next = !state.spinning.get_untracked();
                state.spinning.set(next);
                engine.send(&protocol::Command::SetSpin { spinning: next });
            }),
        )
        .with_keybinding("s")
        .with_group("Scene"),
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
        )
        .with_keybinding("mod+j")
        .with_group("View"),
        Command::new(
            "palette.open",
            "Command palette",
            Callback::new(move |_| state.palette_open.set(true)),
        )
        .with_keybinding("mod+k")
        .with_group("View"),
    ]);

    view! {
        <KeymapProvider>
            <AppShell>
                <div
                    class="ed-grid"
                    style=move || {
                        format!(
                            "grid-template-rows: 48px minmax(0,1fr) 6px {}px",
                            state.dock_height.get(),
                        )
                    }
                >
                    <Toolbar engine=engine state=state />
                    <div
                        class="ed-body"
                        style=move || {
                            format!(
                                "grid-template-columns: {}px 6px minmax(0,1fr)",
                                state.sidebar_width.get(),
                            )
                        }
                    >
                        <Sidebar engine=engine state=state />
                        <ResizeHandle
                            value=state.sidebar_width
                            axis=ResizeAxis::Horizontal
                            min=200.0
                            max=560.0
                        />
                        <div class="ed-viewport-cell">
                            <EngineViewport engine=engine />
                            <Loader ready=engine.state.ready />
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
            <CommandPalette open=state.palette_open />
        </KeymapProvider>
    }
}
