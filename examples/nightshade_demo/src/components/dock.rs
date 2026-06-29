use leptos::prelude::*;
use leptos_musaic::{CodeEditor, TabBar, highlight_rhai};

use crate::state::DemoState;

#[component]
pub fn Dock(state: DemoState) -> impl IntoView {
    view! {
        <div class="ed-dock">
            <TabBar
                tabs=vec![("script".into(), "Script".into()), ("log".into(), "Log".into())]
                active=state.dock_tab
            />
            <div class="ed-dock-body">
                <Show
                    when=move || state.dock_tab.get() == "script"
                    fallback=move || view! { <LogView state=state /> }
                >
                    <div class="ed-script">
                        <CodeEditor value=state.script highlighter=highlight_rhai fill=true />
                        <div class="ed-script-note">
                            "Live rhai highlighting via the musaic code-editor. Scratchpad; this demo does not execute it."
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn LogView(state: DemoState) -> impl IntoView {
    view! {
        <div class="ed-log">
            {move || {
                state
                    .log
                    .get()
                    .into_iter()
                    .rev()
                    .map(|line| view! { <div class="ed-log-line">{line}</div> })
                    .collect_view()
            }}
        </div>
    }
}
