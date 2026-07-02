//! A windowed list that renders only the rows currently in view.

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::visible_range;

/// A virtualized list that renders only the visible slice of `count` items,
/// calling `render` with each item index. `item_height` (fixed row height),
/// `height` (viewport height), and `overscan` (extra rows rendered off-screen)
/// tune the windowing.
#[component]
pub fn VirtualList<F>(
    #[prop(into)] count: Signal<usize>,
    #[prop(default = 32.0)] item_height: f64,
    #[prop(default = 360.0)] height: f64,
    #[prop(default = 8)] overscan: usize,
    render: F,
) -> impl IntoView
where
    F: Fn(usize) -> AnyView + 'static,
{
    let scroll_top = RwSignal::new(0.0);
    let viewport_height = RwSignal::new(height);
    let render = StoredValue::new_local(render);
    let wrap_ref = NodeRef::<html::Div>::new();

    let on_scroll = move |event: web_sys::Event| {
        if let Some(element) = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
        {
            scroll_top.set(element.scroll_top() as f64);
            viewport_height.set(element.client_height() as f64);
        }
    };

    Effect::new(move |_| {
        if let Some(element) = wrap_ref.get() {
            viewport_height.set(element.client_height() as f64);
        }
    });

    view! {
        <div
            class="musaic-virtual-list"
            node_ref=wrap_ref
            style=format!("height:{height}px")
            on:scroll=on_scroll
        >
            {move || {
                let total = count.get();
                let (start, end) = visible_range(
                    scroll_top.get(),
                    viewport_height.get(),
                    item_height,
                    overscan,
                    total,
                );
                let top_pad = start as f64 * item_height;
                let bottom_pad = total.saturating_sub(end) as f64 * item_height;
                let rows = (start..end)
                    .map(|index| {
                        let content = render.with_value(|render| render(index));
                        view! {
                            <div
                                class="musaic-virtual-row"
                                style=format!("height:{item_height}px")
                            >
                                {content}
                            </div>
                        }
                    })
                    .collect_view();
                view! {
                    <div style=format!("height:{top_pad}px")></div>
                    {rows}
                    <div style=format!("height:{bottom_pad}px")></div>
                }
            }}
        </div>
    }
}
