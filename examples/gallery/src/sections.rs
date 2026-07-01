use std::collections::HashSet;

use leptos::prelude::*;
use leptos_musaic::{
    Badge, Button, Card, CheckField, CodeEditor, ColorField, ContextMenu, DockLayout, DockMain,
    DockPanel, DockSide, IconButton, Inspector, InspectorRow, InspectorSection, Menu, MenuItem,
    MenuSeparator, Modal, NumberField, Panel, Progress, ResizeAxis, ResizeHandle, Select,
    SliderField, Spinner, SplitAxis, Submenu, Switch, TabBar, Table, TextField, Theme, ThemePicker,
    Tooltip, Tree, TreeItem, Vec3Field, highlight_rhai, pretty_binding, register_theme,
    use_commands, use_theme, use_toaster,
};
use web_sys::MouseEvent;

use crate::app::GalleryCtx;

pub const DEFAULT: &str = "overview";

struct Page {
    id: &'static str,
    title: &'static str,
}

struct Category {
    id: &'static str,
    title: &'static str,
    icon: &'static str,
    pages: &'static [Page],
}

const CATEGORIES: &[Category] = &[
    Category {
        id: "start",
        title: "Getting started",
        icon: "\u{1f4d8}",
        pages: &[
            Page {
                id: "overview",
                title: "Overview",
            },
            Page {
                id: "theming",
                title: "Theming",
            },
            Page {
                id: "command-palette",
                title: "Command palette",
            },
            Page {
                id: "keybindings",
                title: "Commands & keys",
            },
        ],
    },
    Category {
        id: "base",
        title: "Base",
        icon: "\u{1f9f1}",
        pages: &[
            Page {
                id: "buttons",
                title: "Buttons",
            },
            Page {
                id: "badges",
                title: "Badges",
            },
            Page {
                id: "card",
                title: "Card",
            },
            Page {
                id: "panel",
                title: "Panel",
            },
            Page {
                id: "progress",
                title: "Progress",
            },
            Page {
                id: "tooltip",
                title: "Tooltip",
            },
            Page {
                id: "spinner",
                title: "Spinner",
            },
            Page {
                id: "modal",
                title: "Modal",
            },
            Page {
                id: "toasts",
                title: "Toasts",
            },
            Page {
                id: "layout",
                title: "Layout & resize",
            },
        ],
    },
    Category {
        id: "forms",
        title: "Forms",
        icon: "\u{1f5a9}",
        pages: &[
            Page {
                id: "number-field",
                title: "NumberField",
            },
            Page {
                id: "text-field",
                title: "TextField",
            },
            Page {
                id: "check-field",
                title: "CheckField",
            },
            Page {
                id: "switch",
                title: "Switch",
            },
            Page {
                id: "slider",
                title: "SliderField",
            },
            Page {
                id: "color-field",
                title: "ColorField",
            },
            Page {
                id: "select",
                title: "Select",
            },
            Page {
                id: "vec3",
                title: "Vec3Field",
            },
        ],
    },
    Category {
        id: "menus",
        title: "Menus & navigation",
        icon: "\u{1f9ed}",
        pages: &[
            Page {
                id: "menu",
                title: "Menu",
            },
            Page {
                id: "context-menu",
                title: "ContextMenu",
            },
            Page {
                id: "tabs",
                title: "TabBar",
            },
        ],
    },
    Category {
        id: "data",
        title: "Data",
        icon: "\u{1f5c2}",
        pages: &[
            Page {
                id: "table",
                title: "Table",
            },
            Page {
                id: "tree",
                title: "Tree",
            },
            Page {
                id: "inspector",
                title: "Inspector",
            },
        ],
    },
    Category {
        id: "layout-sys",
        title: "Layout system",
        icon: "\u{1f5c3}",
        pages: &[Page {
            id: "dock",
            title: "Dock",
        }],
    },
    Category {
        id: "editor",
        title: "Editor",
        icon: "\u{1f4dd}",
        pages: &[Page {
            id: "code-editor",
            title: "CodeEditor",
        }],
    },
    Category {
        id: "engine-cat",
        title: "Engine",
        icon: "\u{1f3ae}",
        pages: &[Page {
            id: "engine",
            title: "Engine integration",
        }],
    },
];

pub fn pages() -> Vec<(&'static str, &'static str)> {
    CATEGORIES
        .iter()
        .flat_map(|category| category.pages.iter())
        .map(|page| (page.id, page.title))
        .collect()
}

pub fn nav_tree() -> Vec<TreeItem> {
    CATEGORIES
        .iter()
        .map(|category| {
            let children = category
                .pages
                .iter()
                .map(|page| TreeItem::leaf(page.id, page.title))
                .collect();
            TreeItem::branch(category.id, category.title, children).with_icon(category.icon)
        })
        .collect()
}

