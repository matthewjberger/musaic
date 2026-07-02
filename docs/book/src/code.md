# Code and Text

musaic has three text surfaces at different levels of ambition, plus an ANSI terminal, a diff, and a
Markdown renderer.

## Editing code

- `CodeEditor` (the `code-editor` feature) is a practical editable code field: a syntax-highlight
  layer over a textarea, an optional gutter, diagnostic markers, and find/replace. Pair it with
  `CodeTabs` to edit multiple `CodeDocument`s. `highlight_code(source, keywords, commands)` is a
  generic keyword/command highlighter you pass in, so it is not tied to any one language.

  ```rust
  view! { <CodeEditor value=script highlighter=my_highlighter fill=true/> }
  ```

- `MultiEditor` (the `code-surface` feature) is a multi-cursor editor on a virtualized monospace
  grid: add cursors above and below (Ctrl/Cmd+Alt+Arrow), add-next-occurrence (Ctrl/Cmd+D),
  multi-selection, drag-select, Home/End, clipboard copy/cut/paste, and IME composition through a
  hidden input sink. Carets are positioned in `ch` units, so there is no DOM measuring.

## Viewing code

`CodeSurface` (the `code-surface` feature) is a read-only, virtualized viewer with brace-based code
folding. Use it to show large files or logs of code where you do not need editing.

## Terminal

- `Terminal` (the `terminal` feature) is a REPL-style surface of tone-colored lines with a prompt.
- `AnsiTerminal` (the `terminal` feature) is a real ANSI/VT emulator: it parses 16/256/truecolor
  SGR, bold and inverse, cursor movement with save and restore, erasing, scroll regions, and the
  alternate screen into a cell grid. Drive it with a `TerminalHandle` (from `terminal_grid(cols,
  rows)`): call `handle.feed(bytes)` to push output and read key input back through `on_key`.

## Diff and Markdown

- `Diff` (the `diff` feature) renders an LCS line diff with +/- markers and old/new line numbers;
  `diff_lines` is the pure algorithm if you want the data.
- `Markdown` (the `markdown` feature) is a dependency-free renderer for headings, emphasis, code,
  lists, links, and quotes.
