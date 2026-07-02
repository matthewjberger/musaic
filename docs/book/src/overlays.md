# Overlays and Menus

## Floating surfaces

The `overlays` feature provides anchored floating UI that stays on screen:

- `Popover`: content anchored to a trigger, which flips and shifts to stay within the viewport. Set
  the preferred `Side` and `Align`.
- `Dropdown`: a button that opens an anchored panel.
- `Combobox`: a filterable select over `ComboOption`s.
- `Dialog`: a centered modal dialog.

`Modal`, `Overlay`, and `Scrim` live in the base layer. `Modal` traps Tab focus, closes on Escape,
restores focus on close, and dims the backdrop; reach for it when you need a true modal and for
`Dialog` when you want the ready-made dialog chrome.

## Menus

The `menus` feature covers menu surfaces, all keyboard-navigable:

- `Menu` with `MenuItem` (checkable, disabled, shortcut hint), `Submenu`, and `MenuSeparator`.
- `ContextMenu`: a portalled menu opened at the pointer on right-click.
- `TabBar`: a row of tabs bound to an active-id signal.

The `toolbar` feature adds the top-of-window surfaces: `Toolbar` with `ToolbarGroup`,
`ToolbarSpacer`, and `ToolButton`; `MenuBar` with `MenuBarMenu` (a desktop-style menu bar where
hovering a sibling switches the open menu); and `ActivityBar`, the icon rail down the side.

## Building menus from commands

Because menus and the command registry share the same `Command` type, you can build a menu directly
from registered commands rather than duplicating actions. Register the capability once (see
[Commands and Keybindings](commands.md)) and surface it in a `Menu`, the `CommandPalette`, and a
keybinding without repeating yourself.
