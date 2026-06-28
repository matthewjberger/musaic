use leptos::prelude::*;
use musaic::{Button, ThemePicker};

use crate::state::DemoState;

#[component]
pub fn Toolbar(state: DemoState, send: Callback<protocol::ClientMessage>) -> impl IntoView {
    let spawn_cube = Callback::new(move |_event: web_sys::MouseEvent| {
        send.run(protocol::ClientMessage::SpawnCube);
        state.log_line("spawned cube");
    });
    let spawn_sphere = Callback::new(move |_event: web_sys::MouseEvent| {
        send.run(protocol::ClientMessage::SpawnSphere);
        state.log_line("spawned sphere");
    });
    let open_palette =
        Callback::new(move |_event: web_sys::MouseEvent| state.palette_open.set(true));

    view! {
        <div class="ed-toolbar">
            <div class="ed-brand">
                <span class="ed-dot"></span>
                "musaic studio"
            </div>
            <span class="ed-sub">"built entirely from musaic components"</span>
            <div class="ed-spacer"></div>
            <Button on_click=spawn_cube>"+ Cube"</Button>
            <Button on_click=spawn_sphere>"+ Sphere"</Button>
            <Button class="ghost" on_click=open_palette>"⌘K  Commands"</Button>
            <ThemePicker />
        </div>
    }
}
