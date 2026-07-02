use leptos::prelude::*;
use leptos_musaic::{
    AnsiTerminal, Button, Card, CodeEditor, ColorField, Command, CommandPalette, EditorShell,
    Highlighter, Inspector, InspectorSection, KeymapProvider, LogEntry, LogKind, LogView,
    MusaicStyles, Select, SliderField, StatusBar, StatusItem, StatusSpacer, Switch, THEMES, TabBar,
    Table, TerminalHandle, ThemeMenu, ThemeProvider, ToastHub, ToolButton, Toolbar, ToolbarGroup,
    ToolbarSpacer, provide_command_registry, terminal_grid, use_theme, use_toaster,
};

const RUST_KEYWORDS: &[&str] = &[
    "use", "fn", "let", "move", "impl", "pub", "mod", "match", "if", "else", "for", "in", "struct",
    "enum", "true", "false", "self",
];

const BOOTSTRAP: &str = r#"use leptos_musaic::prelude::*;

#[component]
fn App() -> impl IntoView {
    let count = RwSignal::new(0);
    view! {
        <MusaicStyles/>
        <ThemeProvider>
            <EditorShell
                toolbar=move || view! { <Toolbar>"my app"</Toolbar> }
                status=move || view! { <StatusBar>"ready"</StatusBar> }
            >
                <Panel title="Hello">
                    <ThemeMenu/>
                    <Button on_click=Callback::new(move |_| count.update(|n| *n += 1))>
                        "clicked " {move || count.get()}
                    </Button>
                </Panel>
            </EditorShell>
        </ThemeProvider>
    }
}"#;

fn rust_highlight(source: &str) -> Vec<(&'static str, String)> {
    leptos_musaic::highlight_code(source, RUST_KEYWORDS, &[])
}

const HIGHLIGHTER: Highlighter = rust_highlight;

fn rgb_to_hex(rgb: [f32; 3]) -> String {
    let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!(
        "#{:02x}{:02x}{:02x}",
        channel(rgb[0]),
        channel(rgb[1]),
        channel(rgb[2])
    )
}

fn set_accent(rgb: [f32; 3]) {
    if let Some(body) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.body())
    {
        let _ = body
            .style()
            .set_property("--musaic-accent", &rgb_to_hex(rgb));
    }
}

fn navigate(url: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_href(url);
    }
}

#[derive(Clone, Copy)]
struct Showcase {
    palette_open: RwSignal<bool>,
    sidebar_open: RwSignal<bool>,
    sidebar_width: RwSignal<f64>,
    dock_height: RwSignal<f64>,
    dock_tab: RwSignal<String>,
    density: RwSignal<f64>,
    animated: RwSignal<bool>,
    layout: RwSignal<String>,
    accent: RwSignal<[f32; 3]>,
    script: RwSignal<String>,
    log: RwSignal<Vec<LogEntry>>,
    log_seq: RwSignal<usize>,
}

impl Showcase {
    fn new() -> Self {
        Self {
            palette_open: RwSignal::new(false),
            sidebar_open: RwSignal::new(true),
            sidebar_width: RwSignal::new(320.0),
            dock_height: RwSignal::new(210.0),
            dock_tab: RwSignal::new("terminal".to_string()),
            density: RwSignal::new(1.0),
            animated: RwSignal::new(true),
            layout: RwSignal::new("comfortable".to_string()),
            accent: RwSignal::new([0.98, 0.45, 0.24]),
            script: RwSignal::new(BOOTSTRAP.to_string()),
            log: RwSignal::new(vec![
                LogEntry::new(0, LogKind::Info, "musaic showcase ready"),
                LogEntry::new(1, LogKind::Event, "theme engine online").with_detail("9 themes"),
            ]),
            log_seq: RwSignal::new(2),
        }
    }

    fn log_line(&self, kind: LogKind, message: impl Into<String>) {
        let id = self.log_seq.get_untracked();
        self.log_seq.set(id + 1);
        self.log.update(|entries| {
            entries.push(LogEntry::new(id, kind, message));
            if entries.len() > 200 {
                entries.remove(0);
            }
        });
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <MusaicStyles/>
        <ThemeProvider>
            <ToastHub>
                <Stage/>
            </ToastHub>
        </ThemeProvider>
    }
}

