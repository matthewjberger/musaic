# Theming

Every musaic component draws its colors from a small set of semantic CSS custom properties. A theme
sets those properties; switching themes restyles the entire surface, including components you add,
with no per-component work.

## The tokens

The core ones are `--musaic-bg`, `--musaic-panel`, `--musaic-panel-2`, `--musaic-panel-border`,
`--musaic-text`, `--musaic-text-dim`, `--musaic-accent`, `--musaic-input-bg`, `--musaic-danger`,
and a set of syntax tokens (`--musaic-tok-keyword`, `--musaic-tok-string`, and so on). Your own
components should use these too, so they theme automatically:

```css
.my-panel {
    background: var(--musaic-panel);
    color: var(--musaic-text);
    border: 1px solid var(--musaic-panel-border);
}
```

## Bundled themes and switching

`<ThemeProvider>` provides the theme context, injects the generated theme CSS, sets `data-theme` on
the document element, and persists the choice to `localStorage`. Give the user a switcher:

- `<ThemePicker/>` is a native select of the available themes.
- `<ThemeMenu/>` is a button that opens a list and previews each theme on hover, committing on
  click (the way editors do it).

`use_theme()` returns the current-theme `RwSignal` if you want to drive it yourself (for example a
"cycle theme" command).

## Custom themes

A `Theme` is a typed struct of token values. Register one and it appears in the pickers:

```rust
register_theme(Theme {
    id: "ember".into(),
    label: "Ember".into(),
    light: false,
    bg: "#140d0d".into(),
    panel: "#1d1414".into(),
    // ... the rest of the tokens ...
    accent: "#ff6b3d".into(),
    // ...
});
```

Call it inside a component under the `ThemeProvider` (the gallery does this in its theming demo).
`builtin_themes()` returns the bundled set if you want to start from one and tweak it.

## The cascade layer

The whole stylesheet `MusaicStyles` injects is wrapped in `@layer musaic`. Unlayered CSS beats any
layered CSS regardless of specificity, so **your own stylesheet always wins** over musaic's defaults
without `!important`. Write plain selectors in your app CSS and they override musaic cleanly.
