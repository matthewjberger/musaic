//! Floating overlay components: popovers, dropdowns, comboboxes, and dialogs that anchor
//! to a trigger and reposition to stay within the viewport.

use leptos::html;
use leptos::prelude::*;

use crate::base::Overlay;

/// The preferred side of the anchor on which to place a floating element. Placement may
/// flip to the opposite side when there is not enough room.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

/// Alignment of a floating element along the anchor's cross axis: to its start, centered,
/// or to its end.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy)]
struct Rect {
    left: f64,
    top: f64,
    width: f64,
    height: f64,
}

const MARGIN: f64 = 8.0;

fn align_main(anchor_start: f64, anchor_size: f64, floating_size: f64, align: Align) -> f64 {
    match align {
        Align::Start => anchor_start,
        Align::Center => anchor_start + (anchor_size - floating_size) / 2.0,
        Align::End => anchor_start + anchor_size - floating_size,
    }
}

fn resolve(
    anchor: Rect,
    floating: (f64, f64),
    side: Side,
    align: Align,
    offset: f64,
    viewport: (f64, f64),
) -> (f64, f64) {
    let (floating_width, floating_height) = floating;
    let (viewport_width, viewport_height) = viewport;
    let fits_below = anchor.top + anchor.height + offset + floating_height <= viewport_height;
    let fits_above = anchor.top - offset - floating_height >= MARGIN;
    let fits_right = anchor.left + anchor.width + offset + floating_width <= viewport_width;
    let fits_left = anchor.left - offset - floating_width >= MARGIN;

    let side = match side {
        Side::Bottom if !fits_below && fits_above => Side::Top,
        Side::Top if !fits_above && fits_below => Side::Bottom,
        Side::Right if !fits_right && fits_left => Side::Left,
        Side::Left if !fits_left && fits_right => Side::Right,
        other => other,
    };

    let (left, top) = match side {
        Side::Bottom => (
            align_main(anchor.left, anchor.width, floating_width, align),
            anchor.top + anchor.height + offset,
        ),
        Side::Top => (
            align_main(anchor.left, anchor.width, floating_width, align),
            anchor.top - offset - floating_height,
        ),
        Side::Right => (
            anchor.left + anchor.width + offset,
            align_main(anchor.top, anchor.height, floating_height, align),
        ),
        Side::Left => (
            anchor.left - offset - floating_width,
            align_main(anchor.top, anchor.height, floating_height, align),
        ),
    };

    let max_left = (viewport_width - floating_width - MARGIN).max(MARGIN);
    let max_top = (viewport_height - floating_height - MARGIN).max(MARGIN);
    (left.clamp(MARGIN, max_left), top.clamp(MARGIN, max_top))
}

fn viewport() -> (f64, f64) {
    let window = web_sys::window();
    let width = window
        .as_ref()
        .and_then(|window| window.inner_width().ok())
        .and_then(|value| value.as_f64())
        .unwrap_or(1280.0);
    let height = window
        .as_ref()
        .and_then(|window| window.inner_height().ok())
        .and_then(|value| value.as_f64())
        .unwrap_or(800.0);
    (width, height)
}

/// A floating panel anchored to a `trigger` element, toggled by the `open` signal and by
/// clicking the trigger. It repositions on scroll and resize to stay in the viewport,
/// flipping `side` and shifting as needed, and dismisses on outside click via `on_dismiss`.
#[component]
pub fn Popover(
    open: RwSignal<bool>,
    #[prop(into)] trigger: ViewFn,
    #[prop(default = Side::Bottom)] side: Side,
    #[prop(default = Align::Start)] align: Align,
    #[prop(default = 8.0)] offset: f64,
    #[prop(optional)] on_dismiss: Option<Callback<()>>,
    children: ChildrenFn,
) -> impl IntoView {
    let anchor_ref = NodeRef::<html::Span>::new();
    let content_ref = NodeRef::<html::Div>::new();
    let position = RwSignal::new((0.0_f64, 0.0_f64));
    let children = StoredValue::new(children);

    let reposition = move || {
        if let (Some(anchor), Some(content)) = (anchor_ref.get(), content_ref.get()) {
            let anchor_rect = anchor.get_bounding_client_rect();
            let content_rect = content.get_bounding_client_rect();
            let (viewport_width, viewport_height) = viewport();
            let placed = resolve(
                Rect {
                    left: anchor_rect.left(),
                    top: anchor_rect.top(),
                    width: anchor_rect.width(),
                    height: anchor_rect.height(),
                },
                (content_rect.width(), content_rect.height()),
                side,
                align,
                offset,
                (viewport_width, viewport_height),
            );
            position.set(placed);
        }
    };

    Effect::new(move |_| {
        if open.get() {
            reposition();
        }
    });

    let scroll_handle = window_event_listener(leptos::ev::scroll, move |_| {
        if open.get_untracked() {
            reposition();
        }
    });
    let resize_handle = window_event_listener(leptos::ev::resize, move |_| {
        if open.get_untracked() {
            reposition();
        }
    });
    on_cleanup(move || {
        scroll_handle.remove();
        resize_handle.remove();
    });

    let dismiss = move || {
        open.set(false);
        if let Some(callback) = on_dismiss {
            callback.run(());
        }
    };

    view! {
        <span
            class="musaic-popover-anchor"
            node_ref=anchor_ref
            on:click=move |_| open.update(|value| *value = !*value)
        >
            {trigger.run()}
        </span>
        <Show when=move || open.get() fallback=|| ()>
            <Overlay>
                <div
                    class="musaic-popover-catcher"
                    on:pointerdown=move |_| dismiss()
                ></div>
                <div
                    class="musaic-popover"
                    node_ref=content_ref
                    style=move || {
                        let (left, top) = position.get();
                        format!("left:{left}px;top:{top}px")
                    }
                    on:pointerdown=|event| event.stop_propagation()
                >
                    {children.with_value(|render| render())}
                </div>
            </Overlay>
        </Show>
    }
}

