# Rendering Surfaces

musaic's core never links a renderer. If your app draws into a canvas from a web worker, two
optional features wire that up for you. Skip this chapter if your UI has no rendering surface.

## viewport

The `viewport` feature gives a generic render surface. `Viewport` owns the canvas and
`OffscreenCanvas` handoff to a worker and all the pointer, touch, and wheel bookkeeping, emitting
`ViewportEvent`s you map to your own protocol. `WebGpuGate` renders a fallback notice when the
browser lacks WebGPU, and `Loader` is a "starting up" overlay bound to a ready signal.

## engine

The `engine` feature goes further. `use_engine("worker.js")` returns a `Copy` `EngineState` (ready,
adapter, fps, entity count, selection, grabbing) plus a bridge, with the wiring done: keyboard
forwarding, input mapping, and lifecycle decoding. You send app-specific messages with
`engine.send(&YourMessage)` and receive them with `engine.on_custom(...)`. `EngineViewport` renders
the surface for that engine handle.

```rust
let engine = use_engine("runtime/worker.js");
engine.on_custom(Callback::new(move |value| { /* decode your events */ }));

view! {
    <EditorShell ...>
        <EngineViewport engine=engine/>
        <Loader ready=engine.state.ready/>
    </EditorShell>
}
```

## The protocol

The page and the worker share a small `serde` message protocol (`ToWorker`, `FromWorker`, and the
viewport wire types) exposed as `leptos_musaic::protocol` behind the `protocol` feature. Because
that feature pulls no DOM dependencies, the worker crate depends on `leptos-musaic` with
`default-features = false, features = ["protocol"]` and shares the exact same types as the page, so
there is one source of truth and no drift. Both sides also have a `Custom(Value)` escape hatch for
app-specific messages, which is what `engine.send` / `engine.on_custom` ride on.
