use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

#[component]
pub fn Spinner() -> impl IntoView {
    view! { <span class="musaic-spinner"></span> }
}

#[derive(Clone)]
pub struct Toast {
    pub id: usize,
    pub text: String,
    pub kind: &'static str,
    pub action: Option<(String, Callback<()>)>,
}

#[derive(Clone, Copy)]
pub struct Toaster {
    toasts: RwSignal<Vec<Toast>>,
    next: RwSignal<usize>,
}

impl Toaster {
    pub fn info(&self, text: impl Into<String>) {
        self.push(text.into(), "info", 3200, None);
    }

    pub fn success(&self, text: impl Into<String>) {
        self.push(text.into(), "success", 3200, None);
    }

    pub fn warning(&self, text: impl Into<String>) {
        self.push(text.into(), "warn", 4200, None);
    }

    pub fn error(&self, text: impl Into<String>) {
        self.push(text.into(), "error", 5000, None);
    }

    pub fn action(
        &self,
        text: impl Into<String>,
        label: impl Into<String>,
        on_action: Callback<()>,
    ) {
        self.push(text.into(), "info", 6000, Some((label.into(), on_action)));
    }

    fn push(
        &self,
        text: String,
        kind: &'static str,
        duration_ms: i32,
        action: Option<(String, Callback<()>)>,
    ) {
        let id = self.next.get_untracked();
        self.next.set(id + 1);
        self.toasts.update(|list| {
            list.push(Toast {
                id,
                text,
                kind,
                action,
            })
        });
        let toasts = self.toasts;
        after(duration_ms, move || {
            toasts.update(|list| list.retain(|toast| toast.id != id));
        });
    }

    fn dismiss(&self, id: usize) {
        self.toasts
            .update(|list| list.retain(|toast| toast.id != id));
    }
}

pub fn use_toaster() -> Toaster {
    use_context::<Toaster>().unwrap_or_else(|| Toaster {
        toasts: RwSignal::new(Vec::new()),
        next: RwSignal::new(0),
    })
}

#[component]
pub fn ToastHub(children: Children) -> impl IntoView {
    let toaster = Toaster {
        toasts: RwSignal::new(Vec::new()),
        next: RwSignal::new(0),
    };
    provide_context(toaster);
    view! {
        {children()}
        <div class="musaic-toast-stack">
            <For each=move || toaster.toasts.get() key=|toast| toast.id let:toast>
                {
                    let id = toast.id;
                    let action = toast.action.clone();
                    view! {
                        <div class=format!("musaic-toast {}", toast.kind)>
                            <span class="musaic-toast-text">{toast.text.clone()}</span>
                            {action
                                .map(|(label, callback)| {
                                    view! {
                                        <button
                                            class="musaic-toast-action"
                                            on:click=move |_| {
                                                callback.run(());
                                                toaster.dismiss(id);
                                            }
                                        >
                                            {label}
                                        </button>
                                    }
                                })}
                            <button
                                class="musaic-toast-close"
                                aria-label="Dismiss"
                                on:click=move |_| toaster.dismiss(id)
                            >
                                "\u{00d7}"
                            </button>
                        </div>
                    }
                }
            </For>
        </div>
    }
}

fn after(milliseconds: i32, callback: impl FnOnce() + 'static) {
    let closure = Closure::once_into_js(callback);
    if let Some(window) = web_sys::window() {
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.unchecked_ref(),
            milliseconds,
        );
    }
}
