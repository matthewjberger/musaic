use leptos::prelude::*;

#[component]
pub fn NumberField(
    label: &'static str,
    value: Signal<f64>,
    #[prop(default = 0.1)] step: f64,
    on_change: Callback<(f64, bool)>,
) -> impl IntoView {
    view! {
        <label class="musaic-field">
            <span class="musaic-field-label">{label}</span>
            <input
                type="number"
                step=step
                prop:value=move || format!("{:.3}", value.get())
                on:input=move |event| {
                    if let Ok(parsed) = event_target_value(&event).parse::<f64>() {
                        on_change.run((parsed, false));
                    }
                }
                on:change=move |event| {
                    if let Ok(parsed) = event_target_value(&event).parse::<f64>() {
                        on_change.run((parsed, true));
                    }
                }
            />
        </label>
    }
}

#[component]
pub fn CheckField(
    label: &'static str,
    value: Signal<bool>,
    on_change: Callback<bool>,
) -> impl IntoView {
    view! {
        <label class="musaic-field check">
            <input
                type="checkbox"
                prop:checked=move || value.get()
                on:change=move |event| on_change.run(event_target_checked(&event))
            />
            <span class="musaic-field-label">{label}</span>
        </label>
    }
}

#[component]
pub fn TextField(
    label: &'static str,
    value: Signal<String>,
    on_commit: Callback<String>,
) -> impl IntoView {
    view! {
        <label class="musaic-field">
            <span class="musaic-field-label">{label}</span>
            <input
                type="text"
                prop:value=move || value.get()
                on:change=move |event| on_commit.run(event_target_value(&event))
            />
        </label>
    }
}

#[component]
pub fn SliderField(
    label: &'static str,
    value: Signal<f64>,
    #[prop(into)] min: Signal<f64>,
    #[prop(into)] max: Signal<f64>,
    #[prop(default = 0.01)] step: f64,
    on_change: Callback<(f64, bool)>,
) -> impl IntoView {
    view! {
        <label class="musaic-field">
            <span class="musaic-field-label">{label}</span>
            <input
                type="range"
                min=move || min.get()
                max=move || max.get()
                step=step
                prop:value=move || value.get()
                on:input=move |event| {
                    if let Ok(parsed) = event_target_value(&event).parse::<f64>() {
                        on_change.run((parsed, false));
                    }
                }
                on:change=move |event| {
                    if let Ok(parsed) = event_target_value(&event).parse::<f64>() {
                        on_change.run((parsed, true));
                    }
                }
            />
            <span class="musaic-field-value">{move || format!("{:.2}", value.get())}</span>
        </label>
    }
}

#[component]
pub fn ColorField(
    label: &'static str,
    value: Signal<[f32; 3]>,
    on_change: Callback<([f32; 3], bool)>,
) -> impl IntoView {
    view! {
        <label class="musaic-field">
            <span class="musaic-field-label">{label}</span>
            <input
                type="color"
                prop:value=move || rgb_to_hex(value.get())
                on:input=move |event| {
                    if let Some(rgb) = hex_to_rgb(&event_target_value(&event)) {
                        on_change.run((rgb, false));
                    }
                }
                on:change=move |event| {
                    if let Some(rgb) = hex_to_rgb(&event_target_value(&event)) {
                        on_change.run((rgb, true));
                    }
                }
            />
        </label>
    }
}

#[component]
pub fn Select(
    label: &'static str,
    value: Signal<String>,
    options: Vec<(String, String)>,
    on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <label class="musaic-field">
            <span class="musaic-field-label">{label}</span>
            <select
                class="musaic-select"
                prop:value=move || value.get()
                on:change=move |event| on_change.run(event_target_value(&event))
            >
                {options
                    .into_iter()
                    .map(|(option_value, text)| view! { <option value=option_value>{text}</option> })
                    .collect_view()}
            </select>
        </label>
    }
}

fn rgb_to_hex(rgb: [f32; 3]) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        (rgb[0].clamp(0.0, 1.0) * 255.0) as u8,
        (rgb[1].clamp(0.0, 1.0) * 255.0) as u8,
        (rgb[2].clamp(0.0, 1.0) * 255.0) as u8,
    )
}

fn hex_to_rgb(hex: &str) -> Option<[f32; 3]> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some([
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
    ])
}
