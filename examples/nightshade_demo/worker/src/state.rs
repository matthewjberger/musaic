use crate::systems;
use nightshade::prelude::*;

/// The application state carried across frames by the offscreen driver. Plain
/// data: the spawned cubes, the spin toggle, and the current selection. This
/// is the `nightshade-api` style, no user-side ECS. The engine `State` trait
/// is the one piece of plumbing the offscreen loop needs, so this implements
/// it and forwards each hook to a free function in `systems/`.
pub struct Scene {
    pub cubes: Vec<Entity>,
    pub selected: Option<Entity>,
    pub spinning: bool,
    pub spin_speed: f32,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            cubes: Vec::new(),
            selected: None,
            spinning: true,
            spin_speed: 1.0,
        }
    }
}

impl State for Scene {
    fn initialize(&mut self, world: &mut World) {
        systems::setup::initialize(self, world);
    }

    fn run_systems(&mut self, world: &mut World) {
        camera_controllers_system(world);
        systems::example::tick(self, world);
    }
}
