# The App Shell

`EditorShell` is the frame an editor-style app hangs off: a toolbar across the top, sidebars on
either side, a dock along the bottom, a status bar underneath, and your main content in the center.
Panels can collapse and resize. It is one component instead of a hand-built grid.

## Slots

Each region is an optional slot; `children` is the center.

```rust
view! {
    <EditorShell
        toolbar=move || view! { <Toolbar/> }
        left=move || view! { <Sidebar/> }
        right=move || view! { <Inspector/> }
        bottom=move || view! { <Console/> }
        status=move || view! { <StatusBar>...</StatusBar> }
    >
        // center content
        <Viewport/>
    </EditorShell>
}
```

An empty slot collapses to nothing, so you pay only for the regions you use.

## Collapsing and resizing

Pass a `RwSignal<bool>` to `left_open` / `right_open` / `bottom_open` to toggle a panel (for
example from a toolbar button or a command). Pass a `RwSignal<f64>` to `left_size` / `right_size` /
`bottom_size` to make that panel resizable: the shell renders a drag handle on the panel's inner
edge and writes the new size back to your signal.

```rust
let left_open = RwSignal::new(true);
let sidebar_width = RwSignal::new(280.0);
let dock_height = RwSignal::new(200.0);

view! {
    <EditorShell
        left_open=left_open
        left_size=sidebar_width
        bottom_size=dock_height
        left=move || view! { <Sidebar/> }
        bottom=move || view! { <Console/> }
    >
        <Viewport/>
    </EditorShell>
}
```

Because the sizes are your signals, you can persist them (`use_persisted`), reset them from a
command, or drive them however you like.

## What goes where

- **toolbar**: a `Toolbar` with `ToolButton`s and a `ThemeMenu`, or a `MenuBar`.
- **left / right**: an `Inspector` of collapsible sections, a `Tree`, a `SearchList`.
- **bottom**: a `TabDock` or a plain `TabBar` switching between a code editor and a `LogView`.
- **status**: a `StatusBar` of `StatusItem`s (adapter, counts, messages).
- **center**: whatever the app is about, a document, a canvas, a rendering surface.

The [Component Catalog](catalog.md) lists everything that can fill these slots. The
`examples/nightshade_demo` app is a full `EditorShell` in use.
