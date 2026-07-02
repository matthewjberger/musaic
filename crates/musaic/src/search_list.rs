//! A filterable list of titled entries with an expandable detail pane.

use leptos::prelude::*;

/// An entry in a [`SearchList`], with a stable `id`, a `title` and `subtitle`
/// (both searched), and a `detail` body shown when the entry is selected.
#[derive(Clone)]
pub struct SearchItem {
    /// Stable identifier used for selection and callbacks.
    pub id: String,
    /// Primary text, matched against the search query.
    pub title: String,
    /// Secondary text, matched against the search query.
    pub subtitle: String,
    /// Longer body shown in a detail pane when the entry is active.
    pub detail: String,
}

impl SearchItem {
    /// Builds an item from an `id` and `title`, leaving subtitle and detail empty.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            subtitle: String::new(),
            detail: String::new(),
        }
    }

    /// Sets the item's subtitle, returning the updated item.
    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    /// Sets the item's detail body, returning the updated item.
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    fn matches(&self, needle: &str) -> bool {
        needle.is_empty()
            || self.title.to_lowercase().contains(needle)
            || self.subtitle.to_lowercase().contains(needle)
    }
}

fn dom_id(id: &str) -> String {
    let sanitized: String = id
        .chars()
        .map(|character| {
            if character.is_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect();
    format!("musaic-sl-{sanitized}")
}

/// A searchable list of `items` with a text input that filters by title and
/// subtitle. The active entry expands to show its detail body and scrolls into
/// view; `selected` and `on_select` track the current selection and
/// `placeholder` customizes the search box.
#[component]
pub fn SearchList(
    #[prop(into)] items: Signal<Vec<SearchItem>>,
    #[prop(optional)] selected: Option<RwSignal<Option<String>>>,
    #[prop(optional)] on_select: Option<Callback<String>>,
    #[prop(into, optional)] placeholder: String,
) -> impl IntoView {
    let query = RwSignal::new(String::new());
    let selected = selected.unwrap_or_else(|| RwSignal::new(None));
    let placeholder = if placeholder.is_empty() {
        "Search…".to_string()
    } else {
        placeholder
    };

    let filtered = move || {
        let needle = query.get().to_lowercase();
        items
            .get()
            .into_iter()
            .filter(|item| item.matches(&needle))
            .collect::<Vec<_>>()
    };

    Effect::new(move |_| {
        if let Some(id) = selected.get()
            && let Some(element) = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.get_element_by_id(&dom_id(&id)))
        {
            element.scroll_into_view_with_bool(false);
        }
    });

    view! {
        <div class="musaic-search-list">
            <input
                class="musaic-search-list-input"
                type="search"
                placeholder=placeholder
                prop:value=move || query.get()
                on:input=move |event| query.set(event_target_value(&event))
            />
            <div class="musaic-search-list-items">
                {move || {
                    let rows = filtered();
                    if rows.is_empty() {
                        return view! {
                            <div class="musaic-search-list-empty">"No matches"</div>
                        }
                            .into_any();
                    }
                    rows.into_iter()
                        .map(|item| {
                            let row_id = StoredValue::new(item.id.clone());
                            let select_id = item.id.clone();
                            let is_active = move || {
                                row_id.with_value(|value| {
                                    selected.get().as_deref() == Some(value.as_str())
                                })
                            };
                            let subtitle = item.subtitle.clone();
                            let detail = item.detail.clone();
                            let on_row = move |_| {
                                selected.set(Some(select_id.clone()));
                                if let Some(callback) = on_select {
                                    callback.run(select_id.clone());
                                }
                            };
                            view! {
                                <div id=dom_id(&item.id) class="musaic-search-list-row">
                                    <button
                                        class="musaic-search-list-head"
                                        class:active=is_active
                                        on:click=on_row
                                    >
                                        <span class="musaic-search-list-title">{item.title}</span>
                                        {(!subtitle.is_empty())
                                            .then(|| {
                                                view! {
                                                    <span class="musaic-search-list-subtitle">
                                                        {subtitle}
                                                    </span>
                                                }
                                            })}
                                    </button>
                                    {(!detail.is_empty())
                                        .then(move || {
                                            view! {
                                                <Show when=is_active fallback=|| ()>
                                                    <pre class="musaic-search-list-detail">
                                                        {detail.clone()}
                                                    </pre>
                                                </Show>
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
