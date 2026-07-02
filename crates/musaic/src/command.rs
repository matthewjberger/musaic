//! Command model and registry backing the palette, keymap, and menus.
//!
//! A [`Command`] is a leaf action or a submenu of nested commands; the
//! [`CommandRegistry`] stores them in reactive context, tracks recently run
//! ids, and dispatches by id.

use leptos::prelude::*;

/// A single palette action or a submenu of nested commands.
///
/// Build leaves with [`Command::new`] and groupings with [`Command::submenu`],
/// then chain the `with_*` builders to attach a group label, keybinding, or an
/// `enabled` signal that gates whether the command can run.
#[derive(Clone)]
pub struct Command {
    pub id: String,
    pub title: String,
    pub group: String,
    pub keybinding: Option<String>,
    pub run: Option<Callback<()>>,
    pub children: Vec<Command>,
    enabled: Signal<bool>,
}

impl Command {
    /// Creates a runnable leaf command with the given id, title, and callback.
    pub fn new(id: impl Into<String>, title: impl Into<String>, run: Callback<()>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            group: String::new(),
            keybinding: None,
            run: Some(run),
            children: Vec::new(),
            enabled: Signal::derive(|| true),
        }
    }

    /// Creates a non-runnable submenu command whose `children` open as a nested level.
    pub fn submenu(
        id: impl Into<String>,
        title: impl Into<String>,
        children: Vec<Command>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            group: String::new(),
            keybinding: None,
            run: None,
            children,
            enabled: Signal::derive(|| true),
        }
    }

    /// Sets the group label shown alongside the command in the palette.
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = group.into();
        self
    }

    /// Attaches a keybinding string (for example `"mod+k"`) used by the keymap and shown as a hint.
    pub fn with_keybinding(mut self, keybinding: impl Into<String>) -> Self {
        self.keybinding = Some(keybinding.into());
        self
    }

    /// Alias for [`Command::with_group`], reading better when the group is used as a trailing hint.
    pub fn with_hint(mut self, group: impl Into<String>) -> Self {
        self.group = group.into();
        self
    }

    /// Gates the command behind a reactive `enabled` signal; disabled commands are hidden and never run.
    pub fn with_enabled(mut self, enabled: impl Into<Signal<bool>>) -> Self {
        self.enabled = enabled.into();
        self
    }

    /// Returns `true` when this command has children and opens as a submenu rather than running.
    pub fn is_submenu(&self) -> bool {
        !self.children.is_empty()
    }

    /// Reactively reads whether the command is currently enabled.
    pub fn enabled(&self) -> bool {
        self.enabled.get()
    }

    fn enabled_untracked(&self) -> bool {
        self.enabled.get_untracked()
    }
}

const RECENT_LIMIT: usize = 8;

/// Reactive store of registered commands plus a bounded most-recently-run list.
///
/// It is `Copy` and lives in Leptos context; obtain it with [`use_commands`]
/// after calling [`provide_command_registry`] near the app root.
#[derive(Clone, Copy)]
pub struct CommandRegistry {
    commands: RwSignal<Vec<Command>>,
    recent: RwSignal<Vec<String>>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self {
            commands: RwSignal::new(Vec::new()),
            recent: RwSignal::new(Vec::new()),
        }
    }
}

impl CommandRegistry {
    /// Adds a command, replacing any existing one with the same id.
    pub fn register(&self, command: Command) {
        self.commands.update(|list| {
            if let Some(slot) = list.iter_mut().find(|existing| existing.id == command.id) {
                *slot = command;
            } else {
                list.push(command);
            }
        });
    }

    /// Registers each command in the iterator in order.
    pub fn register_all(&self, commands: impl IntoIterator<Item = Command>) {
        for command in commands {
            self.register(command);
        }
    }

    /// Removes the top-level command with the given id, if present.
    pub fn unregister(&self, id: &str) {
        self.commands
            .update(|list| list.retain(|command| command.id != id));
    }

    /// Removes every registered command.
    pub fn clear(&self) {
        self.commands.update(Vec::clear);
    }

    /// Reactively reads the registered top-level commands.
    pub fn commands(&self) -> Vec<Command> {
        self.commands.get()
    }

    /// Reads the registered top-level commands without tracking reactivity.
    pub fn commands_untracked(&self) -> Vec<Command> {
        self.commands.get_untracked()
    }

    /// Searches the command tree, descending into submenus, for a command with the given id.
    pub fn find(&self, id: &str) -> Option<Command> {
        find_command(&self.commands.get_untracked(), id)
    }

    /// Reactively reads the most-recently-run command ids, newest first.
    pub fn recent(&self) -> Vec<String> {
        self.recent.get()
    }

    /// Runs the command with the given id, recording it as recent; returns `false` if it is missing, has no callback, or is disabled.
    pub fn run(&self, id: &str) -> bool {
        let Some(command) = self.find(id) else {
            return false;
        };
        let Some(callback) = command.run else {
            return false;
        };
        if !command.enabled_untracked() {
            return false;
        }
        self.record(id);
        callback.run(());
        true
    }

    fn record(&self, id: &str) {
        self.recent.update(|list| {
            list.retain(|existing| existing != id);
            list.insert(0, id.to_string());
            list.truncate(RECENT_LIMIT);
        });
    }
}

fn find_command(commands: &[Command], id: &str) -> Option<Command> {
    for command in commands {
        if command.id == id {
            return Some(command.clone());
        }
        if let Some(found) = find_command(&command.children, id) {
            return Some(found);
        }
    }
    None
}

/// Creates a [`CommandRegistry`], provides it as context, and returns it.
pub fn provide_command_registry() -> CommandRegistry {
    let registry = CommandRegistry::default();
    provide_context(registry);
    registry
}

/// Retrieves the [`CommandRegistry`] from context, or a fresh empty default if none was provided.
pub fn use_commands() -> CommandRegistry {
    use_context::<CommandRegistry>().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::Command;
    use super::find_command;
    use leptos::prelude::Callback;

    fn leaf(id: &str) -> Command {
        Command::new(id, id, Callback::new(|_| {}))
    }

    #[test]
    fn find_command_descends_into_submenus() {
        let tree = vec![
            leaf("a"),
            Command::submenu("group", "Group", vec![leaf("b"), leaf("c")]),
        ];
        assert!(find_command(&tree, "a").is_some());
        assert!(find_command(&tree, "c").is_some());
        assert!(find_command(&tree, "missing").is_none());
    }
}
