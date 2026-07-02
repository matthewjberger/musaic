# Commands and Keybindings

musaic keeps actions in one place. A command registry holds your `Command`s, and the command
palette, the keybinding layer, and your menus all read from it. Add a capability once and it shows
up everywhere.

## The registry

Provide the registry near the root, then register commands. Each has an id, a title, an optional
group, an optional keybinding, and a `run` callback.

```rust
let registry = provide_command_registry();
registry.register_all([
    Command::new("spawn-cube", "Spawn cube", Callback::new(move |_| {
        state.spawn_cube();
    }))
    .with_keybinding("c")
    .with_group("Scene"),

    Command::new("toggle-sidebar", "Toggle sidebar", Callback::new(move |_| {
        left_open.update(|open| *open = !*open);
    }))
    .with_keybinding("mod+b")
    .with_group("View"),
]);
```

`mod` is Cmd on macOS and Ctrl elsewhere. Commands can also nest children (submenus) and carry an
`enabled` predicate that greys them out when unavailable.

## The palette

Bind a signal to open a `CommandPalette`. It is fuzzy-ranked, shows recents and each command's
keybinding hint, and descends into submenus.

```rust
let palette_open = RwSignal::new(false);
// give the user a way in: a command, a toolbar button, or mod+k
view! { <CommandPalette open=palette_open/> }
```

## Keybindings

Wrap the app in `<KeymapProvider>`. It installs one global key listener that parses bindings like
`Mod+K` or the two-key chord `g d`, dispatches to the registry, and skips bare keys while a text
field is focused. When a chord prefix is pending it shows a which-key hint overlay of the possible
completions.

```rust
view! {
    <KeymapProvider>
        <EditorShell ...>...</EditorShell>
        <CommandPalette open=palette_open/>
    </KeymapProvider>
}
```

That is the whole loop: register a command with a keybinding, and it is instantly runnable from the
key, from the palette, and from any menu you build off the registry. `pretty_binding` formats a
binding string for display if you want to show it elsewhere.
