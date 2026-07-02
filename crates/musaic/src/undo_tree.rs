use leptos::prelude::*;

#[derive(Clone)]
struct Node<T> {
    value: T,
    parent: Option<usize>,
    label: String,
}

pub struct UndoHistory<T: Send + Sync + 'static> {
    nodes: RwSignal<Vec<Node<T>>>,
    current: RwSignal<usize>,
}

impl<T: Send + Sync + 'static> Clone for UndoHistory<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Send + Sync + 'static> Copy for UndoHistory<T> {}

#[derive(Clone)]
pub struct HistoryNode {
    pub id: usize,
    pub label: String,
    pub depth: usize,
    pub current: bool,
}

impl<T: Clone + Send + Sync + 'static> UndoHistory<T> {
    pub fn new(initial: T) -> Self {
        Self {
            nodes: RwSignal::new(vec![Node {
                value: initial,
                parent: None,
                label: "initial".to_string(),
            }]),
            current: RwSignal::new(0),
        }
    }

    pub fn value(&self) -> T {
        let current = self.current.get();
        self.nodes.with(|nodes| nodes[current].value.clone())
    }

    pub fn push(&self, value: T, label: impl Into<String>) {
        let parent = self.current.get_untracked();
        let id = self.nodes.with_untracked(Vec::len);
        self.nodes.update(|nodes| {
            nodes.push(Node {
                value,
                parent: Some(parent),
                label: label.into(),
            });
        });
        self.current.set(id);
    }

    pub fn undo(&self) {
        let parent = self
            .nodes
            .with_untracked(|nodes| nodes[self.current.get_untracked()].parent);
        if let Some(parent) = parent {
            self.current.set(parent);
        }
    }

    pub fn redo(&self) {
        let current = self.current.get_untracked();
        let child = self.nodes.with_untracked(|nodes| {
            nodes
                .iter()
                .enumerate()
                .rev()
                .find(|(_, node)| node.parent == Some(current))
                .map(|(index, _)| index)
        });
        if let Some(child) = child {
            self.current.set(child);
        }
    }

    pub fn restore(&self, id: usize) {
        if self.nodes.with_untracked(|nodes| id < nodes.len()) {
            self.current.set(id);
        }
    }

    pub fn rows(&self) -> Vec<HistoryNode> {
        let current = self.current.get();
        self.nodes.with(|nodes| {
            nodes
                .iter()
                .enumerate()
                .map(|(id, node)| {
                    let mut depth = 0;
                    let mut cursor = node.parent;
                    while let Some(parent) = cursor {
                        depth += 1;
                        cursor = nodes[parent].parent;
                    }
                    HistoryNode {
                        id,
                        label: node.label.clone(),
                        depth,
                        current: id == current,
                    }
                })
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::UndoHistory;
    use leptos::prelude::*;

    #[test]
    fn undo_redo_and_branching_navigate_the_tree() {
        let _owner = Owner::new();
        _owner.set();
        let history = UndoHistory::new("a".to_string());
        history.push("b".to_string(), "b");
        history.push("c".to_string(), "c");
        assert_eq!(history.value(), "c");
        history.undo();
        assert_eq!(history.value(), "b");
        history.undo();
        assert_eq!(history.value(), "a");
        history.redo();
        assert_eq!(history.value(), "b");
        history.push("d".to_string(), "d");
        assert_eq!(history.value(), "d");
        assert_eq!(history.rows().len(), 4);
    }
}

#[component]
pub fn UndoTree(
    #[prop(into)] nodes: Signal<Vec<HistoryNode>>,
    on_restore: Callback<usize>,
) -> impl IntoView {
    view! {
        <div class="musaic-undo-tree">
            {move || {
                nodes
                    .get()
                    .into_iter()
                    .rev()
                    .map(|node| {
                        let id = node.id;
                        let indent = format!("padding-left:{}px", 8 + node.depth * 14);
                        view! {
                            <button
                                class="musaic-undo-node"
                                class:current=node.current
                                style=indent
                                on:click=move |_| on_restore.run(id)
                            >
                                <span class="musaic-undo-dot"></span>
                                <span class="musaic-undo-label">{node.label}</span>
                            </button>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
