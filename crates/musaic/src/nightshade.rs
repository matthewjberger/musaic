use std::collections::HashSet;

use leptos::prelude::*;

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
    let commands: HashSet<&'static str> = COMMANDS.iter().copied().collect();
    scan(source, &commands)
}

fn scan(source: &str, commands: &HashSet<&'static str>) -> Vec<(&'static str, String)> {
    let characters: Vec<char> = source.chars().collect();
    let count = characters.len();
    let mut runs: Vec<(&'static str, String)> = Vec::new();
    let mut index = 0;
    while index < count {
        let current = characters[index];
        if current == '/' && index + 1 < count && characters[index + 1] == '*' {
            let start = index;
            index += 2;
            while index < count
                && !(characters[index] == '*' && index + 1 < count && characters[index + 1] == '/')
            {
                index += 1;
            }
            index = (index + 2).min(count);
            runs.push(("tok-comment", characters[start..index].iter().collect()));
        } else if current == '/' && index + 1 < count && characters[index + 1] == '/' {
            let start = index;
            while index < count && characters[index] != '\n' {
                index += 1;
            }
            runs.push(("tok-comment", characters[start..index].iter().collect()));
        } else if current == '"' {
            let start = index;
            index += 1;
            while index < count {
                if characters[index] == '\\' && index + 1 < count {
                    index += 2;
                    continue;
                }
                let quote = characters[index] == '"';
                index += 1;
                if quote {
                    break;
                }
            }
            runs.push(("tok-string", characters[start..index].iter().collect()));
        } else if current.is_ascii_digit() {
            let start = index;
            while index < count
                && (characters[index].is_ascii_alphanumeric() || characters[index] == '.')
            {
                index += 1;
            }
            runs.push(("tok-number", characters[start..index].iter().collect()));
        } else if current.is_alphabetic() || current == '_' {
            let start = index;
            while index < count && (characters[index].is_alphanumeric() || characters[index] == '_')
            {
                index += 1;
            }
            let word: String = characters[start..index].iter().collect();
            let class = if KEYWORDS.contains(&word.as_str()) {
                "tok-keyword"
            } else if commands.contains(word.as_str()) {
                "tok-command"
            } else {
                "tok-plain"
            };
            runs.push((class, word));
        } else {
            let start = index;
            index += 1;
            while index < count {
                let next = characters[index];
                let token_start = (next == '/'
                    && index + 1 < count
                    && (characters[index + 1] == '/' || characters[index + 1] == '*'))
                    || next == '"'
                    || next.is_ascii_digit()
                    || next.is_alphabetic()
                    || next == '_';
                if token_start {
                    break;
                }
                index += 1;
            }
            runs.push(("tok-plain", characters[start..index].iter().collect()));
        }
    }
    runs
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectedEntity {
    pub id: u32,
    pub name: String,
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
