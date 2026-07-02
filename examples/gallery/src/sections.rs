use std::collections::HashSet;

use leptos::prelude::*;
use leptos_musaic::{
    Accordion, AccordionItem, AssetGrid, AssetItem, Badge, Button, Card, Chat, ChatMessage,
    ChatRole, CheckField, ChipGroup, CodeDocument, CodeEditor, CodeTabs, ColorField, ComboOption,
    Combobox, ContextMenu, Dialog, Disclosure, DockLayout, DockMain, DockPanel, DockSide, Dropdown,
    DynamicForm, FieldSchema, FormField, IconButton, Inspector, InspectorRow, InspectorSection,
    ListItem, LogEntry, LogKind, LogView, Markdown, Menu, MenuBar, MenuBarMenu, MenuItem,
    MenuSeparator, Modal, NavGizmo, NumberField, OrderedList, Panel, Popover, Progress, ResizeAxis,
    ResizeHandle, SearchItem, SearchList, Select, Side, SliderField, Spinner, SplitAxis, StatusBar,
    StatusItem, StatusSpacer, Submenu, SwatchPalette, Switch, TabBar, Table, TagInput, TextField,
    Theme, ThemeMenu, ThemePicker, ToggleChip, ToolButton, Toolbar, ToolbarGroup, ToolbarSpacer,
    Tooltip, Tree, TreeItem, Vec3Field, ViewportOverlay, VirtualList, highlight_rhai,
    pretty_binding, register_theme, use_commands, use_theme, use_toaster,
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
            Page {
                id: "popover",
                title: "Popover & Dropdown",
            },
            Page {
                id: "combobox",
                title: "Combobox",
            },
            Page {
                id: "dialog",
                title: "Dialog",
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
        id: "editor-kit",
        title: "Editor kit",
        icon: "\u{1f6e0}",
        pages: &[
            Page {
                id: "toolbar",
                title: "Toolbar & MenuBar",
            },
            Page {
                id: "status-bar",
                title: "StatusBar",
            },
            Page {
                id: "disclosure",
                title: "Disclosure",
            },
            Page {
                id: "log",
                title: "LogView",
            },
            Page {
                id: "search-list",
                title: "SearchList",
            },
            Page {
                id: "asset-grid",
                title: "AssetGrid",
            },
            Page {
                id: "list-editor",
                title: "OrderedList",
            },
            Page {
                id: "chips",
                title: "Chips, tags, swatches",
            },
            Page {
                id: "dynamic-form",
                title: "DynamicForm",
            },
            Page {
                id: "chat",
                title: "Chat",
            },
            Page {
                id: "markdown",
                title: "Markdown",
            },
            Page {
                id: "nav-gizmo",
                title: "NavGizmo",
            },
            Page {
                id: "virtual-list",
                title: "VirtualList",
            },
        ],
    },
    Category {
        id: "editor",
        title: "Editor",
        icon: "\u{1f4dd}",
        pages: &[
            Page {
                id: "code-editor",
                title: "CodeEditor",
            },
            Page {
                id: "code-tabs",
                title: "CodeTabs",
            },
        ],
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
        "popover" => view! { <PopoverDemo /> }.into_any(),
        "combobox" => view! { <ComboboxDemo /> }.into_any(),
        "dialog" => view! { <DialogDemo /> }.into_any(),
        "table" => view! { <TableDemo /> }.into_any(),
        "tree" => view! { <TreeDemo /> }.into_any(),
        "inspector" => view! { <InspectorDemo /> }.into_any(),
        "code-editor" => view! { <CodeEditorDemo /> }.into_any(),
        "code-tabs" => view! { <CodeTabsDemo /> }.into_any(),
        "toolbar" => view! { <ToolbarDemo /> }.into_any(),
        "status-bar" => view! { <StatusBarDemo /> }.into_any(),
        "disclosure" => view! { <DisclosureDemo /> }.into_any(),
        "log" => view! { <LogDemo /> }.into_any(),
        "search-list" => view! { <SearchListDemo /> }.into_any(),
        "asset-grid" => view! { <AssetGridDemo /> }.into_any(),
        "list-editor" => view! { <ListEditorDemo /> }.into_any(),
        "chips" => view! { <ChipsDemo /> }.into_any(),
        "dynamic-form" => view! { <DynamicFormDemo /> }.into_any(),
        "chat" => view! { <ChatDemo /> }.into_any(),
        "markdown" => view! { <MarkdownDemo /> }.into_any(),
        "nav-gizmo" => view! { <NavGizmoDemo /> }.into_any(),
        "virtual-list" => view! { <VirtualListDemo /> }.into_any(),
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
        <Demo title="Toasts" blurb="Transient notifications with info, success, warning, and error kinds, a dismiss button, and optional inline actions. Mount ToastHub once at the root, then call use_toaster() anywhere.">
            <div class="gallery-row">
                <Button on_click=Callback::new(move |_| toaster.info("Saved to disk"))>
                    "Info"
                </Button>
                <Button on_click=Callback::new(move |_| toaster.success("Export complete"))>
                    "Success"
                </Button>
                <Button on_click=Callback::new(move |_| toaster.warning("Low memory"))>
                    "Warning"
                </Button>
                <Button
                    class="danger"
                    on_click=Callback::new(move |_| toaster.error("Export failed"))
                >
                    "Error"
                </Button>
                <Button on_click=Callback::new(move |_| {
                    toaster
                        .action(
                            "Deleted 3 entities",
                            "Undo",
                            Callback::new(move |_| toaster.success("Restored")),
                        )
                })>"With action"</Button>
            </div>
            <Snippet code="toaster.success(\"Saved\");\ntoaster.action(\"Deleted\", \"Undo\", on_undo);" />
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
                columns_toggle=true
                on_row_click=Callback::new(move |index: usize| selected.set(Some(index)))
                on_cell_edit=Callback::new(move |(row, col, value): (usize, usize, String)| {
                    rows.update(|data| {
                        if let Some(cell) = data.get_mut(row).and_then(|row| row.get_mut(col)) {
                            *cell = value;
                        }
                    })
                })
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
    let toaster = use_toaster();
    view! {
        <Demo title="Inspector" blurb="A property grid of collapsible sections and label/control rows. Each section header has an actions slot, so component-style sections can carry a remove button, the shape a real entity inspector needs.">
            <Inspector>
                <InspectorSection
                    title="Transform"
                    actions=move || {
                        view! {
                            <IconButton on_click=Callback::new(move |_| {
                                toaster.info("Remove component")
                            })>"\u{00d7}"</IconButton>
                        }
                    }
                >
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
        <Demo title="CodeEditor" blurb="A textarea over a synced, tokenized highlight layer, with an optional line-number gutter, diagnostic markers, and a find/replace bar (press Ctrl+F). Pass a highlighter and fill to grow with its container.">
            <div class="gallery-editor">
                <CodeEditor
                    value=code
                    highlighter=highlight_rhai
                    fill=true
                    gutter=true
                    find=true
                    diagnostics=Signal::derive(|| vec![2usize])
                />
            </div>
            <span class="gallery-readout">"Press Ctrl+F to find and replace."</span>
            <Snippet code="<CodeEditor value=code highlighter=highlight_rhai gutter=true find=true diagnostics=errors fill=true />" />
        </Demo>
    }
}

#[component]
fn CodeTabsDemo() -> impl IntoView {
    let docs = RwSignal::new(vec![
        CodeDocument::new(
            "build",
            "build.rhai",
            RwSignal::new(SAMPLE_SCRIPT.to_string()),
        ),
        CodeDocument::new(
            "spin",
            "spin.rhai",
            RwSignal::new("fn tick(dt) {\n    rotate(cube, dt * 0.5);\n}\n".to_string()),
        ),
    ]);
    let active = RwSignal::new("build".to_string());
    view! {
        <Demo title="CodeTabs" blurb="Multiple open documents over one editor: a tab bar plus the active document's CodeEditor, each tab carrying its own reactive buffer. Close tabs with the x.">
            <div class="gallery-editor">
                <CodeTabs
                    documents=Signal::derive(move || docs.get())
                    active=active
                    highlighter=highlight_rhai
                    find=true
                    on_close=Callback::new(move |id: String| {
                        docs.update(|list| list.retain(|document| document.id != id));
                    })
                />
            </div>
            <Snippet code="<CodeTabs documents=docs active=active highlighter=highlight_rhai on_close=cb find=true />" />
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
fn ToolbarDemo() -> impl IntoView {
    let toaster = use_toaster();
    let snap = RwSignal::new(true);
    let wireframe = RwSignal::new(false);
    view! {
        <Demo title="Toolbar & MenuBar" blurb="Toolbar groups tool buttons with a spacer; ToolButton takes a reactive active flag. MenuBar coordinates a set of dropdowns so hovering a sibling switches the open menu, the standard desktop menu-bar behavior.">
            <Toolbar>
                <MenuBar>
                    <MenuBarMenu id="file" label="File">
                        <MenuItem label="New" shortcut="Ctrl+N" on_select=Callback::new(move |_| toaster.info("New")) />
                        <MenuItem label="Open" shortcut="Ctrl+O" on_select=Callback::new(move |_| toaster.info("Open")) />
                    </MenuBarMenu>
                    <MenuBarMenu id="edit" label="Edit">
                        <MenuItem label="Undo" shortcut="Ctrl+Z" on_select=Callback::new(move |_| toaster.info("Undo")) />
                        <MenuItem label="Redo" shortcut="Ctrl+Y" on_select=Callback::new(move |_| toaster.info("Redo")) />
                    </MenuBarMenu>
                </MenuBar>
                <ToolbarGroup>
                    <ToolButton
                        active=Signal::derive(move || snap.get())
                        title="Snap"
                        on_click=Callback::new(move |_| snap.update(|value| *value = !*value))
                    >
                        "\u{2317} Snap"
                    </ToolButton>
                    <ToolButton
                        active=Signal::derive(move || wireframe.get())
                        title="Wireframe"
                        on_click=Callback::new(move |_| wireframe.update(|value| *value = !*value))
                    >
                        "\u{25a6} Wireframe"
                    </ToolButton>
                </ToolbarGroup>
                <ToolbarSpacer />
                <ToolbarGroup>
                    <ToolButton on_click=Callback::new(move |_| toaster.info("Play"))>"\u{25b6}"</ToolButton>
                </ToolbarGroup>
            </Toolbar>
            <span class="gallery-readout">
                {move || format!("snap = {}, wireframe = {}", snap.get(), wireframe.get())}
            </span>
            <Snippet code="<Toolbar><MenuBar><MenuBarMenu id=\"file\" label=\"File\">...</MenuBarMenu></MenuBar><ToolButton active=on>...</ToolButton></Toolbar>" />
        </Demo>
    }
}

#[component]
fn StatusBarDemo() -> impl IntoView {
    let fps = RwSignal::new(60);
    let dirty = RwSignal::new(true);
    view! {
        <Demo title="StatusBar" blurb="A bottom chrome bar: labeled StatusItems separated by a StatusSpacer that pushes trailing items to the right.">
            <StatusBar>
                <StatusItem icon="\u{25b8}">{move || format!("{} fps", fps.get())}</StatusItem>
                <StatusItem>"12 entities"</StatusItem>
                <StatusItem>"perspective"</StatusItem>
                <StatusSpacer />
                <StatusItem>{move || if dirty.get() { "untitled *" } else { "untitled" }}</StatusItem>
                <StatusItem icon="\u{25cf}">"webgpu"</StatusItem>
            </StatusBar>
            <div class="gallery-row">
                <Button on_click=Callback::new(move |_| fps.update(|value| *value = (*value + 5) % 145))>
                    "Bump fps"
                </Button>
                <Button on_click=Callback::new(move |_| dirty.update(|value| *value = !*value))>
                    "Toggle dirty"
                </Button>
            </div>
            <Snippet code="<StatusBar><StatusItem icon=\"...\">{fps}</StatusItem><StatusSpacer /><StatusItem>{name}</StatusItem></StatusBar>" />
        </Demo>
    }
}

#[component]
fn DisclosureDemo() -> impl IntoView {
    view! {
        <Demo title="Disclosure & Accordion" blurb="Disclosure is a standalone collapsible section. Accordion coordinates a set of items so only one is open at a time.">
            <Panel>
                <Disclosure title="Standalone section" default_open=true>
                    <span class="gallery-readout">"Body content that collapses independently."</span>
                </Disclosure>
            </Panel>
            <Accordion default_open="a">
                <AccordionItem id="a" title="Transform">
                    <span class="gallery-readout">"Position, rotation, scale."</span>
                </AccordionItem>
                <AccordionItem id="b" title="Material">
                    <span class="gallery-readout">"Albedo, metallic, roughness."</span>
                </AccordionItem>
                <AccordionItem id="c" title="Physics">
                    <span class="gallery-readout">"Collider, mass, restitution."</span>
                </AccordionItem>
            </Accordion>
            <Snippet code="<Accordion default_open=\"a\"><AccordionItem id=\"a\" title=\"Transform\">...</AccordionItem></Accordion>" />
        </Demo>
    }
}

#[component]
fn LogDemo() -> impl IntoView {
    let entries = RwSignal::new(vec![
        LogEntry::new(0, LogKind::Info, "engine ready").with_detail("adapter: webgpu"),
        LogEntry::new(1, LogKind::Command, "spawn_cube"),
        LogEntry::new(2, LogKind::Event, "selected").with_detail("entity 7"),
        LogEntry::new(3, LogKind::Warn, "texture missing").with_count(3),
        LogEntry::new(4, LogKind::Error, "shader compile failed"),
    ]);
    let next = RwSignal::new(5usize);
    let readout = RwSignal::new(String::new());
    view! {
        <Demo title="LogView" blurb="A live console: kind-colored rows with a tag, an optional detail, and an xN dedup badge. It auto-scrolls to the newest row, rows are click-selectable, and Clear empties it.">
            <div style="height:220px;">
                <LogView
                    entries=Signal::derive(move || entries.get())
                    on_select=Callback::new(move |id: usize| readout.set(format!("selected entry {id}")))
                    on_clear=Callback::new(move |_| entries.set(Vec::new()))
                />
            </div>
            <div class="gallery-row">
                <Button on_click=Callback::new(move |_| {
                    let id = next.get_untracked();
                    next.set(id + 1);
                    entries.update(|list| list.push(LogEntry::new(id, LogKind::Command, format!("command #{id}"))));
                })>
                    "Append entry"
                </Button>
                <span class="gallery-readout">{move || readout.get()}</span>
            </div>
            <Snippet code="<LogView entries=entries on_select=cb on_clear=cb />" />
        </Demo>
    }
}

#[component]
fn SearchListDemo() -> impl IntoView {
    let items = vec![
        SearchItem::new("spawn_cube", "spawn_cube")
            .with_subtitle("commands")
            .with_detail("spawn_cube() -> entity\nSpawns a unit cube at the origin."),
        SearchItem::new("set_color", "set_color")
            .with_subtitle("commands")
            .with_detail("set_color(entity, rgb)\nSets the base color of an entity."),
        SearchItem::new("point_light", "point_light")
            .with_subtitle("lights")
            .with_detail("point_light(pos, color, intensity) -> entity"),
        SearchItem::new("set_background", "set_background")
            .with_subtitle("world")
            .with_detail("set_background(name)\nSwitches the environment."),
    ];
    view! {
        <Demo title="SearchList" blurb="A filterable list with expandable detail rows, the shape editors use for a searchable API reference. Selecting a row reveals its detail and scrolls it into view.">
            <div style="height:260px;">
                <SearchList items=items placeholder="Search commands…" />
            </div>
            <Snippet code="<SearchList items=items placeholder=\"Search…\" on_select=cb />" />
        </Demo>
    }
}

fn swatch_uri(color: &str) -> String {
    let svg = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='96' height='96'><rect width='96' height='96' fill='{color}'/></svg>"
    );
    format!(
        "data:image/svg+xml,{}",
        svg.replace('#', "%23").replace(' ', "%20")
    )
}

#[component]
fn AssetGridDemo() -> impl IntoView {
    let selected = RwSignal::new(String::new());
    let items = vec![
        ("cube", "Cube", "#fb923c"),
        ("sphere", "Sphere", "#f5c2e7"),
        ("torus", "Torus", "#7aa2f7"),
        ("cone", "Cone", "#9ece6a"),
        ("plane", "Plane", "#e0af68"),
        ("cylinder", "Cylinder", "#bb9af7"),
    ]
    .into_iter()
    .map(|(id, label, color)| {
        AssetItem::new(id, label, swatch_uri(color)).with_subtitle("primitive")
    })
    .collect::<Vec<_>>();
    view! {
        <Demo title="AssetGrid" blurb="A searchable thumbnail grid with lazy-loaded images, the pattern used for material and model browsers. Cards reflow to fill the width.">
            <div style="height:300px;">
                <AssetGrid
                    items=items
                    on_select=Callback::new(move |id: String| selected.set(id))
                />
            </div>
            <span class="gallery-readout">{move || format!("selected = {}", selected.get())}</span>
            <Snippet code="<AssetGrid items=items on_select=cb />" />
        </Demo>
    }
}

#[component]
fn ListEditorDemo() -> impl IntoView {
    let items = RwSignal::new(vec![
        ListItem::new("intro", "intro.rhai"),
        ListItem::new("spin", "spin.rhai"),
        ListItem::new("lights", "lights.rhai"),
    ]);
    let shift = move |id: String, delta: i32| {
        items.update(|list| {
            if let Some(index) = list.iter().position(|item| item.id == id) {
                let target = index as i32 + delta;
                if target >= 0 && (target as usize) < list.len() {
                    list.swap(index, target as usize);
                }
            }
        });
    };
    view! {
        <Demo title="OrderedList" blurb="A reorderable list with per-row actions, the shape used for script stacks and layer lists. Move rows up or down or remove them.">
            <Panel>
                <OrderedList
                    items=Signal::derive(move || items.get())
                    on_move_up=Callback::new(move |id: String| shift(id, -1))
                    on_move_down=Callback::new(move |id: String| shift(id, 1))
                    on_remove=Callback::new(move |id: String| {
                        items.update(|list| list.retain(|item| item.id != id))
                    })
                />
            </Panel>
            <Snippet code="<OrderedList items=items on_move_up=cb on_move_down=cb on_remove=cb />" />
        </Demo>
    }
}

#[component]
fn ChipsDemo() -> impl IntoView {
    let shape = RwSignal::new("cube".to_string());
    let tags = RwSignal::new(vec!["hero".to_string(), "static".to_string()]);
    let color = RwSignal::new("#fb923c".to_string());
    let shapes = ["cube", "sphere", "cone", "torus"];
    view! {
        <Demo title="Chips, tags & swatches" blurb="Small selection primitives: a ChipGroup of ToggleChips, a removable TagInput, and a SwatchPalette of colors.">
            <Panel title="Shape">
                <ChipGroup>
                    {shapes
                        .into_iter()
                        .map(|name| {
                            view! {
                                <ToggleChip
                                    label=name
                                    active=Signal::derive(move || shape.get() == name)
                                    on_toggle=Callback::new(move |_| shape.set(name.to_string()))
                                />
                            }
                        })
                        .collect_view()}
                </ChipGroup>
            </Panel>
            <Panel title="Tags">
                <TagInput
                    tags=Signal::derive(move || tags.get())
                    on_add=Callback::new(move |tag: String| tags.update(|list| list.push(tag)))
                    on_remove=Callback::new(move |tag: String| {
                        tags.update(|list| list.retain(|existing| existing != &tag))
                    })
                />
            </Panel>
            <Panel title="Color">
                <SwatchPalette
                    colors=vec![
                        "#fb923c".into(),
                        "#f5c2e7".into(),
                        "#7aa2f7".into(),
                        "#9ece6a".into(),
                        "#f7768e".into(),
                    ]
                    selected=Signal::derive(move || color.get())
                    on_select=Callback::new(move |value: String| color.set(value))
                />
            </Panel>
            <span class="gallery-readout">
                {move || format!("shape={}, color={}, tags={:?}", shape.get(), color.get(), tags.get())}
            </span>
            <Snippet code="<ChipGroup><ToggleChip label=\"cube\" active=on on_toggle=cb /></ChipGroup>" />
        </Demo>
    }
}

#[component]
fn DynamicFormDemo() -> impl IntoView {
    let readout = RwSignal::new(String::new());
    let fields = vec![
        FormField::new("name", "Name", FieldSchema::Text),
        FormField::new(
            "count",
            "Count",
            FieldSchema::Number {
                min: Some(0.0),
                max: Some(100.0),
                integer: true,
            },
        ),
        FormField::new("visible", "Visible", FieldSchema::Bool),
        FormField::new(
            "kind",
            "Kind",
            FieldSchema::Enum(vec!["cube".into(), "sphere".into(), "light".into()]),
        ),
        FormField::new("position", "Position", FieldSchema::Vector(3)),
    ];
    view! {
        <Demo title="DynamicForm" blurb="A form generated from a schema. Each field is described by a FieldSchema, and the form emits a JSON object as it changes, the exact pattern editors use to build command arguments from a manifest.">
            <Panel>
                <DynamicForm
                    fields=fields
                    on_change=Callback::new(move |value: serde_json::Value| {
                        readout.set(value.to_string())
                    })
                />
            </Panel>
            <pre class="gallery-code">{move || readout.get()}</pre>
            <Snippet code="<DynamicForm fields=vec![FormField::new(\"count\", \"Count\", FieldSchema::Number{..})] on_change=cb />" />
        </Demo>
    }
}

#[component]
fn ChatDemo() -> impl IntoView {
    let messages = RwSignal::new(vec![
        ChatMessage::new(0, ChatRole::Info, "Connected to a local echo agent."),
        ChatMessage::new(1, ChatRole::Assistant, "Ask me to spawn something."),
    ]);
    let next = RwSignal::new(2usize);
    let on_send = Callback::new(move |text: String| {
        let id = next.get_untracked();
        next.set(id + 2);
        messages.update(|list| {
            list.push(ChatMessage::new(id, ChatRole::User, text.clone()));
            list.push(ChatMessage::new(
                id + 1,
                ChatRole::Assistant,
                format!("echo: {text}"),
            ));
        });
    });
    view! {
        <Demo title="Chat" blurb="A role-styled message list with an auto-scrolling body, a connection indicator, and a compose box (Enter sends, Shift+Enter for a newline). The transport is yours; this demo echoes locally.">
            <div style="height:340px;">
                <Chat
                    messages=Signal::derive(move || messages.get())
                    on_send=on_send
                    connected=Signal::derive(|| true)
                />
            </div>
            <Snippet code="<Chat messages=messages on_send=cb connected=is_connected busy=is_busy />" />
        </Demo>
    }
}

const MARKDOWN_SAMPLE: &str = r#"# Markdown

musaic ships a small **Markdown** renderer for docs and help panels.

## Features

- Headings, paragraphs, and `inline code`
- **Bold**, *italic*, and [links](https://leptos.dev)
- Ordered and unordered lists
- Fenced code blocks

```
fn build(commands) {
    commands.spawn_cube();
}
```

> Blockquotes render too, themed from the same tokens.
"#;

#[component]
fn MarkdownDemo() -> impl IntoView {
    let source = RwSignal::new(MARKDOWN_SAMPLE.to_string());
    view! {
        <Demo title="Markdown" blurb="A dependency-free Markdown renderer for docs, help, and agent output: headings, emphasis, inline and fenced code, lists, links, and blockquotes, all themed. Edit the source and watch it render live.">
            <div class="gallery-row" style="align-items:stretch; gap:16px;">
                <div style="flex:1; min-width:0;">
                    <CodeEditor value=source height="360px" />
                </div>
                <div style="flex:1; min-width:0; overflow:auto; max-height:360px;">
                    <Markdown source=Signal::derive(move || source.get()) />
                </div>
            </div>
            <Snippet code="<Markdown source=Signal::derive(move || doc.get()) />" />
        </Demo>
    }
}

#[component]
fn NavGizmoDemo() -> impl IntoView {
    let yaw = RwSignal::new(0.6);
    let readout = RwSignal::new(String::new());
    let basis = Signal::derive(move || {
        let angle = yaw.get() as f32;
        let (sin, cos) = (angle.sin(), angle.cos());
        [[cos, 0.0, -sin], [0.0, 1.0, 0.0], [sin, 0.0, cos]]
    });
    view! {
        <Demo title="NavGizmo & ViewportOverlay" blurb="ViewportOverlay is a HUD layer that passes pointer input through to the canvas except on its own controls. NavGizmo is an orientation cube driven by a camera-basis signal; click an axis to snap. Drag the slider to orbit.">
            <div style="position:relative; height:220px; border:1px solid var(--musaic-panel-border); border-radius:9px; overflow:hidden; background:var(--musaic-panel-2);">
                <div style="display:flex; align-items:center; justify-content:center; height:100%; color:var(--musaic-text-dim);">
                    "engine surface"
                </div>
                <ViewportOverlay>
                    <div style="position:absolute; top:10px; right:10px; pointer-events:auto;">
                        <NavGizmo
                            basis=basis
                            on_axis=Callback::new(move |index: usize| {
                                readout.set(format!("clicked axis {index}"))
                            })
                        />
                    </div>
                </ViewportOverlay>
            </div>
            <SliderField
                label="Orbit"
                value=Signal::derive(move || yaw.get())
                min=Signal::derive(|| 0.0)
                max=Signal::derive(|| std::f64::consts::TAU)
                step=0.02
                on_change=Callback::new(move |(next, _): (f64, bool)| yaw.set(next))
            />
            <span class="gallery-readout">{move || readout.get()}</span>
            <Snippet code="<ViewportOverlay><NavGizmo basis=camera_basis on_axis=cb /></ViewportOverlay>" />
        </Demo>
    }
}

#[component]
fn VirtualListDemo() -> impl IntoView {
    let count = RwSignal::new(20_000usize);
    view! {
        <Demo title="VirtualList" blurb="A reusable windowed-rendering primitive: give it a count, an item height, and a render closure, and it renders only the rows in view. This list holds 20,000 rows and scrolls smoothly.">
            <VirtualList
                count=Signal::derive(move || count.get())
                item_height=30.0
                height=320.0
                render=move |index| {
                    view! {
                        <div style="padding:0 12px; display:flex; gap:12px;">
                            <span class="gallery-readout" style="width:70px;">
                                {format!("#{index:05}")}
                            </span>
                            <span>{format!("row payload {}", (index * 2654435761usize) % 9973)}</span>
                        </div>
                    }
                        .into_any()
                }
            />
            <Snippet code="<VirtualList count=n item_height=30.0 height=320.0 render=move |i| view!{ ... }.into_any() />" />
        </Demo>
    }
}

#[component]
fn PopoverDemo() -> impl IntoView {
    let toaster = use_toaster();
    let open = RwSignal::new(false);
    view! {
        <Demo title="Popover & Dropdown" blurb="Popover anchors a floating panel to a trigger and flips or shifts it to stay on screen, portalled out of any clipping ancestor. Dropdown is a Popover preset over a menu. Scroll or resize and it follows.">
            <div class="gallery-row">
                <Dropdown label="Actions">
                    <MenuItem label="Duplicate" on_select=Callback::new(move |_| toaster.info("Duplicate")) />
                    <MenuItem label="Rename" on_select=Callback::new(move |_| toaster.info("Rename")) />
                    <MenuSeparator />
                    <MenuItem label="Delete" on_select=Callback::new(move |_| toaster.error("Delete")) />
                </Dropdown>
                <Popover
                    open=open
                    side=Side::Top
                    trigger=move || view! { <Button>"Popover above"</Button> }
                >
                    <div style="padding:12px; max-width:220px;">
                        <strong>"Anchored"</strong>
                        <p class="gallery-readout" style="margin:6px 0 0;">
                            "Positioned relative to the trigger, flipped to stay visible."
                        </p>
                    </div>
                </Popover>
            </div>
            <Snippet code="<Popover open=open side=Side::Top trigger=move || view!{ <Button>\"Open\"</Button> }>{content}</Popover>" />
        </Demo>
    }
}

#[component]
fn ComboboxDemo() -> impl IntoView {
    let value = RwSignal::new("nord".to_string());
    let options = vec![
        ComboOption::new("nightshade", "Nightshade"),
        ComboOption::new("dracula", "Dracula"),
        ComboOption::new("nord", "Nord"),
        ComboOption::new("gruvbox", "Gruvbox Dark"),
        ComboOption::new("tokyo-night", "Tokyo Night"),
        ComboOption::new("catppuccin", "Catppuccin Mocha"),
    ];
    view! {
        <Demo title="Combobox" blurb="A searchable single-select: type to filter, arrow keys to move, Enter to pick. The list is a Popover, so it escapes clipping and repositions on scroll.">
            <div class="gallery-row">
                <Combobox
                    value=Signal::derive(move || value.get())
                    options=options
                    on_select=Callback::new(move |next: String| value.set(next))
                    placeholder="Pick a theme"
                />
                <span class="gallery-readout">{move || format!("value = {}", value.get())}</span>
            </div>
            <Snippet code="<Combobox value=v options=options on_select=cb />" />
        </Demo>
    }
}

#[component]
fn DialogDemo() -> impl IntoView {
    let open = RwSignal::new(false);
    let toaster = use_toaster();
    view! {
        <Demo title="Dialog" blurb="A confirm/cancel dialog built on the portalled Modal: it traps focus, closes on Escape, and reports the choice. Pass danger for destructive actions.">
            <Button class="danger" on_click=Callback::new(move |_| open.set(true))>
                "Delete scene"
            </Button>
            <Dialog
                open=open
                title="Delete scene?"
                confirm_label="Delete"
                cancel_label="Keep"
                danger=true
                on_confirm=Callback::new(move |_| toaster.error("Scene deleted"))
                on_cancel=Callback::new(move |_| toaster.info("Kept"))
            >
                "This removes every entity in the scene. This cannot be undone."
            </Dialog>
            <Snippet code="<Dialog open=open title=\"Delete?\" danger=true on_confirm=cb on_cancel=cb>{body}</Dialog>" />
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
                <ThemeMenu />
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
