//! The command palette component, driven by the [`CommandRegistry`](crate::command::CommandRegistry).

use leptos::prelude::*;

use crate::command::{Command, use_commands};
use crate::keymap::pretty_binding;

fn fuzzy_score(query: &str, target: &str) -> Option<i32> {
    if query.is_empty() {
        return Some(0);
    }
    let target_lower = target.to_lowercase();
    let query_lower = query.to_lowercase();
    let mut needle = query_lower.chars().peekable();
    let mut score = 0;
    let mut streak = 0;
    let mut matched_any = false;
    for (index, character) in target_lower.chars().enumerate() {
        if let Some(want) = needle.peek().copied()
            && want == character
        {
            needle.next();
            matched_any = true;
            streak += 1;
            score += 1 + streak;
            if index == 0 {
                score += 4;
            }
        } else {
            streak = 0;
        }
    }
    if needle.peek().is_none() && matched_any {
        Some(score)
    } else {
        None
    }
}

fn option_id(index: usize) -> String {
    format!("musaic-palette-option-{index}")
}

fn resolve_level(root: Vec<Command>, path: &[String]) -> Vec<Command> {
    let mut current = root;
    for id in path {
        match current.iter().find(|command| &command.id == id) {
            Some(command) => current = command.children.clone(),
            None => return Vec::new(),
        }
    }
    current
}

