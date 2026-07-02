use leptos::prelude::*;
use leptos_musaic::{
    Command, CommandPalette, DragLayer, IconButton, KeymapProvider, MusaicStyles, THEMES,
    ThemePicker, ThemeProvider, ToastHub, Tree, provide_command_registry, provide_drag, use_theme,
    use_toaster,
};

use crate::sections;

#[derive(Clone, Copy)]
pub struct GalleryCtx {
    pub selected: RwSignal<String>,
    pub palette_open: RwSignal<bool>,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <MusaicStyles />
        <ThemeProvider>
            <ToastHub>
                <Shell />
            </ToastHub>
        </ThemeProvider>
    }
}

#[component]
fn Shell() -> impl IntoView {
    let selected = RwSignal::new(sections::DEFAULT.to_string());
    let palette_open = RwSignal::new(false);
    let sidebar_open = RwSignal::new(true);
    let toaster = use_toaster();
    let theme = use_theme();

    provide_context(GalleryCtx {
        selected,
        palette_open,
    });

    provide_drag();
    let registry = provide_command_registry();
    registry.register_all(sections::pages().into_iter().map(|(id, title)| {
        Command::new(
            format!("go-{id}"),
            format!("Go to {title}"),
            Callback::new(move |_| selected.set(id.to_string())),
        )
        .with_group("Navigate")
    }));
    registry.register(
        Command::new(
            "palette.open",
            "Command palette",
            Callback::new(move |_| palette_open.set(true)),
        )
        .with_keybinding("mod+k")
        .with_group("View"),
    );
    registry.register(
        Command::new(
            "sidebar.toggle",
            "Toggle sidebar",
            Callback::new(move |_| sidebar_open.update(|open| *open = !*open)),
        )
        .with_keybinding("mod+b")
        .with_group("View"),
    );
    registry.register(
        Command::new(
            "toast",
            "Fire a toast",
            Callback::new(move |_| toaster.info("Hello from the command palette")),
        )
        .with_group("Demo"),
    );
    let theme_children = THEMES
        .iter()
        .map(|(id, label)| {
            Command::new(
                format!("theme-{id}"),
                *label,
                Callback::new(move |_| theme.set(id.to_string())),
            )
        })
        .collect::<Vec<_>>();
    registry.register(Command::submenu("theme", "Switch theme", theme_children).with_group("View"));
    registry.register(
        Command::new(
            "goto-overview",
            "Go to overview",
            Callback::new(move |_| selected.set("overview".to_string())),
        )
        .with_keybinding("g o")
        .with_group("Go"),
    );
    registry.register(
        Command::new(
            "goto-dock",
            "Go to dock",
            Callback::new(move |_| selected.set("dock".to_string())),
        )
        .with_keybinding("g d")
        .with_group("Go"),
    );

    let nav = sections::nav_tree();
    let on_select = Callback::new(move |id: String| selected.set(id));
    let body_class = move || {
        if sidebar_open.get() {
            "gallery-body"
        } else {
            "gallery-body collapsed"
        }
    };

    view! {
        <KeymapProvider>
            <div class="gallery-shell">
                <div class="gallery-header">
                    <IconButton on_click=Callback::new(move |_| {
                        sidebar_open.update(|open| *open = !*open)
                    })>"\u{2630}"</IconButton>
                    <div class="gallery-brand">
                        <span class="gallery-dot"></span>
                        "musaic gallery"
                    </div>
                    <div class="gallery-spacer"></div>
                    <IconButton on_click=Callback::new(move |_| {
                        palette_open.set(true)
                    })>"\u{2318}K"</IconButton>
                    <ThemePicker />
                </div>
                <div class=body_class>
                    <div class="gallery-sidebar">
                        <div class="gallery-sidebar-title">"Components"</div>
                        <Tree
                            items=nav
                            on_select=on_select
                            selected=Signal::derive(move || Some(selected.get()))
                            default_expanded=true
                        />
                    </div>
                    <div class="gallery-content">{move || sections::render(&selected.get())}</div>
                </div>
            </div>
            <CommandPalette open=palette_open />
            <DragLayer />
        </KeymapProvider>
    }
}
