# leptos-musaic

A feature-gated [Leptos](https://leptos.dev) component library for building beautiful UIs that
run the same code natively (in a webview) and on the web (wasm). It is a reusable, engine-agnostic
set of UI patterns for editor-style and tool apps.

The goal: let you assemble a complex, production-grade Leptos UI with very little code, without
giving up control. Components default to sensible behavior and expose callbacks and reactive props
for the moments you need to steer them. Everything is themed from one set of CSS custom properties,
so the whole surface (including every component added here) restyles with a single `data-theme`
switch.

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
| `code-surface` | `CodeSurface`: a virtualized, brace-foldable code viewer for large files |
| `terminal` | `Terminal`: an interactive REPL surface (colored output lines + prompt) |
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

The wire types live in the tiny no-deps `leptos-musaic-protocol` crate, shared by page and worker, so
there is one source of truth and no drift.

## Crates

- `leptos-musaic` (`crates/musaic`): the component library.
- `leptos-musaic-protocol` (`crates/musaic-protocol`): the shared page/worker wire types (`serde` only).
- `leptos-musaic-shell` (`crates/musaic-shell`): a reusable native shell (`wry` + `winit`) that serves
  a built web bundle into a desktop window. `leptos_musaic_shell::run(title, get)`.
- `leptos-musaic-engine` (`crates/musaic-engine`): the worker-side engine driver used by the example.
  `run_offscreen(scene, setup, tick, on_custom)` owns the render loop, input injection, picking, and
  stats.

A new engine-backed app is roughly: `use_engine()` plus your panels on the page, and
`run_offscreen(scene, setup, tick, on_custom)` plus your scene logic in the worker. The repeated
wiring is gone.

## Example

`examples/nightshade_demo` is a full editor-style app built entirely from musaic components: a
toolbar, a sidebar of live scene controls, an embedded viewport, a resizable script/log dock, a
command palette, and nine themes, driving a 3D engine in a web worker on an `OffscreenCanvas`. It
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
