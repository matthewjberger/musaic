use leptos::prelude::*;

fn parse_number(chars: &[char], pos: &mut usize) -> Option<f64> {
    let start = *pos;
    while *pos < chars.len() && (chars[*pos].is_ascii_digit() || chars[*pos] == '.') {
        *pos += 1;
    }
    if *pos == start {
        return None;
    }
    chars[start..*pos].iter().collect::<String>().parse().ok()
}

fn parse_factor(chars: &[char], pos: &mut usize) -> Option<f64> {
    if *pos >= chars.len() {
        return None;
    }
    match chars[*pos] {
        '-' => {
            *pos += 1;
            Some(-parse_factor(chars, pos)?)
        }
        '+' => {
            *pos += 1;
            parse_factor(chars, pos)
        }
        '(' => {
            *pos += 1;
            let value = parse_expr(chars, pos)?;
            if *pos < chars.len() && chars[*pos] == ')' {
                *pos += 1;
                Some(value)
            } else {
                None
            }
        }
        _ => parse_number(chars, pos),
    }
}

fn parse_term(chars: &[char], pos: &mut usize) -> Option<f64> {
    let mut left = parse_factor(chars, pos)?;
    while *pos < chars.len() && matches!(chars[*pos], '*' | '/') {
        let operator = chars[*pos];
        *pos += 1;
        let right = parse_factor(chars, pos)?;
        left = if operator == '*' {
            left * right
        } else {
            left / right
        };
    }
    Some(left)
}

fn parse_expr(chars: &[char], pos: &mut usize) -> Option<f64> {
    let mut left = parse_term(chars, pos)?;
    while *pos < chars.len() && matches!(chars[*pos], '+' | '-') {
        let operator = chars[*pos];
        *pos += 1;
        let right = parse_term(chars, pos)?;
        left = if operator == '+' {
            left + right
        } else {
            left - right
        };
    }
    Some(left)
}

fn eval_expr(input: &str) -> Option<f64> {
    let chars: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    let mut pos = 0;
    let value = parse_expr(&chars, &mut pos)?;
    if pos == chars.len() && value.is_finite() {
        Some(value)
    } else {
        None
    }
}

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
    #[prop(optional)] default: Option<f64>,
    #[prop(optional)] validate: Option<Callback<f64, Option<String>>>,
    on_change: Callback<(f64, bool)>,
) -> impl IntoView {
    let step = step.unwrap_or(if integer { 1.0 } else { 0.1 });
    let validation = RwSignal::new(String::new());
    let error_signal = Signal::derive(move || {
        if error.is_empty() {
            validation.get()
        } else {
            error.clone()
        }
    });
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
        let parsed = raw.parse::<f64>().ok().or_else(|| eval_expr(&raw));
        match parsed {
            Some(parsed) => {
                let clamped = clamp(parsed);
                if let Some(validate) = validate {
                    validation.set(validate.run(clamped).unwrap_or_default());
                } else {
                    validation.set(String::new());
                }
                on_change.run((clamped, committed));
            }
            None if committed && !raw.trim().is_empty() => {
                validation.set("Enter a number or expression".to_string());
            }
            None => {}
        }
    };
    let live = move |raw: String| {
        if let Ok(parsed) = raw.parse::<f64>() {
            on_change.run((clamp(parsed), false));
        }
    };
    let label_ref = NodeRef::<leptos::html::Span>::new();
    let drag = StoredValue::new(None::<(f64, f64)>);
    let on_scrub_down = move |event: web_sys::PointerEvent| {
        event.prevent_default();
        drag.set_value(Some((event.client_x() as f64, value.get_untracked())));
        if let Some(node) = label_ref.get() {
            let _ = node.set_pointer_capture(event.pointer_id());
        }
    };
    let on_scrub_move = move |event: web_sys::PointerEvent| {
        if let Some((start_x, start_value)) = drag.get_value() {
            let delta = event.client_x() as f64 - start_x;
            on_change.run((clamp(start_value + delta * step), false));
        }
    };
    let on_scrub_up = move |event: web_sys::PointerEvent| {
        if drag.get_value().is_some() {
            drag.set_value(None);
            on_change.run((value.get_untracked(), true));
            if let Some(node) = label_ref.get() {
                let _ = node.release_pointer_capture(event.pointer_id());
            }
        }
    };
    view! {
        <div class="musaic-field-group">
            <label class="musaic-field">
                <span
                    node_ref=label_ref
                    class="musaic-field-label musaic-scrub"
                    on:pointerdown=on_scrub_down
                    on:pointermove=on_scrub_move
                    on:pointerup=on_scrub_up
                >
                    {label}
                </span>
                <input
                    type="text"
                    inputmode="decimal"
                    step=step
                    disabled=move || disabled.get()
                    prop:value=move || format_value(value.get())
                    on:input=move |event| live(event_target_value(&event))
                    on:change=move |event| commit(event_target_value(&event), true)
                />
                {default
                    .map(|fallback| {
                        view! {
                            <span
                                class="musaic-field-reset"
                                role="button"
                                tabindex="0"
                                title="Reset to default"
                                class:hidden=move || value.get() == fallback
                                on:click=move |_| on_change.run((fallback, true))
                            >
                                "\u{21ba}"
                            </span>
                        }
                    })}
            </label>
            <FieldNote help=help error=error_signal />
        </div>
    }
}

