//! Schema-driven form rendering: describe fields with [`FormField`] and [`FieldSchema`]
//! and let [`DynamicForm`] build the inputs and collect their values as JSON.

use leptos::prelude::*;
use serde_json::{Map, Value};

/// The type of a form field, which determines the control [`DynamicForm`] renders and the
/// JSON shape of its value.
#[derive(Clone)]
pub enum FieldSchema {
    Number {
        min: Option<f64>,
        max: Option<f64>,
        integer: bool,
    },
    Text,
    Bool,
    Enum(Vec<String>),
    Vector(usize),
}

/// A single field in a [`DynamicForm`], pairing a JSON `key` and display `label` with its
/// [`FieldSchema`].
#[derive(Clone)]
pub struct FormField {
    /// The key under which this field's value is stored in the output JSON object.
    pub key: String,
    /// The label shown next to the control.
    pub label: String,
    /// The field type describing which control to render.
    pub schema: FieldSchema,
}

impl FormField {
    /// Creates a [`FormField`] from a key, label, and schema.
    pub fn new(key: impl Into<String>, label: impl Into<String>, schema: FieldSchema) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            schema,
        }
    }
}

fn default_value(schema: &FieldSchema) -> Value {
    match schema {
        FieldSchema::Number { .. } => Value::from(0.0),
        FieldSchema::Text => Value::from(""),
        FieldSchema::Bool => Value::from(false),
        FieldSchema::Enum(options) => Value::from(options.first().cloned().unwrap_or_default()),
        FieldSchema::Vector(size) => Value::from(vec![0.0_f64; *size]),
    }
}

type FormState = RwSignal<Map<String, Value>>;

fn number_control(
    key: String,
    min: Option<f64>,
    max: Option<f64>,
    integer: bool,
    state: FormState,
    emit: impl Fn() + Copy + 'static,
) -> AnyView {
    let read_key = key.clone();
    view! {
        <input
            type="number"
            min=min.map(|value| value.to_string())
            max=max.map(|value| value.to_string())
            step=if integer { "1" } else { "any" }
            prop:value=move || {
                state.with(|map| {
                    map.get(&read_key).and_then(Value::as_f64).unwrap_or(0.0).to_string()
                })
            }
            on:input=move |event| {
                if let Ok(parsed) = event_target_value(&event).parse::<f64>() {
                    let parsed = if integer { parsed.round() } else { parsed };
                    state.update(|map| {
                        map.insert(key.clone(), Value::from(parsed));
                    });
                    emit();
                }
            }
        />
    }
    .into_any()
}

fn vector_control(
    key: String,
    size: usize,
    state: FormState,
    emit: impl Fn() + Copy + 'static,
) -> AnyView {
    let axes = (0..size)
        .map(|index| {
            let key = key.clone();
            let read_key = key.clone();
            view! {
                <input
                    type="number"
                    step="any"
                    class="musaic-vec-input"
                    prop:value=move || {
                        state.with(|map| {
                            map.get(&read_key)
                                .and_then(Value::as_array)
                                .and_then(|array| array.get(index))
                                .and_then(Value::as_f64)
                                .unwrap_or(0.0)
                                .to_string()
                        })
                    }
                    on:input=move |event| {
                        if let Ok(parsed) = event_target_value(&event).parse::<f64>() {
                            state.update(|map| {
                                let mut values = map
                                    .get(&key)
                                    .and_then(Value::as_array)
                                    .map(|array| {
                                        array
                                            .iter()
                                            .map(|value| value.as_f64().unwrap_or(0.0))
                                            .collect::<Vec<_>>()
                                    })
                                    .unwrap_or_else(|| vec![0.0; size]);
                                if values.len() < size {
                                    values.resize(size, 0.0);
                                }
                                values[index] = parsed;
                                map.insert(key.clone(), Value::from(values));
                            });
                            emit();
                        }
                    }
                />
            }
        })
        .collect_view();
    view! { <div class="musaic-vec-field">{axes}</div> }.into_any()
}

fn field_view(field: FormField, state: FormState, emit: impl Fn() + Copy + 'static) -> AnyView {
    let label = field.label.clone();
    let control = match field.schema {
        FieldSchema::Number { min, max, integer } => {
            number_control(field.key, min, max, integer, state, emit)
        }
        FieldSchema::Text => {
            let key = field.key.clone();
            let read_key = field.key.clone();
            view! {
                <input
                    type="text"
                    prop:value=move || {
                        state.with(|map| {
                            map.get(&read_key)
                                .and_then(Value::as_str)
                                .unwrap_or("")
                                .to_string()
                        })
                    }
                    on:input=move |event| {
                        state.update(|map| {
                            map.insert(key.clone(), Value::from(event_target_value(&event)));
                        });
                        emit();
                    }
                />
            }
            .into_any()
        }
        FieldSchema::Bool => {
            let key = field.key.clone();
            let read_key = field.key.clone();
            view! {
                <input
                    type="checkbox"
                    prop:checked=move || {
                        state.with(|map| {
                            map.get(&read_key).and_then(Value::as_bool).unwrap_or(false)
                        })
                    }
                    on:change=move |event| {
                        state.update(|map| {
                            map.insert(key.clone(), Value::from(event_target_checked(&event)));
                        });
                        emit();
                    }
                />
            }
            .into_any()
        }
        FieldSchema::Enum(options) => {
            let key = field.key.clone();
            let read_key = field.key.clone();
            view! {
                <select
                    class="musaic-select"
                    prop:value=move || {
                        state.with(|map| {
                            map.get(&read_key)
                                .and_then(Value::as_str)
                                .unwrap_or("")
                                .to_string()
                        })
                    }
                    on:change=move |event| {
                        state.update(|map| {
                            map.insert(key.clone(), Value::from(event_target_value(&event)));
                        });
                        emit();
                    }
                >
                    {options
                        .into_iter()
                        .map(|option| view! { <option value=option.clone()>{option.clone()}</option> })
                        .collect_view()}
                </select>
            }
            .into_any()
        }
        FieldSchema::Vector(size) => vector_control(field.key, size, state, emit),
    };
    view! {
        <label class="musaic-field">
            <span class="musaic-field-label">{label}</span>
            {control}
        </label>
    }
    .into_any()
}

/// Renders a form from a list of [`FormField`]s, maintaining the collected values as a
/// JSON object. Emits the object through `on_change` on every edit and, when an
/// `on_submit` callback is provided, shows a submit button labelled `submit_label`.
#[component]
pub fn DynamicForm(
    fields: Vec<FormField>,
    #[prop(optional)] on_change: Option<Callback<Value>>,
    #[prop(optional)] on_submit: Option<Callback<Value>>,
    #[prop(into, optional)] submit_label: String,
) -> impl IntoView {
    let state: FormState = RwSignal::new({
        let mut map = Map::new();
        for field in &fields {
            map.insert(field.key.clone(), default_value(&field.schema));
        }
        map
    });
    let emit = move || {
        if let Some(callback) = on_change {
            callback.run(Value::Object(state.get_untracked()));
        }
    };
    let submit_label = if submit_label.is_empty() {
        "Submit".to_string()
    } else {
        submit_label
    };
    let rows = fields
        .into_iter()
        .map(|field| field_view(field, state, emit))
        .collect_view();
    view! {
        <div class="musaic-dynamic-form">
            {rows}
            {on_submit
                .map(|callback| {
                    view! {
                        <button
                            class="musaic-button primary"
                            on:click=move |_| callback.run(Value::Object(state.get_untracked()))
                        >
                            {submit_label}
                        </button>
                    }
                })}
        </div>
    }
}