/// A button labelled `label` that opens a [`Popover`] containing a menu of `children`.
/// The menu closes when any item is clicked.
#[component]
pub fn Dropdown(
    #[prop(into)] label: String,
    #[prop(into, optional)] class: String,
    #[prop(default = Side::Bottom)] side: Side,
    #[prop(default = Align::Start)] align: Align,
    children: ChildrenFn,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let label = StoredValue::new(label);
    let class = StoredValue::new(class);
    let children = StoredValue::new(children);
    view! {
        <Popover
            open=open
            side=side
            align=align
            trigger=move || {
                view! {
                    <button
                        class=format!("musaic-button {}", class.get_value())
                        aria-haspopup="menu"
                    >
                        {label.get_value()}
                    </button>
                }
            }
        >
            <div class="musaic-menu-list" role="menu" on:click=move |_| open.set(false)>
                {children.with_value(|render| render())}
            </div>
        </Popover>
    }
}

/// A selectable option for [`Combobox`], pairing a stored `value` with a display `label`.
#[derive(Clone)]
pub struct ComboOption {
    /// The value emitted when this option is chosen.
    pub value: String,
    /// The text shown to the user.
    pub label: String,
}

impl ComboOption {
    /// Creates a [`ComboOption`] from a value and its display label.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// A filterable dropdown selector: the trigger shows the label of the current `value` (or
/// `placeholder`), and the panel offers a text filter plus a keyboard-navigable list of
/// `options`. Emits the chosen option's value through `on_select`.
#[component]
pub fn Combobox(
    #[prop(into)] value: Signal<String>,
    options: Vec<ComboOption>,
    on_select: Callback<String>,
    #[prop(into, optional)] placeholder: String,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let query = RwSignal::new(String::new());
    let active = RwSignal::new(0usize);
    let options = StoredValue::new(options);
    let placeholder = StoredValue::new(if placeholder.is_empty() {
        "Select…".to_string()
    } else {
        placeholder
    });

    let selected_label = move || {
        let current = value.get();
        options.with_value(|list| {
            list.iter()
                .find(|option| option.value == current)
                .map(|option| option.label.clone())
                .unwrap_or_default()
        })
    };

    let filtered = move || {
        let needle = query.get().to_lowercase();
        options.with_value(|list| {
            list.iter()
                .filter(|option| needle.is_empty() || option.label.to_lowercase().contains(&needle))
                .cloned()
                .collect::<Vec<_>>()
        })
    };

    let choose = move |option: ComboOption| {
        on_select.run(option.value);
        open.set(false);
        query.set(String::new());
    };

    let on_key = move |event: web_sys::KeyboardEvent| match event.key().as_str() {
        "ArrowDown" => {
            event.prevent_default();
            let count = filtered().len();
            if count > 0 {
                active.update(|index| *index = (*index + 1) % count);
            }
        }
        "ArrowUp" => {
            event.prevent_default();
            let count = filtered().len();
            if count > 0 {
                active.update(|index| *index = (*index + count - 1) % count);
            }
        }
        "Enter" => {
            event.prevent_default();
            if let Some(option) = filtered().into_iter().nth(active.get()) {
                choose(option);
            }
        }
        "Escape" => open.set(false),
        _ => {}
    };

    view! {
        <Popover
            open=open
            align=Align::Start
            trigger=move || {
                view! {
                    <button class="musaic-combobox-trigger" type="button">
                        <span class="musaic-combobox-value">
                            {move || {
                                let label = selected_label();
                                if label.is_empty() { placeholder.get_value() } else { label }
                            }}
                        </span>
                        <span class="musaic-combobox-caret">"\u{25be}"</span>
                    </button>
                }
            }
        >
            <div class="musaic-combobox-panel">
                <input
                    class="musaic-combobox-input"
                    type="text"
                    placeholder="Filter…"
                    prop:value=move || query.get()
                    on:input=move |event| {
                        query.set(event_target_value(&event));
                        active.set(0);
                    }
                    on:keydown=on_key
                />
                <div class="musaic-combobox-list" role="listbox">
                    {move || {
                        let rows = filtered();
                        if rows.is_empty() {
                            return view! {
                                <div class="musaic-combobox-empty">"No matches"</div>
                            }
                                .into_any();
                        }
                        rows.into_iter()
                            .enumerate()
                            .map(|(index, option)| {
                                let is_active = move || active.get() == index;
                                let chosen = option.clone();
                                view! {
                                    <div
                                        class="musaic-combobox-option"
                                        class:active=is_active
                                        role="option"
                                        aria-selected=move || is_active().to_string()
                                        on:pointerdown=move |event| {
                                            event.prevent_default();
                                            choose(chosen.clone());
                                        }
                                    >
                                        {option.label}
                                    </div>
                                }
                            })
                            .collect_view()
                            .into_any()
                    }}
                </div>
            </div>
        </Popover>
    }
}

/// A modal dialog with a title, body `children`, and cancel/confirm buttons. Toggled by
/// the `open` signal; both buttons close it and run the optional `on_cancel`/`on_confirm`
/// callbacks. The confirm button uses danger styling when `danger` is set.
#[component]
pub fn Dialog(
    open: RwSignal<bool>,
    #[prop(into)] title: String,
    #[prop(into, optional)] confirm_label: String,
    #[prop(into, optional)] cancel_label: String,
    #[prop(optional)] danger: bool,
    #[prop(optional)] on_confirm: Option<Callback<()>>,
    #[prop(optional)] on_cancel: Option<Callback<()>>,
    children: ChildrenFn,
) -> impl IntoView {
    let title = StoredValue::new(title);
    let children = StoredValue::new(children);
    let confirm_label = if confirm_label.is_empty() {
        "Confirm".to_string()
    } else {
        confirm_label
    };
    let cancel_label = if cancel_label.is_empty() {
        "Cancel".to_string()
    } else {
        cancel_label
    };
    let cancel = move || {
        open.set(false);
        if let Some(callback) = on_cancel {
            callback.run(());
        }
    };
    let confirm = move || {
        open.set(false);
        if let Some(callback) = on_confirm {
            callback.run(());
        }
    };
    view! {
        <crate::base::Modal open=open>
            <div class="musaic-dialog">
                <div class="musaic-dialog-title">{move || title.get_value()}</div>
                <div class="musaic-dialog-body">{children.with_value(|render| render())}</div>
                <div class="musaic-dialog-actions">
                    <button class="musaic-button" on:click=move |_| cancel()>
                        {cancel_label.clone()}
                    </button>
                    <button
                        class=if danger { "musaic-button danger" } else { "musaic-button primary" }
                        on:click=move |_| confirm()
                    >
                        {confirm_label.clone()}
                    </button>
                </div>
            </div>
        </crate::base::Modal>
    }
}

#[cfg(test)]
mod tests {
    use super::{Align, Rect, Side, resolve};

