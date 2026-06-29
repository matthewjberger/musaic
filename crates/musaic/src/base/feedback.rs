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
}

#[derive(Clone, Copy)]
pub struct Toaster {
    toasts: RwSignal<Vec<Toast>>,
    next: RwSignal<usize>,
}

impl Toaster {
    pub fn info(&self, text: impl Into<String>) {
        self.push(text.into(), "info");
    }

    pub fn error(&self, text: impl Into<String>) {
        self.push(text.into(), "error");
    }

    fn push(&self, text: String, kind: &'static str) {
        let id = self.next.get_untracked();
        self.next.set(id + 1);
        self.toasts
            .update(|list| list.push(Toast { id, text, kind }));
        let toasts = self.toasts;
        after(3200, move || {
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
                    view! {
                        <div
                            class=format!("musaic-toast {}", toast.kind)
                            on:click=move |_| toaster.dismiss(id)
                        >
                            {toast.text.clone()}
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
