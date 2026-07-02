//! A simple scrollback terminal that renders tone-styled lines and emits
//! submitted prompt input.

use leptos::html;
use leptos::prelude::*;

/// Visual style applied to a terminal line.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TerminalTone {
    /// Default foreground text.
    Normal,
    /// Muted/secondary text.
    Dim,
    /// Success (typically green) text.
    Success,
    /// Warning (typically amber) text.
    Warn,
    /// Error (typically red) text.
    Error,
    /// Echoed command input.
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

/// One line of terminal scrollback: a stable `id` for keyed rendering, its
/// `text`, and its display `tone`.
#[derive(Clone)]
pub struct TerminalLine {
    /// Stable key used to diff the scrollback list.
    pub id: usize,
    /// Line contents.
    pub text: String,
    /// Visual style for the line.
    pub tone: TerminalTone,
}

impl TerminalLine {
    /// Builds a line from an `id`, `text`, and `tone`.
    pub fn new(id: usize, text: impl Into<String>, tone: TerminalTone) -> Self {
        Self {
            id,
            text: text.into(),
            tone,
        }
    }
}

/// A scrollback terminal that renders `lines` (auto-scrolling to the bottom as
/// they change) with a `prompt` sigil (defaulting to `$`) and an input row that
/// fires `on_input` with the trimmed draft when Enter is pressed.
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
