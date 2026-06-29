use leptos::prelude::*;

#[derive(Clone)]
pub struct Command {
    pub id: String,
    pub title: String,
    pub hint: String,
    pub action: Callback<()>,
}

impl Command {
    pub fn new(id: impl Into<String>, title: impl Into<String>, action: Callback<()>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            hint: String::new(),
            action,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = hint.into();
        self
    }
}

fn fuzzy(query: &str, target: &str) -> bool {
    let target = target.to_lowercase();
    let mut needle = query
        .to_lowercase()
        .chars()
        .collect::<Vec<_>>()
        .into_iter()
        .peekable();
    for character in target.chars() {
        if let Some(want) = needle.peek().copied()
            && want == character
        {
            needle.next();
        }
    }
    needle.peek().is_none()
}

fn option_id(index: usize) -> String {
    format!("musaic-palette-option-{index}")
}

#[component]
pub fn CommandPalette(
    open: RwSignal<bool>,
    #[prop(into)] commands: Signal<Vec<Command>>,
) -> impl IntoView {
    let query = RwSignal::new(String::new());
    let active = RwSignal::new(0usize);
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let filtered = move || {
        let needle = query.get();
        commands
            .get()
            .into_iter()
            .filter(|command| fuzzy(&needle, &command.title))
            .collect::<Vec<_>>()
    };

    Effect::new(move |_| {
        let _ = query.get();
        active.set(0);
    });

    Effect::new(move |_| {
        if open.get()
            && let Some(input) = input_ref.get()
        {
            let _ = input.focus();
        }
    });

    let select = Callback::new(move |index: usize| {
        if let Some(command) = filtered().get(index) {
            command.action.run(());
        }
        open.set(false);
        query.set(String::new());
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
        "Escape" => open.set(false),
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

    view! {
        <Show when=move || open.get() fallback=|| ()>
            <div class="musaic-palette-scrim" on:click=move |_| open.set(false)>
                <div class="musaic-palette" on:click=|event| event.stop_propagation()>
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
                                        let hint = command.hint.clone();
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
                                                <span>{command.title}</span>
                                                <span class="hint">{hint}</span>
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
    use super::fuzzy;

    #[test]
    fn fuzzy_matches_subsequences_case_insensitively() {
        assert!(fuzzy("sc", "Spawn Cube"));
        assert!(fuzzy("", "anything"));
        assert!(fuzzy("CUBE", "spawn cube"));
    }

    #[test]
    fn fuzzy_rejects_out_of_order_or_missing_characters() {
        assert!(!fuzzy("cs", "Spawn Cube"));
        assert!(!fuzzy("xyz", "Spawn Cube"));
    }
}