/// Registry-driven command palette: fuzzy-ranked, shows recents when the query is empty, descends into submenus, and displays keybinding hints.
///
/// Toggle visibility through the `open` signal (typically bound to a shortcut).
/// Arrow keys move the selection, Enter runs or opens the active command,
/// Backspace pops a submenu level, and Escape closes.
#[component]
pub fn CommandPalette(open: RwSignal<bool>) -> impl IntoView {
    let registry = use_commands();
    let query = RwSignal::new(String::new());
    let active = RwSignal::new(0usize);
    let path = RwSignal::new(Vec::<String>::new());
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let level = move || resolve_level(registry.commands(), &path.get());

    let filtered = move || {
        let needle = query.get();
        let commands = level()
            .into_iter()
            .filter(Command::enabled)
            .collect::<Vec<_>>();
        if needle.is_empty() {
            if path.get().is_empty() {
                let recent = registry.recent();
                let mut ordered = Vec::new();
                for id in &recent {
                    if let Some(command) = commands.iter().find(|command| &command.id == id) {
                        ordered.push(command.clone());
                    }
                }
                for command in &commands {
                    if !recent.contains(&command.id) {
                        ordered.push(command.clone());
                    }
                }
                return ordered;
            }
            return commands;
        }
        let mut scored = commands
            .into_iter()
            .filter_map(|command| {
                fuzzy_score(&needle, &command.title).map(|score| (score, command))
            })
            .collect::<Vec<_>>();
        scored.sort_by(|(left, _), (right, _)| right.cmp(left));
        scored.into_iter().map(|(_, command)| command).collect()
    };

    Effect::new(move |_| {
        let _ = query.get();
        let _ = path.get();
        active.set(0);
    });

    Effect::new(move |_| {
        if open.get() {
            path.set(Vec::new());
            query.set(String::new());
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        }
    });

    let select = Callback::new(move |index: usize| {
        let Some(command) = filtered().into_iter().nth(index) else {
            return;
        };
        if command.is_submenu() {
            path.update(|stack| stack.push(command.id.clone()));
            query.set(String::new());
            active.set(0);
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
            return;
        }
        registry.run(&command.id);
        open.set(false);
        query.set(String::new());
        path.set(Vec::new());
    });

    let on_key = move |event: web_sys::KeyboardEvent| match event.key().as_str() {
        "ArrowDown" => {
            event.prevent_default();
            let count = filtered().len();
            if count > 0 {
                active.update(|index| *index = (*index + 1) % count);
            }
        }
        "ArrowUp" => {
            event.prevent_default();
            let count = filtered().len();
            if count > 0 {
                active.update(|index| *index = (*index + count - 1) % count);
            }
        }
        "Enter" => {
            event.prevent_default();
            select.run(active.get());
        }
        "Backspace" if query.with(String::is_empty) && !path.with(Vec::is_empty) => {
            event.prevent_default();
            path.update(|stack| {
                stack.pop();
            });
        }
        "Escape" => {
            event.prevent_default();
            open.set(false);
        }
        _ => {}
    };

    let active_descendant = move || {
        let items = filtered();
        if items.is_empty() {
            String::new()
        } else {
            option_id(active.get().min(items.len() - 1))
        }
    };

    let breadcrumb = move || {
        let stack = path.get();
        if stack.is_empty() {
            return String::new();
        }
        let root = registry.commands();
        let mut titles = Vec::new();
        let mut current = root;
        for id in &stack {
            if let Some(command) = current.iter().find(|command| &command.id == id) {
                titles.push(command.title.clone());
                current = command.children.clone();
            }
        }
        titles.join("  ›  ")
    };

    view! {
        <Show when=move || open.get() fallback=|| ()>
            <div class="musaic-palette-scrim" on:click=move |_| open.set(false)>
                <div class="musaic-palette" on:click=|event| event.stop_propagation()>
                    <Show when=move || !path.get().is_empty() fallback=|| ()>
                        <div class="musaic-palette-breadcrumb">{breadcrumb}</div>
                    </Show>
                    <input
                        node_ref=input_ref
                        class="musaic-palette-input"
                        type="text"
                        role="combobox"
                        aria-expanded="true"
                        aria-controls="musaic-palette-list"
                        aria-autocomplete="list"
                        aria-activedescendant=active_descendant
                        placeholder="Type a command…"
                        prop:value=move || query.get()
                        on:input=move |event| query.set(event_target_value(&event))
                        on:keydown=on_key
                    />
                    <div id="musaic-palette-list" class="musaic-palette-list" role="listbox">
                        {move || {
                            let items = filtered();
                            if items.is_empty() {
                                view! {
                                    <div class="musaic-palette-empty">"No matching commands"</div>
                                }
                                    .into_any()
                            } else {
                                items
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, command)| {
                                        let is_submenu = command.is_submenu();
                                        let group = command.group.clone();
                                        let keybinding = command
                                            .keybinding
                                            .as_deref()
                                            .map(pretty_binding)
                                            .unwrap_or_default();
                                        let is_active = move || active.get() == index;
                                        view! {
                                            <div
                                                id=option_id(index)
                                                role="option"
                                                aria-selected=move || is_active().to_string()
                                                class=move || {
                                                    if is_active() {
                                                        "musaic-palette-item active"
                                                    } else {
                                                        "musaic-palette-item"
                                                    }
                                                }
                                                on:click=move |_| select.run(index)
                                            >
                                                <span class="musaic-palette-title">
                                                    {command.title}
                                                    {(!group.is_empty())
                                                        .then(|| {
                                                            view! {
                                                                <span class="musaic-palette-group">{group}</span>
                                                            }
                                                        })}
                                                </span>
                                                {if is_submenu {
                                                    view! { <span class="hint">"›"</span> }.into_any()
                                                } else if !keybinding.is_empty() {
                                                    view! {
                                                        <kbd class="musaic-palette-kbd">{keybinding}</kbd>
                                                    }
                                                        .into_any()
                                                } else {
                                                    ().into_any()
                                                }}
                                            </div>
                                        }
                                    })
                                    .collect_view()
                                    .into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::fuzzy_score;

    #[test]
    fn fuzzy_matches_subsequences_case_insensitively() {
        assert!(fuzzy_score("sc", "Spawn Cube").is_some());
        assert!(fuzzy_score("", "anything").is_some());
        assert!(fuzzy_score("cube", "spawn cube").is_some());
    }

    #[test]
    fn fuzzy_rejects_out_of_order_or_missing_characters() {
        assert!(fuzzy_score("cs", "Spawn Cube").is_none());
        assert!(fuzzy_score("xyz", "Spawn Cube").is_none());
    }

    #[test]
    fn prefix_matches_outrank_scattered_ones() {
        let prefix = fuzzy_score("spa", "Spawn Cube").expect("prefix matches");
        let scattered = fuzzy_score("spa", "Set Parallax").expect("scattered matches");
        assert!(prefix > scattered);
    }
}
