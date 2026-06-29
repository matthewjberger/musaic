use leptos::prelude::*;

#[component]
pub fn NumberField(
    #[prop(into)] label: String,
    value: Signal<f64>,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] integer: bool,
    #[prop(into, optional)] help: String,
    #[prop(into, optional)] error: String,
    #[prop(into, optional)] disabled: Signal<bool>,
    on_change: Callback<(f64, bool)>,
) -> impl IntoView {
    let step = step.unwrap_or(if integer { 1.0 } else { 0.1 });
    let clamp = move |parsed: f64| {
        let mut result = if integer { parsed.round() } else { parsed };
        if let Some(min) = min {
            result = result.max(min);
        }
        if let Some(max) = max {
            result = result.min(max);
        }
        result
    };
    let format_value = move |raw: f64| {
        if integer {
            format!("{:.0}", raw.round())
        } else {
            format!("{raw:.3}")
        }
    };
    let commit = move |raw: String, committed: bool| {
        if let Ok(parsed) = raw.parse::<f64>() {
            on_change.run((clamp(parsed), committed));
        }
    };
    view! {
        <div class="musaic-field-group">
            <label class="musaic-field">
                <span class="musaic-field-label">{label}</span>
                <input
                    type="number"
                    step=step
                    min=min.map(|value| value.to_string())
                    max=max.map(|value| value.to_string())
                    disabled=move || disabled.get()
                    prop:value=move || format_value(value.get())
                    on:input=move |event| commit(event_target_value(&event), false)
                    on:change=move |event| commit(event_target_value(&event), true)
                />
            </label>
            <FieldNote help=help error=error />
        </div>
    }
}

#[component]
pub fn CheckField(
    #[prop(into)] label: String,
    value: Signal<bool>,
    on_change: Callback<bool>,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <label class="musaic-field check">
            <input
                type="checkbox"
                disabled=move || disabled.get()
                prop:checked=move || value.get()
                on:change=move |event| on_change.run(event_target_checked(&event))
            />
            <span class="musaic-field-label">{label}</span>
        </label>
    }
}

#[component]
pub fn Switch(
    #[prop(into)] label: String,
    value: Signal<bool>,
    on_change: Callback<bool>,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <label class="musaic-field musaic-switch-field">
            <span class="musaic-field-label">{label}</span>
            <button
                type="button"
                role="switch"
                class="musaic-switch"
                class:on=move || value.get()
                aria-checked=move || value.get().to_string()
                disabled=move || disabled.get()
                on:click=move |_| on_change.run(!value.get_untracked())
            >
                <span class="musaic-switch-thumb"></span>
            </button>
        </label>
    }
}

#[component]
pub fn TextField(
    #[prop(into)] label: String,
    value: Signal<String>,
    on_commit: Callback<String>,
    #[prop(into, optional)] placeholder: String,
    #[prop(into, optional)] help: String,
    #[prop(into, optional)] error: String,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="musaic-field-group">
            <label class="musaic-field">
                <span class="musaic-field-label">{label}</span>
                <input
                    type="text"
                    placeholder=placeholder
                    disabled=move || disabled.get()
                    prop:value=move || value.get()
                    on:change=move |event| on_commit.run(event_target_value(&event))
                />
            </label>
            <FieldNote help=help error=error />
        </div>
    }
}

#[component]
pub fn SliderField(
    #[prop(into)] label: String,
    value: Signal<f64>,
    #[prop(into)] min: Signal<f64>,
    #[prop(into)] max: Signal<f64>,
    #[prop(default = 0.01)] step: f64,
    #[prop(into, optional)] disabled: Signal<bool>,
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
                disabled=move || disabled.get()
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
    #[prop(into)] label: String,
    value: Signal<[f32; 3]>,
    on_change: Callback<([f32; 3], bool)>,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <label class="musaic-field">
            <span class="musaic-field-label">{label}</span>
            <input
                type="color"
                disabled=move || disabled.get()
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
    #[prop(into)] label: String,
    value: Signal<String>,
    options: Vec<(String, String)>,
    on_change: Callback<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <label class="musaic-field">
            <span class="musaic-field-label">{label}</span>
            <select
                class="musaic-select"
                disabled=move || disabled.get()
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

#[component]
fn FieldNote(
    #[prop(into, optional)] help: String,
    #[prop(into, optional)] error: String,
) -> impl IntoView {
    let show_error = !error.is_empty();
    let show_help = !help.is_empty() && !show_error;
    view! {
        {show_error
            .then(|| {
                view! {
                    <div class="musaic-field-footer">
                        <span class="musaic-field-error">{error.clone()}</span>
                    </div>
                }
            })}
        {show_help
            .then(|| {
                view! {
                    <div class="musaic-field-footer">
                        <span class="musaic-field-help">{help.clone()}</span>
                    </div>
                }
            })}
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

#[cfg(test)]
mod tests {
    use super::{hex_to_rgb, rgb_to_hex};

    #[test]
    fn hex_round_trips_through_rgb() {
        assert_eq!(rgb_to_hex([1.0, 0.0, 0.0]), "#ff0000");
        assert_eq!(rgb_to_hex([0.0, 1.0, 0.0]), "#00ff00");
        let parsed = hex_to_rgb("#3366cc").expect("valid hex");
        assert_eq!(rgb_to_hex(parsed), "#3366cc");
    }

    #[test]
    fn malformed_hex_is_rejected() {
        assert!(hex_to_rgb("3366cc").is_none());
        assert!(hex_to_rgb("#fff").is_none());
        assert!(hex_to_rgb("#gggggg").is_none());
    }
}
