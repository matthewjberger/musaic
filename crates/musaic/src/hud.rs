//! Overlay chrome for a render surface: floating panels and an orientation gizmo.

use leptos::prelude::*;

/// A positioning layer stacked over the render surface to hold HUD elements.
#[component]
pub fn ViewportOverlay(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("musaic-viewport-overlay {class}")>{children()}</div> }
}

/// A floating panel for on-surface controls or readouts.
#[component]
pub fn HudPanel(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("musaic-hud {class}")>{children()}</div> }
}

fn dot(left: [f32; 3], right: [f32; 3]) -> f64 {
    (left[0] * right[0] + left[1] * right[1] + left[2] * right[2]) as f64
}

fn project(basis: [[f32; 3]; 3], world: [f32; 3]) -> (f64, f64, f64) {
    let [right, up, forward] = basis;
    (dot(world, right), -dot(world, up), dot(world, forward))
}

const AXES: [([f32; 3], &str, usize, &str); 6] = [
    ([1.0, 0.0, 0.0], "X", 0, "x"),
    ([-1.0, 0.0, 0.0], "", 1, "x"),
    ([0.0, 1.0, 0.0], "Y", 2, "y"),
    ([0.0, -1.0, 0.0], "", 3, "y"),
    ([0.0, 0.0, 1.0], "Z", 4, "z"),
    ([0.0, 0.0, -1.0], "", 5, "z"),
];

/// An SVG orientation gizmo that projects the camera `basis` (right, up, forward
/// vectors) into six labelled axis dots, depth-sorted so near axes draw on top.
/// Clicking an axis runs `on_axis` with its index.
#[component]
pub fn NavGizmo(
    #[prop(into)] basis: Signal<[[f32; 3]; 3]>,
    #[prop(optional)] on_axis: Option<Callback<usize>>,
) -> impl IntoView {
    let size = 76.0_f64;
    let center = size / 2.0;
    let radius = center - 12.0;
    view! {
        <svg
            class="musaic-nav-gizmo"
            viewBox=format!("0 0 {size} {size}")
            width=size
            height=size
        >
            {move || {
                let current = basis.get();
                let mut projected = AXES
                    .iter()
                    .map(|(vector, label, index, axis)| {
                        let (x, y, depth) = project(current, *vector);
                        (center + x * radius, center + y * radius, depth, *label, *index, *axis, index % 2 == 0)
                    })
                    .collect::<Vec<_>>();
                projected.sort_by(|left, right| {
                    left.2.partial_cmp(&right.2).unwrap_or(std::cmp::Ordering::Equal)
                });
                projected
                    .into_iter()
                    .map(|(px, py, depth, label, index, axis, positive)| {
                        let opacity = 0.35 + 0.65 * ((depth + 1.0) / 2.0);
                        let dot_class = format!(
                            "musaic-gizmo-axis {axis} {}",
                            if positive { "pos" } else { "neg" },
                        );
                        let handle = move |_: web_sys::MouseEvent| {
                            if let Some(callback) = on_axis {
                                callback.run(index);
                            }
                        };
                        view! {
                            <g style=format!("opacity:{opacity:.3}") on:click=handle>
                                <line
                                    class="musaic-gizmo-line"
                                    x1=center
                                    y1=center
                                    x2=px
                                    y2=py
                                />
                                <circle class=dot_class cx=px cy=py r=if positive { 8.0 } else { 5.0 } />
                                <text
                                    class="musaic-gizmo-label"
                                    x=px
                                    y=py
                                    text-anchor="middle"
                                    dominant-baseline="central"
                                >
                                    {label}
                                </text>
                            </g>
                        }
                    })
                    .collect_view()
            }}
        </svg>
    }
}
