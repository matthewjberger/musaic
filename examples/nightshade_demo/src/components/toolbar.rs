use leptos::prelude::*;
use leptos_musaic::{
    Engine, LogKind, ThemeMenu, ToolButton, Toolbar as MusaicToolbar, ToolbarGroup, ToolbarSpacer,
};

use crate::state::DemoState;

#[component]
pub fn Toolbar(engine: Engine, state: DemoState) -> impl IntoView {
    let spawn_cube = Callback::new(move |_event: web_sys::MouseEvent| {
        engine.send(&protocol::Command::SpawnCube);
        state.log_line(LogKind::Command, "spawned cube");
    });
    let spawn_sphere = Callback::new(move |_event: web_sys::MouseEvent| {
        engine.send(&protocol::Command::SpawnSphere);
        state.log_line(LogKind::Command, "spawned sphere");
    });
    let open_palette =
        Callback::new(move |_event: web_sys::MouseEvent| state.palette_open.set(true));

    view! {
        <MusaicToolbar>
            <ToolbarGroup>
                <div class="ed-brand">
                    <span class="ed-dot"></span>
                    "musaic studio"
                </div>
                <span class="ed-sub">"built entirely from musaic components"</span>
            </ToolbarGroup>
            <ToolbarSpacer />
            <ToolButton on_click=spawn_cube>"+ Cube"</ToolButton>
            <ToolButton on_click=spawn_sphere>"+ Sphere"</ToolButton>
            <ToolButton on_click=open_palette>"\u{2318}K  Commands"</ToolButton>
            <ThemeMenu />
        </MusaicToolbar>
    }
}
