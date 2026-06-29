use crate::state::Scene;
use nightshade::prelude::{Entity, Name};
use nightshade_api::prelude::*;
use protocol::{Command, Event};
use serde_json::Value;

const SPIN_RADIANS_PER_SECOND: f32 = 0.8;
const RING_RADIUS: f32 = 3.0;
const GOLDEN_ANGLE_RADIANS: f32 = 2.399_963;

pub fn tick(scene: &mut Scene, world: &mut World) {
    if scene.spinning {
        let step = SPIN_RADIANS_PER_SECOND * scene.spin_speed * delta_time(world);
        for &cube in &scene.cubes {
            rotate(world, cube, Vec3::y(), step);
        }
    }

    if key_pressed(world, KeyCode::Space) {
        spawn_cube_on_ring(scene, world);
    }
}

pub fn apply_custom(scene: &mut Scene, world: &mut World, selected: Option<Entity>, value: Value) {
    let Ok(command) = serde_json::from_value::<Command>(value) else {
        return;
    };
    match command {
        Command::SpawnCube => spawn_cube_on_ring(scene, world),
        Command::SpawnSphere => spawn_sphere_on_ring(scene, world),
        Command::SetSpin { spinning } => scene.spinning = spinning,
        Command::SetSpinSpeed { speed } => scene.spin_speed = speed,
        Command::SetBackgroundPreset { preset } => set_scene_background(world, &preset),
        Command::SetBackgroundColor { red, green, blue } => {
            set_background_color(world, [red, green, blue])
        }
        Command::SetSelectedColor { red, green, blue } => {
            if let Some(entity) = selected {
                set_color(world, entity, [red, green, blue, 1.0]);
            }
        }
        Command::SetSelectedScale { scale } => {
            if let Some(entity) = selected {
                set_scale(world, entity, vec3(scale, scale, scale));
            }
        }
    }
}

pub fn spawn_cube_on_ring(scene: &mut Scene, world: &mut World) {
    let index = scene.cubes.len();
    let entity = spawn_cube(world, ring_position(index));
    set_color(world, entity, color_for(index));
    world.core.set_name(entity, Name(format!("Cube {index}")));
    register(scene, entity);
}

pub fn spawn_sphere_on_ring(scene: &mut Scene, world: &mut World) {
    let index = scene.cubes.len();
    let entity = spawn_sphere(world, ring_position(index));
    set_color(world, entity, color_for(index));
    world.core.set_name(entity, Name(format!("Sphere {index}")));
    register(scene, entity);
}

fn register(scene: &mut Scene, entity: Entity) {
    scene.cubes.push(entity);
    leptos_musaic_engine::post_custom(&Event::ObjectCount {
        count: scene.cubes.len() as u32,
    });
}

fn set_scene_background(world: &mut World, preset: &str) {
    set_background(world, background_for(preset));
}

fn set_background_color(world: &mut World, rgb: [f32; 3]) {
    set_background(world, Background::Color([rgb[0], rgb[1], rgb[2], 1.0]));
}

fn background_for(preset: &str) -> Background {
    match preset {
        "sky" => Background::Sky,
        "cloudy" => Background::CloudySky,
        "space" => Background::Space,
        "sunset" => Background::Sunset,
        _ => Background::Nebula,
    }
}

fn ring_position(index: usize) -> Vec3 {
    if index == 0 {
        vec3(0.0, 0.5, 0.0)
    } else {
        let angle = index as f32 * GOLDEN_ANGLE_RADIANS;
        vec3(angle.cos() * RING_RADIUS, 0.5, angle.sin() * RING_RADIUS)
    }
}

fn color_for(index: usize) -> [f32; 4] {
    let hue = (index as f32 * GOLDEN_ANGLE_RADIANS).rem_euclid(std::f32::consts::TAU);
    let sector = hue / std::f32::consts::FRAC_PI_3;
    let fraction = sector - sector.floor();
    let rising = fraction;
    let falling = 1.0 - fraction;
    let (red, green, blue) = match sector as u32 % 6 {
        0 => (1.0, rising, 0.0),
        1 => (falling, 1.0, 0.0),
        2 => (0.0, 1.0, rising),
        3 => (0.0, falling, 1.0),
        4 => (rising, 0.0, 1.0),
        _ => (1.0, 0.0, falling),
    };
    [
        0.25 + red * 0.75,
        0.25 + green * 0.75,
        0.25 + blue * 0.75,
        1.0,
    ]
}
