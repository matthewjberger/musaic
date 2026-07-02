use std::cmp::Ordering;

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, PartialEq)]
enum SortDirection {
    Ascending,
    Descending,
}

fn compare_cells(left: &str, right: &str) -> Ordering {
    match (left.parse::<f64>(), right.parse::<f64>()) {
        (Ok(left), Ok(right)) => left.partial_cmp(&right).unwrap_or(Ordering::Equal),
        _ => left.cmp(right),
    }
}

fn compare_rows(left: &[String], right: &[String], sort: &[(usize, SortDirection)]) -> Ordering {
    for (column, direction) in sort {
        let left_cell = left.get(*column).map(String::as_str).unwrap_or("");
        let right_cell = right.get(*column).map(String::as_str).unwrap_or("");
        let ordering = compare_cells(left_cell, right_cell);
        let ordering = match direction {
            SortDirection::Ascending => ordering,
            SortDirection::Descending => ordering.reverse(),
        };
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    Ordering::Equal
}

#[component]
pub fn Table(
    headers: Vec<String>,
    #[prop(into)] rows: Signal<Vec<Vec<String>>>,
    #[prop(optional)] sortable: bool,
    #[prop(optional)] filterable: bool,
    #[prop(optional)] resizable: bool,
    #[prop(optional)] virtualized: bool,
    #[prop(optional)] columns_toggle: bool,
    #[prop(optional)] page_size: Option<usize>,
    #[prop(default = 34.0)] row_height: f64,
    #[prop(default = 12)] overscan: usize,
    #[prop(optional)] height: Option<f64>,
    #[prop(optional)] on_row_click: Option<Callback<usize>>,
    #[prop(optional)] on_cell_edit: Option<Callback<(usize, usize, String)>>,
    #[prop(optional, into)] selected_row: Option<Signal<Option<usize>>>,
) -> impl IntoView {
    let column_count = headers.len();
    let header_labels = StoredValue::new(headers.clone());
    let sort = RwSignal::new(Vec::<(usize, SortDirection)>::new());
    let filter = RwSignal::new(String::new());
    let widths = RwSignal::new(vec![None::<f64>; column_count]);
    let visible = RwSignal::new(vec![true; column_count]);
    let columns_menu = RwSignal::new(false);
    let page = RwSignal::new(0usize);
    let editing = RwSignal::new(None::<(usize, usize)>);
    let draft = RwSignal::new(String::new());
    let scroll_top = RwSignal::new(0.0);
    let viewport_height = RwSignal::new(height.unwrap_or(360.0));
    let selected_row = selected_row.unwrap_or_else(|| Signal::derive(|| None));
    let wrap_ref = NodeRef::<html::Div>::new();
    let drag = StoredValue::new(None::<(usize, f64, f64)>);
    let paginated = page_size.is_some();
    let virtualized = virtualized && !paginated;

    Effect::new(move |_| {
        filter.track();
        page.set(0);
    });

    let cell_input = NodeRef::<html::Input>::new();
    Effect::new(move |_| {
        if editing.get().is_some()
            && let Some(input) = cell_input.get()
        {
            let _ = input.focus();
        }
    });

    let is_visible =
        move |column: usize| visible.with(|list| list.get(column).copied().unwrap_or(true));

    let toggle_sort = move |column: usize, additive: bool| {
        sort.update(|stack| {
            let existing = stack.iter().position(|(active, _)| *active == column);
            if additive {
                match existing {
                    Some(index) => match stack[index].1 {
                        SortDirection::Ascending => {
                            stack[index].1 = SortDirection::Descending;
                        }
                        SortDirection::Descending => {
                            stack.remove(index);
                        }
                    },
                    None => stack.push((column, SortDirection::Ascending)),
                }
            } else {
                *stack = match existing {
                    Some(index) if stack.len() == 1 => match stack[index].1 {
                        SortDirection::Ascending => vec![(column, SortDirection::Descending)],
                        SortDirection::Descending => Vec::new(),
                    },
                    _ => vec![(column, SortDirection::Ascending)],
                };
            }
        });
    };

    let processed = move || {
        let needle = filter.get().to_lowercase();
        let mut data = rows
            .get()
            .into_iter()
            .enumerate()
            .filter(|(_, row)| {
                needle.is_empty() || row.iter().any(|cell| cell.to_lowercase().contains(&needle))
            })
            .collect::<Vec<(usize, Vec<String>)>>();
        let sort = sort.get();
        if !sort.is_empty() {
            data.sort_by(|(_, left), (_, right)| compare_rows(left, right, &sort));
        }
        data
    };

    let page_count = move || {
        let Some(size) = page_size else {
            return 1;
        };
        processed().len().div_ceil(size).max(1)
    };

    let on_pointermove = move |event: web_sys::PointerEvent| {
        if let Some((column, start_x, start_width)) = drag.get_value() {
            let delta = event.client_x() as f64 - start_x;
            let next = (start_width + delta).max(48.0);
            widths.update(|list| {
                if let Some(slot) = list.get_mut(column) {
                    *slot = Some(next);
                }
            });
        }
    };
    let on_pointerup = move |_event: web_sys::PointerEvent| drag.set_value(None);

    let header_cells = (0..column_count)
        .map(|index| {
            let label = header_labels.with_value(|list| list[index].clone());
            let indicator = move || {
                let stack = sort.get();
                match stack.iter().find(|(active, _)| *active == index) {
                    Some((_, SortDirection::Ascending)) => " \u{25b2}".to_string(),
                    Some((_, SortDirection::Descending)) => " \u{25bc}".to_string(),
                    None => String::new(),
                }
            };
            let rank = move || {
                let stack = sort.get();
                if stack.len() > 1
                    && let Some(position) = stack.iter().position(|(active, _)| *active == index)
                {
                    return format!("{}", position + 1);
                }
                String::new()
            };
            let aria_sort = move || {
                let stack = sort.get();
                match stack.iter().find(|(active, _)| *active == index) {
                    Some((_, SortDirection::Ascending)) => "ascending",
                    Some((_, SortDirection::Descending)) => "descending",
                    _ if sortable => "none",
                    _ => "",
                }
            };
            let on_click = move |event: web_sys::MouseEvent| {
                if sortable {
                    toggle_sort(index, event.shift_key());
                }
            };
            let on_grip_down = move |event: web_sys::PointerEvent| {
                event.stop_propagation();
                event.prevent_default();
                let start_width = widths.with_untracked(|list| list.get(index).copied().flatten());
                let start_width = start_width.unwrap_or_else(|| {
                    event
                        .target()
                        .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
                        .and_then(|element| element.parent_element())
                        .map(|parent| parent.get_bounding_client_rect().width())
                        .unwrap_or(120.0)
                });
                drag.set_value(Some((index, event.client_x() as f64, start_width)));
                if let Some(element) = event
                    .target()
                    .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                {
                    let _ = element.set_pointer_capture(event.pointer_id());
                }
            };
            view! {
                <Show when=move || is_visible(index) fallback=|| ()>
                    <th
                        scope="col"
                        class="musaic-th"
                        class:sortable=sortable
                        aria-sort=aria_sort
                        on:click=on_click
                    >
                        <span class="musaic-th-label">{label.clone()}{indicator}</span>
                        <span class="musaic-th-rank">{rank}</span>
                        {resizable
                            .then(|| {
                                view! {
                                    <span
                                        class="musaic-th-grip"
                                        on:pointerdown=on_grip_down
                                        on:pointermove=on_pointermove
                                        on:pointerup=on_pointerup
                                        on:click=|event| event.stop_propagation()
                                    ></span>
                                }
                            })}
                    </th>
                </Show>
            }
        })
        .collect_view();

    let colgroup = move || {
        (0..column_count)
            .map(|index| {
                let width = widths.with(|list| list.get(index).copied().flatten());
                let style = width
                    .map(|value| format!("width:{value}px"))
                    .unwrap_or_default();
                view! {
                    <Show when=move || is_visible(index) fallback=|| ()>
                        <col style=style.clone() />
                    </Show>
                }
            })
            .collect_view()
    };

    let visible_range = move |total: usize| {
        if !virtualized {
            return (0usize, total);
        }
        let view_height = viewport_height.get().max(row_height);
        let first = ((scroll_top.get() / row_height).floor() as usize).saturating_sub(overscan);
        let count = (view_height / row_height).ceil() as usize + overscan * 2 + 1;
        let start = first.min(total);
        let end = (start + count).min(total);
        (start, end)
    };

    let render_cell = move |original_index: usize, column: usize, cell: String| {
        let editable = on_cell_edit.is_some();
        let is_editing = move || editing.get() == Some((original_index, column));
        let cell = StoredValue::new(cell);
        let start_edit = move |_: web_sys::MouseEvent| {
            if editable {
                draft.set(cell.get_value());
                editing.set(Some((original_index, column)));
            }
        };
        let commit = move || {
            if let Some(callback) = on_cell_edit {
                callback.run((original_index, column, draft.get_untracked()));
            }
            editing.set(None);
        };
        view! {
            <Show when=move || is_visible(column) fallback=|| ()>
                <td class:editable=editable on:dblclick=start_edit>
                    {move || {
                        if is_editing() {
                            view! {
                                <input
                                    node_ref=cell_input
                                    class="musaic-cell-input"
                                    prop:value=move || draft.get()
                                    on:click=|event| event.stop_propagation()
                                    on:input=move |event| draft.set(event_target_value(&event))
                                    on:keydown=move |event| {
                                        match event.key().as_str() {
                                            "Enter" => {
                                                event.prevent_default();
                                                commit();
                                            }
                                            "Escape" => {
                                                event.prevent_default();
                                                editing.set(None);
                                            }
                                            _ => {}
                                        }
                                    }
                                    on:blur=move |_| commit()
                                />
                            }
                                .into_any()
                        } else {
                            view! { <span>{cell.get_value()}</span> }.into_any()
                        }
                    }}
                </td>
            </Show>
        }
    };

    let body = move || {
        let data = processed();
        let total = data.len();
        let data = if let Some(size) = page_size {
            let start = (page.get() * size).min(total);
            let end = (start + size).min(total);
            data[start..end].to_vec()
        } else {
            data
        };
        let visible_total = data.len();
        let (start, end) = visible_range(visible_total);
        let top_pad = start as f64 * row_height;
        let bottom_pad = visible_total.saturating_sub(end) as f64 * row_height;
        let visible_rows = data
            .into_iter()
            .skip(start)
            .take(end - start)
            .map(|(original_index, row)| {
                let is_selected = move || selected_row.get() == Some(original_index);
                let on_click = move |_| {
                    if let Some(callback) = on_row_click {
                        callback.run(original_index);
                    }
                };
                let style = virtualized.then(|| format!("height:{row_height}px"));
                let cells = row
                    .into_iter()
                    .enumerate()
                    .map(|(column, cell)| render_cell(original_index, column, cell))
                    .collect_view();
                view! {
                    <tr
                        class:selected=is_selected
                        class:clickable=on_row_click.is_some()
                        style=style
                        aria-selected=move || is_selected().to_string()
                        on:click=on_click
                    >
                        {cells}
                    </tr>
                }
            })
            .collect_view();
        let spacer = move |pixels: f64| {
            (pixels > 0.5).then(|| {
                view! {
                    <tr class="musaic-spacer-row" aria-hidden="true">
                        <td colspan=column_count style=format!("height:{pixels}px;padding:0")></td>
                    </tr>
                }
            })
        };
        view! {
            {spacer(top_pad)}
            {visible_rows}
            {spacer(bottom_pad)}
        }
    };

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

    let wrap_style = height.map(|value| format!("max-height:{value}px;overflow:auto"));

    view! {
        <div class="musaic-table-shell">
            <div class="musaic-table-toolbar">
                {filterable
                    .then(|| {
                        view! {
                            <input
                                class="musaic-table-filter"
                                type="search"
                                placeholder="Filter rows…"
                                prop:value=move || filter.get()
                                on:input=move |event| filter.set(event_target_value(&event))
                            />
                        }
                    })}
                {columns_toggle
                    .then(|| {
                        view! {
                            <div class="musaic-table-columns">
                                <button
                                    class="musaic-button"
                                    on:click=move |_| columns_menu.update(|open| *open = !*open)
                                >
                                    "Columns"
                                </button>
                                <Show when=move || columns_menu.get() fallback=|| ()>
                                    <div class="musaic-table-columns-menu">
                                        {(0..column_count)
                                            .map(|index| {
                                                let label = header_labels
                                                    .with_value(|list| list[index].clone());
                                                view! {
                                                    <label class="musaic-table-column-row">
                                                        <input
                                                            type="checkbox"
                                                            prop:checked=move || is_visible(index)
                                                            on:change=move |_| {
                                                                visible
                                                                    .update(|list| {
                                                                        if let Some(slot) = list.get_mut(index) {
                                                                            *slot = !*slot;
                                                                        }
                                                                    })
                                                            }
                                                        />
                                                        {label}
                                                    </label>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </Show>
                            </div>
                        }
                    })}
            </div>
            <div
                class="musaic-table-wrap"
                class:sticky=virtualized || height.is_some()
                node_ref=wrap_ref
                style=wrap_style
                on:scroll=on_scroll
            >
                <table class="musaic-table">
                    <colgroup>{colgroup}</colgroup>
                    <thead>
                        <tr>{header_cells}</tr>
                    </thead>
                    <tbody>{body}</tbody>
                </table>
            </div>
            {paginated
                .then(|| {
                    view! {
                        <div class="musaic-table-pager">
                            <button
                                class="musaic-button"
                                disabled=move || page.get() == 0
                                on:click=move |_| page.update(|value| *value = value.saturating_sub(1))
                            >
                                "Prev"
                            </button>
                            <span class="musaic-table-page">
                                {move || format!("Page {} / {}", page.get() + 1, page_count())}
                            </span>
                            <button
                                class="musaic-button"
                                disabled=move || page.get() + 1 >= page_count()
                                on:click=move |_| {
                                    let last = page_count() - 1;
                                    page.update(|value| *value = (*value + 1).min(last))
                                }
                            >
                                "Next"
                            </button>
                        </div>
                    }
                })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{SortDirection, compare_cells, compare_rows};
    use std::cmp::Ordering;

    #[test]
    fn numeric_cells_compare_by_value_not_lexically() {
        assert_eq!(compare_cells("9", "10"), Ordering::Less);
        assert_eq!(compare_cells("100", "20"), Ordering::Greater);
    }

    #[test]
    fn non_numeric_cells_compare_lexically() {
        assert_eq!(compare_cells("alpha", "beta"), Ordering::Less);
        assert_eq!(compare_cells("zeta", "alpha"), Ordering::Greater);
    }

    #[test]
    fn multi_column_sort_breaks_ties_with_secondary_key() {
        let left = vec!["group".to_string(), "1".to_string()];
        let right = vec!["group".to_string(), "2".to_string()];
        let sort = vec![
            (0, SortDirection::Ascending),
            (1, SortDirection::Descending),
        ];
        assert_eq!(compare_rows(&left, &right, &sort), Ordering::Greater);
    }
}
