use std::collections::HashSet;

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

#[derive(Clone)]
pub struct TreeItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub lazy: bool,
    pub children: Vec<TreeItem>,
}

impl TreeItem {
    pub fn leaf(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            lazy: false,
            children: Vec::new(),
        }
    }

    pub fn branch(
        id: impl Into<String>,
        label: impl Into<String>,
        children: Vec<TreeItem>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            lazy: false,
            children,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn lazy(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.id = id.into();
        self.label = label.into();
        self.lazy = true;
        self
    }
}

#[derive(Clone, Copy)]
struct TreeApi {
    on_select: Callback<String>,
    selected: Signal<Option<String>>,
    selection: Option<RwSignal<HashSet<String>>>,
    on_rename: Option<Callback<(String, String)>>,
    on_move: Option<Callback<(String, String)>>,
    on_expand: Option<Callback<String>>,
    editing: RwSignal<Option<String>>,
    draft: RwSignal<String>,
    dragging: RwSignal<Option<String>>,
    drop_target: RwSignal<Option<String>>,
}

#[component]
pub fn Tree(
    items: Vec<TreeItem>,
    #[prop(optional)] on_select: Option<Callback<String>>,
    #[prop(optional, into)] selected: Option<Signal<Option<String>>>,
    #[prop(optional)] selection: Option<RwSignal<HashSet<String>>>,
    #[prop(optional)] on_rename: Option<Callback<(String, String)>>,
    #[prop(optional)] on_move: Option<Callback<(String, String)>>,
    #[prop(optional)] on_expand: Option<Callback<String>>,
    #[prop(optional)] default_expanded: bool,
) -> impl IntoView {
    let initial = if default_expanded {
        collect_branch_ids(&items)
    } else {
        HashSet::new()
    };
    let expanded = RwSignal::new(initial);
    let api = TreeApi {
        on_select: on_select.unwrap_or_else(|| Callback::new(|_| {})),
        selected: selected.unwrap_or_else(|| Signal::derive(|| None)),
        selection,
        on_rename,
        on_move,
        on_expand,
        editing: RwSignal::new(None),
        draft: RwSignal::new(String::new()),
        dragging: RwSignal::new(None),
        drop_target: RwSignal::new(None),
    };
    let tree_ref = NodeRef::<html::Div>::new();

    let on_key = move |event: KeyboardEvent| {
        let Some(container) = tree_ref.get() else {
            return;
        };
        let rows = rows_within(&container);
        let Some(active) = active_element() else {
            return;
        };
        let Some(current) = rows.iter().position(|row| same_element(row, &active)) else {
            return;
        };
        let row = &rows[current];
        let id = row.get_attribute("data-tree-id").unwrap_or_default();
        let is_branch = row.get_attribute("data-tree-branch").as_deref() == Some("1");
        let is_open = expanded.with_untracked(|set| set.contains(&id));
        match event.key().as_str() {
            "ArrowDown" => {
                event.prevent_default();
                if let Some(next) = rows.get(current + 1) {
                    let _ = next.focus();
                }
            }
            "ArrowUp" => {
                event.prevent_default();
                if current > 0 {
                    let _ = rows[current - 1].focus();
                }
            }
            "ArrowRight" => {
                event.prevent_default();
                if is_branch && !is_open {
                    if let Some(callback) = api.on_expand {
                        callback.run(id.clone());
                    }
                    expanded.update(|set| {
                        set.insert(id);
                    });
                } else if let Some(next) = rows.get(current + 1) {
                    let _ = next.focus();
                }
            }
            "ArrowLeft" => {
                event.prevent_default();
                if is_branch && is_open {
                    expanded.update(|set| {
                        set.remove(&id);
                    });
                } else if current > 0 {
                    let _ = rows[current - 1].focus();
                }
            }
            "F2" => {
                event.prevent_default();
                if api.on_rename.is_some() {
                    api.draft
                        .set(row.get_attribute("data-tree-label").unwrap_or_default());
                    api.editing.set(Some(id));
                }
            }
            "Enter" | " " => {
                event.prevent_default();
                api.on_select.run(id);
            }
            _ => {}
        }
    };

    view! {
        <div class="musaic-tree" role="tree" node_ref=tree_ref on:keydown=on_key>
            {items
                .into_iter()
                .map(|item| {
                    view! { <Branch item=item expanded=expanded api=api depth=0 /> }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn Branch(
    item: TreeItem,
    expanded: RwSignal<HashSet<String>>,
    api: TreeApi,
    depth: usize,
) -> impl IntoView {
    let expandable = !item.children.is_empty() || item.lazy;
    let label = item.label.clone();
    let icon = item.icon.clone();
    let children = item.children.clone();
    let indent = format!("padding-left:{}px", 6 + depth * 14);
    let id = StoredValue::new(item.id.clone());
    let label_store = StoredValue::new(item.label.clone());

    let is_selected = move || {
        let id = id.get_value();
        api.selected.get().as_deref() == Some(id.as_str())
            || api
                .selection
                .is_some_and(|selection| selection.with(|set| set.contains(&id)))
    };
    let is_editing = move || api.editing.get().as_deref() == Some(id.get_value().as_str());
    let is_drop_target = move || api.drop_target.get().as_deref() == Some(id.get_value().as_str());
    let is_open_chevron = move || expanded.get().contains(&id.get_value());
    let is_open_show = move || expanded.get().contains(&id.get_value());
    let aria_expanded = move || {
        if expandable {
            expanded.get().contains(&id.get_value()).to_string()
        } else {
            String::new()
        }
    };

    let on_row = move |event: web_sys::MouseEvent| {
        let key = id.get_value();
        if let Some(selection) = api.selection {
            if event.ctrl_key() || event.meta_key() {
                selection.update(|set| {
                    if !set.remove(&key) {
                        set.insert(key.clone());
                    }
                });
                return;
            }
            selection.update(|set| {
                set.clear();
                set.insert(key.clone());
            });
        }
        api.on_select.run(key);
    };
    let on_chevron = move |event: web_sys::MouseEvent| {
        event.stop_propagation();
        let key = id.get_value();
        let opening = !expanded.with_untracked(|set| set.contains(&key));
        if opening && let Some(callback) = api.on_expand {
            callback.run(key.clone());
        }
        expanded.update(|set| {
            if !set.remove(&key) {
                set.insert(key);
            }
        });
    };
    let on_double = move |_event: web_sys::MouseEvent| {
        if api.on_rename.is_some() {
            api.draft.set(label_store.get_value());
            api.editing.set(Some(id.get_value()));
        }
    };

    let commit_rename = move || {
        if let Some(callback) = api.on_rename {
            callback.run((id.get_value(), api.draft.get_untracked()));
        }
        api.editing.set(None);
    };
    let on_edit_key = move |event: KeyboardEvent| match event.key().as_str() {
        "Enter" => {
            event.prevent_default();
            commit_rename();
        }
        "Escape" => {
            event.prevent_default();
            api.editing.set(None);
        }
        _ => {}
    };

    let draggable = api.on_move.is_some();
    let on_dragstart = move |event: web_sys::DragEvent| {
        let key = id.get_value();
        api.dragging.set(Some(key.clone()));
        if let Some(transfer) = event.data_transfer() {
            let _ = transfer.set_data("text/plain", &key);
        }
    };
    let on_dragover = move |event: web_sys::DragEvent| {
        event.prevent_default();
        api.drop_target.set(Some(id.get_value()));
    };
    let on_dragleave = move |_event: web_sys::DragEvent| {
        if api.drop_target.get_untracked().as_deref() == Some(id.get_value().as_str()) {
            api.drop_target.set(None);
        }
    };
    let on_drop = move |event: web_sys::DragEvent| {
        event.prevent_default();
        let target = id.get_value();
        let source = api.dragging.get_untracked();
        api.drop_target.set(None);
        api.dragging.set(None);
        if let (Some(source), Some(callback)) = (source, api.on_move)
            && source != target
        {
            callback.run((source, target));
        }
    };
    let on_dragend = move |_event: web_sys::DragEvent| {
        api.dragging.set(None);
        api.drop_target.set(None);
    };

    view! {
        <div
            class="musaic-tree-row"
            class:selected=is_selected
            class:drop-target=is_drop_target
            role="treeitem"
            tabindex="0"
            draggable=draggable.then_some("true")
            data-tree-id=item.id.clone()
            data-tree-label=label_store.get_value()
            data-tree-branch=if expandable { "1" } else { "0" }
            aria-selected=move || is_selected().to_string()
            aria-expanded=aria_expanded
            style=indent
            on:click=on_row
            on:dblclick=on_double
            on:dragstart=on_dragstart
            on:dragover=on_dragover
            on:dragleave=on_dragleave
            on:drop=on_drop
            on:dragend=on_dragend
        >
            {if expandable {
                view! {
                    <span class="musaic-tree-chevron" class:open=is_open_chevron on:click=on_chevron>
                        "\u{25b8}"
                    </span>
                }
                    .into_any()
            } else {
                view! { <span class="musaic-tree-spacer"></span> }.into_any()
            }}
            {icon.map(|glyph| view! { <span class="musaic-tree-icon">{glyph}</span> })}
            <Show
                when=is_editing
                fallback=move || view! { <span class="musaic-tree-label">{label.clone()}</span> }
            >
                <input
                    class="musaic-tree-rename"
                    prop:value=move || api.draft.get()
                    on:click=|event| event.stop_propagation()
                    on:input=move |event| api.draft.set(event_target_value(&event))
                    on:keydown=on_edit_key
                    on:blur=move |_| commit_rename()
                />
            </Show>
        </div>
        {expandable
            .then(move || {
                view! {
                    <Show when=is_open_show fallback=|| ()>
                        {children
                            .clone()
                            .into_iter()
                            .map(|child| {
                                view! {
                                    <Branch
                                        item=child
                                        expanded=expanded
                                        api=api
                                        depth=depth + 1
                                    />
                                }
                            })
                            .collect_view()}
                    </Show>
                }
            })}
    }
    .into_any()
}

fn collect_branch_ids(items: &[TreeItem]) -> HashSet<String> {
    fn walk(items: &[TreeItem], set: &mut HashSet<String>) {
        for item in items {
            if !item.children.is_empty() {
                set.insert(item.id.clone());
                walk(&item.children, set);
            }
        }
    }
    let mut set = HashSet::new();
    walk(items, &mut set);
    set
}

fn rows_within(container: &HtmlElement) -> Vec<HtmlElement> {
    let Ok(list) = container.query_selector_all(".musaic-tree-row") else {
        return Vec::new();
    };
    (0..list.length())
        .filter_map(|index| list.item(index))
        .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
        .collect()
}

fn active_element() -> Option<HtmlElement> {
    web_sys::window()?
        .document()?
        .active_element()
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
}

fn same_element(left: &HtmlElement, right: &HtmlElement) -> bool {
    let left: &wasm_bindgen::JsValue = left.as_ref();
    let right: &wasm_bindgen::JsValue = right.as_ref();
    left == right
}
