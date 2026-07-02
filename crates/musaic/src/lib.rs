//! A feature-gated [Leptos](https://leptos.dev) 0.7 (CSR) component library for building
//! beautiful, editor-grade UIs that run the same code on the web (wasm) and natively (in a
//! webview). It is engine-agnostic: a reusable set of themed UI patterns, panels, forms, menus,
//! tables, trees, a command palette, keybindings, a code editor, toasts, a status bar, drag and
//! drop, and a resizable app shell, all driven from one set of CSS custom properties.
//!
//! # Quick start
//!
//! Add the crate with the features you use (`default = ["forms", "menus", "themes"]`, or `full`
//! for everything), then import the prelude and drop the stylesheet and a theme provider at the
//! root:
//!
//! ```ignore
//! use leptos_musaic::prelude::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <MusaicStyles/>
//!         <ThemeProvider>
//!             <Panel title="Hello">
//!                 <ThemePicker/>
//!                 <Button>"Click me"</Button>
//!             </Panel>
//!         </ThemeProvider>
//!     }
//! }
//! ```
//!
//! [`MusaicStyles`] injects the design-token stylesheet (wrapped in `@layer musaic`, so your own
//! CSS always wins) into the document head. [`ThemeProvider`] sets `data-theme` and persists the
//! choice. From there, compose components; [`EditorShell`] is the recommended app frame.
//!
//! # Concepts
//!
//! - **Feature gates.** Every component is behind a Cargo feature; enable only what you use. A type
//!   that does not resolve usually means a missing feature.
//!   Components are browser/CSR only, except the [`protocol`] module (leptos-free `serde` wire
//!   types behind the `protocol` feature, for sharing messages with a worker).
//! - **Reactivity.** Hold state in `RwSignal`s, pass reactive props as `Signal::derive(...)`, pass
//!   events as `Callback::new(...)`, and bundle a screen's signals into one `#[derive(Clone,
//!   Copy)]` handle struct passed by value.
//! - **Theming.** Components read semantic tokens (`--musaic-accent`, `--musaic-panel`, and
//!   friends); [`register_theme`] adds a custom [`Theme`] that restyles the whole surface.
//!
//! The book under `docs/book` is the full guide, and `examples/gallery` renders a live demo of
//! every component.

#[cfg(feature = "protocol")]
pub mod protocol;
#[cfg(feature = "protocol")]
pub use protocol::*;

#[cfg(feature = "_dom")]
mod base;
#[cfg(feature = "_dom")]
mod command;
#[cfg(feature = "_dom")]
mod editor_shell;
#[cfg(feature = "_dom")]
mod keymap;
#[cfg(feature = "_dom")]
mod styles;
#[cfg(feature = "_dom")]
mod theme;
#[cfg(feature = "_dom")]
mod util;
#[cfg(feature = "_dom")]
mod web;

#[cfg(feature = "_dom")]
pub use base::*;
#[cfg(feature = "_dom")]
pub use command::*;
#[cfg(feature = "_dom")]
pub use editor_shell::*;
#[cfg(feature = "_dom")]
pub use keymap::*;
#[cfg(feature = "_dom")]
pub use styles::{MusaicStyles, stylesheet};
#[cfg(feature = "_dom")]
pub use theme::*;
#[cfg(feature = "_dom")]
pub use util::*;
#[cfg(feature = "_dom")]
pub use web::*;

#[cfg(feature = "forms")]
mod forms;
#[cfg(feature = "forms")]
pub use forms::*;

#[cfg(feature = "menus")]
mod menus;
#[cfg(feature = "menus")]
pub use menus::*;

#[cfg(feature = "command-palette")]
mod palette;
#[cfg(feature = "command-palette")]
pub use palette::*;

#[cfg(feature = "code-editor")]
mod code_editor;
#[cfg(feature = "code-editor")]
pub use code_editor::*;

#[cfg(feature = "viewport")]
mod viewport;
#[cfg(feature = "viewport")]
pub use viewport::*;

#[cfg(feature = "viewport")]
mod hud;
#[cfg(feature = "viewport")]
pub use hud::*;

#[cfg(feature = "engine")]
mod engine;
#[cfg(feature = "engine")]
pub use engine::*;

#[cfg(feature = "table")]
mod table;
#[cfg(feature = "table")]
pub use table::*;

#[cfg(feature = "tree")]
mod tree;
#[cfg(feature = "tree")]
pub use tree::*;

#[cfg(feature = "inspector")]
mod inspector;
#[cfg(feature = "inspector")]
pub use inspector::*;

#[cfg(feature = "dock")]
mod dock;
#[cfg(feature = "dock")]
pub use dock::*;

#[cfg(feature = "overlays")]
mod floating;
#[cfg(feature = "overlays")]
pub use floating::*;

#[cfg(feature = "virtual-list")]
mod virtual_list;
#[cfg(feature = "virtual-list")]
pub use virtual_list::*;

#[cfg(feature = "diff")]
mod diff;
#[cfg(feature = "diff")]
pub use diff::*;

#[cfg(feature = "drag")]
mod pointer_drag;
#[cfg(feature = "drag")]
pub use pointer_drag::*;

#[cfg(feature = "undo-tree")]
mod undo_tree;
#[cfg(feature = "undo-tree")]
pub use undo_tree::*;

#[cfg(feature = "jump")]
mod jump;
#[cfg(feature = "jump")]
pub use jump::*;

#[cfg(feature = "terminal")]
mod terminal;
#[cfg(feature = "terminal")]
pub use terminal::*;

#[cfg(feature = "terminal")]
mod ansi_terminal;
#[cfg(feature = "terminal")]
pub use ansi_terminal::*;

#[cfg(feature = "workspace")]
mod workspace;
#[cfg(feature = "workspace")]
pub use workspace::*;

#[cfg(feature = "code-surface")]
mod code_surface;
#[cfg(feature = "code-surface")]
pub use code_surface::*;

#[cfg(feature = "code-surface")]
mod multi_editor;
#[cfg(feature = "code-surface")]
pub use multi_editor::*;

#[cfg(feature = "disclosure")]
mod disclosure;
#[cfg(feature = "disclosure")]
pub use disclosure::*;

#[cfg(feature = "status-bar")]
mod status_bar;
#[cfg(feature = "status-bar")]
pub use status_bar::*;

#[cfg(feature = "toolbar")]
mod toolbar;
#[cfg(feature = "toolbar")]
pub use toolbar::*;

#[cfg(feature = "log")]
mod log_view;
#[cfg(feature = "log")]
pub use log_view::*;

#[cfg(feature = "markdown")]
mod markdown;
#[cfg(feature = "markdown")]
pub use markdown::*;

#[cfg(feature = "search-list")]
mod search_list;
#[cfg(feature = "search-list")]
pub use search_list::*;

#[cfg(feature = "asset-grid")]
mod asset_grid;
#[cfg(feature = "asset-grid")]
pub use asset_grid::*;

#[cfg(feature = "list-editor")]
mod list_editor;
#[cfg(feature = "list-editor")]
pub use list_editor::*;

#[cfg(feature = "chat")]
mod chat;
#[cfg(feature = "chat")]
pub use chat::*;

#[cfg(feature = "dynamic-form")]
mod dynamic_form;
#[cfg(feature = "dynamic-form")]
pub use dynamic_form::*;

#[cfg(feature = "nightshade")]
mod nightshade;
#[cfg(feature = "nightshade")]
pub use nightshade::*;

#[cfg(feature = "_dom")]
pub mod prelude {
    pub use crate::*;
    pub use leptos::prelude::*;
}
