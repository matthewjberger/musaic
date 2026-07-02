use std::collections::HashSet;

use leptos::prelude::*;
use wasm_bindgen::JsCast;

pub type Highlighter = fn(&str) -> Vec<(&'static str, String)>;

pub fn highlight_code(
    source: &str,
    keywords: &[&'static str],
    commands: &[&'static str],
) -> Vec<(&'static str, String)> {
    let keyword_set: HashSet<&'static str> = keywords.iter().copied().collect();
    let command_set: HashSet<&'static str> = commands.iter().copied().collect();
    scan(source, &keyword_set, &command_set)
}

fn scan(
    source: &str,
    keywords: &HashSet<&'static str>,
    commands: &HashSet<&'static str>,
) -> Vec<(&'static str, String)> {
    let characters: Vec<char> = source.chars().collect();
    let count = characters.len();
    let mut runs: Vec<(&'static str, String)> = Vec::new();
    let mut index = 0;
    while index < count {
        let current = characters[index];
        if current == '/' && index + 1 < count && characters[index + 1] == '*' {
            let start = index;
            index += 2;
            while index < count
                && !(characters[index] == '*' && index + 1 < count && characters[index + 1] == '/')
            {
                index += 1;
            }
            index = (index + 2).min(count);
            runs.push(("tok-comment", characters[start..index].iter().collect()));
        } else if current == '/' && index + 1 < count && characters[index + 1] == '/' {
            let start = index;
            while index < count && characters[index] != '\n' {
                index += 1;
            }
            runs.push(("tok-comment", characters[start..index].iter().collect()));
        } else if current == '"' {
            let start = index;
            index += 1;
            while index < count {
                if characters[index] == '\\' && index + 1 < count {
                    index += 2;
                    continue;
                }
                let quote = characters[index] == '"';
                index += 1;
                if quote {
                    break;
                }
            }
            runs.push(("tok-string", characters[start..index].iter().collect()));
        } else if current.is_ascii_digit() {
            let start = index;
            while index < count
                && (characters[index].is_ascii_alphanumeric() || characters[index] == '.')
            {
                index += 1;
            }
            runs.push(("tok-number", characters[start..index].iter().collect()));
        } else if current.is_alphabetic() || current == '_' {
            let start = index;
            while index < count && (characters[index].is_alphanumeric() || characters[index] == '_')
            {
                index += 1;
            }
            let word: String = characters[start..index].iter().collect();
            let class = if keywords.contains(word.as_str()) {
                "tok-keyword"
            } else if commands.contains(word.as_str()) {
                "tok-command"
            } else {
                "tok-plain"
            };
            runs.push((class, word));
        } else {
            let start = index;
            index += 1;
            while index < count {
                let next = characters[index];
                let token_start = (next == '/'
                    && index + 1 < count
                    && (characters[index + 1] == '/' || characters[index + 1] == '*'))
                    || next == '"'
                    || next.is_ascii_digit()
                    || next.is_alphabetic()
                    || next == '_';
                if token_start {
                    break;
                }
                index += 1;
            }
            runs.push(("tok-plain", characters[start..index].iter().collect()));
        }
    }
    runs
}

#[component]
pub fn CodeEditor(
    value: RwSignal<String>,
    #[prop(optional)] highlighter: Option<Highlighter>,
    #[prop(into, optional, default = "240px".to_string())] height: String,
    #[prop(optional)] fill: bool,
    #[prop(optional)] gutter: bool,
    #[prop(into, optional)] diagnostics: Signal<Vec<usize>>,
    #[prop(optional)] find: bool,
) -> impl IntoView {
    let pre_ref = NodeRef::<leptos::html::Pre>::new();
    let gutter_ref = NodeRef::<leptos::html::Div>::new();
    let area_ref = NodeRef::<leptos::html::Textarea>::new();
    let find_open = RwSignal::new(false);
    let query = RwSignal::new(String::new());
    let replacement = RwSignal::new(String::new());
    let class = if fill {
        "musaic-code-editor fill"
    } else {
        "musaic-code-editor"
    };
    let style = (!fill).then(|| format!("height:{height}"));

    let spans = move || {
        let text = value.get();
        match highlighter {
            Some(highlight) => highlight(&text),
            None => vec![("tok-plain", text)],
        }
    };

    let line_count = move || value.get().lines().count().max(1);

    let on_scroll = move |event: web_sys::Event| {
        let area = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok());
        if let Some(area) = area {
            if let Some(pre) = pre_ref.get() {
                pre.set_scroll_top(area.scroll_top());
                pre.set_scroll_left(area.scroll_left());
            }
            if let Some(gutter) = gutter_ref.get() {
                gutter.set_scroll_top(area.scroll_top());
            }
        }
    };

    let find_next = move || {
        let needle = query.get_untracked();
        if needle.is_empty() {
            return;
        }
        if let Some(area) = area_ref.get() {
            let text = value.get_untracked();
            let from = area.selection_end().ok().flatten().unwrap_or(0) as usize;
            let found = text
                .get(from.min(text.len())..)
                .and_then(|rest| rest.find(&needle))
                .map(|offset| from + offset)
                .or_else(|| text.find(&needle));
            if let Some(position) = found {
                let _ = area.focus();
                let _ = area.set_selection_range(position as u32, (position + needle.len()) as u32);
            }
        }
    };

    let replace_one = move || {
        let needle = query.get_untracked();
        let with = replacement.get_untracked();
        if needle.is_empty() {
            return;
        }
        if let Some(area) = area_ref.get() {
            let start = area.selection_start().ok().flatten().unwrap_or(0) as usize;
            let end = area.selection_end().ok().flatten().unwrap_or(0) as usize;
            let text = value.get_untracked();
            if end > start && text.get(start..end) == Some(needle.as_str()) {
                let mut next = text.clone();
                next.replace_range(start..end, &with);
                value.set(next);
            }
        }
        find_next();
    };

    let replace_all = move || {
        let needle = query.get_untracked();
        if needle.is_empty() {
            return;
        }
        let with = replacement.get_untracked();
        value.set(value.get_untracked().replace(&needle, &with));
    };

    let on_editor_key = move |event: web_sys::KeyboardEvent| {
        if find && (event.ctrl_key() || event.meta_key()) && event.key() == "f" {
            event.prevent_default();
            find_open.set(true);
        }
    };

    view! {
        <div class=class style=style on:keydown=on_editor_key>
            {find
                .then(|| {
                    view! {
                        <Show when=move || find_open.get() fallback=|| ()>
                            <div class="musaic-code-find">
                                <input
                                    class="musaic-code-find-input"
                                    placeholder="Find"
                                    prop:value=move || query.get()
                                    on:input=move |event| query.set(event_target_value(&event))
                                    on:keydown=move |event| {
                                        if event.key() == "Enter" {
                                            event.prevent_default();
                                            find_next();
                                        }
                                    }
                                />
                                <input
                                    class="musaic-code-find-input"
                                    placeholder="Replace"
                                    prop:value=move || replacement.get()
                                    on:input=move |event| replacement.set(event_target_value(&event))
                                />
                                <button class="musaic-button" on:click=move |_| find_next()>
                                    "Next"
                                </button>
                                <button class="musaic-button" on:click=move |_| replace_one()>
                                    "Replace"
                                </button>
                                <button class="musaic-button" on:click=move |_| replace_all()>
                                    "All"
                                </button>
                                <button class="musaic-button" on:click=move |_| find_open.set(false)>
                                    "\u{00d7}"
                                </button>
                            </div>
                        </Show>
                    }
                })}
            <div class="musaic-code-columns">
                {gutter
                .then(|| {
                    view! {
                        <div class="musaic-code-gutter" node_ref=gutter_ref>
                            {move || {
                                let errors = diagnostics.get();
                                (1..=line_count())
                                    .map(|line| {
                                        let is_error = errors.contains(&line);
                                        view! {
                                            <div class="musaic-code-gutter-line" class:error=is_error>
                                                {line}
                                            </div>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </div>
                    }
                })}
            <div class="musaic-code-surface">
                <pre node_ref=pre_ref class="musaic-code-highlight" aria-hidden="true">
                    {move || {
                        spans()
                            .into_iter()
                            .map(|(class, text)| view! { <span class=class>{text}</span> })
                            .collect_view()
                    }}
                </pre>
                <textarea
                    node_ref=area_ref
                    class="musaic-code-textarea"
                    spellcheck="false"
                    prop:value=move || value.get()
                    on:input=move |event| value.set(event_target_value(&event))
                    on:scroll=on_scroll
                ></textarea>
            </div>
            </div>
        </div>
    }
}

#[derive(Clone)]
pub struct CodeDocument {
    pub id: String,
    pub title: String,
    pub value: RwSignal<String>,
}

impl CodeDocument {
    pub fn new(id: impl Into<String>, title: impl Into<String>, value: RwSignal<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            value,
        }
    }
}

#[component]
pub fn CodeTabs(
    #[prop(into)] documents: Signal<Vec<CodeDocument>>,
    active: RwSignal<String>,
    #[prop(optional)] highlighter: Option<Highlighter>,
    #[prop(optional)] on_close: Option<Callback<String>>,
    #[prop(optional)] find: bool,
) -> impl IntoView {
    let active_doc = move || {
        documents
            .get()
            .into_iter()
            .find(|document| document.id == active.get())
    };
    view! {
        <div class="musaic-code-tabs">
            <div class="musaic-code-tabbar" role="tablist">
                {move || {
                    documents
                        .get()
                        .into_iter()
                        .map(|document| {
                            let id_select = document.id.clone();
                            let id_close = document.id.clone();
                            let id_active = document.id.clone();
                            let is_active = move || active.get() == id_active;
                            view! {
                                <div class="musaic-code-tab" class:active=is_active>
                                    <button
                                        class="musaic-code-tab-label"
                                        on:click=move |_| active.set(id_select.clone())
                                    >
                                        {document.title}
                                    </button>
                                    {on_close
                                        .map(|callback| {
                                            view! {
                                                <button
                                                    class="musaic-code-tab-close"
                                                    aria-label="Close"
                                                    on:click=move |_| callback.run(id_close.clone())
                                                >
                                                    "\u{00d7}"
                                                </button>
                                            }
                                        })}
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>
            <div class="musaic-code-tab-body">
                {move || match (active_doc(), highlighter) {
                    (Some(document), Some(highlighter)) => {
                        view! {
                            <CodeEditor
                                value=document.value
                                highlighter=highlighter
                                fill=true
                                find=find
                            />
                        }
                            .into_any()
                    }
                    (Some(document), None) => {
                        view! { <CodeEditor value=document.value fill=true find=find /> }.into_any()
                    }
                    (None, _) => {
                        view! { <div class="musaic-code-empty">"No open document"</div> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
