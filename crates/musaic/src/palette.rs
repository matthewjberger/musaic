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

    view! {
        <Show when=move || open.get() fallback=|| ()>
            <div class="musaic-palette-scrim" on:click=move |_| open.set(false)>
                <div class="musaic-palette" on:click=|event| event.stop_propagation()>
                    <input
                        node_ref=input_ref
                        class="musaic-palette-input"
                        type="text"
                        placeholder="Type a command…"
                        prop:value=move || query.get()
                        on:input=move |event| query.set(event_target_value(&event))
                        on:keydown=on_key
                    />
                    <div class="musaic-palette-list">
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
                                        view! {
                                            <div
                                                class=move || {
                                                    if active.get() == index {
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
