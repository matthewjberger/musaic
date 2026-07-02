# Feature Gates

Every component lives behind a Cargo feature so you compile only what you use. If a component type
does not resolve, or a component renders nothing, the first thing to check is whether its feature is
enabled.

## Defaults and `full`

```toml
# default = ["forms", "menus", "themes"]
leptos-musaic = { git = "...", features = ["forms", "themes", "command-palette"] }
# or everything:
leptos-musaic = { git = "...", features = ["full"] }
```

The base layer is always compiled and needs no feature: the design tokens and typed theming,
`Button`, `IconButton`, `Panel`, `Card`, the layout primitives (`Row`, `Column`, `Grid`,
`AppShell`, `ResizeHandle`), `Overlay` / `Modal`, `Badge`, `Progress`, `Tooltip`, `Spinner`, the
toasts, the `CommandRegistry`, the `KeymapProvider`, the `EditorShell` frame, and the
`use_persisted` / `use_reconnecting_socket` / `download_text` / `pick_file_text` browser helpers.

## The feature map

Each feature and its components are listed in the `README.md` table. The frequently used ones:

| feature | gives you |
| --- | --- |
| `forms` | `NumberField`, `Vec3Field`, `CheckField`, `Switch`, `TextField`, `SliderField`, `ColorField`, `Select`, chips, tags, swatches |
| `menus` | `Menu`, `MenuItem`, `Submenu`, `ContextMenu`, `TabBar` |
| `themes` | bundled themes, `register_theme`, `ThemePicker`, `ThemeMenu` |
| `command-palette` | `CommandPalette` |
| `code-editor` | `CodeEditor`, `CodeTabs`, `highlight_code` |
| `table` / `tree` / `inspector` | `Table`, `Tree`, `Inspector` |
| `overlays` | `Popover`, `Dropdown`, `Combobox`, `Dialog` |
| `toolbar` / `status-bar` | `Toolbar` family, `ActivityBar`; `StatusBar`, `StatusItem` |
| `log` | `LogView` |
| `drag` / `workspace` | pointer drag and drop; `TabDock` tab tear-off |
| `code-surface` | `CodeSurface`, `MultiEditor` |
| `terminal` | `Terminal`, `AnsiTerminal` |
| `virtual-list` / `diff` / `disclosure` / `markdown` / `search-list` / `asset-grid` / `list-editor` / `chat` / `dynamic-form` / `undo-tree` / `jump` / `dock` | one component family each, see the README table |
| `viewport` / `engine` | a worker-backed rendering surface and its turnkey wiring |
| `protocol` | leptos-free `serde` wire types (no DOM) |

See the [Component Catalog](catalog.md) for a one-line description of each.

## How features compose

Some features imply others (`command-palette` implies `menus`, `workspace` implies `drag`,
`viewport` implies `protocol`, and so on), so you rarely list more than the top-level ones you want.

Internally every UI feature turns on a private `_dom` feature that activates the Leptos and web-sys
dependency stack. That is why enabling `protocol` alone pulls no DOM code: it is the one feature that
does not turn on `_dom`. You will not reference `_dom` directly; it is an implementation detail that
keeps the `protocol`-only build lean.
