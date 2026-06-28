use crate::state::Scene;
use nightshade::prelude::input_inject_cursor_moved;
use nightshade_api::prelude::*;
use protocol::{SelectedEntity, WorkerMessage};

/// Picks at a pixel in physical canvas coordinates and selects the result.
/// `entity_under_cursor` is the facade's pick: it reads the cursor position,
/// ray-casts against the scene, and skips the api's own cameras and lights, so
/// the cursor is moved there first. The pick is synchronous, the selection is
/// known the moment the click arrives.
pub fn pick(scene: &mut Scene, world: &mut World, x: f32, y: f32) {
    input_inject_cursor_moved(world, Vec2::new(x.max(0.0), y.max(0.0)));
    let entity = entity_under_cursor(world);
    select(scene, world, entity);
}

/// Sets the selection, syncs it to the engine's outline pass, and reports it
/// to the page.
pub fn select(scene: &mut Scene, world: &mut World, entity: Option<Entity>) {
    scene.selected = entity;
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
    crate::post(&WorkerMessage::Selected { detail });
}
