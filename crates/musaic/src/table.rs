use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum SortDirection {
    Ascending,
    Descending,
}

#[component]
pub fn Table(
    headers: Vec<String>,
    #[prop(into)] rows: Signal<Vec<Vec<String>>>,
    #[prop(optional)] sortable: bool,
) -> impl IntoView {
    let sort = RwSignal::new(None::<(usize, SortDirection)>);

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
        let mut data = rows.get();
        if let Some((column, direction)) = sort.get() {
            data.sort_by(|left, right| {
                let left = left.get(column).map(String::as_str).unwrap_or("");
                let right = right.get(column).map(String::as_str).unwrap_or("");
                let ordering = match (left.parse::<f64>(), right.parse::<f64>()) {
                    (Ok(left), Ok(right)) => left
                        .partial_cmp(&right)
                        .unwrap_or(std::cmp::Ordering::Equal),
                    _ => left.cmp(right),
                };
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
            let on_click = move |_| {
                if sortable {
                    toggle_sort(index);
                }
            };
            view! {
                <th class=if sortable { "sortable" } else { "" } on:click=on_click>
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
                            .map(|row| {
                                view! {
                                    <tr>
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
