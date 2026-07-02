# leptos-musaic

A feature-gated [Leptos](https://leptos.dev) component library for building beautiful UIs that
run the same code natively (in a webview) and on the web (wasm). It is a reusable, engine-agnostic
set of UI patterns for editor-style and tool apps.

The goal: let you assemble a complex, production-grade Leptos UI with very little code, without
giving up control. Components default to sensible behavior and expose callbacks and reactive props
for the moments you need to steer them. Everything is themed from one set of CSS custom properties,
so the whole surface (including every component added here) restyles with a single `data-theme`
switch.

## Quickstart

musaic is a client-side-rendered (CSR) Leptos app. This is the whole path from an empty crate to a
themed, running UI.

**1. Add the dependencies.** musaic is consumed as a git dependency (it is not on crates.io):

```toml
[dependencies]
leptos = { version = "0.7", features = ["csr"] }
leptos-musaic = { git = "https://github.com/matthewjberger/musaic", features = ["forms", "themes", "command-palette"] }
console_error_panic_hook = "0.1"
```

Enable the features whose components you use, or `features = ["full"]` for everything. See
[Feature gates](#feature-gates).

**2. Mount the root component** (`src/main.rs`):

```rust
fn main() {
    console_error_panic_hook::set_once();
    leptos::prelude::mount_to_body(app::App);
}
```

**3. Write the UI** (`src/app.rs`). `use leptos_musaic::prelude::*;` pulls in every enabled
component plus `leptos::prelude::*`, so one import is usually enough:

```rust
use leptos_musaic::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let count = RwSignal::new(0);
    view! {
        <MusaicStyles/>
        <ThemeProvider>
            <Panel title="Hello">
                <ThemePicker/>
                <Button on_click=Callback::new(move |_| count.update(|n| *n += 1))>
                    "clicked " {move || count.get()}
                </Button>
            </Panel>
        </ThemeProvider>
    }
}
```

Two pieces make this work. `<MusaicStyles/>`, dropped once at the root, injects the design-token
stylesheet (wrapped in `@layer musaic`, so your own CSS always wins) into the document head, no file
to copy and no build step. `<ThemeProvider>` sets `data-theme` on the document, persists the choice,
and makes theming available to everything inside it.

**4. Add an `index.html`** at the crate root for [Trunk](https://trunkrs.dev):

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8"/>
    <link data-trunk rel="rust" data-wasm-opt="z"/>
  </head>
  <body></body>
</html>
```

**5. Run it:**

```
trunk serve --open
```

This exact app is a runnable crate at [`examples/minimal`](examples/minimal); `just run-minimal`
serves it. From here, grow the single `Panel` into a full layout with
[`EditorShell`](docs/book/src/app-shell.md) and reach for components by task. The
[guide](docs/book) and the runnable [gallery](examples/gallery) are the next stops.

## Documentation

- The **guide** is an mdbook under `docs/book` (`just book` to build, `just book-serve` to read it
  live). Start with `introduction.md`, then Getting Started and the task-by-task chapters.
- The **API reference** is the rustdoc: `just doc` (or `cargo doc --features full --open`). Every
  component and public type is documented.
- The **gallery** (`examples/gallery`) is the living reference: a runnable demo of every component.
  `just run-gallery`.
- `CLAUDE.md` is a dense orientation for building UIs with musaic.

## Feature gates

The base layer (design tokens, typed theming, `Button`, `IconButton`, `Panel`, `Card`, layout,
`ResizeHandle`, `Overlay`/`Modal`, `Badge`, `Progress`, `Tooltip`, `Spinner`, toasts
(info/success/warning/error, dismiss, inline actions), the `CommandRegistry`, the `KeymapProvider`,
the `EditorShell` frame, and the `use_persisted` / `use_reconnecting_socket` / `download_text` /
`pick_file_text` hooks) is always compiled. Everything else is opt-in:

| feature | components |
| --- | --- |
| `forms` | `NumberField` (expression input, `validate`, live drag-to-scrub, reset), `Vec3Field`, `CheckField`, `Switch`, `TextField` (optional `debounce`), `SliderField`, `ColorField`, `Select`, `ChipGroup`/`ToggleChip`, `TagInput`, `Swatch`/`SwatchPalette` |
| `menus` | `Menu`, `MenuItem` (checkable, disabled, shortcuts), `Submenu`, `MenuSeparator`, `ContextMenu` (portalled), `TabBar`, all keyboard-navigable |
| `themes` | the bundled themes, `register_theme` for custom typed themes, `ThemePicker`, and `ThemeMenu` (hover-preview) |
| `command-palette` | `CommandPalette` (registry-driven, fuzzy-ranked, recents, keybinding hints, nested submenus) |
| `code-editor` | `CodeEditor` (highlight layer, optional gutter, diagnostic markers, find/replace), `CodeTabs` for multiple documents, and `highlight_code`, a generic keyword/command highlighter |
| `table` | `Table` (multi-column sort, filter, column resize + show/hide, sticky header, pagination, inline cell edit, optional virtualization) |
| `tree` | `Tree`, `TreeItem` (collapsible hierarchy, multi-select, inline rename, drag-and-drop, lazy expand, arrow-key nav) |
| `inspector` | `Inspector`, `InspectorSection` (header actions slot), `InspectorRow` |
| `dock` | `DockLayout`, `DockPanel`, `DockMain`: resizable, collapsible panels docked around a main region |
| `overlays` | `Popover` (anchored positioning that flips/shifts to stay on screen), `Dropdown`, `Combobox`, `Dialog` |
| `virtual-list` | `VirtualList`: a windowed-rendering primitive over any item count and render closure |
| `diff` | `Diff` + `diff_lines`: an LCS line-diff with +/- markers and old/new line numbers |
| `drag` | pointer-based drag-and-drop (works in webviews where HTML5 DnD does not): `DragSource`, `DropZone`, `DragLayer`, `provide_drag` |
| `workspace` | `TabDock`: split panes with tab tear-off between panes (built on `drag`) |
| `code-surface` | `CodeSurface` (virtualized, brace-foldable viewer) and `MultiEditor` (multi-cursor code editor: add-cursor-below, add-next-occurrence, multi-selection, drag-select, Home/End, clipboard copy/cut/paste, and IME composition via a hidden input sink) |
| `terminal` | `Terminal` (REPL surface) and `AnsiTerminal` (a real ANSI/VT parser: 16/256/truecolor SGR, bold/inverse, cursor movement + save/restore, erase, scroll regions, alternate screen) |
| `undo-tree` | `UndoHistory<T>` (generic branching history) + `UndoTree` panel |
| `jump` | `JumpOverlay`: an avy-style "label every target and jump by typing" overlay |
| `disclosure` | `Disclosure`, `Accordion`, `AccordionItem` |
| `status-bar` | `StatusBar`, `StatusItem`, `StatusSpacer` |
| `toolbar` | `Toolbar`, `ToolbarGroup`, `ToolbarSpacer`, `ToolButton`, `MenuBar`, `MenuBarMenu`, `ActivityBar` (icon rail) |
| `log` | `LogView` (kind-colored, deduped, auto-tailing console) |
| `markdown` | `Markdown`: a dependency-free renderer (headings, emphasis, code, lists, links, quotes) |
| `search-list` | `SearchList`: filterable list with expandable detail and scroll-to-selected |
| `asset-grid` | `AssetGrid`, `AssetItem`: searchable thumbnail grid with lazy images |
| `list-editor` | `OrderedList`, `ListItem`: reorderable list with per-row actions |
| `chat` | `Chat`, `ChatMessage`: role-styled message list with compose box and status |
| `dynamic-form` | `DynamicForm`, `FormField`, `FieldSchema`: a form generated from a schema, emitting JSON |
| `viewport` | `Viewport`, `Bridge`, `Loader`, `WebGpuGate`, `ViewportOverlay`, `HudPanel`, `NavGizmo` |
| `engine` | `use_engine`, `EngineViewport`: turnkey wiring (input, keyboard, lifecycle) over the shared protocol |
| `nightshade` | the rhai syntax highlighter and `SelectedCard` |

`default = ["forms", "menus", "themes"]`. Use `full` to turn on everything.

## Commands and keybindings

Actions live in one place. A `CommandRegistry` (from `provide_command_registry()`) holds `Command`s,
each with an id, title, group, optional `enabled` predicate, optional keybinding, and optional nested
children. The `CommandPalette`, `KeymapProvider`, and your menus all read from that single registry,
so a new capability is one `register` call instead of three wired surfaces. `KeymapProvider` installs
one global key listener that parses bindings like `Mod+K` or the chord `g d` and dispatches to the
registry, skipping bare keys while a text field has focus. When a chord prefix is pending it shows a
which-key hint overlay of the possible completions.

## Theming

Every component draws its colors from a small set of semantic CSS custom properties
(`--musaic-accent`, `--musaic-panel`, `--musaic-text`, `--musaic-danger`, and friends). A theme is a
typed Rust `Theme` struct that emits those tokens; the nine bundled themes are defined in Rust and
`ThemeProvider` injects their generated CSS, selected by `data-theme` on the document element, and
persists the choice to `localStorage`. A consumer defines a new theme in code and calls
`register_theme(...)`; it shows up in `ThemePicker` and restyles every component with no per-component
work. The component stylesheet is wrapped in `@layer musaic`, so your own CSS always wins.

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

The wire types live behind the `protocol` feature (`serde` only, no Leptos), exposed as
`leptos_musaic::protocol`. A worker shares them with the page by depending on `leptos-musaic` with
`default-features = false, features = ["protocol"]`, so there is one source of truth, no drift, and
still just one crate to publish.

## Crates

- `leptos-musaic` (`crates/musaic`): the component library, and the only crate meant for publishing.
  Its `protocol` feature exposes the shared page/worker wire types with no Leptos dependency.
- `leptos-musaic-shell` (`crates/musaic-shell`): a reusable native shell (`wry` + `winit`) that serves
  a built web bundle into a desktop window. `leptos_musaic_shell::run(title, get)`.
- `leptos-musaic-engine` (`crates/musaic-engine`): the worker-side engine driver used by the example.
  `run_offscreen(scene, setup, tick, on_custom)` owns the render loop, input injection, picking, and
  stats.

A new engine-backed app is roughly: `use_engine()` plus your panels on the page, and
`run_offscreen(scene, setup, tick, on_custom)` plus your scene logic in the worker. The repeated
wiring is gone.

## Examples

Three example apps, all built entirely from musaic components:

- `examples/landing` is the showcase that fronts the site: an `EditorShell` app with a toolbar,
  a live controls sidebar, a command palette, an ANSI terminal, a table, and a code editor, wired
  to demonstrate the library. `just run-landing`.
- `examples/gallery` is the interactive catalog, a runnable demo of every component.
  `just run-gallery`.
- `examples/nightshade_demo` is a full editor-style app: a toolbar, a sidebar of live scene
  controls, an embedded viewport, a resizable script/log dock, a command palette, and nine themes,
  driving a renderer in a web worker on an `OffscreenCanvas`. It runs on the web (`just dev`) and
  as a native desktop app (`just run`) from the same code.

```
just init    # install the pinned toolchain (rust, wasm-bindgen, wasm-opt, trunk) via mise
just dev     # serve the demo at http://127.0.0.1:8080
just run     # build the bundle and open it in a native webview window
just dist    # production web bundle in examples/nightshade_demo/dist
just check   # type-check the library (all features) and the demo workspace
just lint    # clippy, denying warnings
just test    # run the unit tests on the host target
```

On every push to `main`, `.github/workflows/pages.yml` deploys the whole site to GitHub Pages: the
landing showcase at the root, the demo at `/demo/`, the gallery at `/gallery/`, and the guide book
at `/book/`.

## License

MIT OR Apache-2.0.
