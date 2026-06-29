# leptos-musaic

A feature-gated [Leptos](https://leptos.dev) component library for building beautiful UIs that
run the same code natively (in a webview) and on the web (wasm). musaic is the UI patterns behind
apps like the nightshade editor, extracted into a reusable, engine-agnostic crate.

The goal is the same one `nightshade-api` has for nightshade: let you assemble a complex,
production-grade Leptos UI with very little code, without giving up control. Components default to
sensible behavior and expose callbacks and reactive props for the moments you need to steer them.
Everything is themed from one set of CSS custom properties, so the whole surface — including every
component added here — restyles with a single `data-theme` switch.

```toml
[dependencies]
leptos-musaic = { version = "0.1", features = ["forms", "themes", "command-palette"] }
```

```rust
use leptos::prelude::*;
use leptos_musaic::{MusaicStyles, ThemeProvider, ThemePicker, Panel, Button};

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

Prefer a single import? `use leptos_musaic::prelude::*;` pulls in every enabled component plus
`leptos::prelude::*`, so a typical module needs just that one line.

## Feature gates

The base layer (design tokens, theming, `Button`, `IconButton`, `Panel`, `Card`, layout,
`ResizeHandle`, `Modal`, `Badge`, `Progress`, `Tooltip`, `Spinner`, toasts) is always compiled.
Everything else is opt-in:

| feature | components |
| --- | --- |
| `forms` | `NumberField` (min/max, integer mode), `CheckField`, `Switch`, `TextField`, `SliderField`, `ColorField`, `Select`, all with `disabled` and inline help/error |
| `menus` | `Menu`, `MenuItem`, `ContextMenu`, `TabBar` |
| `themes` | the bundled themes plus `ThemePicker` |
| `command-palette` | `CommandPalette` (Ctrl+K fuzzy commands) |
| `code-editor` | `CodeEditor` (textarea over a synced highlight layer) |
| `table` | `Table` (optional click-to-sort, numeric-aware, row selection, `aria-sort`) |
| `tree` | `Tree`, `TreeItem` (collapsible hierarchy, selection, icons, arrow-key navigation) |
| `inspector` | `Inspector`, `InspectorSection`, `InspectorRow` (collapsible property panels) |
| `viewport` | `Viewport`, `Bridge`, `Loader`, `WebGpuGate`: a worker-backed render surface, engine-agnostic |
| `engine` | `use_engine`, `EngineViewport`: turnkey wiring (input, keyboard, lifecycle) over the shared protocol |
| `nightshade` | engine-shaped UI: the rhai highlighter and `SelectedCard` |

`default = ["forms", "menus", "themes"]`. Use `full` to turn on everything.

## Theming

Every component draws its colors from a small set of semantic CSS custom properties
(`--musaic-accent`, `--musaic-panel`, `--musaic-text`, `--musaic-danger`, and friends). A theme is
just a block that overrides those tokens, selected by `data-theme` on the document element —
`ThemeProvider` and `ThemePicker` handle that and persist the choice to `localStorage`. Because the
new primitives (`Badge`, `Card`, `Progress`, `Tooltip`, `Switch`, table/tree selection) are all
built from the same tokens, they restyle automatically across the nine bundled themes, and a
consumer can add a theme without touching any component. The stylesheet is wrapped in
`@layer musaic`, so your own CSS always wins.

## Accessibility

Interactive components ship with the roles and keyboard behavior assistive tech expects: `Modal`
traps Tab focus, closes on `Escape`, and restores focus on close; `CommandPalette` is a
`listbox`/`combobox` with `aria-activedescendant`; `Tree` exposes `treeitem`/`aria-expanded` with
arrow-key navigation; `Table` reports `aria-sort`; `Switch` is a `role="switch"`; menus and tabs
carry `menu`/`menuitem` and `tablist`/`tab` semantics. Focus-visible outlines use the theme accent.

## Engine integration

musaic's core never links a game engine. Two layers sit on top, both optional:

- The `viewport` feature gives a generic render surface: it owns the canvas/`OffscreenCanvas`
  handoff and the pointer/touch/wheel bookkeeping, emitting `ViewportEvent`s you map to your own
  protocol.
- The `engine` feature goes further. `use_engine("worker.js")` returns a standard `EngineState`
  (ready, adapter, fps, entities, selection, grabbing) plus a bridge, with all the wiring done:
  keyboard forwarding, input mapping, and lifecycle decoding. You send app-specific messages with
  `engine.send(&YourCommand)` and receive them with `engine.on_custom(...)`. Your page drops the
  boilerplate and keeps only its panels.

The wire types live in the tiny no-deps `leptos-musaic-protocol` crate, shared by page and worker, so
there is one source of truth and no drift.

## Crates

- `leptos-musaic` (`crates/musaic`): the component library.
- `leptos-musaic-protocol` (`crates/musaic-protocol`): the shared page/worker wire types (`serde` only).
- `leptos-musaic-shell` (`crates/musaic-shell`): a reusable native shell (`wry` + `winit`) that serves
  a built web bundle into a desktop window. `leptos_musaic_shell::run(title, get)`.
- `leptos-musaic-engine` (`crates/musaic-engine`): the worker-side driver for nightshade apps.
  `run_offscreen(scene, setup, tick, on_custom)` owns the render loop, input injection, picking, and
  stats. This is the only crate that links `nightshade-api`.

A new nightshade app is roughly: `use_engine()` plus your panels on the page, and
`run_offscreen(scene, setup, tick, on_custom)` plus your scene logic in the worker. The repeated
wiring is gone.

## Example: `examples/nightshade_demo`

A full nightshade integration built entirely from musaic components: a toolbar, a sidebar of live
scene controls, an embedded viewport, a resizable script/log dock, a command palette, and nine
themes, driving the nightshade engine (from crates.io) in a web worker on an `OffscreenCanvas`. It
runs on the web (`just dev`) and as a native desktop app (`just run`) from the same code.

```
just init    # install the pinned toolchain (rust, wasm-bindgen, wasm-opt, trunk) via mise
just dev     # serve the demo at http://127.0.0.1:8080
just run     # build the bundle and open it in a native webview window
just dist    # production web bundle in examples/nightshade_demo/dist
just check   # type-check the library (all features) and the demo workspace
just lint    # clippy, denying warnings
just test    # run the unit tests on the host target
```

The demo deploys to GitHub Pages on every push to `main` via `.github/workflows/pages.yml`.

## License

MIT OR Apache-2.0.
