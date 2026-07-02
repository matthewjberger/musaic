# leptos-musaic

A feature-gated [Leptos](https://leptos.dev) 0.7 (CSR) component library for building beautiful,
editor-grade UIs that run the same code on the web (wasm) and natively (in a webview). It is
engine-agnostic: a reusable set of themed UI patterns, not tied to any app or backend.

New here, or building a UI with musaic? Read the book in `docs/book/src` (start with
`introduction.md`, then `installation.md` and `first-ui.md`). The **gallery is the living reference**:
every component has a runnable demo in `examples/gallery/src/sections.rs`. When in doubt about a
component's props or idiom, open its gallery demo. `README.md` has the full feature-gate table, and
`examples/minimal` is the runnable quickstart.

Starting a brand-new app rather than working in this repo? `docs/TEMPLATE.md` explains how to clone
and update the project starter template that musaic layers onto.

## Consuming it

```toml
leptos-musaic = { git = "https://github.com/matthewjberger/musaic", features = ["forms", "themes", "command-palette"] }
```

`use leptos_musaic::prelude::*;` pulls in every enabled component plus `leptos::prelude::*`.
`default = ["forms", "menus", "themes"]`. Use `full` to turn everything on. Enable exactly the
features whose components you use, each row in the README table lists what a feature provides.

## The four things at the root

Drop these once, near the top of your app:

- `<MusaicStyles/>` injects the design-token stylesheet (wrapped in `@layer musaic`, so your own CSS
  always wins) into the document head. No file to copy, no build step.
- Wrap the app in `<ThemeProvider>` to get theming (`data-theme` on `<html>`, persisted to
  `localStorage`).
- Optional: `provide_command_registry()` then `<KeymapProvider>...</KeymapProvider>` and a
  `<CommandPalette open=some_signal/>` to get one source of truth for actions, keybindings, and the
  palette.
- Optional: `provide_drag()` and one `<DragLayer/>` if you use `DragSource`/`DropZone`.

`EditorShell` is the recommended frame: named `toolbar` / `left` / `right` / `bottom` / `status`
slots around `children` (the center), with optional `*_open` toggle signals and `*_size` signals
that wire built-in resize handles.

## Reactivity idioms (data-oriented, no OOP)

- Hold state in `RwSignal`s. Bundle the related signals for a screen into one plain
  `#[derive(Clone, Copy)]` handle struct whose fields are `RwSignal`/`StoredValue`, and pass that
  handle by value into components. This is the pattern the demo and gallery use.
- Pass reactive props as `Signal::derive(move || ...)`; pass event handlers as `Callback::new(...)`.
- Logic is free functions over plain data; components are `#[component] fn`s. No inheritance, no
  trait-object polymorphism, no hand-rolled widgets for things musaic already ships (`LogView`,
  `StatusBar`, `Toolbar`, `Inspector`, `Table`, and so on).

## Theming

Every component draws from semantic CSS custom properties (`--musaic-accent`, `--musaic-panel`,
`--musaic-text`, `--musaic-danger`, and friends). A `Theme` is a typed Rust struct that emits those
tokens; the bundled themes live in `theme.rs`. `register_theme(...)` adds a custom one and it shows
up in `ThemePicker` / `ThemeMenu` (which previews on hover) and restyles every component with no
per-component work. The stylesheet is in `@layer musaic`, so unlayered app CSS always wins.

## Gotchas

- A component renders nothing / a type is missing? You did not enable its feature.
- Drag and drop is pointer-event based (`provide_drag`, `DragSource`, `DropZone`, `DragLayer`) so it
  works inside webviews where HTML5 DnD never fires. Do not reach for HTML5 `draggable`.
- Components are browser/CSR only. The `protocol` feature is the exception: leptos-free `serde` wire
  types (`leptos_musaic::protocol`) a worker can share with the page.
- Monospace editors (`CodeSurface`, `MultiEditor`) position carets in `ch` units, no DOM measuring.

## Extending musaic (when working in this repo)

Adding a component is a fixed checklist:

1. New module `crates/musaic/src/<name>.rs`.
2. In `lib.rs`, gate it: `#[cfg(feature = "<name>")] mod <name>;` + `#[cfg(feature = "<name>")] pub use <name>::*;`.
3. In `Cargo.toml`, add `<name> = ["_dom"]` (UI features pull the internal `_dom` feature that
   activates leptos/web-sys) and add `"<name>"` to the `full` list.
4. Add `crates/musaic/css/<name>.css` and a matching `#[cfg(feature = "<name>")] css.push_str(include_str!("../css/<name>.css"));`
   line in `styles.rs`. All selectors are prefixed `.musaic-<name>...`; the whole sheet is wrapped in
   `@layer musaic`.
5. Add a demo page in `examples/gallery/src/sections.rs` (a `Page` entry, a render-match arm, and a
   `Demo`/`Snippet` component).
6. Add a row to the README feature-gate table.

House rules: no `//` comments in Rust code (rustdoc `///` is expected and wanted), no abbreviations,
no `unsafe`, no `#[allow(...)]` to silence lints (fix the cause), 2024-edition module style
(no `mod.rs`). Never use the em-dash character anywhere.

## Build and gate

Use the justfile, the gate is per-target (the library builds for wasm, desktop crates for the host),
not a single `cargo clippy --all-targets`:

```
just check   # type-check library (all features) + wasm example crates + desktop crates + fmt
just lint    # clippy -D warnings across the same set
just test    # cargo test -p leptos-musaic --features full  (unit + protocol round-trip tests)
just dev     # serve the example app;  just run-gallery serves the gallery natively
```
