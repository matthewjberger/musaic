# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## Unreleased

### Changed

- Renamed the crates to follow the Leptos community convention: `musaic` is now `leptos-musaic`,
  with `leptos-musaic-protocol`, `leptos-musaic-engine`, and `leptos-musaic-shell`. Imports use the
  `leptos_musaic` path. The `musaic` CSS class prefix, design tokens, and `MusaicStyles` keep their
  names.
- Loosened the `wasm-bindgen` dependency from the exact `=0.2.115` pin to `0.2` so downstream crates
  no longer hit version conflicts.
- `NumberField`, `CheckField`, `TextField`, `SliderField`, `ColorField`, and `Select` labels are now
  `String` (accepting owned and dynamic/i18n text) and gained a reactive `disabled` prop.

### Added

- Base primitives, always compiled and fully themeable: `Badge`, `Card`, `Progress`, `Tooltip`.
- `Switch` form control (`role="switch"`), distinct from `CheckField`.
- `NumberField` `min`/`max` bounds and an `integer` mode; inline `help`/`error` text on
  `NumberField` and `TextField`.
- `Table` row selection (`on_row_click`, `selected_row`) that survives sorting, plus `aria-sort`.
- `Tree` selection highlighting, optional per-item icons (`TreeItem::with_icon`), and full arrow-key
  navigation.
- Accessibility pass: focus trap, `Escape`, and focus restoration in `Modal`; `listbox`/`combobox`
  semantics in `CommandPalette`; `treeitem`/`aria-expanded` in `Tree`; `menu`/`menuitem` and
  `tablist`/`tab` roles in menus; theme-accent focus-visible outlines.
- Unit tests for the pure logic (fuzzy matching, hex/RGB conversion, table sort comparator, theme
  resolution) and a `just test` recipe.

### Fixed

- `Viewport` now releases its worker, event listeners, and `ResizeObserver` on unmount via
  `on_cleanup`, and degrades gracefully (optional `on_error` callback) instead of panicking when the
  offscreen-canvas transfer or worker spawn fails.
