# Introduction

musaic is a component library for [Leptos](https://leptos.dev). It gives you the pieces an
editor-style application is made of, panels, forms, menus, tables, trees, a command palette,
keybindings, a code editor, toasts, a status bar, drag and drop, and a resizable app shell, all
themed from one set of CSS custom properties and all rendered client-side (CSR). The same code runs
on the web as wasm and natively inside a webview.

The goal is that you assemble a dense, production-grade UI with very little code, without giving up
control. Components default to sensible behavior and expose reactive props and callbacks for the
moments you need to steer them. There is no design system to fight and no build step: drop one
component at the root and the stylesheet is injected for you.

## What you get

- A **themed component set**. Every widget draws from semantic tokens (`--musaic-accent`,
  `--musaic-panel`, and friends). Switch `data-theme` on the document and the whole surface,
  including any component you add, restyles at once.
- **One source of truth for actions**. A command registry feeds the command palette, the keybinding
  layer, and your menus from the same list, so a new capability is one `register` call, not three
  wired surfaces.
- **An app frame**. `EditorShell` lays out toolbar, sidebars, a bottom dock, a status bar, and a
  center region, with collapsible and resizable panels, in one component.
- **Editor-grade building blocks**. A multi-cursor code editor, a foldable code viewer, an ANSI
  terminal, an LCS diff, virtualized lists and tables, an avy-style jump overlay, a branching undo
  history, and pointer-based drag and drop that works inside webviews.

## How this book is organized

- **Getting Started** installs the crate, mounts a first UI, and shows how a musaic app is laid out.
- **Concepts** covers the three things you need in your head: how reactivity and state handles work,
  how feature gates select components, and how theming works.
- **Building the UI** is a tour by task: the app shell, commands, forms, data views, code, feedback,
  overlays, drag and drop, and optional rendering surfaces.
- **Reference** is the full component catalog and the recipe for adding your own component to the
  library.

The single best companion to this book is the **gallery**: `examples/gallery` renders a live,
interactive demo of every component. When you want to see a component in use, open its page in
`examples/gallery/src/sections.rs`.
