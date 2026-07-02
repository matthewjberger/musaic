//! A property-inspector panel built from collapsible sections and labeled rows.

use leptos::prelude::*;

/// The outer container for an inspector panel; wraps its `children` sections.
#[component]
pub fn Inspector(children: Children) -> impl IntoView {
    view! { <div class="musaic-inspector">{children()}</div> }
}

/// A collapsible inspector section with a `title` header, optional `actions`
/// rendered on the header row, and a body of `children`. `default_open`
/// controls the initial expanded state.
#[component]
pub fn InspectorSection(
    #[prop(into)] title: String,
    #[prop(default = true)] default_open: bool,
    #[prop(optional, into)] actions: ViewFn,
    children: ChildrenFn,
) -> impl IntoView {
    let open = RwSignal::new(default_open);
    view! {
        <div class="musaic-inspector-section">
            <div class="musaic-inspector-headrow">
                <button
                    class="musaic-inspector-header"
                    on:click=move |_| open.update(|value| *value = !*value)
                >
                    <span class="musaic-inspector-caret" class:open=move || open.get()>
                        "\u{25b8}"
                    </span>
                    {title}
                </button>
                <div class="musaic-inspector-actions">{actions.run()}</div>
            </div>
            <Show when=move || open.get() fallback=|| ()>
                <div class="musaic-inspector-body">{children()}</div>
            </Show>
        </div>
    }
}

/// A labeled inspector row pairing a `label` with a control rendered from
/// `children`.
#[component]
pub fn InspectorRow(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! {
        <div class="musaic-inspector-row">
            <span class="musaic-inspector-label">{label}</span>
            <div class="musaic-inspector-control">{children()}</div>
        </div>
    }
}
