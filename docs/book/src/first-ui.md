# Your First UI

A musaic UI is an ordinary Leptos CSR app. You mount a root component, drop the stylesheet and a
theme provider near the top, and compose musaic components from there.

## Mounting

```rust
fn main() {
    console_error_panic_hook::set_once();
    leptos::prelude::mount_to_body(App);
}
```

## The minimum viable UI

```rust
use leptos_musaic::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <MusaicStyles/>
        <ThemeProvider>
            <Panel title="Hello">
                <ThemePicker/>
                <Button>"Click me"</Button>
            </Panel>
        </ThemeProvider>
    }
}
```

That is a complete, themed app. Two pieces make it work:

- `<MusaicStyles/>` injects the design-token stylesheet into the document head, once. It is wrapped
  in `@layer musaic`, so any unlayered CSS you write always wins. There is no file to copy.
- `<ThemeProvider>` sets `data-theme` on the document element (persisted to `localStorage`) and
  makes `use_theme()` and `register_theme()` available to descendants. `ThemePicker` is a dropdown
  of the bundled themes.

## Adding state and interaction

State lives in `RwSignal`s. Reactive props take a `Signal`; events take a `Callback`.

```rust
#[component]
fn Counter() -> impl IntoView {
    let count = RwSignal::new(0);
    view! {
        <Panel title="Counter">
            <p>{move || format!("count: {}", count.get())}</p>
            <Button on_click=Callback::new(move |_| count.update(|n| *n += 1))>
                "Increment"
            </Button>
        </Panel>
    }
}
```

The next chapter, [Reactivity and Handles](reactivity.md), covers the patterns musaic apps use to
pass state around cleanly. Then [The App Shell](app-shell.md) shows how to grow this from a single
panel into a full editor layout.
