use leptos_musaic::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let count = RwSignal::new(0);
    view! {
        <MusaicStyles/>
        <ThemeProvider>
            <Panel title="Hello">
                <ThemePicker/>
                <Button on_click=Callback::new(move |_| count.update(|n| *n += 1))>
                    "clicked " {move || count.get()}
                </Button>
            </Panel>
        </ThemeProvider>
    }
}
