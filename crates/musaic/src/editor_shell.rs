use leptos::prelude::*;

#[component]
pub fn EditorShell(
    #[prop(optional, into)] toolbar: ViewFn,
    #[prop(optional, into)] left: ViewFn,
    #[prop(optional, into)] right: ViewFn,
    #[prop(optional, into)] status: ViewFn,
    #[prop(optional)] left_open: Option<RwSignal<bool>>,
    #[prop(optional)] right_open: Option<RwSignal<bool>>,
    children: Children,
) -> impl IntoView {
    let left_open = left_open.unwrap_or_else(|| RwSignal::new(true));
    let right_open = right_open.unwrap_or_else(|| RwSignal::new(true));
    view! {
        <div class="musaic-editor-shell">
            <div class="musaic-editor-toolbar">{toolbar.run()}</div>
            <aside class="musaic-editor-left" class:closed=move || !left_open.get()>
                {left.run()}
            </aside>
            <main class="musaic-editor-center">{children()}</main>
            <aside class="musaic-editor-right" class:closed=move || !right_open.get()>
                {right.run()}
            </aside>
            <footer class="musaic-editor-status">{status.run()}</footer>
        </div>
    }
}
