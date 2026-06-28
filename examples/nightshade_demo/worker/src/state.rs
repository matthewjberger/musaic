use nightshade::prelude::Entity;

pub struct Scene {
    pub cubes: Vec<Entity>,
    pub spinning: bool,
    pub spin_speed: f32,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            cubes: Vec::new(),
            spinning: true,
            spin_speed: 1.0,
        }
    }
}
