# Displaying Data

## Inspector

`Inspector` is the sidebar container; `InspectorSection` is a collapsible, titled group with an
optional actions slot; `InspectorRow` is a labelled row for a single control. This is the idiomatic
home for a panel of form fields.

```rust
view! {
    <Inspector>
        <InspectorSection title="Motion">
            <Switch label="Spin" value=... on_change=.../>
            <SliderField label="Speed" value=... .../>
        </InspectorSection>
        <InspectorSection title="Environment">
            <Select label="Sky" .../>
        </InspectorSection>
    </Inspector>
}
```

## Table

`Table` (the `table` feature) is a data grid over `Signal<Vec<Vec<String>>>` with multi-column sort
(shift-click adds a column), a text filter, resizable and show/hide columns, a sticky header,
pagination, inline cell edit, and optional row virtualization for large sets.

```rust
view! {
    <Table
        headers=vec!["Name".into(), "Kind".into(), "Count".into()]
        rows=Signal::derive(move || data.get())
        sortable=true
        filterable=true
        on_cell_edit=Callback::new(move |(row, col, value)| { /* apply */ })
    />
}
```

## Tree

`Tree` (the `tree` feature) renders a hierarchy of `TreeItem`s (`leaf`, `branch`, `with_icon`,
`lazy`) with arrow-key navigation, F2 inline rename, drag and drop moves, single or multi selection,
and lazy branch expansion. Build the items, pass the callbacks you need (`on_select`, `on_rename`,
`on_move`, `on_expand`), and it handles the interaction.

## Lists

- `VirtualList` (`virtual-list`): a windowed-rendering primitive over any item count and a render
  closure. Use it when a list is long enough that rendering every row hurts.
- `SearchList` (`search-list`): a filterable list with expandable detail and scroll-to-selected.
- `AssetGrid` (`asset-grid`): a searchable thumbnail grid with lazy images.
- `OrderedList` (`list-editor`): a reorderable list with per-row actions.

Pick `Table` for tabular rows, `Tree` for hierarchy, `Inspector` for a property panel, and the list
components for everything else.
