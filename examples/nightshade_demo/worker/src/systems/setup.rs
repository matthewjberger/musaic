use crate::state::Scene;
use crate::systems::example;
use nightshade::prelude::{load_procedural_textures, spawn_sun};
use nightshade_api::prelude::*;

pub fn initialize(scene: &mut Scene, world: &mut World) {
    if let Some((width, height)) = world.resources.window.cached_viewport_size {
        world.resources.window.active_viewport_rect =
            Some(nightshade::ecs::window::resources::ViewportRect {
                x: 0.0,
                y: 0.0,
                width: width as f32,
                height: height as f32,
            });
    }

    set_background(world, Background::Nebula);
    show_grid(world, true);
    world.resources.debug_draw.selection_outline_enabled = true;
    world.resources.debug_draw.selection_outline_color = [1.0, 0.5, 0.15, 1.0];

    load_procedural_textures(world);
    spawn_sun(world);
    orbit_camera(world, vec3(0.0, 0.5, 0.0), 8.0);

    example::spawn_cube_on_ring(scene, world);
}
