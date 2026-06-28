use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Command {
    SpawnCube,
    SpawnSphere,
    SetSpin { spinning: bool },
    SetSpinSpeed { speed: f32 },
    SetBackgroundPreset { preset: String },
    SetBackgroundColor { red: f32, green: f32, blue: f32 },
    SetSelectedColor { red: f32, green: f32, blue: f32 },
    SetSelectedScale { scale: f32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    ObjectCount { count: u32 },
}
