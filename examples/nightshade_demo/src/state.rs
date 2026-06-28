use leptos::prelude::*;

const SAMPLE_SCRIPT: &str = r#"// musaic studio scene script (scratchpad)
fn build(commands) {
    set_background("nebula");
    let count = 8;
    for index in 0..count {
        let cube = commands.spawn_cube();
        commands.set_color(cube, hsv(index * 45.0, 0.7, 1.0));
        commands.rotate(cube, 0.2);
    }
}
"#;

#[derive(Clone, Copy)]
pub struct DemoState {
    pub spinning: RwSignal<bool>,
    pub spin_speed: RwSignal<f64>,
    pub background: RwSignal<String>,
    pub bg_color: RwSignal<[f32; 3]>,
    pub sel_color: RwSignal<[f32; 3]>,
    pub sel_scale: RwSignal<f64>,
    pub object_count: RwSignal<u32>,
    pub dock_tab: RwSignal<String>,
    pub script: RwSignal<String>,
    pub log: RwSignal<Vec<String>>,
    pub palette_open: RwSignal<bool>,
    pub sidebar_width: RwSignal<f64>,
    pub dock_height: RwSignal<f64>,
}

impl DemoState {
    pub fn new() -> Self {
        Self {
            spinning: RwSignal::new(true),
            spin_speed: RwSignal::new(1.0),
            background: RwSignal::new("nebula".to_string()),
            bg_color: RwSignal::new([0.05, 0.05, 0.09]),
            sel_color: RwSignal::new([1.0, 0.5, 0.15]),
            sel_scale: RwSignal::new(1.0),
            object_count: RwSignal::new(0),
            dock_tab: RwSignal::new("script".to_string()),
            script: RwSignal::new(SAMPLE_SCRIPT.to_string()),
            log: RwSignal::new(vec!["musaic studio ready".to_string()]),
            palette_open: RwSignal::new(false),
            sidebar_width: RwSignal::new(296.0),
            dock_height: RwSignal::new(232.0),
        }
    }

    pub fn log_line(&self, message: impl Into<String>) {
        self.log.update(|lines| {
            lines.push(message.into());
            if lines.len() > 200 {
                lines.remove(0);
            }
        });
    }
}

impl Default for DemoState {
    fn default() -> Self {
        Self::new()
    }
}
