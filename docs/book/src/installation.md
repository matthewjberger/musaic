# Installation

musaic is a Leptos 0.7 crate. Add it to a client-side-rendered Leptos app and enable the features
whose components you use.

```toml
[dependencies]
leptos = { version = "0.7", features = ["csr"] }
leptos-musaic = { git = "https://github.com/matthewjberger/musaic", features = ["forms", "themes", "command-palette"] }
```

Consuming it as a git dependency is the recommended path while the API is still moving: your app
tracks `main` and picks up new components without waiting for a release.

## Features

Components are behind feature gates so you compile only what you use. The defaults are a good
starting point:

```toml
# default = ["forms", "menus", "themes"]
leptos-musaic = { git = "...", features = ["full"] }   # or turn everything on
```

`full` enables every component. Each feature and the components it unlocks is listed in the feature
table in the repository `README.md`. If a component type does not resolve, or renders nothing, the
usual cause is a missing feature. See [Feature Gates](feature-gates.md) for the full list and how
they compose.

There is one non-UI feature, `protocol`, which exposes leptos-free `serde` wire types
(`leptos_musaic::protocol`) for sharing messages between a page and a worker. It pulls no DOM
dependencies, so a worker crate can depend on `leptos-musaic` with `default-features = false,
features = ["protocol"]`.

## The prelude

One import pulls in every enabled component plus `leptos::prelude::*`:

```rust
use leptos_musaic::prelude::*;
```

A typical module needs only that line. The rest of this book assumes the prelude is in scope.

## Running the gallery

To browse every component interactively, run the gallery from the repository:

```
just run-gallery   # native window
# or serve it for the browser with trunk
```