#[component]
fn Stage() -> impl IntoView {
    let state = Showcase::new();
    let theme = use_theme();
    let toaster = use_toaster();

    Effect::new(move |_| set_accent(state.accent.get()));

    let registry = provide_command_registry();
    registry.register_all([
        Command::new(
            "palette.open",
            "Command palette",
            Callback::new(move |_| state.palette_open.set(true)),
        )
        .with_keybinding("mod+k")
        .with_group("View"),
        Command::new(
            "toggle-sidebar",
            "Toggle sidebar",
            Callback::new(move |_| state.sidebar_open.update(|open| *open = !*open)),
        )
        .with_keybinding("mod+b")
        .with_group("View"),
        Command::new(
            "cycle-theme",
            "Cycle theme",
            Callback::new(move |_| {
                let current = theme.get_untracked();
                let index = THEMES
                    .iter()
                    .position(|(id, _)| *id == current)
                    .unwrap_or(0);
                let next = THEMES[(index + 1) % THEMES.len()].0;
                theme.set(next.to_string());
                state.log_line(LogKind::Command, format!("theme -> {next}"));
            }),
        )
        .with_keybinding("mod+j")
        .with_group("View"),
        Command::new(
            "toast",
            "Raise a toast",
            Callback::new(move |_| {
                toaster.success("built entirely from musaic");
                state.log_line(LogKind::Event, "toast raised");
            }),
        )
        .with_group("Demo"),
        Command::new(
            "open-gallery",
            "Open the gallery",
            Callback::new(move |_| navigate("gallery/")),
        )
        .with_group("Links"),
        Command::new(
            "open-guide",
            "Open the guide",
            Callback::new(move |_| navigate("book/")),
        )
        .with_group("Links"),
    ]);

    view! {
        <KeymapProvider>
            <EditorShell
                left_open=state.sidebar_open
                left_size=state.sidebar_width
                bottom_size=state.dock_height
                toolbar=move || view! { <TopBar state=state /> }
                left=move || view! { <SidePanel state=state /> }
                bottom=move || view! { <Dock state=state /> }
                status=move || view! { <StatusStrip /> }
            >
                <Center state=state />
            </EditorShell>
            <CommandPalette open=state.palette_open />
        </KeymapProvider>
    }
}

#[component]
fn TopBar(state: Showcase) -> impl IntoView {
    view! {
        <Toolbar>
            <ToolbarGroup>
                <div class="brand">
                    <span class="brand-dot"></span>
                    "musaic"
                </div>
                <span class="brand-sub">"a Leptos component library"</span>
            </ToolbarGroup>
            <ToolbarSpacer />
            <ToolButton on_click=Callback::new(move |_| navigate("gallery/"))>"Gallery"</ToolButton>
            <ToolButton on_click=Callback::new(move |_| navigate("demo/"))>"Live demo"</ToolButton>
            <ToolButton on_click=Callback::new(move |_| navigate("book/"))>"Guide"</ToolButton>
            <ToolButton on_click=Callback::new(move |_| {
                navigate("https://github.com/matthewjberger/musaic")
            })>"GitHub"</ToolButton>
            <ToolButton on_click=Callback::new(move |_| state.palette_open.set(true))>
                "\u{2318}K"
            </ToolButton>
            <ThemeMenu />
        </Toolbar>
    }
}

#[component]
fn SidePanel(state: Showcase) -> impl IntoView {
    view! {
        <Inspector>
            <InspectorSection title="Appearance">
                <ColorField
                    label="Accent"
                    value=Signal::derive(move || state.accent.get())
                    on_change=Callback::new(move |(rgb, _committed): ([f32; 3], bool)| {
                        state.accent.set(rgb)
                    })
                />
                <span class="hint">"Live edits the --musaic-accent token."</span>
            </InspectorSection>
            <InspectorSection title="Layout">
                <SliderField
                    label="Density"
                    value=Signal::derive(move || state.density.get())
                    min=Signal::derive(|| 0.5)
                    max=Signal::derive(|| 2.0)
                    step=0.05
                    on_change=Callback::new(move |(value, _c): (f64, bool)| state.density.set(value))
                />
                <Switch
                    label="Animated"
                    value=Signal::derive(move || state.animated.get())
                    on_change=Callback::new(move |on: bool| state.animated.set(on))
                />
                <Select
                    label="Mode"
                    value=Signal::derive(move || state.layout.get())
                    options=vec![
                        ("comfortable".into(), "Comfortable".into()),
                        ("compact".into(), "Compact".into()),
                        ("spacious".into(), "Spacious".into()),
                    ]
                    on_change=Callback::new(move |value: String| state.layout.set(value))
                />
            </InspectorSection>
        </Inspector>
    }
}

