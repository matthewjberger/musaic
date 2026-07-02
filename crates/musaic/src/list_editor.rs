//! An editable, reorderable list component ([`OrderedList`]) and its [`ListItem`] entry.

use leptos::prelude::*;

/// An entry in an [`OrderedList`], identified by `id` with a display `label`.
#[derive(Clone)]
pub struct ListItem {
    /// The stable identifier passed to the list's callbacks.
    pub id: String,
    /// The text shown for this row.
    pub label: String,
}

impl ListItem {
    /// Creates a [`ListItem`] from an id and its display label.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

/// A reorderable list of [`ListItem`]s, showing move-up, move-down, and remove actions
/// only for the callbacks that are provided (up/down are disabled at the ends). Each
/// callback receives the affected item's id, and `on_select` fires when a row's label is
/// clicked. Renders an "Empty" placeholder when there are no items.
#[component]
pub fn OrderedList(
    #[prop(into)] items: Signal<Vec<ListItem>>,
    #[prop(optional)] on_move_up: Option<Callback<String>>,
    #[prop(optional)] on_move_down: Option<Callback<String>>,
    #[prop(optional)] on_remove: Option<Callback<String>>,
    #[prop(optional)] on_select: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <div class="musaic-ordered-list" role="list">
            {move || {
                let rows = items.get();
                let count = rows.len();
                if count == 0 {
                    return view! { <div class="musaic-ordered-empty">"Empty"</div> }.into_any();
                }
                rows.into_iter()
                    .enumerate()
                    .map(|(index, item)| {
                        let select_id = item.id.clone();
                        let up_id = item.id.clone();
                        let down_id = item.id.clone();
                        let remove_id = item.id.clone();
                        let is_first = index == 0;
                        let is_last = index + 1 == count;
                        view! {
                            <div class="musaic-ordered-row" role="listitem">
                                <button
                                    class="musaic-ordered-label"
                                    on:click=move |_| {
                                        if let Some(callback) = on_select {
                                            callback.run(select_id.clone());
                                        }
                                    }
                                >
                                    {item.label}
                                </button>
                                <div class="musaic-ordered-actions">
                                    {on_move_up
                                        .map(|callback| {
                                            view! {
                                                <button
                                                    class="musaic-ordered-action"
                                                    aria-label="Move up"
                                                    disabled=is_first
                                                    on:click=move |_| callback.run(up_id.clone())
                                                >
                                                    "\u{25b2}"
                                                </button>
                                            }
                                        })}
                                    {on_move_down
                                        .map(|callback| {
                                            view! {
                                                <button
                                                    class="musaic-ordered-action"
                                                    aria-label="Move down"
                                                    disabled=is_last
                                                    on:click=move |_| callback.run(down_id.clone())
                                                >
                                                    "\u{25bc}"
                                                </button>
                                            }
                                        })}
                                    {on_remove
                                        .map(|callback| {
                                            view! {
                                                <button
                                                    class="musaic-ordered-action danger"
                                                    aria-label="Remove"
                                                    on:click=move |_| callback.run(remove_id.clone())
                                                >
                                                    "\u{00d7}"
                                                </button>
                                            }
                                        })}
                                </div>
                            </div>
                        }
                    })
                    .collect_view()
                    .into_any()
            }}
        </div>
    }
}
