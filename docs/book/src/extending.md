# Extending musaic

Adding a component to the library is a fixed checklist. Follow it and the new component is
feature-gated, themed, documented, and demoed like every other.

## The checklist

Say you are adding a `Gauge`:

1. **Module.** Create `crates/musaic/src/gauge.rs` with a `#[component] pub fn Gauge(...)`.

2. **Wire it in `lib.rs`**, gated on a new feature:

   ```rust
   #[cfg(feature = "gauge")]
   mod gauge;
   #[cfg(feature = "gauge")]
   pub use gauge::*;
   ```

3. **Declare the feature in `Cargo.toml`.** UI features turn on the internal `_dom` feature (which
   activates the Leptos and web-sys stack), and go into `full`:

   ```toml
   gauge = ["_dom"]
   # add "gauge" to the full = [...] list
   ```

4. **Styles.** Add `crates/musaic/css/gauge.css`, with every selector prefixed `.musaic-gauge...`,
   and include it (gated) in `styles.rs`:

   ```rust
   #[cfg(feature = "gauge")]
   css.push_str(include_str!("../css/gauge.css"));
   ```

   Draw only from the theme tokens (`var(--musaic-accent)`, and so on) so the component themes for
   free. The whole sheet is wrapped in `@layer musaic`.

5. **Gallery demo.** In `examples/gallery/src/sections.rs`, add a `Page` entry, a render-match arm,
   and a `Demo` (with a `Snippet`) that exercises the component. The gallery is the acceptance test:
   if it looks right there, it works.

6. **README row.** Add the feature and its component to the feature-gate table in `README.md`, and,
   if it is notable, to the catalog in this book.

7. **Rustdoc.** Give every public item a `///` doc comment: what it renders and its important props,
   in one to three sentences, written from the actual behavior.

## Style rules

musaic is data-oriented. A component is a `#[component] fn`; its state is signals; logic is free
functions over plain data; anything shared is a `Copy` handle struct. No inheritance, no trait-object
widget hierarchies, no hand-rolled versions of components the library already ships.

Rust house rules: no `//` code comments (rustdoc `///` is expected), no abbreviations, no `unsafe`,
no `#[allow(...)]` to silence a lint (fix the cause), 2024-edition module layout (no `mod.rs`).
Never use the em-dash character.

## Gate before you commit

The build is per-target: the library compiles for `wasm32-unknown-unknown`, the desktop crates for
the host. Use the justfile rather than a single `cargo clippy --all-targets`:

```
just check   # build library (all features) + wasm example crates + desktop crates + fmt
just lint    # clippy -D warnings across the same set
just test    # cargo test -p leptos-musaic --features full
```

Fix every warning; dead code is removed, not silenced.
