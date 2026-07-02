# Drag and Drop

musaic's drag and drop is pointer-event based, not HTML5 `draggable`. HTML5 drag events do not fire
inside many webviews (including the ones desktop builds use), so musaic tracks pointer down, move,
and up itself. It works the same on the web and natively.

## Wiring

Three pieces:

1. `provide_drag()` once near the root, to create the shared drag session.
2. `<DragLayer/>` once, to render the floating preview that follows the pointer.
3. `DragSource` around draggable items and `DropZone` around targets.

```rust
// at the root
provide_drag();
view! {
    <DragLayer/>
    // ...
    <DragSource kind="card" id="card-7" label="Card 7">
        <div class="card">"Card 7"</div>
    </DragSource>

    <DropZone id="column-b" on_drop=Callback::new(move |payload: DragPayload| {
        if payload.kind == "card" {
            move_card(payload.id, "column-b");
        }
    })>
        // column contents
    </DropZone>
}
```

## How it behaves

A drag arms on pointer-down but only starts once the pointer moves past a small threshold, so a
click is still a click. While dragging, the `DragLayer` shows the payload's `label`; a `DropZone`
highlights (`class:over`) while the pointer is over it; on release the zone under the pointer runs
its `on_drop` with the `DragPayload`. Reject unwanted drags by checking `payload.kind` in
`on_drop`.

## TabDock

`TabDock` (the `workspace` feature) is built on this: split panes whose tabs you can drag from one
pane to another (tab tear-off). Pass it the tabs signal, the pane ids, an active-tab signal, and a
render closure for tab bodies. It is the ready-made version of the pattern above for a docking UI.
