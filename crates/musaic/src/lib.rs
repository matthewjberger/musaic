mod base;
mod command;
mod editor_shell;
mod keymap;
mod styles;
mod theme;
mod web;

pub use base::*;
pub use command::*;
pub use editor_shell::*;
pub use keymap::*;
pub use leptos_musaic_protocol::{SelectedEntity, TouchPhase};
pub use styles::{MusaicStyles, stylesheet};
pub use theme::*;
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

pub mod prelude {
    pub use crate::*;
    pub use leptos::prelude::*;
}
