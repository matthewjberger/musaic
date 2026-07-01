use leptos::prelude::*;

#[component]
pub fn Disclosure(
    #[prop(into)] title: String,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(default = true)] default_open: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let open = open.unwrap_or_else(|| RwSignal::new(default_open));
    let children = StoredValue::new(children);
    view! {
        <div class="musaic-disclosure">
            <button
                class="musaic-disclosure-header"
                aria-expanded=move || open.get().to_string()
                on:click=move |_| open.update(|value| *value = !*value)
            >
                <span class="musaic-disclosure-caret" class:open=move || open.get()>
                    "\u{25b8}"
                </span>
                <span class="musaic-disclosure-title">{title}</span>
            </button>
            <Show when=move || open.get() fallback=|| ()>
                <div class="musaic-disclosure-body">{children.with_value(|render| render())}</div>
            </Show>
        </div>
    }
}

#[derive(Clone, Copy)]
struct AccordionContext(RwSignal<Option<String>>);

#[component]
pub fn Accordion(
    #[prop(into, optional)] default_open: Option<String>,
    children: Children,
) -> impl IntoView {
    let active = RwSignal::new(default_open);
    provide_context(AccordionContext(active));
    view! { <div class="musaic-accordion">{children()}</div> }
}

#[component]
pub fn AccordionItem(
    #[prop(into)] id: String,
    #[prop(into)] title: String,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_context::<AccordionContext>().map(|context| context.0);
    let id = StoredValue::new(id);
    let children = StoredValue::new(children);
    let is_open = move || {
        context.is_some_and(|active| active.get().as_deref() == Some(id.get_value().as_str()))
    };
    let toggle = move |_| {
        if let Some(active) = context {
            let key = id.get_value();
            active.update(|current| {
                *current = if current.as_deref() == Some(key.as_str()) {
                    None
                } else {
                    Some(key)
                };
            });
        }
    };
    view! {
        <div class="musaic-disclosure">
            <button
                class="musaic-disclosure-header"
                aria-expanded=move || is_open().to_string()
                on:click=toggle
            >
                <span class="musaic-disclosure-caret" class:open=is_open>
                    "\u{25b8}"
                </span>
                <span class="musaic-disclosure-title">{title}</span>
            </button>
            <Show when=is_open fallback=|| ()>
                <div class="musaic-disclosure-body">{children.with_value(|render| render())}</div>
            </Show>
        </div>
    }
}
