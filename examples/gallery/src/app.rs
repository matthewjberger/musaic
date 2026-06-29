use leptos::prelude::*;
use leptos_musaic::{
    Command, CommandPalette, IconButton, MusaicStyles, ThemePicker, ThemeProvider, ToastHub, Tree,
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

    provide_context(GalleryCtx {
        selected,
        palette_open,
    });

    let _ = window_event_listener(leptos::ev::keydown, move |event| {
        if (event.ctrl_key() || event.meta_key()) && event.key() == "k" {
            event.prevent_default();
            palette_open.update(|open| *open = !*open);
        }
    });

    let commands = Signal::derive(move || {
        let mut commands = sections::pages()
            .into_iter()
            .map(|(id, title)| {
                Command::new(
                    format!("go-{id}"),
                    format!("Go to {title}"),
                    Callback::new(move |_| selected.set(id.to_string())),
                )
            })
            .collect::<Vec<_>>();
        commands.push(
            Command::new(
                "toast",
                "Fire a toast",
                Callback::new(move |_| toaster.info("Hello from the command palette")),
            )
            .with_hint("demo"),
        );
        commands
    });

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
        <CommandPalette open=palette_open commands=commands />
    }
}
