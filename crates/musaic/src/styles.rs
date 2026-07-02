//! Assembles and injects the library's CSS. Enabled Cargo features determine
//! which per-component stylesheets are bundled into the emitted `@layer musaic`
//! block.

use leptos::prelude::*;

/// Concatenate the core stylesheet with the CSS for every enabled feature,
/// wrapped in an `@layer musaic { ... }` cascade layer.
pub fn stylesheet() -> String {
    let mut css = String::new();
    css.push_str(include_str!("../css/core.css"));
    #[cfg(feature = "forms")]
    css.push_str(include_str!("../css/forms.css"));
    #[cfg(feature = "menus")]
    css.push_str(include_str!("../css/menus.css"));
    #[cfg(feature = "command-palette")]
    css.push_str(include_str!("../css/palette.css"));
    #[cfg(feature = "code-editor")]
    css.push_str(include_str!("../css/code_editor.css"));
    #[cfg(feature = "table")]
    css.push_str(include_str!("../css/table.css"));
    #[cfg(feature = "tree")]
    css.push_str(include_str!("../css/tree.css"));
    #[cfg(feature = "inspector")]
    css.push_str(include_str!("../css/inspector.css"));
    #[cfg(feature = "dock")]
    css.push_str(include_str!("../css/dock.css"));
    #[cfg(feature = "overlays")]
    css.push_str(include_str!("../css/overlays.css"));
    #[cfg(feature = "virtual-list")]
    css.push_str(include_str!("../css/virtual_list.css"));
    #[cfg(feature = "diff")]
    css.push_str(include_str!("../css/diff.css"));
    #[cfg(feature = "drag")]
    css.push_str(include_str!("../css/drag.css"));
    #[cfg(feature = "undo-tree")]
    css.push_str(include_str!("../css/undo_tree.css"));
    #[cfg(feature = "jump")]
    css.push_str(include_str!("../css/jump.css"));
    #[cfg(feature = "terminal")]
    css.push_str(include_str!("../css/terminal.css"));
    #[cfg(feature = "workspace")]
    css.push_str(include_str!("../css/workspace.css"));
    #[cfg(feature = "code-surface")]
    css.push_str(include_str!("../css/code_surface.css"));
    #[cfg(feature = "disclosure")]
    css.push_str(include_str!("../css/disclosure.css"));
    #[cfg(feature = "status-bar")]
    css.push_str(include_str!("../css/status_bar.css"));
    #[cfg(feature = "toolbar")]
    css.push_str(include_str!("../css/toolbar.css"));
    #[cfg(feature = "log")]
    css.push_str(include_str!("../css/log.css"));
    #[cfg(feature = "markdown")]
    css.push_str(include_str!("../css/markdown.css"));
    #[cfg(feature = "search-list")]
    css.push_str(include_str!("../css/search_list.css"));
    #[cfg(feature = "asset-grid")]
    css.push_str(include_str!("../css/asset_grid.css"));
    #[cfg(feature = "list-editor")]
    css.push_str(include_str!("../css/list_editor.css"));
    #[cfg(feature = "chat")]
    css.push_str(include_str!("../css/chat.css"));
    #[cfg(feature = "dynamic-form")]
    css.push_str(include_str!("../css/dynamic_form.css"));
    #[cfg(feature = "viewport")]
    css.push_str(include_str!("../css/viewport.css"));
    format!("@layer musaic {{\n{css}\n}}\n")
}

const STYLE_ID: &str = "musaic-styles";

fn inject() {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    if document.get_element_by_id(STYLE_ID).is_some() {
        return;
    }
    let Some(head) = document.head() else {
        return;
    };
    let Ok(element) = document.create_element("style") else {
        return;
    };
    let _ = element.set_attribute("id", STYLE_ID);
    element.set_text_content(Some(&stylesheet()));
    let _ = head.append_child(&element);
}

/// Injects the [`stylesheet`] into `<head>` once (idempotent by element id).
/// Render this near the root of your app.
#[component]
pub fn MusaicStyles() -> impl IntoView {
    inject();
}
