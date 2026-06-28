use crate::state::Scene;
use nightshade::prelude::Name;
use nightshade_api::prelude::*;
use protocol::WorkerMessage;

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

pub fn set_scene_background(world: &mut World, preset: &str) {
    set_background(world, background_for(preset));
}

pub fn set_background_color(world: &mut World, rgb: [f32; 3]) {
    set_background(world, Background::Color([rgb[0], rgb[1], rgb[2], 1.0]));
}

pub fn recolor_selected(scene: &Scene, world: &mut World, rgb: [f32; 3]) {
    if let Some(entity) = scene.selected {
        set_color(world, entity, [rgb[0], rgb[1], rgb[2], 1.0]);
    }
}

pub fn rescale_selected(scene: &Scene, world: &mut World, scale: f32) {
    if let Some(entity) = scene.selected {
        set_scale(world, entity, vec3(scale, scale, scale));
    }
}

fn register(scene: &mut Scene, entity: Entity) {
    scene.cubes.push(entity);
    crate::post(&WorkerMessage::CubeCount {
        count: scene.cubes.len() as u32,
    });
}

fn ring_position(index: usize) -> Vec3 {
    if index == 0 {
        vec3(0.0, 0.5, 0.0)
    } else {
        let angle = index as f32 * GOLDEN_ANGLE_RADIANS;
        vec3(angle.cos() * RING_RADIUS, 0.5, angle.sin() * RING_RADIUS)
    }
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
