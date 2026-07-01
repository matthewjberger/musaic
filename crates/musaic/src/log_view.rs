use leptos::html;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogKind {
    Info,
    Command,
    Event,
    Warn,
    Error,
}

impl LogKind {
    fn class(self) -> &'static str {
        match self {
            LogKind::Info => "info",
            LogKind::Command => "command",
            LogKind::Event => "event",
            LogKind::Warn => "warn",
            LogKind::Error => "error",
        }
    }

    fn tag(self) -> &'static str {
        match self {
            LogKind::Info => "info",
            LogKind::Command => "cmd",
            LogKind::Event => "evt",
            LogKind::Warn => "warn",
            LogKind::Error => "err",
        }
    }
}

#[derive(Clone)]
pub struct LogEntry {
    pub id: usize,
    pub kind: LogKind,
    pub label: String,
    pub detail: String,
    pub count: usize,
}

impl LogEntry {
    pub fn new(id: usize, kind: LogKind, label: impl Into<String>) -> Self {
        Self {
            id,
            kind,
            label: label.into(),
            detail: String::new(),
            count: 1,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }
}

#[component]
pub fn LogView(
    #[prop(into)] entries: Signal<Vec<LogEntry>>,
    #[prop(optional)] on_select: Option<Callback<usize>>,
    #[prop(optional)] on_clear: Option<Callback<()>>,
    #[prop(default = true)] autoscroll: bool,
    #[prop(into, optional)] empty: String,
) -> impl IntoView {
    let body_ref = NodeRef::<html::Div>::new();
    let empty = if empty.is_empty() {
        "No entries".to_string()
    } else {
        empty
    };

    Effect::new(move |_| {
        let _ = entries.get();
        if autoscroll && let Some(body) = body_ref.get() {
            body.set_scroll_top(body.scroll_height());
        }
    });

    view! {
        <div class="musaic-log">
            <div class="musaic-log-head">
                <span class="musaic-log-title">
                    {move || format!("{} entries", entries.get().len())}
                </span>
                {on_clear
                    .map(|callback| {
                        view! {
                            <button
                                class="musaic-log-clear"
                                on:click=move |_| callback.run(())
                            >
                                "Clear"
                            </button>
                        }
                    })}
            </div>
            <div class="musaic-log-body" node_ref=body_ref>
                {move || {
                    let rows = entries.get();
                    if rows.is_empty() {
                        return view! { <div class="musaic-log-empty">{empty.clone()}</div> }
                            .into_any();
                    }
                    rows.into_iter()
                        .map(|entry| {
                            let id = entry.id;
                            let selectable = on_select.is_some();
                            let on_row = move |_| {
                                if let Some(callback) = on_select {
                                    callback.run(id);
                                }
                            };
                            let detail = entry.detail.clone();
                            view! {
                                <div
                                    class=format!("musaic-log-row {}", entry.kind.class())
                                    class:selectable=selectable
                                    on:click=on_row
                                >
                                    <span class="musaic-log-tag">{entry.kind.tag()}</span>
                                    <span class="musaic-log-label">{entry.label}</span>
                                    {(!detail.is_empty())
                                        .then(|| {
                                            view! { <span class="musaic-log-detail">{detail}</span> }
                                        })}
                                    {(entry.count > 1)
                                        .then(|| {
                                            view! {
                                                <span class="musaic-log-count">
                                                    {format!("x{}", entry.count)}
                                                </span>
                                            }
                                        })}
                                </div>
                            }
                        })
                        .collect_view()
                        .into_any()
                }}
            </div>
        </div>
    }
}
