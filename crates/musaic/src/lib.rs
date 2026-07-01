mod base;
mod command;
mod keymap;
mod styles;
mod theme;

pub use base::*;
pub use command::*;
pub use keymap::*;
pub use leptos_musaic_protocol::{SelectedEntity, TouchPhase};
pub use styles::{MusaicStyles, stylesheet};
pub use theme::*;

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

#[cfg(feature = "nightshade")]
mod nightshade;
#[cfg(feature = "nightshade")]
pub use nightshade::*;

pub mod prelude {
    pub use crate::*;
    pub use leptos::prelude::*;
}
