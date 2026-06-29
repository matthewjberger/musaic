use std::cmp::Ordering;

use leptos::prelude::*;

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

#[component]
pub fn Table(
    headers: Vec<String>,
    #[prop(into)] rows: Signal<Vec<Vec<String>>>,
    #[prop(optional)] sortable: bool,
    #[prop(optional)] on_row_click: Option<Callback<usize>>,
    #[prop(optional, into)] selected_row: Option<Signal<Option<usize>>>,
) -> impl IntoView {
    let sort = RwSignal::new(None::<(usize, SortDirection)>);
    let selected_row = selected_row.unwrap_or_else(|| Signal::derive(|| None));

    let toggle_sort = move |column: usize| {
        sort.update(|current| {
            *current = match *current {
                Some((active, SortDirection::Ascending)) if active == column => {
                    Some((column, SortDirection::Descending))
                }
                Some((active, SortDirection::Descending)) if active == column => None,
                _ => Some((column, SortDirection::Ascending)),
            };
        });
    };

    let sorted_rows = move || {
        let mut data = rows
            .get()
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, Vec<String>)>>();
        if let Some((column, direction)) = sort.get() {
            data.sort_by(|(_, left), (_, right)| {
                let left = left.get(column).map(String::as_str).unwrap_or("");
                let right = right.get(column).map(String::as_str).unwrap_or("");
                let ordering = compare_cells(left, right);
                match direction {
                    SortDirection::Ascending => ordering,
                    SortDirection::Descending => ordering.reverse(),
                }
            });
        }
        data
    };

    let header_cells = headers
        .into_iter()
        .enumerate()
        .map(|(index, label)| {
            let indicator = move || match sort.get() {
                Some((active, SortDirection::Ascending)) if active == index => " \u{25b2}",
                Some((active, SortDirection::Descending)) if active == index => " \u{25bc}",
                _ => "",
            };
            let aria_sort = move || match sort.get() {
                Some((active, SortDirection::Ascending)) if active == index => "ascending",
                Some((active, SortDirection::Descending)) if active == index => "descending",
                _ if sortable => "none",
                _ => "",
            };
            let on_click = move |_| {
                if sortable {
                    toggle_sort(index);
                }
            };
            view! {
                <th
                    scope="col"
                    class=if sortable { "sortable" } else { "" }
                    aria-sort=aria_sort
                    on:click=on_click
                >
                    {label}
                    {indicator}
                </th>
            }
        })
        .collect_view();

    view! {
        <div class="musaic-table-wrap">
            <table class="musaic-table">
                <thead>
                    <tr>{header_cells}</tr>
                </thead>
                <tbody>
                    {move || {
                        sorted_rows()
                            .into_iter()
                            .map(|(original_index, row)| {
                                let is_selected = move || {
                                    selected_row.get() == Some(original_index)
                                };
                                let on_click = move |_| {
                                    if let Some(callback) = on_row_click {
                                        callback.run(original_index);
                                    }
                                };
                                view! {
                                    <tr
                                        class:selected=is_selected
                                        class:clickable=on_row_click.is_some()
                                        aria-selected=move || is_selected().to_string()
                                        on:click=on_click
                                    >
                                        {row
                                            .into_iter()
                                            .map(|cell| view! { <td>{cell}</td> })
                                            .collect_view()}
                                    </tr>
                                }
                            })
                            .collect_view()
                    }}
                </tbody>
            </table>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::compare_cells;
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
}
