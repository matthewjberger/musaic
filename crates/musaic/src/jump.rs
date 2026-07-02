//! A keyboard "jump to" overlay that labels on-screen targets and selects one by typed prefix.

use leptos::prelude::*;

/// A place the jump overlay can send you: a stable `id` reported on selection and
/// the screen position (`x`, `y`, in pixels) where its label is drawn.
#[derive(Clone)]
pub struct JumpTarget {
    /// Identifier passed to the jump callback when this target is chosen.
    pub id: String,
    /// Label position in pixels from the left of the overlay.
    pub x: f64,
    /// Label position in pixels from the top of the overlay.
    pub y: f64,
}

impl JumpTarget {
    /// Creates a target with the given id and pixel position.
    pub fn new(id: impl Into<String>, x: f64, y: f64) -> Self {
        Self {
            id: id.into(),
            x,
            y,
        }
    }
}

fn labels(count: usize) -> Vec<String> {
    let alphabet: Vec<char> = "asdfghjklqwertyuiopzxcvbnm".chars().collect();
    if count <= alphabet.len() {
        alphabet
            .iter()
            .take(count)
            .map(|character| character.to_string())
            .collect()
    } else {
        let mut out = Vec::new();
        'outer: for first in &alphabet {
            for second in &alphabet {
                out.push(format!("{first}{second}"));
                if out.len() >= count {
                    break 'outer;
                }
            }
        }
        out
    }
}

/// A full-screen overlay that assigns short letter labels to each `targets`
/// entry while `open` is set. Typing narrows the labels by prefix; a full match
/// closes the overlay and runs `on_jump` with that target's id, and Escape
/// cancels.
#[component]
pub fn JumpOverlay(
    open: RwSignal<bool>,
    #[prop(into)] targets: Signal<Vec<JumpTarget>>,
    on_jump: Callback<String>,
) -> impl IntoView {
    let typed = RwSignal::new(String::new());

    Effect::new(move |_| {
        if open.get() {
            typed.set(String::new());
        }
    });

    let labelled = move || {
        let list = targets.get();
        let names = labels(list.len());
        list.into_iter()
            .zip(names)
            .map(|(target, label)| (label, target))
            .collect::<Vec<_>>()
    };

    let handle = window_event_listener(leptos::ev::keydown, move |event| {
        if !open.get_untracked() {
            return;
        }
        match event.key().as_str() {
            "Escape" => {
                event.prevent_default();
                open.set(false);
            }
            "Backspace" => {
                event.prevent_default();
                typed.update(|value| {
                    value.pop();
                });
            }
            key if key.len() == 1 && key.chars().all(|character| character.is_alphabetic()) => {
                event.prevent_default();
                let mut next = typed.get_untracked();
                next.push_str(&key.to_lowercase());
                let matched = labelled()
                    .into_iter()
                    .find(|(label, _)| label == &next)
                    .map(|(_, target)| target.id);
                if let Some(id) = matched {
                    open.set(false);
                    typed.set(String::new());
                    on_jump.run(id);
                } else if labelled().iter().any(|(label, _)| label.starts_with(&next)) {
                    typed.set(next);
                } else {
                    typed.set(String::new());
                }
            }
            _ => {}
        }
    });
    on_cleanup(move || handle.remove());

    view! {
        <Show when=move || open.get() fallback=|| ()>
            <div class="musaic-jump-overlay" on:click=move |_| open.set(false)>
                {move || {
                    let prefix = typed.get();
                    labelled()
                        .into_iter()
                        .filter(|(label, _)| label.starts_with(&prefix))
                        .map(|(label, target)| {
                            let (head, tail) = label.split_at(prefix.len());
                            view! {
                                <span
                                    class="musaic-jump-label"
                                    style=format!("left:{}px;top:{}px", target.x, target.y)
                                >
                                    <span class="musaic-jump-typed">{head.to_string()}</span>
                                    {tail.to_string()}
                                </span>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </Show>
    }
}
