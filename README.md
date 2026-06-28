# musaic

A feature-gated [Leptos](https://leptos.dev) component library for building beautiful UIs that
run the same code natively (in a webview) and on the web (wasm). musaic is the UI patterns behind
apps like the nightshade editor, extracted into a reusable, engine-agnostic crate.

```toml
[dependencies]
musaic = { version = "0.1", features = ["forms", "themes", "command-palette"] }
```

```rust
use leptos::prelude::*;
use musaic::{MusaicStyles, ThemeProvider, ThemePicker, Panel, Button};

#[component]
fn App() -> impl IntoView {
    view! {
        <MusaicStyles />
        <ThemeProvider>
            <Panel title="Hello">
                <ThemePicker />
                <Button>"Click me"</Button>
            </Panel>
        </ThemeProvider>
    }
}
```

Drop `<MusaicStyles/>` at the root once: it injects the design-token stylesheet (wrapped in
`@layer musaic`, so your own CSS always wins) into the document head. No file to copy, no build step.

## Feature gates

The base layer (design tokens, theming, `Button`, `Panel`, layout, `Modal`, `Spinner`, toasts) is
always compiled. Everything else is opt-in:

| feature | components |
| --- | --- |
| `forms` | `NumberField`, `CheckField`, `TextField`, `SliderField`, `ColorField`, `Select` |
| `menus` | `Menu`, `MenuItem`, `ContextMenu`, `TabBar` |
| `themes` | the bundled themes + `ThemePicker` |
| `command-palette` | `CommandPalette` (`Ctrl+K` fuzzy commands) |
| `code-editor` | `CodeEditor` (textarea over a synced highlight layer) |
| `viewport` | `Viewport`, `Bridge`, `Loader`, `WebGpuGate` — a worker-backed render surface, engine-agnostic |
| `nightshade` | `viewport` + `code-editor` plus engine-shaped UI: the rhai highlighter and `SelectedCard` |

`default = ["forms", "menus", "themes"]`. Use `full` to turn on everything.

## Engine integration

musaic's core never links a game engine. The `viewport` feature gives you a generic render surface:
it owns the canvas/`OffscreenCanvas` handoff, pointer/touch/wheel bookkeeping, and a `Bridge` that
speaks any `serde` protocol you define — you wire its events to your own worker messages. The
optional `nightshade` feature layers on UI shaped for the
[nightshade](https://crates.io/crates/nightshade-api) engine without ever pulling the engine into
your page bundle.

## Crates

- `crates/musaic` — the component library.
- `crates/musaic-shell` — a reusable native shell (`wry` + `winit`) that serves a built web bundle
  into a desktop window. `musaic_shell::run(title, get)`.

## Example: `examples/nightshade_demo`

A full nightshade integration: a Leptos page built entirely from musaic components, driving the
nightshade engine (from crates.io) in a web worker on an `OffscreenCanvas`. It runs on the web
(`just dev`) and as a native desktop app (`just run`) from the same code.

```
just init    # install the pinned toolchain (rust, wasm-bindgen, wasm-opt, trunk) via mise
just dev     # serve the demo at http://127.0.0.1:8080
just run     # build the bundle and open it in a native webview window
just dist    # production web bundle in examples/nightshade_demo/dist
just check   # type-check the library (all features) and the demo workspace
just lint    # clippy, denying warnings
```

The demo deploys to GitHub Pages on every push to `main` via `.github/workflows/pages.yml`.

## License

MIT OR Apache-2.0.