    fn anchor() -> Rect {
        Rect {
            left: 100.0,
            top: 100.0,
            width: 80.0,
            height: 30.0,
        }
    }

    #[test]
    fn bottom_start_places_below_and_left_aligned() {
        let (left, top) = resolve(
            anchor(),
            (120.0, 60.0),
            Side::Bottom,
            Align::Start,
            8.0,
            (1000.0, 1000.0),
        );
        assert_eq!(left, 100.0);
        assert_eq!(top, 138.0);
    }

    #[test]
    fn flips_to_top_when_no_room_below() {
        let tall_anchor = Rect {
            left: 100.0,
            top: 950.0,
            width: 80.0,
            height: 30.0,
        };
        let (_, top) = resolve(
            tall_anchor,
            (120.0, 60.0),
            Side::Bottom,
            Align::Start,
            8.0,
            (1000.0, 1000.0),
        );
        assert!(top < 950.0);
    }

    #[test]
    fn shifts_to_stay_within_viewport() {
        let edge_anchor = Rect {
            left: 960.0,
            top: 100.0,
            width: 80.0,
            height: 30.0,
        };
        let (left, _) = resolve(
            edge_anchor,
            (200.0, 60.0),
            Side::Bottom,
            Align::Start,
            8.0,
            (1000.0, 1000.0),
        );
        assert!(left + 200.0 <= 1000.0);
    }
}
