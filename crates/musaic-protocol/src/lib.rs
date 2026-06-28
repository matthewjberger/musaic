use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const MESSAGE_KEY: &str = "message";
pub const CANVAS_KEY: &str = "canvas";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectedEntity {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ToWorker {
    Init {
        width: f32,
        height: f32,
    },
    Resize {
        width: f32,
        height: f32,
    },
    PointerMove {
        x: f32,
        y: f32,
    },
    PointerButton {
        button: u8,
        pressed: bool,
    },
    Wheel {
        delta: f32,
    },
    Touch {
        id: u64,
        phase: TouchPhase,
        x: f32,
        y: f32,
    },
    Key {
        code: String,
        pressed: bool,
        text: Option<String>,
    },
    Pick {
        x: f32,
        y: f32,
    },
    Custom(Value),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FromWorker {
    Ready { adapter: String },
    Stats { fps: f32, entity_count: u32 },
    Selected { detail: Option<SelectedEntity> },
    Custom(Value),
}
