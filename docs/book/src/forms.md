# Forms and Inputs

The `forms` feature covers the inputs a control panel needs. Every field takes a `value` signal and
emits changes through an `on_change` callback, so they slot straight into the handle pattern.

## Fields

- `NumberField`: numeric input that evaluates arithmetic expressions (`2 * pi`), supports
  drag-to-scrub on its label, a reset affordance, and live validation.
- `SliderField`: a labelled slider with min/max/step. Its `on_change` reports `(value, committed)`
  so you can act on every drag frame and know when the drag ends.
- `Vec3Field`: three number fields for an `[f32; 3]`.
- `ColorField`: a color picker over an `[f32; 3]`, also reporting `(value, committed)`.
- `Switch`: a `role="switch"` toggle over a `bool`.
- `CheckField`: a labelled checkbox.
- `TextField`: a text input with optional `debounce`.
- `Select`: a labelled dropdown over `(value, label)` options.
- `ChipGroup` / `ToggleChip`, `TagInput`, `Swatch` / `SwatchPalette`: chips, free-form tags, and
  color swatches.

## Example

```rust
let speed = RwSignal::new(1.0);
let sky = RwSignal::new("nebula".to_string());

view! {
    <SliderField
        label="Speed"
        value=Signal::derive(move || speed.get())
        min=Signal::derive(|| 0.0)
        max=Signal::derive(|| 4.0)
        step=0.05
        on_change=Callback::new(move |(v, _committed): (f64, bool)| speed.set(v))
    />
    <Select
        label="Sky"
        value=Signal::derive(move || sky.get())
        options=vec![
            ("nebula".into(), "Nebula".into()),
            ("sky".into(), "Sky".into()),
        ]
        on_change=Callback::new(move |v: String| sky.set(v))
    />
}
```

## Schema-driven forms

When the fields are data rather than code, `DynamicForm` (the `dynamic-form` feature) builds a form
from a `Vec<FormField>` of `FieldSchema`s and emits the result as JSON. Reach for it for settings
panels and generated inspectors; reach for the individual fields when you want full control over
layout and wiring.

Group related fields under `Inspector` / `InspectorSection` (see [Displaying Data](data.md)) to get
the collapsible, labelled sidebar look.
