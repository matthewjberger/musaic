use leptos::prelude::*;

pub fn stylesheet() -> String {
    let mut css = String::new();
    css.push_str(include_str!("../css/core.css"));
    #[cfg(feature = "themes")]
    css.push_str(include_str!("../css/themes.css"));
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

#[component]
pub fn MusaicStyles() -> impl IntoView {
    inject();
}
