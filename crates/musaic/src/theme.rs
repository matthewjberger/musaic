use leptos::prelude::*;

pub const THEMES: &[(&str, &str)] = &[
    ("nightshade", "Nightshade"),
    ("nightshade-light", "Nightshade Light"),
    ("dracula", "Dracula"),
    ("nord", "Nord"),
    ("gruvbox", "Gruvbox Dark"),
    ("one-dark", "One Dark"),
    ("catppuccin", "Catppuccin Mocha"),
    ("tokyo-night", "Tokyo Night"),
    ("solarized-light", "Solarized Light"),
];

const THEME_KEY: &str = "musaic-theme";

pub fn stored_theme() -> String {
    web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(THEME_KEY).ok().flatten())
        .filter(|stored| THEMES.iter().any(|(id, _)| id == stored))
        .unwrap_or_else(|| THEMES[0].0.to_string())
}

pub fn preview_theme(id: &str) {
    if let Some(element) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    {
        let _ = element.set_attribute("data-theme", id);
    }
}

pub fn apply_theme(id: &str) {
    preview_theme(id);
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.set_item(THEME_KEY, id);
    }
}

#[derive(Clone, Copy)]
struct ThemeContext(RwSignal<String>);

pub fn use_theme() -> RwSignal<String> {
    use_context::<ThemeContext>()
        .map(|context| context.0)
        .unwrap_or_else(|| RwSignal::new(stored_theme()))
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme = RwSignal::new(stored_theme());
    provide_context(ThemeContext(theme));
    Effect::new(move |_| apply_theme(&theme.get()));
    children()
}

#[cfg(feature = "themes")]
#[component]
pub fn ThemePicker() -> impl IntoView {
    let theme = use_theme();
    view! {
        <select
            class="musaic-theme-picker"
            prop:value=move || theme.get()
            on:change=move |event| theme.set(event_target_value(&event))
        >
            {THEMES
                .iter()
                .map(|(id, label)| view! { <option value=*id>{*label}</option> })
                .collect_view()}
        </select>
    }
}
