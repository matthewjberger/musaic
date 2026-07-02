//! A searchable grid of thumbnail asset cards.

use leptos::prelude::*;

/// A card in an [`AssetGrid`], with a stable `id`, a `label`, a `thumbnail`
/// image source, and an optional `subtitle`.
#[derive(Clone)]
pub struct AssetItem {
    /// Stable identifier passed to the selection callback.
    pub id: String,
    /// Primary caption, also matched against the search query.
    pub label: String,
    /// Image source used for the card thumbnail.
    pub thumbnail: String,
    /// Optional secondary caption shown beneath the label.
    pub subtitle: String,
}

impl AssetItem {
    /// Builds an item from an `id`, `label`, and `thumbnail`, leaving the
    /// subtitle empty.
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        thumbnail: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            thumbnail: thumbnail.into(),
            subtitle: String::new(),
        }
    }

    /// Sets the item's subtitle, returning the updated item.
    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = subtitle.into();
        self
    }
}

/// A grid of thumbnail cards built from `items`, invoking `on_select` with an
/// item's id when its card is clicked. When `searchable` is set a filter box
/// (labeled by `placeholder`) narrows cards by label.
#[component]
pub fn AssetGrid(
    #[prop(into)] items: Signal<Vec<AssetItem>>,
    on_select: Callback<String>,
    #[prop(default = true)] searchable: bool,
    #[prop(into, optional)] placeholder: String,
) -> impl IntoView {
    let query = RwSignal::new(String::new());
    let placeholder = if placeholder.is_empty() {
        "Search assets…".to_string()
    } else {
        placeholder
    };

    let filtered = move || {
        let needle = query.get().to_lowercase();
        items
            .get()
            .into_iter()
            .filter(|item| needle.is_empty() || item.label.to_lowercase().contains(&needle))
            .collect::<Vec<_>>()
    };

    view! {
        <div class="musaic-asset-grid">
            {searchable
                .then(|| {
                    view! {
                        <input
                            class="musaic-asset-search"
                            type="search"
                            placeholder=placeholder.clone()
                            prop:value=move || query.get()
                            on:input=move |event| query.set(event_target_value(&event))
                        />
                    }
                })}
            <div class="musaic-asset-cards">
                {move || {
                    let rows = filtered();
                    if rows.is_empty() {
                        return view! { <div class="musaic-asset-empty">"No assets"</div> }
                            .into_any();
                    }
                    rows.into_iter()
                        .map(|item| {
                            let id = item.id.clone();
                            let label = item.label.clone();
                            let subtitle = item.subtitle.clone();
                            view! {
                                <button
                                    class="musaic-asset-card"
                                    title=label.clone()
                                    on:click=move |_| on_select.run(id.clone())
                                >
                                    <span class="musaic-asset-thumb">
                                        <img loading="lazy" src=item.thumbnail alt=label.clone() />
                                    </span>
                                    <span class="musaic-asset-label">{label.clone()}</span>
                                    {(!subtitle.is_empty())
                                        .then(|| {
                                            view! {
                                                <span class="musaic-asset-subtitle">{subtitle}</span>
                                            }
                                        })}
                                </button>
                            }
                        })
                        .collect_view()
                        .into_any()
                }}
            </div>
        </div>
    }
}