pub fn render(id: &str) -> AnyView {
    match id {
        "overview" => view! { <Overview /> }.into_any(),
        "theming" => view! { <ThemingDemo /> }.into_any(),
        "command-palette" => view! { <CommandPaletteDemo /> }.into_any(),
        "keybindings" => view! { <KeybindingsDemo /> }.into_any(),
        "buttons" => view! { <ButtonsDemo /> }.into_any(),
        "badges" => view! { <BadgesDemo /> }.into_any(),
        "card" => view! { <CardDemo /> }.into_any(),
        "panel" => view! { <PanelDemo /> }.into_any(),
        "progress" => view! { <ProgressDemo /> }.into_any(),
        "tooltip" => view! { <TooltipDemo /> }.into_any(),
        "spinner" => view! { <SpinnerDemo /> }.into_any(),
        "modal" => view! { <ModalDemo /> }.into_any(),
        "toasts" => view! { <ToastsDemo /> }.into_any(),
        "layout" => view! { <LayoutDemo /> }.into_any(),
        "number-field" => view! { <NumberFieldDemo /> }.into_any(),
        "text-field" => view! { <TextFieldDemo /> }.into_any(),
        "check-field" => view! { <CheckFieldDemo /> }.into_any(),
        "switch" => view! { <SwitchDemo /> }.into_any(),
        "slider" => view! { <SliderDemo /> }.into_any(),
        "color-field" => view! { <ColorFieldDemo /> }.into_any(),
        "select" => view! { <SelectDemo /> }.into_any(),
        "vec3" => view! { <VecFieldDemo /> }.into_any(),
        "dock" => view! { <DockDemo /> }.into_any(),
        "menu" => view! { <MenuDemo /> }.into_any(),
        "context-menu" => view! { <ContextMenuDemo /> }.into_any(),
        "tabs" => view! { <TabsDemo /> }.into_any(),
        "table" => view! { <TableDemo /> }.into_any(),
        "tree" => view! { <TreeDemo /> }.into_any(),
        "inspector" => view! { <InspectorDemo /> }.into_any(),
        "code-editor" => view! { <CodeEditorDemo /> }.into_any(),
        "engine" => view! { <EngineDemo /> }.into_any(),
        other => match CATEGORIES.iter().find(|category| category.id == other) {
            Some(category) => category_landing(category),
            None => view! { <Overview /> }.into_any(),
        },
    }
}

