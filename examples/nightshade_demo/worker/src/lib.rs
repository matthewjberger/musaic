mod state;
mod systems;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    leptos_musaic_engine::run_offscreen(
        state::Scene::new(),
        systems::setup::initialize,
        systems::tick,
        systems::apply_custom,
    );
}
