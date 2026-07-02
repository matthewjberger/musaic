use leptos::prelude::*;

use crate::pointer_drag::{DragPayload, DragSource, DropZone};

#[derive(Clone)]
pub struct DockTab {
    pub id: String,
    pub title: String,
    pub pane: String,
}

impl DockTab {
    pub fn new(id: impl Into<String>, title: impl Into<String>, pane: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            pane: pane.into(),
        }
    }
}

#[component]
pub fn TabDock<F>(
    tabs: RwSignal<Vec<DockTab>>,
    panes: Vec<String>,
    active: RwSignal<String>,
    render: F,
) -> impl IntoView
where
    F: Fn(String) -> AnyView + 'static,
{
    let render = StoredValue::new_local(render);
    view! {
        <div class="musaic-tabdock">
            {panes
                .into_iter()
                .map(|pane| {
                    let pane_key = StoredValue::new(pane.clone());
                    let tabs_in = move || {
                        tabs.get()
                            .into_iter()
                            .filter(|tab| tab.pane == pane_key.get_value())
                            .collect::<Vec<_>>()
                    };
                    let active_in_pane = move || {
                        let current = active.get();
                        let list = tabs_in();
                        list.iter()
                            .find(|tab| tab.id == current)
                            .map(|tab| tab.id.clone())
                            .or_else(|| list.first().map(|tab| tab.id.clone()))
                    };
                    let on_drop = Callback::new(move |payload: DragPayload| {
                        if payload.kind == "dock-tab" {
                            let target = pane_key.get_value();
                            tabs.update(|list| {
                                if let Some(tab) = list.iter_mut().find(|tab| tab.id == payload.id) {
                                    tab.pane = target;
                                }
                            });
                            active.set(payload.id);
                        }
                    });
                    view! {
                        <DropZone
                            id=format!("pane-{}", pane_key.get_value())
                            on_drop=on_drop
                            class="musaic-tabdock-pane"
                        >
                            <div class="musaic-tabdock-tabs">
                                {move || {
                                    tabs_in()
                                        .into_iter()
                                        .map(|tab| {
                                            let id_click = tab.id.clone();
                                            let id_active = tab.id.clone();
                                            let is_active = move || active.get() == id_active;
                                            view! {
                                                <DragSource
                                                    kind="dock-tab"
                                                    id=tab.id.clone()
                                                    label=tab.title.clone()
                                                    class="musaic-tabdock-tab"
                                                >
                                                    <button
                                                        class="musaic-tabdock-tab-btn"
                                                        class:active=is_active
                                                        on:click=move |_| active.set(id_click.clone())
                                                    >
                                                        {tab.title}
                                                    </button>
                                                </DragSource>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </div>
                            <div class="musaic-tabdock-body">
                                {move || match active_in_pane() {
                                    Some(id) => render.with_value(|render| render(id)),
                                    None => {
                                        view! {
                                            <div class="musaic-tabdock-empty">"Drop a tab here"</div>
                                        }
                                            .into_any()
                                    }
                                }}
                            </div>
                        </DropZone>
                    }
                })
                .collect_view()}
        </div>
    }
}
