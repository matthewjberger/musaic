use leptos::html;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color {
    Default,
    Indexed(u8),
    Rgb(u8, u8, u8),
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cell {
    glyph: char,
    fg: Color,
    bg: Color,
    bold: bool,
    inverse: bool,
}

impl Cell {
    fn blank() -> Self {
        Self {
            glyph: ' ',
            fg: Color::Default,
            bg: Color::Default,
            bold: false,
            inverse: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Parse {
    Ground,
    Escape,
    Csi,
}

#[derive(Clone)]
pub struct TerminalGrid {
    cols: usize,
    rows: usize,
    cells: Vec<Cell>,
    row: usize,
    col: usize,
    fg: Color,
    bg: Color,
    bold: bool,
    inverse: bool,
    state: Parse,
    params: Vec<u16>,
    acc: Option<u16>,
    private: bool,
    saved_cursor: Option<(usize, usize)>,
    scroll_top: usize,
    scroll_bottom: usize,
    alt: Option<(Vec<Cell>, usize, usize)>,
}

impl TerminalGrid {
    fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            cells: vec![Cell::blank(); cols * rows],
            row: 0,
            col: 0,
            fg: Color::Default,
            bg: Color::Default,
            bold: false,
            inverse: false,
            state: Parse::Ground,
            params: Vec::new(),
            acc: None,
            private: false,
            saved_cursor: None,
            scroll_top: 0,
            scroll_bottom: rows - 1,
            alt: None,
        }
    }

    fn clear_row(&mut self, row: usize) {
        let start = row * self.cols;
        for cell in &mut self.cells[start..start + self.cols] {
            *cell = Cell::blank();
        }
    }

    fn scroll_region_up(&mut self) {
        for row in self.scroll_top..self.scroll_bottom {
            let (dst, src) = (row * self.cols, (row + 1) * self.cols);
            self.cells.copy_within(src..src + self.cols, dst);
        }
        self.clear_row(self.scroll_bottom);
    }

    fn newline(&mut self) {
        if self.row >= self.scroll_bottom {
            self.scroll_region_up();
        } else {
            self.row += 1;
        }
    }

    fn put(&mut self, glyph: char) {
        if self.col >= self.cols {
            self.col = 0;
            self.newline();
        }
        let index = self.row * self.cols + self.col;
        self.cells[index] = Cell {
            glyph,
            fg: self.fg,
            bg: self.bg,
            bold: self.bold,
            inverse: self.inverse,
        };
        self.col += 1;
    }

    fn param(&self, index: usize, fallback: u16) -> u16 {
        self.params.get(index).copied().unwrap_or(fallback)
    }

    fn apply_sgr(&mut self) {
        if self.params.is_empty() {
            self.params.push(0);
        }
        let params = self.params.clone();
        let mut index = 0;
        while index < params.len() {
            match params[index] {
                0 => {
                    self.fg = Color::Default;
                    self.bg = Color::Default;
                    self.bold = false;
                    self.inverse = false;
                }
                1 => self.bold = true,
                7 => self.inverse = true,
                22 => self.bold = false,
                27 => self.inverse = false,
                30..=37 => self.fg = Color::Indexed((params[index] - 30) as u8),
                39 => self.fg = Color::Default,
                40..=47 => self.bg = Color::Indexed((params[index] - 40) as u8),
                49 => self.bg = Color::Default,
                90..=97 => self.fg = Color::Indexed((params[index] - 90 + 8) as u8),
                100..=107 => self.bg = Color::Indexed((params[index] - 100 + 8) as u8),
                38 | 48 => {
                    let target_fg = params[index] == 38;
                    let color = match params.get(index + 1) {
                        Some(5) => {
                            let value = params.get(index + 2).copied().unwrap_or(0) as u8;
                            index += 2;
                            Color::Indexed(value)
                        }
                        Some(2) => {
                            let red = params.get(index + 2).copied().unwrap_or(0) as u8;
                            let green = params.get(index + 3).copied().unwrap_or(0) as u8;
                            let blue = params.get(index + 4).copied().unwrap_or(0) as u8;
                            index += 4;
                            Color::Rgb(red, green, blue)
                        }
                        _ => Color::Default,
                    };
                    if target_fg {
                        self.fg = color;
                    } else {
                        self.bg = color;
                    }
                }
                _ => {}
            }
            index += 1;
        }
    }

    fn erase_line(&mut self) {
        let start = self.row * self.cols;
        let len = self.cells.len();
        let (from, to) = match self.param(0, 0) {
            1 => (start, start + self.col + 1),
            2 => (start, start + self.cols),
            _ => (start + self.col, start + self.cols),
        };
        for cell in &mut self.cells[from..to.min(len)] {
            *cell = Cell::blank();
        }
    }

    fn erase_display(&mut self) {
        let cursor = self.row * self.cols + self.col;
        let len = self.cells.len();
        let (from, to) = match self.param(0, 0) {
            1 => (0, cursor + 1),
            2 => (0, len),
            _ => (cursor, len),
        };
        for cell in &mut self.cells[from..to.min(len)] {
            *cell = Cell::blank();
        }
    }

    fn enter_alt(&mut self) {
        if self.alt.is_none() {
            let saved =
                std::mem::replace(&mut self.cells, vec![Cell::blank(); self.cols * self.rows]);
            self.alt = Some((saved, self.row, self.col));
            self.row = 0;
            self.col = 0;
        }
    }

    fn exit_alt(&mut self) {
        if let Some((cells, row, col)) = self.alt.take() {
            self.cells = cells;
            self.row = row.min(self.rows - 1);
            self.col = col.min(self.cols - 1);
        }
    }

    fn execute(&mut self, final_byte: char) {
        match final_byte {
            'm' => self.apply_sgr(),
            'A' => self.row = self.row.saturating_sub(self.param(0, 1).max(1) as usize),
            'B' => self.row = (self.row + self.param(0, 1).max(1) as usize).min(self.rows - 1),
            'C' => self.col = (self.col + self.param(0, 1).max(1) as usize).min(self.cols - 1),
            'D' => self.col = self.col.saturating_sub(self.param(0, 1).max(1) as usize),
            'G' => {
                self.col = (self.param(0, 1) as usize)
                    .saturating_sub(1)
                    .min(self.cols - 1)
            }
            'H' | 'f' => {
                self.row = (self.param(0, 1) as usize)
                    .saturating_sub(1)
                    .min(self.rows - 1);
                self.col = (self.param(1, 1) as usize)
                    .saturating_sub(1)
                    .min(self.cols - 1);
            }
            'J' => self.erase_display(),
            'K' => self.erase_line(),
            'r' => {
                let top = (self.param(0, 1) as usize).saturating_sub(1);
                let bottom = (self.param(1, self.rows as u16) as usize)
                    .saturating_sub(1)
                    .min(self.rows - 1);
                if top < bottom {
                    self.scroll_top = top;
                    self.scroll_bottom = bottom;
                }
            }
            's' => self.saved_cursor = Some((self.row, self.col)),
            'u' => {
                if let Some((row, col)) = self.saved_cursor {
                    self.row = row.min(self.rows - 1);
                    self.col = col.min(self.cols - 1);
                }
            }
            'h' if self.private => {
                if matches!(self.param(0, 0), 47 | 1047 | 1049) {
                    self.enter_alt();
                }
            }
            'l' if self.private => {
                if matches!(self.param(0, 0), 47 | 1047 | 1049) {
                    self.exit_alt();
                }
            }
            _ => {}
        }
    }

    pub fn feed(&mut self, text: &str) {
        for glyph in text.chars() {
            match self.state {
                Parse::Ground => match glyph {
                    '\u{1b}' => self.state = Parse::Escape,
                    '\n' => {
                        self.col = 0;
                        self.newline();
                    }
                    '\r' => self.col = 0,
                    '\t' => self.col = (((self.col / 8) + 1) * 8).min(self.cols - 1),
                    '\u{8}' => self.col = self.col.saturating_sub(1),
                    glyph if !glyph.is_control() => self.put(glyph),
                    _ => {}
                },
                Parse::Escape => match glyph {
                    '[' => {
                        self.state = Parse::Csi;
                        self.params.clear();
                        self.acc = None;
                        self.private = false;
                    }
                    '7' => {
                        self.saved_cursor = Some((self.row, self.col));
                        self.state = Parse::Ground;
                    }
                    '8' => {
                        if let Some((row, col)) = self.saved_cursor {
                            self.row = row.min(self.rows - 1);
                            self.col = col.min(self.cols - 1);
                        }
                        self.state = Parse::Ground;
                    }
                    _ => self.state = Parse::Ground,
                },
                Parse::Csi => match glyph {
                    '?' => self.private = true,
                    '0'..='9' => {
                        let digit = glyph as u16 - '0' as u16;
                        self.acc = Some(self.acc.unwrap_or(0) * 10 + digit);
                    }
                    ';' => self.params.push(self.acc.take().unwrap_or(0)),
                    '\u{40}'..='\u{7e}' => {
                        if let Some(value) = self.acc.take() {
                            self.params.push(value);
                        }
                        self.execute(glyph);
                        self.state = Parse::Ground;
                    }
                    _ => {}
                },
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct TerminalHandle {
    grid: RwSignal<TerminalGrid>,
}

impl TerminalHandle {
    pub fn feed(&self, text: &str) {
        self.grid.update(|grid| grid.feed(text));
    }

    pub fn reset(&self, cols: usize, rows: usize) {
        self.grid.set(TerminalGrid::new(cols, rows));
    }
}

pub fn terminal_grid(cols: usize, rows: usize) -> TerminalHandle {
    TerminalHandle {
        grid: RwSignal::new(TerminalGrid::new(cols, rows)),
    }
}

const PALETTE: [&str; 16] = [
    "#3b3b3b", "#f87171", "#4ade80", "#fbbf24", "#60a5fa", "#c084fc", "#22d3ee", "#d4d4d4",
    "#6b7280", "#fca5a5", "#86efac", "#fde68a", "#93c5fd", "#d8b4fe", "#67e8f9", "#ffffff",
];

fn cube_channel(value: u8) -> u8 {
    if value == 0 { 0 } else { 55 + value * 40 }
}

fn color(value: Color, default: &str) -> String {
    match value {
        Color::Default => default.to_string(),
        Color::Indexed(index) if index < 16 => PALETTE[index as usize].to_string(),
        Color::Indexed(index) if index < 232 => {
            let index = index - 16;
            let red = cube_channel(index / 36);
            let green = cube_channel((index / 6) % 6);
            let blue = cube_channel(index % 6);
            format!("#{red:02x}{green:02x}{blue:02x}")
        }
        Color::Indexed(index) => {
            let level = 8 + (index - 232) * 10;
            format!("#{level:02x}{level:02x}{level:02x}")
        }
        Color::Rgb(red, green, blue) => format!("#{red:02x}{green:02x}{blue:02x}"),
    }
}

fn cell_style(cell: Cell) -> String {
    let mut foreground = color(cell.fg, "var(--musaic-text)");
    let mut background = color(cell.bg, "transparent");
    if cell.inverse {
        if background == "transparent" {
            background = "var(--musaic-text)".to_string();
            foreground = "var(--musaic-bg)".to_string();
        } else {
            std::mem::swap(&mut foreground, &mut background);
        }
    }
    let weight = if cell.bold { "font-weight:700;" } else { "" };
    format!("color:{foreground};background:{background};{weight}")
}

fn key_to_bytes(event: &web_sys::KeyboardEvent) -> Option<String> {
    let key = event.key();
    if event.ctrl_key() && key.len() == 1 {
        let character = key.chars().next()?.to_ascii_lowercase();
        if character.is_ascii_alphabetic() {
            return Some(((character as u8 - b'a' + 1) as char).to_string());
        }
    }
    match key.as_str() {
        "Enter" => Some("\r".to_string()),
        "Backspace" => Some("\u{7f}".to_string()),
        "Tab" => Some("\t".to_string()),
        "Escape" => Some("\u{1b}".to_string()),
        "ArrowUp" => Some("\u{1b}[A".to_string()),
        "ArrowDown" => Some("\u{1b}[B".to_string()),
        "ArrowRight" => Some("\u{1b}[C".to_string()),
        "ArrowLeft" => Some("\u{1b}[D".to_string()),
        other if other.chars().count() == 1 => Some(other.to_string()),
        _ => None,
    }
}

#[component]
pub fn AnsiTerminal(
    handle: TerminalHandle,
    #[prop(optional)] on_key: Option<Callback<String>>,
) -> impl IntoView {
    let body_ref = NodeRef::<html::Div>::new();
    let grid = handle.grid;

    view! {
        <div
            class="musaic-ansi"
            tabindex="0"
            on:keydown=move |event: web_sys::KeyboardEvent| {
                if let Some(callback) = on_key
                    && let Some(bytes) = key_to_bytes(&event)
                {
                    event.prevent_default();
                    callback.run(bytes);
                }
            }
        >
            <div class="musaic-ansi-body" node_ref=body_ref>
                {move || {
                    let grid = grid.get();
                    let cols = grid.cols;
                    (0..grid.rows)
                        .map(|row| {
                            let cells = &grid.cells[row * cols..row * cols + cols];
                            let mut runs: Vec<(String, String)> = Vec::new();
                            for cell in cells {
                                let style = cell_style(*cell);
                                match runs.last_mut() {
                                    Some((last_style, text)) if *last_style == style => {
                                        text.push(cell.glyph);
                                    }
                                    _ => runs.push((style, cell.glyph.to_string())),
                                }
                            }
                            let spans = runs
                                .into_iter()
                                .map(|(style, text)| view! { <span style=style>{text}</span> })
                                .collect_view();
                            view! { <div class="musaic-ansi-row">{spans}</div> }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, TerminalGrid};

    fn text_at(grid: &TerminalGrid, row: usize) -> String {
        grid.cells[row * grid.cols..row * grid.cols + grid.cols]
            .iter()
            .map(|cell| cell.glyph)
            .collect::<String>()
            .trim_end()
            .to_string()
    }

    #[test]
    fn plain_text_and_newlines_write_rows() {
        let mut grid = TerminalGrid::new(20, 4);
        grid.feed("hello\nworld");
        assert_eq!(text_at(&grid, 0), "hello");
        assert_eq!(text_at(&grid, 1), "world");
    }

    #[test]
    fn sgr_sets_and_resets_color() {
        let mut grid = TerminalGrid::new(20, 4);
        grid.feed("\u{1b}[32mgreen\u{1b}[0m.");
        assert_eq!(grid.cells[0].fg, Color::Indexed(2));
        assert_eq!(grid.cells[5].fg, Color::Default);
    }

    #[test]
    fn sgr_256_and_truecolor() {
        let mut grid = TerminalGrid::new(20, 2);
        grid.feed("\u{1b}[38;5;196mA\u{1b}[38;2;10;20;30mB");
        assert_eq!(grid.cells[0].fg, Color::Indexed(196));
        assert_eq!(grid.cells[1].fg, Color::Rgb(10, 20, 30));
    }

    #[test]
    fn cursor_save_restore_returns_to_saved_position() {
        let mut grid = TerminalGrid::new(10, 4);
        grid.feed("\u{1b}7abc\u{1b}8Z");
        assert_eq!(grid.cells[0].glyph, 'Z');
        assert_eq!(grid.cells[1].glyph, 'b');
    }

    #[test]
    fn scroll_region_limits_scrolling() {
        let mut grid = TerminalGrid::new(4, 4);
        grid.feed("\u{1b}[1;2r");
        grid.feed("a\nb\nc\nd");
        assert_eq!(grid.scroll_top, 0);
        assert_eq!(grid.scroll_bottom, 1);
    }

    #[test]
    fn alt_screen_saves_and_restores() {
        let mut grid = TerminalGrid::new(6, 2);
        grid.feed("main");
        grid.feed("\u{1b}[?1049h");
        assert_eq!(text_at(&grid, 0), "");
        grid.feed("\u{1b}[?1049l");
        assert_eq!(text_at(&grid, 0), "main");
    }
}
