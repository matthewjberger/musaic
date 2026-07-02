# Project Structure

musaic does not dictate a layout, but the apps that use it converge on the same shape. A typical
client-side Leptos app looks like this:

```text
src/
  main.rs          mounts the root component
  app.rs           the root: providers, command registry, EditorShell
  state.rs         one Copy "handle" struct of RwSignals for shared UI state
  components/      your app-specific panels, split by area
    toolbar.rs
    sidebar.rs
    ...
index.html         trunk entry; links your app CSS
public/
  app.css          your own styles (unlayered, so they win over musaic)
```

## The root component

The root does four things, in order: inject styles, provide theming, provide the command registry
(if you use commands), and lay out the shell.

```rust
#[component]
pub fn App() -> impl IntoView {
    view! {
        <MusaicStyles/>
        <ThemeProvider>
            <Stage/>
        </ThemeProvider>
    }
}

#[component]
fn Stage() -> impl IntoView {
    let state = AppState::new();
    let registry = provide_command_registry();
    // register_all([...]) here

    view! {
        <KeymapProvider>
            <EditorShell
                left_size=state.sidebar_width
                toolbar=move || view! { <Toolbar state=state/> }
                left=move || view! { <Sidebar state=state/> }
                status=move || view! { <StatusStrip state=state/> }
            >
                // center content
            </EditorShell>
            <CommandPalette open=state.palette_open/>
        </KeymapProvider>
    }
}
```

## Shared state

Put the signals a screen shares into one plain struct that derives `Clone, Copy`, and pass it by
value into each component. Every field is an `RwSignal` (or `StoredValue`), which are themselves
`Copy` handles into a reactive arena, so copying the struct is cheap and every copy points at the
same state.

```rust
#[derive(Clone, Copy)]
pub struct AppState {
    pub sidebar_width: RwSignal<f64>,
    pub palette_open: RwSignal<bool>,
    pub selection: RwSignal<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sidebar_width: RwSignal::new(296.0),
            palette_open: RwSignal::new(false),
            selection: RwSignal::new(None),
        }
    }
}
```

This is the whole state story: no context juggling for app state, no `Rc<RefCell<...>>`. The next
chapter explains why it works and how to pass reactive values into components.

The `examples/nightshade_demo` app in the repository is a complete, working instance of this
structure; read it alongside this chapter.
