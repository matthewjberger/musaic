use leptos::prelude::*;
use leptos_musaic::{ColorField, Engine, Panel, Select, SelectedCard, SliderField, Switch};

use crate::state::DemoState;

#[component]
pub fn Sidebar(engine: Engine, state: DemoState) -> impl IntoView {
    let toggle_spin = Callback::new(move |value: bool| {
        state.spinning.set(value);
        engine.send(&protocol::Command::SetSpin { spinning: value });
    });
    let spin_speed = Callback::new(move |(value, _committed): (f64, bool)| {
        state.spin_speed.set(value);
        engine.send(&protocol::Command::SetSpinSpeed {
            speed: value as f32,
        });
    });
    let background = Callback::new(move |value: String| {
        state.background.set(value.clone());
        engine.send(&protocol::Command::SetBackgroundPreset {
            preset: value.clone(),
        });
        state.log_line(format!("background -> {value}"));
    });
    let bg_color = Callback::new(move |(rgb, _committed): ([f32; 3], bool)| {
        state.bg_color.set(rgb);
        engine.send(&protocol::Command::SetBackgroundColor {
            red: rgb[0],
            green: rgb[1],
            blue: rgb[2],
        });
    });
    let sel_color = Callback::new(move |(rgb, _committed): ([f32; 3], bool)| {
        state.sel_color.set(rgb);
        engine.send(&protocol::Command::SetSelectedColor {
            red: rgb[0],
            green: rgb[1],
            blue: rgb[2],
        });
    });
    let sel_scale = Callback::new(move |(value, _committed): (f64, bool)| {
        state.sel_scale.set(value);
        engine.send(&protocol::Command::SetSelectedScale {
            scale: value as f32,
        });
    });

    view! {
        <div class="ed-sidebar">
            <Panel title="Scene">
                <Stat label="Adapter" value=Signal::derive(move || engine.state.adapter.get()) />
                <Stat
                    label="FPS"
                    value=Signal::derive(move || format!("{:.0}", engine.state.fps.get()))
                />
                <Stat
                    label="Entities"
                    value=Signal::derive(move || engine.state.entity_count.get().to_string())
                />
                <Stat
                    label="Objects"
                    value=Signal::derive(move || state.object_count.get().to_string())
                />
            </Panel>

            <Panel title="Motion">
                <Switch
                    label="Spin"
                    value=Signal::derive(move || state.spinning.get())
                    on_change=toggle_spin
                />
                <SliderField
                    label="Speed"
                    value=Signal::derive(move || state.spin_speed.get())
                    min=Signal::derive(|| 0.0)
                    max=Signal::derive(|| 4.0)
                    step=0.05
                    on_change=spin_speed
                />
            </Panel>

            <Panel title="Environment">
                <Select
                    label="Sky"
                    value=Signal::derive(move || state.background.get())
                    options=vec![
                        ("nebula".into(), "Nebula".into()),
                        ("sky".into(), "Sky".into()),
                        ("cloudy".into(), "Cloudy".into()),
                        ("space".into(), "Space".into()),
                        ("sunset".into(), "Sunset".into()),
                    ]
                    on_change=background
                />
                <ColorField
                    label="Clear color"
                    value=Signal::derive(move || state.bg_color.get())
                    on_change=bg_color
                />
            </Panel>

            <Panel title="Selection">
                <SelectedCard selected=engine.state.selected />
                <ColorField
                    label="Color"
                    value=Signal::derive(move || state.sel_color.get())
                    on_change=sel_color
                />
                <SliderField
                    label="Scale"
                    value=Signal::derive(move || state.sel_scale.get())
                    min=Signal::derive(|| 0.2)
                    max=Signal::derive(|| 3.0)
                    step=0.05
                    on_change=sel_scale
                />
            </Panel>
        </div>
    }
}

#[component]
fn Stat(label: &'static str, value: Signal<String>) -> impl IntoView {
    view! {
        <div class="ed-stat">
            <span class="ed-key">{label}</span>
            <span>{move || value.get()}</span>
        </div>
    }
}
