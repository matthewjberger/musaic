use leptos::prelude::*;

/// A horizontal progress bar. The reactive `value` is divided by `max`
/// (default `1.0`) and clamped to fill 0% to 100% of the track.
#[component]
pub fn Progress(
    #[prop(into)] value: Signal<f64>,
    #[prop(default = 1.0)] max: f64,
) -> impl IntoView {
    let ceiling = if max <= 0.0 { 1.0 } else { max };
    let percent = move || (value.get() / ceiling).clamp(0.0, 1.0) * 100.0;
    view! {
        <div
            class="musaic-progress"
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax=ceiling.to_string()
            aria-valuenow=move || value.get().to_string()
        >
            <div class="musaic-progress-fill" style=move || format!("width:{}%", percent())></div>
        </div>
    }
}
