//! A chat transcript with a compose box, a connection indicator, and a busy spinner.

use leptos::html;
use leptos::prelude::*;

/// Who or what produced a `ChatMessage`, used to pick the bubble's styling.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChatRole {
    /// A message from the user.
    User,
    /// A message from the assistant.
    Assistant,
    /// Assistant reasoning shown as a thinking bubble.
    Thinking,
    /// Output from a tool invocation.
    Tool,
    /// An informational system message.
    Info,
    /// An error message.
    Error,
}

impl ChatRole {
    fn class(self) -> &'static str {
        match self {
            ChatRole::User => "user",
            ChatRole::Assistant => "assistant",
            ChatRole::Thinking => "thinking",
            ChatRole::Tool => "tool",
            ChatRole::Info => "info",
            ChatRole::Error => "error",
        }
    }
}

/// A single message in the transcript: a stable `id` for keyed rendering, its
/// `role`, and the message `text`.
#[derive(Clone)]
pub struct ChatMessage {
    /// Stable identifier used as the render key.
    pub id: usize,
    /// The author role controlling the bubble styling.
    pub role: ChatRole,
    /// The message body.
    pub text: String,
}

impl ChatMessage {
    /// Creates a message with the given id, role, and text.
    pub fn new(id: usize, role: ChatRole, text: impl Into<String>) -> Self {
        Self {
            id,
            role,
            text: text.into(),
        }
    }
}

/// A chat panel that renders the `messages` transcript, auto-scrolling to the
/// latest, and a compose box that fires `on_send` with the trimmed text on click
/// or Enter (Shift+Enter inserts a newline). The header shows a `connected` dot,
/// a `busy` spinner appears while working, and an optional `on_reset` renders a
/// "New" button. `placeholder` sets the input hint.
#[component]
pub fn Chat(
    #[prop(into)] messages: Signal<Vec<ChatMessage>>,
    on_send: Callback<String>,
    #[prop(into, optional)] busy: Signal<bool>,
    #[prop(into, optional)] connected: Signal<bool>,
    #[prop(optional)] on_reset: Option<Callback<()>>,
    #[prop(into, optional)] placeholder: String,
) -> impl IntoView {
    let draft = RwSignal::new(String::new());
    let body_ref = NodeRef::<html::Div>::new();
    let placeholder = if placeholder.is_empty() {
        "Message…".to_string()
    } else {
        placeholder
    };

    Effect::new(move |_| {
        let _ = messages.get();
        if let Some(body) = body_ref.get() {
            body.set_scroll_top(body.scroll_height());
        }
    });

    let submit = move || {
        let text = draft.get_untracked().trim().to_string();
        if !text.is_empty() {
            on_send.run(text);
            draft.set(String::new());
        }
    };

    view! {
        <div class="musaic-chat">
            <div class="musaic-chat-head">
                <span class="musaic-chat-status" class:online=move || connected.get()></span>
                <span class="musaic-chat-status-text">
                    {move || if connected.get() { "Connected" } else { "Offline" }}
                </span>
                <span class="musaic-chat-spacer"></span>
                {on_reset
                    .map(|callback| {
                        view! {
                            <button class="musaic-chat-reset" on:click=move |_| callback.run(())>
                                "New"
                            </button>
                        }
                    })}
            </div>
            <div class="musaic-chat-body" node_ref=body_ref>
                <For each=move || messages.get() key=|message| message.id let:message>
                    <div class=format!("musaic-chat-message {}", message.role.class())>
                        {message.text.clone()}
                    </div>
                </For>
                <Show when=move || busy.get() fallback=|| ()>
                    <div class="musaic-chat-busy">
                        <span class="musaic-spinner"></span>
                        "Working…"
                    </div>
                </Show>
            </div>
            <div class="musaic-chat-compose">
                <textarea
                    class="musaic-chat-input"
                    placeholder=placeholder
                    prop:value=move || draft.get()
                    on:input=move |event| draft.set(event_target_value(&event))
                    on:keydown=move |event| {
                        if event.key() == "Enter" && !event.shift_key() {
                            event.prevent_default();
                            submit();
                        }
                    }
                ></textarea>
                <button
                    class="musaic-button primary musaic-chat-send"
                    disabled=move || draft.get().trim().is_empty()
                    on:click=move |_| submit()
                >
                    "Send"
                </button>
            </div>
        </div>
    }
}