#[component]
pub fn Vec3Field(
    #[prop(into)] label: String,
    value: Signal<[f64; 3]>,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(into, optional)] disabled: Signal<bool>,
    on_change: Callback<([f64; 3], bool)>,
) -> impl IntoView {
    let clamp = move |parsed: f64| {
        let mut result = parsed;
        if let Some(min) = min {
            result = result.max(min);
        }
        if let Some(max) = max {
            result = result.min(max);
        }
        result
    };
    let axis = move |index: usize, tag: &'static str| {
        let apply = move |raw: String, committed: bool| {
            let parsed = if committed {
                raw.parse::<f64>().ok().or_else(|| eval_expr(&raw))
            } else {
                raw.parse::<f64>().ok()
            };
            if let Some(parsed) = parsed {
                let mut next = value.get_untracked();
                next[index] = clamp(parsed);
                on_change.run((next, committed));
            }
        };
        view! {
            <div class="musaic-vec-axis">
                <span class="musaic-vec-tag">{tag}</span>
                <input
                    type="text"
                    inputmode="decimal"
                    step=step.unwrap_or(0.1)
                    disabled=move || disabled.get()
                    prop:value=move || format!("{:.3}", value.get()[index])
                    on:input=move |event| apply(event_target_value(&event), false)
                    on:change=move |event| apply(event_target_value(&event), true)
                />
            </div>
        }
    };
    view! {
        <div class="musaic-field-group">
            <span class="musaic-field-label">{label}</span>
            <div class="musaic-vec-field">
                {axis(0, "X")} {axis(1, "Y")} {axis(2, "Z")}
            </div>
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
    #[prop(optional)] debounce: Option<u32>,
) -> impl IntoView {
    let error_signal = Signal::derive(move || error.clone());
    let generation = StoredValue::new(0u32);
    let on_input = move |event: web_sys::Event| {
        let Some(delay) = debounce else {
            return;
        };
        let text = event_target_value(&event);
        let current = generation.get_value().wrapping_add(1);
        generation.set_value(current);
        set_timeout(
            move || {
                if generation.get_value() == current {
                    on_commit.run(text.clone());
                }
            },
            std::time::Duration::from_millis(delay as u64),
        );
    };
    view! {
        <div class="musaic-field-group">
            <label class="musaic-field">
                <span class="musaic-field-label">{label}</span>
                <input
                    type="text"
                    placeholder=placeholder
                    disabled=move || disabled.get()
                    prop:value=move || value.get()
                    on:input=on_input
                    on:change=move |event| {
                        if debounce.is_none() {
                            on_commit.run(event_target_value(&event));
                        }
                    }
                />
            </label>
            <FieldNote help=help error=error_signal />
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
pub fn ChipGroup(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("musaic-chip-group {class}")>{children()}</div> }
}

#[component]
pub fn ToggleChip(
    #[prop(into)] label: String,
    #[prop(into)] active: Signal<bool>,
    on_toggle: Callback<()>,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="musaic-chip"
            class:active=move || active.get()
            aria-pressed=move || active.get().to_string()
            disabled=move || disabled.get()
            on:click=move |_| on_toggle.run(())
        >
            {label}
        </button>
    }
}

