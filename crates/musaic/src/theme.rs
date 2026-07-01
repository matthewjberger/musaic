use leptos::prelude::*;

#[derive(Clone)]
pub struct Theme {
    pub id: String,
    pub label: String,
    pub light: bool,
    pub bg: String,
    pub panel: String,
    pub panel_2: String,
    pub panel_border: String,
    pub text: String,
    pub text_dim: String,
    pub accent: String,
    pub input_bg: String,
    pub danger: String,
    pub keyword: String,
    pub string: String,
    pub number: String,
    pub comment: String,
    pub command: String,
}

impl Theme {
    pub fn to_css(&self) -> String {
        let scheme = if self.light {
            "\n  color-scheme: light;"
        } else {
            ""
        };
        format!(
            ":root[data-theme=\"{id}\"] {{{scheme}\n  \
             --musaic-bg: {bg};\n  \
             --musaic-panel: {panel};\n  \
             --musaic-panel-2: {panel_2};\n  \
             --musaic-panel-border: {panel_border};\n  \
             --musaic-text: {text};\n  \
             --musaic-text-dim: {text_dim};\n  \
             --musaic-accent: {accent};\n  \
             --musaic-input-bg: {input_bg};\n  \
             --musaic-danger: {danger};\n  \
             --musaic-tok-keyword: {keyword};\n  \
             --musaic-tok-string: {string};\n  \
             --musaic-tok-number: {number};\n  \
             --musaic-tok-comment: {comment};\n  \
             --musaic-tok-command: {command};\n}}\n",
            id = self.id,
            bg = self.bg,
            panel = self.panel,
            panel_2 = self.panel_2,
            panel_border = self.panel_border,
            text = self.text,
            text_dim = self.text_dim,
            accent = self.accent,
            input_bg = self.input_bg,
            danger = self.danger,
            keyword = self.keyword,
            string = self.string,
            number = self.number,
            comment = self.comment,
            command = self.command,
        )
    }
}

macro_rules! theme {
    (
        $id:literal, $label:literal, $light:literal,
        $bg:literal, $panel:literal, $panel_2:literal, $panel_border:literal,
        $text:literal, $text_dim:literal, $accent:literal, $input_bg:literal, $danger:literal,
        $keyword:literal, $string:literal, $number:literal, $comment:literal, $command:literal
    ) => {
        Theme {
            id: $id.to_string(),
            label: $label.to_string(),
            light: $light,
            bg: $bg.to_string(),
            panel: $panel.to_string(),
            panel_2: $panel_2.to_string(),
            panel_border: $panel_border.to_string(),
            text: $text.to_string(),
            text_dim: $text_dim.to_string(),
            accent: $accent.to_string(),
            input_bg: $input_bg.to_string(),
            danger: $danger.to_string(),
            keyword: $keyword.to_string(),
            string: $string.to_string(),
            number: $number.to_string(),
            comment: $comment.to_string(),
            command: $command.to_string(),
        }
    };
}

pub fn builtin_themes() -> Vec<Theme> {
    vec![
        theme!(
            "nightshade",
            "Nightshade",
            false,
            "#0c0d12",
            "#15171d",
            "#11131a",
            "#2a2d36",
            "rgba(255, 255, 255, 0.86)",
            "rgba(255, 255, 255, 0.52)",
            "#fb923c",
            "rgba(255, 255, 255, 0.06)",
            "#f87171",
            "#c792ea",
            "#c3e88d",
            "#f78c6c",
            "#5c6370",
            "#82aaff"
        ),
        theme!(
            "nightshade-light",
            "Nightshade Light",
            true,
            "#f4f5f8",
            "#ffffff",
            "#eceef3",
            "#d4d7e0",
            "rgba(20, 22, 28, 0.9)",
            "rgba(20, 22, 28, 0.55)",
            "#d9700f",
            "rgba(0, 0, 0, 0.04)",
            "#d23b3b",
            "#8b32c9",
            "#3a8a2f",
            "#b5500a",
            "#9098a6",
            "#1f5fd0"
        ),
        theme!(
            "dracula",
            "Dracula",
            false,
            "#21222c",
            "#282a36",
            "#1e1f29",
            "#3a3c4e",
            "#f8f8f2",
            "rgba(248, 248, 242, 0.55)",
            "#bd93f9",
            "rgba(255, 255, 255, 0.06)",
            "#ff5555",
            "#ff79c6",
            "#f1fa8c",
            "#bd93f9",
            "#6272a4",
            "#8be9fd"
        ),
        theme!(
            "nord",
            "Nord",
            false,
            "#2e3440",
            "#3b4252",
            "#353c4a",
            "#4c566a",
            "#eceff4",
            "rgba(236, 239, 244, 0.55)",
            "#88c0d0",
            "rgba(255, 255, 255, 0.06)",
            "#bf616a",
            "#81a1c1",
            "#a3be8c",
            "#b48ead",
            "#616e88",
            "#8fbcbb"
        ),
        theme!(
            "gruvbox",
            "Gruvbox Dark",
            false,
            "#282828",
            "#32302f",
            "#1d2021",
            "#504945",
            "#ebdbb2",
            "rgba(235, 219, 178, 0.55)",
            "#fe8019",
            "rgba(255, 255, 255, 0.06)",
            "#fb4934",
            "#fb4934",
            "#b8bb26",
            "#d3869b",
            "#928374",
            "#83a598"
        ),
        theme!(
            "one-dark",
            "One Dark",
            false,
            "#21252b",
            "#282c34",
            "#1e2228",
            "#3b4048",
            "#abb2bf",
            "rgba(171, 178, 191, 0.55)",
            "#61afef",
            "rgba(255, 255, 255, 0.05)",
            "#e06c75",
            "#c678dd",
            "#98c379",
            "#d19a66",
            "#5c6370",
            "#61afef"
        ),
        theme!(
            "catppuccin",
            "Catppuccin Mocha",
            false,
            "#181825",
            "#1e1e2e",
            "#161623",
            "#313244",
            "#cdd6f4",
            "rgba(205, 214, 244, 0.55)",
            "#f5c2e7",
            "rgba(255, 255, 255, 0.05)",
            "#f38ba8",
            "#cba6f7",
            "#a6e3a1",
            "#fab387",
            "#6c7086",
            "#89b4fa"
        ),
        theme!(
            "tokyo-night",
            "Tokyo Night",
            false,
            "#1a1b26",
            "#1f2335",
            "#16161e",
            "#2f334d",
            "#c0caf5",
            "rgba(192, 202, 245, 0.55)",
            "#7aa2f7",
            "rgba(255, 255, 255, 0.05)",
            "#f7768e",
            "#bb9af7",
            "#9ece6a",
            "#ff9e64",
            "#565f89",
            "#7dcfff"
        ),
        theme!(
            "solarized-light",
            "Solarized Light",
            true,
            "#fdf6e3",
            "#eee8d5",
            "#e6dfc8",
            "#d6cfb8",
            "#4c5b61",
            "rgba(76, 91, 97, 0.6)",
            "#b58900",
            "rgba(0, 0, 0, 0.04)",
            "#dc322f",
            "#859900",
            "#2aa198",
            "#d33682",
            "#93a1a1",
            "#268bd2"
        ),
    ]
}

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
const THEME_STYLE_ID: &str = "musaic-theme-styles";

