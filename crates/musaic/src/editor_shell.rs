use leptos::prelude::*;

use crate::{ResizeAxis, ResizeHandle};

/// A code-editor layout with a top toolbar, collapsible and resizable left,
/// right, and bottom slots around the central `children`, and a status footer.
/// Each `*_open` signal toggles a slot's visibility; providing a matching
/// `*_size` signal makes that slot resizable (and drives its pixel size) via an
/// inserted [`ResizeHandle`].
#[component]
pub fn EditorShell(
    #[prop(optional, into)] toolbar: ViewFn,
    #[prop(optional, into)] left: ViewFn,
    #[prop(optional, into)] right: ViewFn,
    #[prop(optional, into)] bottom: ViewFn,
    #[prop(optional, into)] status: ViewFn,
    #[prop(optional)] left_open: Option<RwSignal<bool>>,
    #[prop(optional)] right_open: Option<RwSignal<bool>>,
    #[prop(optional)] bottom_open: Option<RwSignal<bool>>,
    #[prop(optional)] left_size: Option<RwSignal<f64>>,
    #[prop(optional)] right_size: Option<RwSignal<f64>>,
    #[prop(optional)] bottom_size: Option<RwSignal<f64>>,
    children: Children,
) -> impl IntoView {
    let left_open = left_open.unwrap_or_else(|| RwSignal::new(true));
    let right_open = right_open.unwrap_or_else(|| RwSignal::new(true));
    let bottom_open = bottom_open.unwrap_or_else(|| RwSignal::new(true));

    let left_style = move || match left_size {
        Some(size) if left_open.get() => format!("width:{}px", size.get()),
        _ => String::new(),
    };
    let right_style = move || match right_size {
        Some(size) if right_open.get() => format!("width:{}px", size.get()),
        _ => String::new(),
    };
    let bottom_style = move || match bottom_size {
        Some(size) if bottom_open.get() => format!("height:{}px", size.get()),
        _ => String::new(),
    };

    view! {
        <div class="musaic-editor-shell">
            <div class="musaic-editor-toolbar">{toolbar.run()}</div>
            <div class="musaic-editor-main">
                <aside class="musaic-editor-left" class:closed=move || !left_open.get() style=left_style>
                    {left.run()}
                </aside>
                {left_size
                    .map(|size| {
                        view! {
                            <Show when=move || left_open.get() fallback=|| ()>
                                <ResizeHandle value=size axis=ResizeAxis::Horizontal min=140.0 max=680.0 />
                            </Show>
                        }
                    })}
                <main class="musaic-editor-center">{children()}</main>
                {right_size
                    .map(|size| {
                        view! {
                            <Show when=move || right_open.get() fallback=|| ()>
                                <ResizeHandle
                                    value=size
                                    axis=ResizeAxis::Horizontal
                                    min=180.0
                                    max=680.0
                                    invert=true
                                />
                            </Show>
                        }
                    })}
                <aside
                    class="musaic-editor-right"
                    class:closed=move || !right_open.get()
                    style=right_style
                >
                    {right.run()}
                </aside>
            </div>
            {bottom_size
                .map(|size| {
                    view! {
                        <Show when=move || bottom_open.get() fallback=|| ()>
                            <ResizeHandle
                                value=size
                                axis=ResizeAxis::Vertical
                                min=100.0
                                max=680.0
                                invert=true
                            />
                        </Show>
                    }
                })}
            <section
                class="musaic-editor-bottom"
                class:closed=move || !bottom_open.get()
                style=bottom_style
            >
                {bottom.run()}
            </section>
            <footer class="musaic-editor-status">{status.run()}</footer>
        </div>
    }
}
