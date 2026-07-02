# Component Catalog

Every component, by feature. Types resolve only when their feature is enabled (the base layer needs
no feature). For live examples, open the matching page in `examples/gallery/src/sections.rs`.

## Base layer (always compiled)

- `Button`, `IconButton`: themed buttons.
- `Panel`, `Card`: titled containers.
- `Row`, `Column`, `Grid`, `AppShell`: layout primitives.
- `ResizeHandle`: turns a `RwSignal<f64>` into a draggable splitter.
- `EditorShell`: the toolbar/left/center/right/bottom/status app frame.
- `Overlay`, `Modal`, `Scrim`: portalled overlays; `Modal` traps focus and closes on Escape.
- `Badge`, `Progress`, `Spinner`, `Tooltip`: small indicators.
- `Toaster` / `ToastHub` / `use_toaster`: transient notifications.
- `CommandRegistry` / `provide_command_registry` / `use_commands` / `Command`: the action registry.
- `KeymapProvider`, `pretty_binding`: global keybinding dispatch with which-key hints.
- `use_persisted`, `use_reconnecting_socket`, `download_text`, `pick_file_text`: browser helpers.

## Feature-gated

| feature | components |
| --- | --- |
| `forms` | `NumberField`, `Vec3Field`, `CheckField`, `Switch`, `TextField`, `SliderField`, `ColorField`, `Select`, `ChipGroup`, `ToggleChip`, `TagInput`, `Swatch`, `SwatchPalette` |
| `menus` | `Menu`, `MenuItem`, `Submenu`, `MenuSeparator`, `ContextMenu`, `TabBar` |
| `themes` | `ThemePicker`, `ThemeMenu`, `register_theme`, `use_theme`, bundled `Theme`s |
| `command-palette` | `CommandPalette` |
| `code-editor` | `CodeEditor`, `CodeTabs`, `CodeDocument`, `highlight_code` |
| `table` | `Table` |
| `tree` | `Tree`, `TreeItem` |
| `inspector` | `Inspector`, `InspectorSection`, `InspectorRow` |
| `dock` | `DockLayout`, `DockPanel`, `DockMain` |
| `overlays` | `Popover`, `Dropdown`, `Combobox`, `Dialog` |
| `virtual-list` | `VirtualList` |
| `diff` | `Diff`, `diff_lines` |
| `drag` | `provide_drag`, `DragSource`, `DropZone`, `DragLayer` |
| `workspace` | `TabDock`, `DockTab` |
| `code-surface` | `CodeSurface`, `MultiEditor` |
| `terminal` | `Terminal`, `AnsiTerminal`, `terminal_grid` |
| `undo-tree` | `UndoHistory`, `UndoTree` |
| `jump` | `JumpOverlay` |
| `disclosure` | `Disclosure`, `Accordion`, `AccordionItem` |
| `status-bar` | `StatusBar`, `StatusItem`, `StatusSpacer` |
| `toolbar` | `Toolbar`, `ToolbarGroup`, `ToolbarSpacer`, `ToolButton`, `MenuBar`, `MenuBarMenu`, `ActivityBar` |
| `log` | `LogView`, `LogEntry`, `LogKind` |
| `markdown` | `Markdown` |
| `search-list` | `SearchList`, `SearchItem` |
| `asset-grid` | `AssetGrid`, `AssetItem` |
| `list-editor` | `OrderedList`, `ListItem` |
| `chat` | `Chat`, `ChatMessage` |
| `dynamic-form` | `DynamicForm`, `FormField`, `FieldSchema` |
| `viewport` | `Viewport`, `Bridge`, `Loader`, `WebGpuGate`, `ViewportOverlay`, `HudPanel`, `NavGizmo` |
| `engine` | `use_engine`, `EngineViewport`, `EngineState` |
| `protocol` | `leptos_musaic::protocol` wire types (no DOM) |

Full prop signatures and behavior are in the rustdoc for each type (`cargo doc --open --features
full`), and every entry has a runnable demo in the gallery.
