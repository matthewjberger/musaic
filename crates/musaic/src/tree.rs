use std::collections::HashSet;

use leptos::prelude::*;

#[derive(Clone)]
pub struct TreeItem {
    pub id: String,
    pub label: String,
    pub children: Vec<TreeItem>,
}

impl TreeItem {
    pub fn leaf(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
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
            children,
        }
    }
}

#[component]
pub fn Tree(
    items: Vec<TreeItem>,
    #[prop(optional)] on_select: Option<Callback<String>>,
) -> impl IntoView {
    let expanded = RwSignal::new(HashSet::<String>::new());
    let on_select = on_select.unwrap_or_else(|| Callback::new(|_| {}));
    view! {
        <div class="musaic-tree">
            {items
                .into_iter()
                .map(|item| {
                    view! { <Branch item=item expanded=expanded on_select=on_select depth=0 /> }
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
    depth: usize,
) -> impl IntoView {
    let has_children = !item.children.is_empty();
    let label = item.label.clone();
    let children = item.children.clone();
    let indent = format!("padding-left:{}px", 6 + depth * 14);

    let id_select = item.id.clone();
    let id_toggle = item.id.clone();
    let id_chevron = item.id.clone();
    let id_show = item.id.clone();

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

    view! {
        <div class="musaic-tree-row" style=indent on:click=on_row>
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
