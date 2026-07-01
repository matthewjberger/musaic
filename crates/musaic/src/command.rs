use leptos::prelude::*;

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

    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = group.into();
        self
    }

    pub fn with_keybinding(mut self, keybinding: impl Into<String>) -> Self {
        self.keybinding = Some(keybinding.into());
        self
    }

    pub fn with_hint(mut self, group: impl Into<String>) -> Self {
        self.group = group.into();
        self
    }

    pub fn with_enabled(mut self, enabled: impl Into<Signal<bool>>) -> Self {
        self.enabled = enabled.into();
        self
    }

    pub fn is_submenu(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn enabled(&self) -> bool {
        self.enabled.get()
    }

    fn enabled_untracked(&self) -> bool {
        self.enabled.get_untracked()
    }
}

const RECENT_LIMIT: usize = 8;

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
    pub fn register(&self, command: Command) {
        self.commands.update(|list| {
            if let Some(slot) = list.iter_mut().find(|existing| existing.id == command.id) {
                *slot = command;
            } else {
                list.push(command);
            }
        });
    }

    pub fn register_all(&self, commands: impl IntoIterator<Item = Command>) {
        for command in commands {
            self.register(command);
        }
    }

    pub fn unregister(&self, id: &str) {
        self.commands
            .update(|list| list.retain(|command| command.id != id));
    }

    pub fn clear(&self) {
        self.commands.update(Vec::clear);
    }

    pub fn commands(&self) -> Vec<Command> {
        self.commands.get()
    }

    pub fn commands_untracked(&self) -> Vec<Command> {
        self.commands.get_untracked()
    }

    pub fn find(&self, id: &str) -> Option<Command> {
        find_command(&self.commands.get_untracked(), id)
    }

    pub fn recent(&self) -> Vec<String> {
        self.recent.get()
    }

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

pub fn provide_command_registry() -> CommandRegistry {
    let registry = CommandRegistry::default();
    provide_context(registry);
    registry
}

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
