# Feedback and Status

## Toasts

Toasts are in the always-on base layer. Render one `<ToastHub/>` near the root, then call the
`use_toaster()` handle from anywhere to raise a message.

```rust
let toaster = use_toaster();
toaster.success("Saved");
toaster.error("Export failed");
toaster.warning("Unsaved changes");
toaster.info("Connected");
```

Each toast auto-dismisses; `Toaster::action` adds an inline action button, and toasts carry a close
control. Because the handle is `Copy`, capture it in event handlers and commands freely.

## Log

`LogView` (the `log` feature) is a console: kind-colored, deduplicated (repeats collapse to a
count), and auto-tailing. Feed it a `Signal<Vec<LogEntry>>`; each `LogEntry` has a `LogKind`
(`Info`, `Command`, `Event`, `Warn`, `Error`), a label, optional detail, and a count. Wire `on_clear`
to empty your buffer.

```rust
view! { <LogView entries=log on_clear=Callback::new(move |_| log.set(Vec::new()))/> }
```

Store log entries as a `Vec<LogEntry>` in your state handle and push `LogEntry::new(id, kind, msg)`
as things happen.

## Status bar

`StatusBar` (the `status-bar` feature) is the strip along the bottom; `StatusItem` is one cell (with
an optional icon) and `StatusSpacer` pushes items apart. It is the idiomatic home for live counters
and short messages.

```rust
view! {
    <StatusBar>
        <StatusItem icon="●">{move || adapter.get()}</StatusItem>
        <StatusItem>{move || format!("{:.0} fps", fps.get())}</StatusItem>
        <StatusSpacer/>
        <StatusItem>{move || format!("{} items", count.get())}</StatusItem>
    </StatusBar>
}
```

Put the `StatusBar` in the `EditorShell` `status` slot. Progress and spinners (`Progress`,
`Spinner`) live in the base layer for inline busy indicators.
