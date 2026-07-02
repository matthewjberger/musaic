use leptos::html;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TerminalTone {
    Normal,
    Dim,
    Success,
    Warn,
    Error,
    Command,
}

impl TerminalTone {
    fn class(self) -> &'static str {
        match self {
            TerminalTone::Normal => "normal",
            TerminalTone::Dim => "dim",
            TerminalTone::Success => "success",
            TerminalTone::Warn => "warn",
            TerminalTone::Error => "error",
            TerminalTone::Command => "command",
        }
    }
}

#[derive(Clone)]
pub struct TerminalLine {
    pub id: usize,
    pub text: String,
    pub tone: TerminalTone,
}

impl TerminalLine {
    pub fn new(id: usize, text: impl Into<String>, tone: TerminalTone) -> Self {
        Self {
            id,
            text: text.into(),
            tone,
        }
    }
}

#[component]
pub fn Terminal(
    #[prop(into)] lines: Signal<Vec<TerminalLine>>,
    on_input: Callback<String>,
    #[prop(into, optional)] prompt: String,
) -> impl IntoView {
    let draft = RwSignal::new(String::new());
    let body_ref = NodeRef::<html::Div>::new();
    let prompt = if prompt.is_empty() {
        "$".to_string()
    } else {
        prompt
    };
    let prompt = StoredValue::new(prompt);

    Effect::new(move |_| {
        let _ = lines.get();
        if let Some(body) = body_ref.get() {
            body.set_scroll_top(body.scroll_height());
        }
    });

    let submit = move || {
        let text = draft.get_untracked();
        if !text.trim().is_empty() {
            on_input.run(text);
            draft.set(String::new());
        }
    };

    view! {
        <div class="musaic-terminal">
            <div class="musaic-terminal-body" node_ref=body_ref>
                <For each=move || lines.get() key=|line| line.id let:line>
                    <div class=format!("musaic-terminal-line {}", line.tone.class())>
                        {line.text.clone()}
                    </div>
                </For>
            </div>
            <div class="musaic-terminal-prompt">
                <span class="musaic-terminal-sigil">{move || prompt.get_value()}</span>
                <input
                    class="musaic-terminal-input"
                    spellcheck="false"
                    autocomplete="off"
                    prop:value=move || draft.get()
                    on:input=move |event| draft.set(event_target_value(&event))
                    on:keydown=move |event| {
                        if event.key() == "Enter" {
                            event.prevent_default();
                            submit();
                        }
                    }
                />
            </div>
        </div>
    }
}
