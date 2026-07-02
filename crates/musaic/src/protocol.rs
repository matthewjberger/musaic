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

#[cfg(test)]
mod tests {
    use super::*;

    fn to_worker_roundtrips(message: ToWorker) {
        let value = serde_json::to_value(&message).expect("serialize");
        let back: ToWorker = serde_json::from_value(value).expect("deserialize");
        assert_eq!(format!("{message:?}"), format!("{back:?}"));
    }

    #[test]
    fn to_worker_survives_the_wire() {
        to_worker_roundtrips(ToWorker::Init {
            width: 800.0,
            height: 600.0,
        });
        to_worker_roundtrips(ToWorker::PointerButton {
            button: 2,
            pressed: true,
        });
        to_worker_roundtrips(ToWorker::Touch {
            id: 7,
            phase: TouchPhase::Moved,
            x: 1.0,
            y: 2.0,
        });
        to_worker_roundtrips(ToWorker::Key {
            code: "KeyW".to_string(),
            pressed: true,
            text: Some("w".to_string()),
        });
        to_worker_roundtrips(ToWorker::Custom(serde_json::json!({ "SpawnCube": null })));
    }

    #[test]
    fn from_worker_survives_the_wire() {
        let message = FromWorker::Selected {
            detail: Some(SelectedEntity {
                id: 3,
                name: "Cube 3".to_string(),
            }),
        };
        let value = serde_json::to_value(&message).expect("serialize");
        let back: FromWorker = serde_json::from_value(value).expect("deserialize");
        assert_eq!(format!("{message:?}"), format!("{back:?}"));
    }
}