fn resolve_theme(stored: Option<String>) -> String {
    stored
        .filter(|stored| THEMES.iter().any(|(id, _)| id == stored))
        .unwrap_or_else(|| THEMES[0].0.to_string())
}

pub fn stored_theme() -> String {
    resolve_theme(
        web_sys::window()
            .and_then(|window| window.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item(THEME_KEY).ok().flatten()),
    )
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

fn inject_theme_css(css: &str) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let element = document.get_element_by_id(THEME_STYLE_ID).or_else(|| {
        let head = document.head()?;
        let element = document.create_element("style").ok()?;
        let _ = element.set_attribute("id", THEME_STYLE_ID);
        head.append_child(&element).ok()?;
        Some(element)
    });
    if let Some(element) = element {
        element.set_text_content(Some(css));
    }
}

#[derive(Clone, Copy)]
struct ThemeContext(RwSignal<String>);

#[derive(Clone, Copy)]
struct ThemeRegistry(RwSignal<Vec<Theme>>);

pub fn use_theme() -> RwSignal<String> {
    use_context::<ThemeContext>()
        .map(|context| context.0)
        .unwrap_or_else(|| RwSignal::new(stored_theme()))
}

pub fn use_themes() -> Vec<Theme> {
    use_context::<ThemeRegistry>()
        .map(|registry| registry.0.get())
        .unwrap_or_else(builtin_themes)
}

pub fn register_theme(theme: Theme) {
    if let Some(registry) = use_context::<ThemeRegistry>() {
        registry.0.update(|themes| {
            if let Some(slot) = themes.iter_mut().find(|existing| existing.id == theme.id) {
                *slot = theme;
            } else {
                themes.push(theme);
            }
        });
    }
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme = RwSignal::new(stored_theme());
    let registry = RwSignal::new(builtin_themes());
    provide_context(ThemeContext(theme));
    provide_context(ThemeRegistry(registry));

    Effect::new(move |_| {
        let css = registry.get().iter().map(Theme::to_css).collect::<String>();
        inject_theme_css(&css);
    });
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
            {move || {
                use_themes()
                    .into_iter()
                    .map(|entry| view! { <option value=entry.id.clone()>{entry.label}</option> })
                    .collect_view()
            }}
        </select>
    }
}

#[cfg(test)]
mod tests {
    use super::{THEMES, builtin_themes, resolve_theme};

    #[test]
    fn known_theme_is_preserved() {
        assert_eq!(resolve_theme(Some("dracula".to_string())), "dracula");
    }

    #[test]
    fn unknown_or_missing_theme_falls_back_to_default() {
        let default = THEMES[0].0;
        assert_eq!(resolve_theme(None), default);
        assert_eq!(resolve_theme(Some("does-not-exist".to_string())), default);
    }

    #[test]
    fn builtins_cover_the_theme_list_and_emit_tokens() {
        let themes = builtin_themes();
        assert_eq!(themes.len(), THEMES.len());
        for (theme, (id, _)) in themes.iter().zip(THEMES) {
            assert_eq!(&theme.id, id);
            assert!(theme.to_css().contains("--musaic-accent"));
        }
    }
}
