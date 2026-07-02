use leptos::prelude::*;
use leptos_musaic::{
    AppShell, Command, CommandPalette, EditorShell, Engine, EngineViewport, KeymapProvider, Loader,
    LogKind, MusaicStyles, StatusBar, StatusItem, StatusSpacer, THEMES, ThemeProvider, WebGpuGate,
    provide_command_registry, use_engine, use_theme,
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
                state.log_line(LogKind::Command, "spawned cube");
            }),
        )
        .with_keybinding("c")
        .with_group("Scene"),
        Command::new(
            "spawn-sphere",
            "Spawn sphere",
            Callback::new(move |_| {
                engine.send(&protocol::Command::SpawnSphere);
                state.log_line(LogKind::Command, "spawned sphere");
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
                <EditorShell
                    left_size=state.sidebar_width
                    bottom_size=state.dock_height
                    toolbar=move || view! { <Toolbar engine=engine state=state /> }
                    left=move || view! { <Sidebar engine=engine state=state /> }
                    bottom=move || view! { <Dock state=state /> }
                    status=move || view! { <StatusStrip engine=engine state=state /> }
                >
                    <EngineViewport engine=engine />
                    <Loader ready=engine.state.ready />
                </EditorShell>
            </AppShell>
            <CommandPalette open=state.palette_open />
        </KeymapProvider>
    }
}

#[component]
fn StatusStrip(engine: Engine, state: DemoState) -> impl IntoView {
    view! {
        <StatusBar>
            <StatusItem icon="\u{25c9}">{move || engine.state.adapter.get()}</StatusItem>
            <StatusItem>{move || format!("{:.0} fps", engine.state.fps.get())}</StatusItem>
            <StatusSpacer />
            <StatusItem>
                {move || format!("{} entities", engine.state.entity_count.get())}
            </StatusItem>
            <StatusItem>{move || format!("{} objects", state.object_count.get())}</StatusItem>
        </StatusBar>
    }
}