#[component]
fn Center(state: Showcase) -> impl IntoView {
    let rows = Signal::derive(|| {
        vec![
            vec!["EditorShell".into(), "layout".into(), "resizable".into()],
            vec!["CommandPalette".into(), "commands".into(), "fuzzy".into()],
            vec!["MultiEditor".into(), "code".into(), "multi-cursor".into()],
            vec!["AnsiTerminal".into(), "terminal".into(), "vt/ansi".into()],
            vec!["Table".into(), "data".into(), "sort/filter".into()],
            vec!["ThemeMenu".into(), "theming".into(), "hover-preview".into()],
        ]
    });

    view! {
        <div class="center-scroll">
            <section class="hero">
                <h1 class="hero-title">"Build beautiful Leptos UIs"</h1>
                <p class="hero-tagline">
                    "musaic is a feature-gated component library for editor-grade UIs that run the "
                    "same code on the web and natively. Everything on this page is built with it."
                </p>
                <div class="hero-cta">
                    <Button on_click=Callback::new(move |_| navigate("gallery/"))>
                        "Explore the gallery"
                    </Button>
                    <Button on_click=Callback::new(move |_| navigate("demo/"))>
                        "Open the live demo"
                    </Button>
                    <Button on_click=Callback::new(move |_| navigate("book/"))>"Read the guide"</Button>
                </div>
                <p class="hero-hint">"Try Cmd/Ctrl+K, or switch themes from the top right."</p>
            </section>

            <section class="feature-grid">
                <Card title="Themed to the token">
                    "Every component draws from one set of CSS custom properties. Edit the accent in "
                    "the sidebar and watch the whole page follow."
                </Card>
                <Card title="Commands and keys">
                    "One registry feeds the palette, the keybindings, and your menus. Register once, "
                    "reach it everywhere."
                </Card>
                <Card title="Editor-grade parts">
                    "A multi-cursor editor, a foldable code view, an ANSI terminal, tables, trees, "
                    "and a resizable app shell."
                </Card>
                <Card title="Web and native">
                    "The same code renders as wasm in the browser and inside a desktop webview."
                </Card>
            </section>

            <section class="playground">
                <div class="playground-panel">
                    <div class="playground-head">"State reacts live"</div>
                    <div class="readout">
                        <div>"density " <b>{move || format!("{:.2}", state.density.get())}</b></div>
                        <div>"mode " <b>{move || state.layout.get()}</b></div>
                        <div>
                            "animated "
                            <b>{move || if state.animated.get() { "on" } else { "off" }}</b>
                        </div>
                    </div>
                    <Table
                        headers=vec!["component".into(), "feature".into(), "note".into()]
                        rows=rows
                        sortable=true
                        filterable=true
                        resizable=true
                    />
                </div>
                <div class="playground-panel">
                    <div class="playground-head">"Four lines to a themed app"</div>
                    <CodeEditor value=state.script highlighter=HIGHLIGHTER fill=true />
                </div>
            </section>
        </div>
    }
}

#[component]
fn Dock(state: Showcase) -> impl IntoView {
    let terminal = terminal_grid(72, 10);
    seed_terminal(terminal);
    view! {
        <div class="dock">
            <TabBar
                tabs=vec![
                    ("terminal".into(), "Terminal".into()),
                    ("console".into(), "Console".into()),
                ]
                active=state.dock_tab
            />
            <div class="dock-body">
                <Show
                    when=move || state.dock_tab.get() == "terminal"
                    fallback=move || {
                        view! {
                            <LogView
                                entries=state.log
                                on_clear=Callback::new(move |_| state.log.update(Vec::clear))
                            />
                        }
                    }
                >
                    <AnsiTerminal handle=terminal />
                </Show>
            </div>
        </div>
    }
}

fn seed_terminal(handle: TerminalHandle) {
    handle.feed("\u{1b}[38;5;208m  ___ ___ _   _ ___ __ _ ___\r\n");
    handle.feed(" | '_ \\ | | | | | | / _` | / _|\r\n");
    handle.feed(
        " | .__/_|_|_|_| |_|_\\__,_|_\\__|\u{1b}[0m   \u{1b}[38;5;213mmusaic\u{1b}[0m\r\n\r\n",
    );
    handle.feed("\u{1b}[32m[ok]\u{1b}[0m theme engine   \u{1b}[90m9 themes\u{1b}[0m\r\n");
    handle.feed("\u{1b}[32m[ok]\u{1b}[0m command palette \u{1b}[90mfuzzy + chords\u{1b}[0m\r\n");
    handle.feed("\u{1b}[36m[..]\u{1b}[0m ansi parser     \u{1b}[90m256 + truecolor\u{1b}[0m\r\n");
    handle.feed("\u{1b}[1mready.\u{1b}[0m ");
}

#[component]
fn StatusStrip() -> impl IntoView {
    view! {
        <StatusBar>
            <StatusItem icon="\u{25c9}">"musaic"</StatusItem>
            <StatusItem>"30+ components"</StatusItem>
            <StatusItem>"9 themes"</StatusItem>
            <StatusSpacer />
            <StatusItem>"web + native"</StatusItem>
            <StatusItem>"built with musaic"</StatusItem>
        </StatusBar>
    }
}
