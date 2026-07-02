use crate::protocol::SelectedEntity;
use leptos::prelude::*;

use crate::code_editor::highlight_code;

const KEYWORDS: &[&str] = &[
    "fn", "let", "const", "if", "else", "for", "in", "while", "loop", "return", "break",
    "continue", "switch", "import", "export", "global", "private", "true", "false", "throw", "try",
    "catch", "this",
];

const COMMANDS: &[&str] = &[
    "commands",
    "spawn_floor",
    "spawn_object",
    "spawn_cube",
    "spawn_sphere",
    "spawn_cylinder",
    "spawn_cone",
    "spawn_plane",
    "spawn_torus",
    "spawn_label",
    "spawn_text",
    "point_light",
    "spot_light",
    "set_sun",
    "set_emissive",
    "set_color",
    "set_bloom",
    "set_metallic_roughness",
    "set_background",
    "set_ambient",
    "set_texture",
    "set_texture_tiling",
    "set_unlit",
    "set_visible",
    "set_parent",
    "draw_cube",
    "draw_sphere",
    "draw_line",
    "emit_firework",
    "emit_burst",
    "emit_particles",
    "emit_fire",
    "emit_smoke",
    "rotate",
    "set_position",
    "set_scale",
    "set_rotation",
    "despawn",
    "push",
    "set_velocity",
    "apply_force",
    "last",
    "result",
    "tag",
    "entity_ref",
    "hsv",
    "rgb",
    "rgba",
    "random",
    "random_range",
    "random_int",
    "log",
];

pub fn highlight_rhai(source: &str) -> Vec<(&'static str, String)> {
    highlight_code(source, KEYWORDS, COMMANDS)
}

#[component]
pub fn SelectedCard(#[prop(into)] selected: Signal<Option<SelectedEntity>>) -> impl IntoView {
    view! {
        <div class="musaic-selected-card">
            {move || match selected.get() {
                Some(entity) => {
                    view! {
                        <div class="musaic-selected-row">
                            <span class="key">"Name"</span>
                            <span>{entity.name}</span>
                        </div>
                        <div class="musaic-selected-row">
                            <span class="key">"Id"</span>
                            <span>{entity.id}</span>
                        </div>
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <div class="musaic-selected-row">
                            <span class="key">"Selection"</span>
                            <span>"None"</span>
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
