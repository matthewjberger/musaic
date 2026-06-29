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
    pub children: Vec<TreeItem>,
}

impl TreeItem {
    pub fn leaf(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
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
            children,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

#[component]
pub fn Tree(
    items: Vec<TreeItem>,
    #[prop(optional)] on_select: Option<Callback<String>>,
    #[prop(optional, into)] selected: Option<Signal<Option<String>>>,
    #[prop(optional)] default_expanded: bool,
) -> impl IntoView {
    let initial = if default_expanded {
        collect_branch_ids(&items)
    } else {
        HashSet::new()
    };
    let expanded = RwSignal::new(initial);
    let on_select = on_select.unwrap_or_else(|| Callback::new(|_| {}));
    let selected = selected.unwrap_or_else(|| Signal::derive(|| None));
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
            "Enter" | " " => {
                event.prevent_default();
                on_select.run(id);
            }
            _ => {}
        }
    };

    view! {
        <div class="musaic-tree" role="tree" node_ref=tree_ref on:keydown=on_key>
            {items
                .into_iter()
                .map(|item| {
                    view! {
                        <Branch
                            item=item
                            expanded=expanded
                            on_select=on_select
                            selected=selected
                            depth=0
                        />
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn Branch(
    item: TreeItem,
    expanded: RwSignal<HashSet<String>>,
    on_select: Callback<String>,
    selected: Signal<Option<String>>,
    depth: usize,
) -> impl IntoView {
    let has_children = !item.children.is_empty();
    let label = item.label.clone();
    let icon = item.icon.clone();
    let children = item.children.clone();
    let indent = format!("padding-left:{}px", 6 + depth * 14);

    let id_select = item.id.clone();
    let id_toggle = item.id.clone();
    let id_chevron = item.id.clone();
    let id_show = item.id.clone();
    let id_selected = item.id.clone();
    let id_selected_aria = item.id.clone();
    let id_expanded = item.id.clone();
    let id_attr = item.id.clone();

    let on_row = move |_: web_sys::MouseEvent| on_select.run(id_select.clone());
    let on_chevron = move |event: web_sys::MouseEvent| {
        event.stop_propagation();
        let key = id_toggle.clone();
        expanded.update(|set| {
            if !set.remove(&key) {
                set.insert(key);
            }
        });
    };
    let is_open_chevron = move || expanded.get().contains(&id_chevron);
    let is_open_show = move || expanded.get().contains(&id_show);
    let is_selected = move || selected.get().as_deref() == Some(id_selected.as_str());
    let is_selected_aria = move || selected.get().as_deref() == Some(id_selected_aria.as_str());
    let aria_expanded = move || {
        if has_children {
            expanded.get().contains(&id_expanded).to_string()
        } else {
            String::new()
        }
    };

    view! {
        <div
            class="musaic-tree-row"
            class:selected=is_selected
            role="treeitem"
            tabindex="0"
            data-tree-id=id_attr
            data-tree-branch=if has_children { "1" } else { "0" }
            aria-selected=move || is_selected_aria().to_string()
            aria-expanded=aria_expanded
            style=indent
            on:click=on_row
        >
            {if has_children {
                view! {
                    <span class="musaic-tree-chevron" class:open=is_open_chevron on:click=on_chevron>
                        "\u{25b8}"
                    </span>
                }
                    .into_any()
            } else {
                view! { <span class="musaic-tree-spacer"></span> }.into_any()
            }}
            {icon
                .map(|glyph| view! { <span class="musaic-tree-icon">{glyph}</span> })}
            <span class="musaic-tree-label">{label}</span>
        </div>
        {has_children
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
                                        on_select=on_select
                                        selected=selected
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