#[component]
pub fn TagInput(
    #[prop(into)] tags: Signal<Vec<String>>,
    on_add: Callback<String>,
    on_remove: Callback<String>,
    #[prop(into, optional)] placeholder: String,
) -> impl IntoView {
    let draft = RwSignal::new(String::new());
    let placeholder = if placeholder.is_empty() {
        "Add tag…".to_string()
    } else {
        placeholder
    };
    let submit = move || {
        let value = draft.get_untracked().trim().to_string();
        if !value.is_empty() {
            on_add.run(value);
            draft.set(String::new());
        }
    };
    view! {
        <div class="musaic-tag-input">
            <For each=move || tags.get() key=|tag| tag.clone() let:tag>
                {
                    let removed = tag.clone();
                    view! {
                        <span class="musaic-tag">
                            {tag.clone()}
                            <button
                                class="musaic-tag-remove"
                                aria-label="Remove tag"
                                on:click=move |_| on_remove.run(removed.clone())
                            >
                                "\u{00d7}"
                            </button>
                        </span>
                    }
                }
            </For>
            <input
                type="text"
                class="musaic-tag-field"
                placeholder=placeholder
                prop:value=move || draft.get()
                on:input=move |event| draft.set(event_target_value(&event))
                on:keydown=move |event| {
                    if event.key() == "Enter" {
                        event.prevent_default();
                        submit();
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn Swatch(
    #[prop(into)] color: String,
    #[prop(into, optional)] active: Signal<bool>,
    #[prop(optional)] on_select: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="musaic-swatch"
            class:active=move || active.get()
            title=color.clone()
            style=format!("background:{color}")
            on:click=move |_| {
                if let Some(callback) = on_select {
                    callback.run(());
                }
            }
        ></button>
    }
}

#[component]
pub fn SwatchPalette(
    colors: Vec<String>,
    #[prop(into)] selected: Signal<String>,
    on_select: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="musaic-swatch-palette">
            {colors
                .into_iter()
                .map(|color| {
                    let value = color.clone();
                    let compare = color.clone();
                    view! {
                        <Swatch
                            color=color
                            active=Signal::derive(move || selected.get() == compare)
                            on_select=Callback::new(move |_| on_select.run(value.clone()))
                        />
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn FieldNote(
    #[prop(into, optional)] help: String,
    #[prop(into)] error: Signal<String>,
) -> impl IntoView {
    let show_help = !help.is_empty();
    view! {
        <Show when=move || !error.get().is_empty() fallback=|| ()>
            <div class="musaic-field-footer">
                <span class="musaic-field-error">{move || error.get()}</span>
            </div>
        </Show>
        {show_help
            .then(|| {
                let help = help.clone();
                view! {
                    <Show when=move || error.get().is_empty() fallback=|| ()>
                        <div class="musaic-field-footer">
                            <span class="musaic-field-help">{help.clone()}</span>
                        </div>
                    </Show>
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
    use super::{eval_expr, hex_to_rgb, rgb_to_hex};

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

    #[test]
    fn evaluates_arithmetic_with_precedence_and_parens() {
        assert_eq!(eval_expr("2+3*4"), Some(14.0));
        assert_eq!(eval_expr("(2+3)*4"), Some(20.0));
        assert_eq!(eval_expr("-1.5 + 2"), Some(0.5));
        assert_eq!(eval_expr("10/4"), Some(2.5));
    }

    #[test]
    fn rejects_malformed_expressions() {
        assert!(eval_expr("2+").is_none());
        assert!(eval_expr("abc").is_none());
        assert!(eval_expr("1/0").is_none());
        assert!(eval_expr("(1+2").is_none());
    }
}
