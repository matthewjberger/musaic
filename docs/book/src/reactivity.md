# Reactivity and Handles

musaic follows Leptos's reactivity and one data-oriented convention on top of it. Get these two
ideas and every component's props make sense.

## Signals, derived signals, callbacks

- `RwSignal<T>` holds state you read and write. `signal.get()` reads (and subscribes in a reactive
  context); `signal.set(v)` / `signal.update(|v| ...)` write.
- A component prop that must stay live takes a `Signal<T>`. Pass a computed value with
  `Signal::derive(move || ...)`, or pass an `RwSignal` directly where the prop's type allows it.
- An event prop takes a `Callback<T>`. Build one with `Callback::new(move |arg| ...)`.

```rust
let spinning = RwSignal::new(true);
view! {
    <Switch
        label="Spin"
        value=Signal::derive(move || spinning.get())
        on_change=Callback::new(move |on| spinning.set(on))
    />
}
```

Reading a signal inside a `move ||` closure (in a prop or in the view) makes that spot re-render when
the signal changes. That is the whole model.

## The handle pattern

Signals are cheap `Copy` handles into a reactive arena, not the data itself. That means you can group
the signals a screen shares into one plain struct, derive `Clone, Copy`, and pass it by value
everywhere. Every copy refers to the same underlying state.

```rust
#[derive(Clone, Copy)]
pub struct EditorState {
    pub script: RwSignal<String>,
    pub dirty: RwSignal<bool>,
    pub selection: RwSignal<Option<u32>>,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            script: RwSignal::new(String::new()),
            dirty: RwSignal::new(false),
            selection: RwSignal::new(None),
        }
    }
}
```

Now a component takes `state: EditorState` by value and reads or writes any field. No `Rc<RefCell>`,
no cloning of data, no context lookups for app state. musaic's own `Copy` handles (`EngineState`,
`SocketHandle`, the drag state, the command registry) follow the same shape, so they compose the
same way.

`StoredValue<T>` is the companion for non-signal data you want to stash in the arena (for example a
value captured by an event closure that does not need to be reactive). Use `new_local` for values
that are not `Send`.

## Why this matters

This is the data-oriented style musaic is built in: plain-data structs, free functions for logic,
and function components. There is no inheritance and no trait-object widget hierarchy. When you
extend the library (see [Extending musaic](extending.md)), match it: a component is a `#[component]
fn`, its state is signals, and anything shared is a `Copy` handle.