fn category_landing(category: &'static Category) -> AnyView {
    let selected = expect_context::<GalleryCtx>().selected;
    view! {
        <article class="gallery-page">
            <h1>{category.title}</h1>
            <p class="gallery-blurb">"Choose a component from this section."</p>
            <div class="gallery-stage">
                <div class="gallery-row">
                    {category
                        .pages
                        .iter()
                        .map(|page| {
                            let id = page.id;
                            view! {
                                <Button on_click=Callback::new(move |_| {
                                    selected.set(id.to_string())
                                })>{page.title}</Button>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </article>
    }
    .into_any()
}

#[component]
fn Demo(
    #[prop(into)] title: String,
    #[prop(into)] blurb: String,
    children: Children,
) -> impl IntoView {
    view! {
        <article class="gallery-page">
            <h1>{title}</h1>
            <p class="gallery-blurb">{blurb}</p>
            <div class="gallery-stage">{children()}</div>
        </article>
    }
}

const SNIPPET_KEYWORDS: &[&str] = &[
    "let", "mut", "fn", "move", "pub", "use", "impl", "for", "in", "if", "else", "match", "as",
    "const", "return", "while", "loop", "true", "false", "self",
];

fn highlight_snippet(source: &str) -> Vec<(&'static str, String)> {
    let chars: Vec<char> = source.chars().collect();
    let count = chars.len();
    let mut runs: Vec<(&'static str, String)> = Vec::new();
    let mut index = 0;
    let mut expect_tag = false;
    while index < count {
        let current = chars[index];
        if current == '/' && index + 1 < count && chars[index + 1] == '/' {
            let start = index;
            while index < count && chars[index] != '\n' {
                index += 1;
            }
            runs.push(("tok-comment", chars[start..index].iter().collect()));
            expect_tag = false;
        } else if current == '"' {
            let start = index;
            index += 1;
            while index < count {
                if chars[index] == '\\' && index + 1 < count {
                    index += 2;
                    continue;
                }
                let quote = chars[index] == '"';
                index += 1;
                if quote {
                    break;
                }
            }
            runs.push(("tok-string", chars[start..index].iter().collect()));
            expect_tag = false;
        } else if current.is_ascii_digit() {
            let start = index;
            while index < count && (chars[index].is_ascii_alphanumeric() || chars[index] == '.') {
                index += 1;
            }
            runs.push(("tok-number", chars[start..index].iter().collect()));
            expect_tag = false;
        } else if current.is_alphabetic() || current == '_' {
            let start = index;
            while index < count && (chars[index].is_alphanumeric() || chars[index] == '_') {
                index += 1;
            }
            let word: String = chars[start..index].iter().collect();
            let starts_upper = word.chars().next().is_some_and(char::is_uppercase);
            let class = if expect_tag || starts_upper {
                "tok-command"
            } else if SNIPPET_KEYWORDS.contains(&word.as_str()) {
                "tok-keyword"
            } else {
                "tok-plain"
            };
            runs.push((class, word));
            expect_tag = false;
        } else {
            let start = index;
            index += 1;
            while index < count {
                let next = chars[index];
                let token_start = (next == '/' && index + 1 < count && chars[index + 1] == '/')
                    || next == '"'
                    || next.is_ascii_digit()
                    || next.is_alphabetic()
                    || next == '_';
                if token_start {
                    break;
                }
                index += 1;
            }
            let run: String = chars[start..index].iter().collect();
            let trimmed = run.trim_end();
            expect_tag = trimmed.ends_with('<') || trimmed.ends_with("</");
            runs.push(("tok-plain", run));
        }
    }
    runs
}

#[component]
fn Snippet(#[prop(into)] code: String) -> impl IntoView {
    let spans = highlight_snippet(&code)
        .into_iter()
        .map(|(class, text)| view! { <span class=class>{text}</span> })
        .collect_view();
    view! { <pre class="gallery-code">{spans}</pre> }
}

#[component]
fn Overview() -> impl IntoView {
    view! {
        <article class="gallery-page">
            <h1>"musaic gallery"</h1>
            <p class="gallery-blurb">
                "Every musaic component, live and interactive. Pick a topic on the left. Switch the theme in the top right and watch the whole gallery restyle, since every widget here is built from the same design tokens. Press Ctrl+K for the command palette, and use the menu button to collapse the sidebar."
            </p>
            <div class="gallery-stage">
                <div class="gallery-row">
                    <Badge variant="accent">"forms"</Badge>
                    <Badge variant="accent">"menus"</Badge>
                    <Badge variant="accent">"themes"</Badge>
                    <Badge variant="accent">"command palette"</Badge>
                    <Badge variant="accent">"code editor"</Badge>
                    <Badge variant="accent">"table"</Badge>
                    <Badge variant="accent">"tree"</Badge>
                    <Badge variant="accent">"inspector"</Badge>
                    <Badge variant="accent">"viewport"</Badge>
                    <Badge variant="accent">"engine"</Badge>
                </div>
                <Card title="Same code, native and web">
                    "Run this gallery in the browser with " <code>"just run-gallery-wasm"</code>
                    " or as a native desktop window with " <code>"just run-gallery"</code>
                    ". Both render the exact same Leptos components."
                </Card>
            </div>
        </article>
    }
}

#[component]
fn ButtonsDemo() -> impl IntoView {
    let count = RwSignal::new(0u32);
    view! {
        <Demo title="Buttons" blurb="Button takes a class for variants and an on_click callback. IconButton is the square, borderless variant for toolbars.">
            <div class="gallery-row">
                <Button>"Default"</Button>
                <Button class="primary">"Primary"</Button>
                <Button class="danger">"Danger"</Button>
                <Button class="ghost">"Ghost"</Button>
                <IconButton>"\u{2605}"</IconButton>
            </div>
            <div class="gallery-row">
                <Button
                    class="primary"
                    on_click=Callback::new(move |_| count.update(|value| *value += 1))
                >
                    "Clicked"
                </Button>
                <span class="gallery-readout">{move || format!("count = {}", count.get())}</span>
            </div>
            <Snippet code="<Button class=\"primary\" on_click=on_save>\"Save\"</Button>" />
        </Demo>
    }
}

#[component]
fn BadgesDemo() -> impl IntoView {
    view! {
        <Demo title="Badges" blurb="Small status pills. The variant prop selects accent or danger; the default is muted.">
            <div class="gallery-row">
                <Badge>"default"</Badge>
                <Badge variant="accent">"accent"</Badge>
                <Badge variant="danger">"danger"</Badge>
            </div>
            <Snippet code="<Badge variant=\"accent\">\"new\"</Badge>" />
        </Demo>
    }
}

#[component]
fn CardDemo() -> impl IntoView {
    view! {
        <Demo title="Card" blurb="A titled container for grouped content. Pass a title or leave it off for a plain surface.">
            <Card title="Render settings">
                "Cards hold arbitrary children and use the panel tokens, so they sit on any theme."
            </Card>
            <Card>"A card with no title is just a padded, bordered surface."</Card>
            <Snippet code="<Card title=\"Render settings\">{children}</Card>" />
        </Demo>
    }
}

#[component]
fn PanelDemo() -> impl IntoView {
    view! {
        <Demo title="Panel" blurb="The workhorse container for sidebars and docks: an uppercase title bar over a padded body.">
            <Panel title="Scene">
                <div class="gallery-row">
                    <Badge variant="accent">"12 entities"</Badge>
                    <Badge>"60 fps"</Badge>
                </div>
            </Panel>
            <Snippet code="<Panel title=\"Scene\">{children}</Panel>" />
        </Demo>
    }
}

#[component]
fn ProgressDemo() -> impl IntoView {
    let value = RwSignal::new(0.35);
    view! {
        <Demo title="Progress" blurb="A determinate progress bar driven by a reactive value. Drag the slider to update it.">
            <Progress value=Signal::derive(move || value.get()) />
            <SliderField
                label="Value"
                value=Signal::derive(move || value.get())
                min=Signal::derive(|| 0.0)
                max=Signal::derive(|| 1.0)
                step=0.01
                on_change=Callback::new(move |(next, _): (f64, bool)| value.set(next))
            />
            <Snippet code="<Progress value=progress max=1.0 />" />
        </Demo>
    }
}

#[component]
fn TooltipDemo() -> impl IntoView {
    view! {
        <Demo title="Tooltip" blurb="Wrap any element to show a hint on hover or keyboard focus. It is CSS-driven, so it costs nothing until shown.">
            <div class="gallery-row">
                <Tooltip text="Saves to local storage">
                    <Button>"Hover me"</Button>
                </Tooltip>
                <Tooltip text="Also shows on focus">
                    <Button class="ghost">"Or tab to me"</Button>
                </Tooltip>
            </div>
            <Snippet code="<Tooltip text=\"Saves to disk\"><Button>\"Save\"</Button></Tooltip>" />
        </Demo>
    }
}

#[component]
fn SpinnerDemo() -> impl IntoView {
    view! {
        <Demo title="Spinner" blurb="A small indeterminate activity indicator that inherits the accent color.">
            <div class="gallery-row">
                <Spinner />
                <span class="gallery-readout">"working..."</span>
            </div>
            <Snippet code="<Spinner />" />
        </Demo>
    }
}

#[component]
fn ModalDemo() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <Demo title="Modal" blurb="A centered dialog over a scrim. It traps Tab focus, closes on Escape or backdrop click, and restores focus when dismissed.">
            <Button class="primary" on_click=Callback::new(move |_| open.set(true))>
                "Open modal"
            </Button>
            <Modal open=open>
                <div style="padding:20px; display:flex; flex-direction:column; gap:12px; min-width:280px;">
                    <strong>"Delete scene?"</strong>
                    <span class="gallery-readout">
                        "Tab cycles the buttons. Escape closes. Focus returns to the trigger."
                    </span>
                    <div class="gallery-row">
                        <Button class="danger" on_click=Callback::new(move |_| open.set(false))>
                            "Delete"
                        </Button>
                        <Button on_click=Callback::new(move |_| open.set(false))>"Cancel"</Button>
                    </div>
                </div>
            </Modal>
            <Snippet code="let open = RwSignal::new(false);\n<Modal open=open>{children}</Modal>" />
        </Demo>
    }
}

#[component]
fn ToastsDemo() -> impl IntoView {
    let toaster = use_toaster();
    view! {
        <Demo title="Toasts" blurb="Transient notifications. Mount ToastHub once at the root, then call use_toaster() anywhere to push info or error toasts.">
            <div class="gallery-row">
                <Button on_click=Callback::new(move |_| toaster.info("Saved to disk"))>
                    "Info toast"
                </Button>
                <Button
                    class="danger"
                    on_click=Callback::new(move |_| toaster.error("Export failed"))
                >
                    "Error toast"
                </Button>
            </div>
            <Snippet code="let toaster = use_toaster();\ntoaster.info(\"Saved\");" />
        </Demo>
    }
}

#[component]
fn LayoutDemo() -> impl IntoView {
    let width = RwSignal::new(180.0);
    view! {
        <Demo title="Layout & resize" blurb="Row, Column, and Grid are thin fl/grid wrappers. ResizeHandle turns a signal into a draggable splitter; here it resizes the left pane.">
            <div class="gallery-split">
                <div
                    class="gallery-split-pane"
                    style=move || format!("width:{}px", width.get())
                >
                    "left"
                </div>
                <ResizeHandle value=width axis=ResizeAxis::Horizontal min=80.0 max=320.0 />
                <div class="gallery-split-pane" style="flex:1">
                    "right"
                </div>
            </div>
            <span class="gallery-readout">{move || format!("left width = {:.0}px", width.get())}</span>
            <Snippet code="<ResizeHandle value=width axis=ResizeAxis::Horizontal min=80.0 max=320.0 />" />
        </Demo>
    }
}

#[component]
fn NumberFieldDemo() -> impl IntoView {
    let value = RwSignal::new(3.0);
    let scale = RwSignal::new(1.0);
    view! {
        <Demo title="NumberField" blurb="A labeled numeric input. It accepts arithmetic expressions (type 2+3*4 or 90/2 and press Enter), clamps to min/max, and can run a custom validator that shows an inline error.">
            <Panel>
                <NumberField
                    label="Copies"
                    value=Signal::derive(move || value.get())
                    min=0.0
                    max=10.0
                    integer=true
                    help="Whole numbers 0 to 10; try typing 2*4"
                    on_change=Callback::new(move |(next, _): (f64, bool)| value.set(next))
                />
                <NumberField
                    label="Scale"
                    value=Signal::derive(move || scale.get())
                    help="Try an expression like 1.5*2"
                    validate=Callback::new(|candidate: f64| {
                        (candidate <= 0.0).then(|| "Scale must be positive".to_string())
                    })
                    on_change=Callback::new(move |(next, _): (f64, bool)| scale.set(next))
                />
            </Panel>
            <span class="gallery-readout">
                {move || format!("copies = {}, scale = {:.3}", value.get(), scale.get())}
            </span>
            <Snippet code="<NumberField label=\"Scale\" value=v validate=Callback::new(|n| (n <= 0.0).then(|| \"must be positive\".into())) on_change=cb />" />
        </Demo>
    }
}

#[component]
fn TextFieldDemo() -> impl IntoView {
    let value = RwSignal::new("untitled".to_string());
    let search = RwSignal::new(String::new());
    view! {
        <Demo title="TextField" blurb="A labeled text input that commits on blur, or, with a debounce set, live as you stop typing. Supports placeholder, help, error, and disabled.">
            <Panel>
                <TextField
                    label="Name"
                    value=Signal::derive(move || value.get())
                    placeholder="Scene name"
                    help="Commits when you click away"
                    on_commit=Callback::new(move |next: String| value.set(next))
                />
                <TextField
                    label="Search"
                    value=Signal::derive(move || search.get())
                    placeholder="Type to search"
                    help="Commits 400ms after you stop typing"
                    debounce=400
                    on_commit=Callback::new(move |next: String| search.set(next))
                />
            </Panel>
            <span class="gallery-readout">{move || format!("committed = {}", value.get())}</span>
            <span class="gallery-readout">{move || format!("debounced search = {}", search.get())}</span>
            <Snippet code="<TextField label=\"Search\" value=v debounce=400 on_commit=cb />" />
        </Demo>
    }
}

#[component]
fn CheckFieldDemo() -> impl IntoView {
    let value = RwSignal::new(true);
    view! {
        <Demo title="CheckField" blurb="A labeled checkbox bound to a boolean signal.">
            <Panel>
                <CheckField
                    label="Cast shadows"
                    value=Signal::derive(move || value.get())
                    on_change=Callback::new(move |next: bool| value.set(next))
                />
            </Panel>
            <span class="gallery-readout">{move || format!("checked = {}", value.get())}</span>
            <Snippet code="<CheckField label=\"Cast shadows\" value=v on_change=cb />" />
        </Demo>
    }
}

#[component]
fn SwitchDemo() -> impl IntoView {
    let value = RwSignal::new(false);
    view! {
        <Demo title="Switch" blurb="A role=switch toggle, distinct from a checkbox, for on/off settings.">
            <Panel>
                <Switch
                    label="Wireframe"
                    value=Signal::derive(move || value.get())
                    on_change=Callback::new(move |next: bool| value.set(next))
                />
            </Panel>
            <span class="gallery-readout">{move || format!("on = {}", value.get())}</span>
            <Snippet code="<Switch label=\"Wireframe\" value=v on_change=cb />" />
        </Demo>
    }
}

#[component]
fn SliderDemo() -> impl IntoView {
    let value = RwSignal::new(1.5);
    view! {
        <Demo title="SliderField" blurb="A labeled range slider with a live numeric readout. min and max are reactive signals.">
            <Panel>
                <SliderField
                    label="Speed"
                    value=Signal::derive(move || value.get())
                    min=Signal::derive(|| 0.0)
                    max=Signal::derive(|| 4.0)
                    step=0.05
                    on_change=Callback::new(move |(next, _): (f64, bool)| value.set(next))
                />
            </Panel>
            <Snippet code="<SliderField label=\"Speed\" value=v min=lo max=hi step=0.05 on_change=cb />" />
        </Demo>
    }
}

#[component]
fn ColorFieldDemo() -> impl IntoView {
    let value = RwSignal::new([0.98, 0.57, 0.24]);
    view! {
        <Demo title="ColorField" blurb="A labeled color picker bound to an RGB float triple, handy for material and light colors.">
            <Panel>
                <ColorField
                    label="Albedo"
                    value=Signal::derive(move || value.get())
                    on_change=Callback::new(move |(next, _): ([f32; 3], bool)| value.set(next))
                />
            </Panel>
            <span class="gallery-readout">
                {move || {
                    let rgb = value.get();
                    format!("rgb = [{:.2}, {:.2}, {:.2}]", rgb[0], rgb[1], rgb[2])
                }}
            </span>
            <Snippet code="<ColorField label=\"Albedo\" value=v on_change=cb />" />
        </Demo>
    }
}

#[component]
fn SelectDemo() -> impl IntoView {
    let value = RwSignal::new("nebula".to_string());
    view! {
        <Demo title="Select" blurb="A labeled dropdown over (value, label) pairs.">
            <Panel>
                <Select
                    label="Sky"
                    value=Signal::derive(move || value.get())
                    options=vec![
                        ("nebula".into(), "Nebula".into()),
                        ("sky".into(), "Sky".into()),
                        ("space".into(), "Space".into()),
                        ("sunset".into(), "Sunset".into()),
                    ]
                    on_change=Callback::new(move |next: String| value.set(next))
                />
            </Panel>
            <span class="gallery-readout">{move || format!("selected = {}", value.get())}</span>
            <Snippet code="<Select label=\"Sky\" value=v options=opts on_change=cb />" />
        </Demo>
    }
}

#[component]
fn VecFieldDemo() -> impl IntoView {
    let value = RwSignal::new([0.0, 1.5, 0.0]);
    view! {
        <Demo title="Vec3Field" blurb="Three linked numeric axes for positions, rotations, and scales. Each axis accepts an arithmetic expression, so typing 1+1 or 90/2 and pressing Enter commits the evaluated value.">
            <Panel>
                <Vec3Field
                    label="Position"
                    value=Signal::derive(move || value.get())
                    step=0.1
                    on_change=Callback::new(move |(next, _): ([f64; 3], bool)| value.set(next))
                />
            </Panel>
            <span class="gallery-readout">
                {move || {
                    let vector = value.get();
                    format!("[{:.2}, {:.2}, {:.2}]", vector[0], vector[1], vector[2])
                }}
            </span>
            <Snippet code="<Vec3Field label=\"Position\" value=v on_change=cb />" />
        </Demo>
    }
}

#[component]
fn DockDemo() -> impl IntoView {
    let left = RwSignal::new(200.0);
    let right = RwSignal::new(220.0);
    let bottom = RwSignal::new(140.0);
    let left_collapsed = RwSignal::new(false);
    view! {
        <Demo title="Dock" blurb="A composable editor layout: resizable, collapsible panels docked around a main region. Drag the edges to resize, and use the minus button to collapse the left panel. This is the backbone of an editor-style app.">
            <div class="gallery-dock-frame">
                <DockLayout axis=SplitAxis::Column>
                    <DockMain>
                        <DockLayout axis=SplitAxis::Row>
                            <DockPanel
                                title="Hierarchy"
                                side=DockSide::Start
                                size=left
                                collapsible=true
                                collapsed=left_collapsed
                            >
                                <div class="gallery-readout">"Scene tree, layers, outliner…"</div>
                            </DockPanel>
                            <DockMain>
                                <div class="gallery-dock-viewport">"Viewport"</div>
                            </DockMain>
                            <DockPanel title="Inspector" side=DockSide::End size=right>
                                <div class="gallery-readout">"Properties for the selection."</div>
                            </DockPanel>
                        </DockLayout>
                    </DockMain>
                    <DockPanel title="Console" side=DockSide::End size=bottom>
                        <div class="gallery-readout">"Logs, timeline, asset browser…"</div>
                    </DockPanel>
                </DockLayout>
            </div>
            <Snippet code="<DockLayout axis=SplitAxis::Row><DockPanel title=\"Hierarchy\" size=w collapsible=true>...</DockPanel><DockMain>...</DockMain></DockLayout>" />
        </Demo>
    }
}

#[component]
fn MenuDemo() -> impl IntoView {
    let toaster = use_toaster();
    let wireframe = RwSignal::new(false);
    let shadows = RwSignal::new(true);
    view! {
        <Demo title="Menu" blurb="A click-to-open dropdown with keyboard navigation (arrow keys, Home/End, Escape) and outside-click dismissal. Items can carry shortcuts, checkable state, separators, and nested submenus.">
            <Menu label="File">
                <MenuItem
                    label="New scene"
                    shortcut="Ctrl+N"
                    on_select=Callback::new(move |_| toaster.info("New scene"))
                />
                <MenuItem
                    label="Open"
                    shortcut="Ctrl+O"
                    on_select=Callback::new(move |_| toaster.info("Open"))
                />
                <MenuSeparator />
                <MenuItem
                    label="Wireframe"
                    checked=Signal::derive(move || wireframe.get())
                    on_select=Callback::new(move |_| wireframe.update(|value| *value = !*value))
                />
                <MenuItem
                    label="Cast shadows"
                    checked=Signal::derive(move || shadows.get())
                    on_select=Callback::new(move |_| shadows.update(|value| *value = !*value))
                />
                <MenuSeparator />
                <Submenu label="Export">
                    <MenuItem
                        label="glTF"
                        on_select=Callback::new(move |_| toaster.info("Export glTF"))
                    />
                    <MenuItem
                        label="OBJ"
                        on_select=Callback::new(move |_| toaster.info("Export OBJ"))
                    />
                </Submenu>
                <MenuItem label="Delete" disabled=Signal::derive(|| true) />
            </Menu>
            <span class="gallery-readout">
                {move || format!("wireframe = {}, shadows = {}", wireframe.get(), shadows.get())}
            </span>
            <Snippet code="<Menu label=\"File\"><MenuItem label=\"Wireframe\" checked=on on_select=cb /><Submenu label=\"Export\">...</Submenu></Menu>" />
        </Demo>
    }
}

#[component]
fn ContextMenuDemo() -> impl IntoView {
    let open = RwSignal::new(false);
    let x = RwSignal::new(0);
    let y = RwSignal::new(0);
    let toaster = use_toaster();
    let on_context = move |event: MouseEvent| {
        event.prevent_default();
        x.set(event.client_x());
        y.set(event.client_y());
        open.set(true);
    };
    view! {
        <Demo title="ContextMenu" blurb="A menu positioned at a point. Right-click the area below to open it where you clicked.">
            <div class="gallery-box" style="height:120px; display:flex; align-items:center; justify-content:center;" on:contextmenu=on_context>
                "Right-click anywhere in this box"
            </div>
            <ContextMenu open=open x=x y=y>
                <MenuItem label="Cut" on_select=Callback::new(move |_| toaster.info("Cut")) />
                <MenuItem label="Copy" on_select=Callback::new(move |_| toaster.info("Copy")) />
                <MenuItem label="Paste" on_select=Callback::new(move |_| toaster.info("Paste")) />
            </ContextMenu>
            <Snippet code="<ContextMenu open=open x=x y=y>{items}</ContextMenu>" />
        </Demo>
    }
}

#[component]
fn TabsDemo() -> impl IntoView {
    let active = RwSignal::new("script".to_string());
    view! {
        <Demo title="TabBar" blurb="A row of tabs bound to a signal, with tablist and tab roles for assistive tech.">
            <TabBar
                tabs=vec![
                    ("script".into(), "Script".into()),
                    ("log".into(), "Log".into()),
                    ("stats".into(), "Stats".into()),
                ]
                active=active
            />
            <div class="gallery-box">
                {move || match active.get().as_str() {
                    "log" => "Log panel content",
                    "stats" => "Stats panel content",
                    _ => "Script panel content",
                }}
            </div>
            <Snippet code="<TabBar tabs=tabs active=active />" />
        </Demo>
    }
}

#[component]
fn TableDemo() -> impl IntoView {
    let rows = RwSignal::new(vec![
        vec!["Cube".to_string(), "12".to_string(), "visible".to_string()],
        vec!["Sphere".to_string(), "3".to_string(), "hidden".to_string()],
        vec![
            "Cylinder".to_string(),
            "27".to_string(),
            "visible".to_string(),
        ],
        vec!["Cone".to_string(), "8".to_string(), "visible".to_string()],
    ]);
    let selected = RwSignal::new(None::<usize>);
    view! {
        <Demo title="Table" blurb="A sortable, filterable, resizable table. Click a header to sort (shift-click to add a secondary key); drag a header edge to resize a column; type in the filter to narrow rows. Numeric columns sort by value and selection survives re-sorting.">
            <Table
                headers=vec!["Name".into(), "Count".into(), "State".into()]
                rows=Signal::derive(move || rows.get())
                sortable=true
                filterable=true
                resizable=true
                on_row_click=Callback::new(move |index: usize| selected.set(Some(index)))
                selected_row=Signal::derive(move || selected.get())
            />
            <span class="gallery-readout">
                {move || match selected.get() {
                    Some(index) => format!("selected row index = {index}"),
                    None => "no row selected".to_string(),
                }}
            </span>
            <h2>"Virtualized: 5,000 rows"</h2>
            <p class="gallery-blurb">
                "With virtualized=true and a fixed height, only the rows in view are rendered, so a table of thousands of rows scrolls smoothly."
            </p>
            <VirtualTableDemo />
            <Snippet code="<Table headers=h rows=r sortable=true filterable=true resizable=true virtualized=true height=360.0 />" />
        </Demo>
    }
}

#[component]
fn VirtualTableDemo() -> impl IntoView {
    let rows = RwSignal::new(
        (0..5000)
            .map(|index| {
                vec![
                    format!("entity-{index:04}"),
                    format!("{}", (index * 37) % 900),
                    if index % 3 == 0 { "visible" } else { "hidden" }.to_string(),
                ]
            })
            .collect::<Vec<_>>(),
    );
    view! {
        <Table
            headers=vec!["Name".into(), "Count".into(), "State".into()]
            rows=Signal::derive(move || rows.get())
            sortable=true
            filterable=true
            virtualized=true
            height=300.0
        />
    }
}

#[component]
fn TreeDemo() -> impl IntoView {
    let labels = RwSignal::new(std::collections::HashMap::<String, String>::new());
    let selection = RwSignal::new(HashSet::<String>::new());
    let moves = RwSignal::new(String::new());
    let label_for = move |id: &str, fallback: &str| {
        labels
            .with(|map| map.get(id).cloned())
            .unwrap_or_else(|| fallback.to_string())
    };
    let items = move || {
        vec![TreeItem::branch(
            "scene",
            label_for("scene", "Scene"),
            vec![
                TreeItem::leaf("camera", label_for("camera", "Camera")).with_icon("\u{1f3a5}"),
                TreeItem::branch(
                    "lights",
                    label_for("lights", "Lights"),
                    vec![
                        TreeItem::leaf("sun", label_for("sun", "Sun")).with_icon("\u{2600}"),
                        TreeItem::leaf("fill", label_for("fill", "Fill")).with_icon("\u{1f4a1}"),
                    ],
                ),
                TreeItem::branch(
                    "meshes",
                    label_for("meshes", "Meshes"),
                    vec![
                        TreeItem::leaf("cube", label_for("cube", "Cube")).with_icon("\u{1f4e6}"),
                        TreeItem::leaf("sphere", label_for("sphere", "Sphere"))
                            .with_icon("\u{1f7e0}"),
                    ],
                ),
            ],
        )]
    };
    view! {
        <Demo title="Tree" blurb="A collapsible hierarchy with multi-select (Ctrl-click), inline rename (double-click or F2), and drag-and-drop. Click chevrons to expand, or focus a row and use the arrow keys.">
            <Panel>
                {move || {
                    view! {
                        <Tree
                            items=items()
                            selection=selection
                            on_select=Callback::new(|_| {})
                            on_rename=Callback::new(move |(id, label): (String, String)| {
                                labels.update(|map| { map.insert(id, label); });
                            })
                            on_move=Callback::new(move |(source, target): (String, String)| {
                                moves.set(format!("moved {source} onto {target}"))
                            })
                            default_expanded=true
                        />
                    }
                }}
            </Panel>
            <span class="gallery-readout">
                {move || {
                    let mut ids = selection.get().into_iter().collect::<Vec<_>>();
                    ids.sort();
                    if ids.is_empty() {
                        "nothing selected".to_string()
                    } else {
                        format!("selected = [{}]", ids.join(", "))
                    }
                }}
            </span>
            <span class="gallery-readout">
                {move || {
                    let last = moves.get();
                    if last.is_empty() { "drag a row onto another".to_string() } else { last }
                }}
            </span>
            <Snippet code="<Tree items=items selection=set on_rename=cb on_move=cb default_expanded=true />" />
        </Demo>
    }
}

#[component]
fn InspectorDemo() -> impl IntoView {
    let visible = RwSignal::new(true);
    view! {
        <Demo title="Inspector" blurb="A property grid of collapsible sections and label/control rows. Drop any control into a row; here we mix plain inputs with a Switch.">
            <Inspector>
                <InspectorSection title="Transform">
                    <InspectorRow label="Position X">
                        <input class="gallery-inline-input" value="0.0" />
                    </InspectorRow>
                    <InspectorRow label="Position Y">
                        <input class="gallery-inline-input" value="1.5" />
                    </InspectorRow>
                    <InspectorRow label="Position Z">
                        <input class="gallery-inline-input" value="0.0" />
                    </InspectorRow>
                </InspectorSection>
                <InspectorSection title="Render">
                    <InspectorRow label="Visible">
                        <Switch
                            label=""
                            value=Signal::derive(move || visible.get())
                            on_change=Callback::new(move |next: bool| visible.set(next))
                        />
                    </InspectorRow>
                </InspectorSection>
            </Inspector>
            <Snippet code="<Inspector><InspectorSection title=\"Transform\"><InspectorRow label=\"X\">{control}</InspectorRow></InspectorSection></Inspector>" />
        </Demo>
    }
}

const SAMPLE_SCRIPT: &str = r#"// rhai scene script
fn build(commands) {
    set_background("nebula");
    let count = 8;
    for index in 0..count {
        let cube = commands.spawn_cube();
        commands.set_color(cube, hsv(index * 45.0, 0.7, 1.0));
        commands.rotate(cube, 0.2);
    }
}
"#;

#[component]
fn CodeEditorDemo() -> impl IntoView {
    let code = RwSignal::new(SAMPLE_SCRIPT.to_string());
    view! {
        <Demo title="CodeEditor" blurb="A lightweight editor: a textarea over a synced, tokenized highlight layer. Pass a highlighter (here the bundled rhai one) and fill to grow with its container.">
            <div class="gallery-editor">
                <CodeEditor value=code highlighter=highlight_rhai fill=true />
            </div>
            <Snippet code="<CodeEditor value=code highlighter=highlight_rhai fill=true />" />
        </Demo>
    }
}

#[component]
fn EngineDemo() -> impl IntoView {
    view! {
        <Demo title="Engine integration" blurb="musaic's core never links a renderer. Two optional layers sit on top for apps that drive a worker-backed WebGPU surface.">
            <Card title="viewport">
                "The viewport feature gives a generic render surface: it owns the OffscreenCanvas handoff and all pointer, touch, and wheel bookkeeping, emitting ViewportEvents you map to your own protocol."
            </Card>
            <Card title="engine">
                "The engine feature goes further: use_engine(\"worker.js\") returns a ready EngineState plus a bridge, with input, keyboard, and lifecycle wiring done. A live, worker-backed example ships in examples/nightshade_demo."
            </Card>
            <Snippet code="let engine = use_engine(\"runtime/worker.js\");\n<EngineViewport engine=engine />" />
        </Demo>
    }
}

#[component]
fn ThemingDemo() -> impl IntoView {
    let theme = use_theme();
    let register_ember = Callback::new(move |_| {
        register_theme(Theme {
            id: "ember".into(),
            label: "Ember".into(),
            light: false,
            bg: "#140f0c".into(),
            panel: "#1e1613".into(),
            panel_2: "#161010".into(),
            panel_border: "#3a2a20".into(),
            text: "rgba(255, 240, 230, 0.9)".into(),
            text_dim: "rgba(255, 240, 230, 0.5)".into(),
            accent: "#ff7a45".into(),
            input_bg: "rgba(255, 255, 255, 0.05)".into(),
            danger: "#ff5d5d".into(),
            keyword: "#ffab70".into(),
            string: "#f2c94c".into(),
            number: "#ff7a45".into(),
            comment: "#7a5c4c".into(),
            command: "#ffd0a0".into(),
        });
        theme.set("ember".into());
    });
    view! {
        <Demo title="Theming" blurb="Every component reads a small set of semantic CSS custom properties. A theme is a typed Rust struct that emits those tokens; ThemeProvider generates and injects the CSS, and register_theme adds custom themes at runtime with no per-component work. Pick a theme, or add one in code, and watch this whole page restyle.">
            <div class="gallery-row">
                <span class="gallery-readout">"Theme:"</span>
                <ThemePicker />
                <Button on_click=register_ember>"Register + apply 'Ember' theme"</Button>
            </div>
            <h2>"Core tokens"</h2>
            <div class="gallery-swatches">
                <Swatch name="--musaic-accent" var="--musaic-accent" />
                <Swatch name="--musaic-bg" var="--musaic-bg" />
                <Swatch name="--musaic-panel" var="--musaic-panel" />
                <Swatch name="--musaic-panel-2" var="--musaic-panel-2" />
                <Swatch name="--musaic-panel-border" var="--musaic-panel-border" />
                <Swatch name="--musaic-text" var="--musaic-text" />
                <Swatch name="--musaic-danger" var="--musaic-danger" />
                <Swatch name="--musaic-input-bg" var="--musaic-input-bg" />
            </div>
            <Snippet code="<ThemeProvider>\n    <MusaicStyles />\n    {app}\n</ThemeProvider>" />
            <Snippet code="register_theme(Theme { id: \"ember\".into(), accent: \"#ff7a45\".into(), ..base });" />
        </Demo>
    }
}

#[component]
fn Swatch(#[prop(into)] name: String, #[prop(into)] var: String) -> impl IntoView {
    view! {
        <div class="gallery-swatch">
            <div class="gallery-chip" style=format!("background:var({var})")></div>
            {name}
        </div>
    }
}

#[component]
fn CommandPaletteDemo() -> impl IntoView {
    let palette_open = expect_context::<GalleryCtx>().palette_open;
    view! {
        <Demo title="Command palette" blurb="A fuzzy command launcher driven by the CommandRegistry. It ranks by match quality, shows recently-used commands first, displays keybinding hints, and descends into nested submenus (try 'Switch theme'). Press Ctrl+K, or use the button, then type to filter.">
            <Button class="primary" on_click=Callback::new(move |_| palette_open.set(true))>
                "Open command palette"
            </Button>
            <span class="gallery-readout">"or press Ctrl+K, then try typing 'theme'"</span>
            <Snippet code="let open = RwSignal::new(false);\nprovide_command_registry().register(Command::new(\"save\", \"Save\", cb).with_keybinding(\"mod+s\"));\n<CommandPalette open=open />" />
        </Demo>
    }
}

#[component]
fn KeybindingsDemo() -> impl IntoView {
    let registry = use_commands();
    view! {
        <Demo title="Commands & keys" blurb="Every action lives in one CommandRegistry. The command palette, the global keymap, and menus all read from it, so registering a command once wires up all three surfaces. KeymapProvider installs a single listener that parses bindings like Mod+K and chords like 'g d'.">
            <Panel title="Registered commands">
                {move || {
                    registry
                        .commands()
                        .into_iter()
                        .map(|command| {
                            let binding = command
                                .keybinding
                                .as_deref()
                                .map(pretty_binding)
                                .unwrap_or_default();
                            let group = command.group.clone();
                            view! {
                                <div class="gallery-keyrow">
                                    <span>{command.title}</span>
                                    <span class="gallery-keyrow-meta">
                                        {(!group.is_empty())
                                            .then(|| {
                                                view! { <span class="gallery-readout">{group}</span> }
                                            })}
                                        {(!binding.is_empty())
                                            .then(|| {
                                                view! { <kbd class="musaic-palette-kbd">{binding}</kbd> }
                                            })}
                                    </span>
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </Panel>
            <span class="gallery-readout">
                "Ctrl+K opens the palette and Ctrl+B toggles the sidebar, both registered commands rather than bespoke listeners."
            </span>
            <Snippet code="registry.register(Command::new(\"save\", \"Save scene\", cb).with_keybinding(\"mod+s\").with_group(\"File\"));" />
        </Demo>
    }
}
