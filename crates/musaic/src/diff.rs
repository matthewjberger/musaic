//! Line-level LCS diff and a side-by-side rendering component.

use leptos::prelude::*;

/// Whether a diffed line is unchanged, added, or removed.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    /// Present unchanged in both sides.
    Equal,
    /// Added in the new text.
    Insert,
    /// Removed from the old text.
    Delete,
}

/// A single line of a computed diff, with its content and its line numbers on
/// each side (`None` where the line is absent).
#[derive(Clone)]
pub struct DiffLine {
    /// Whether the line is equal, inserted, or deleted.
    pub kind: LineKind,
    /// The line's text.
    pub text: String,
    /// Line number in the old text, if present there.
    pub old_line: Option<usize>,
    /// Line number in the new text, if present there.
    pub new_line: Option<usize>,
}

/// Computes a line-level diff of `old` against `new` using a longest-common-
/// subsequence, returning the merged sequence of equal, inserted, and deleted
/// lines in order.
pub fn diff_lines(old: &str, new: &str) -> Vec<DiffLine> {
    let left: Vec<&str> = old.lines().collect();
    let right: Vec<&str> = new.lines().collect();
    let rows = left.len();
    let cols = right.len();

    let mut lcs = vec![vec![0usize; cols + 1]; rows + 1];
    for index in (0..rows).rev() {
        for other in (0..cols).rev() {
            lcs[index][other] = if left[index] == right[other] {
                lcs[index + 1][other + 1] + 1
            } else {
                lcs[index + 1][other].max(lcs[index][other + 1])
            };
        }
    }

    let mut out = Vec::new();
    let mut index = 0;
    let mut other = 0;
    let mut old_line = 1;
    let mut new_line = 1;
    while index < rows && other < cols {
        if left[index] == right[other] {
            out.push(DiffLine {
                kind: LineKind::Equal,
                text: left[index].to_string(),
                old_line: Some(old_line),
                new_line: Some(new_line),
            });
            index += 1;
            other += 1;
            old_line += 1;
            new_line += 1;
        } else if lcs[index + 1][other] >= lcs[index][other + 1] {
            out.push(DiffLine {
                kind: LineKind::Delete,
                text: left[index].to_string(),
                old_line: Some(old_line),
                new_line: None,
            });
            index += 1;
            old_line += 1;
        } else {
            out.push(DiffLine {
                kind: LineKind::Insert,
                text: right[other].to_string(),
                old_line: None,
                new_line: Some(new_line),
            });
            other += 1;
            new_line += 1;
        }
    }
    while index < rows {
        out.push(DiffLine {
            kind: LineKind::Delete,
            text: left[index].to_string(),
            old_line: Some(old_line),
            new_line: None,
        });
        index += 1;
        old_line += 1;
    }
    while other < cols {
        out.push(DiffLine {
            kind: LineKind::Insert,
            text: right[other].to_string(),
            old_line: None,
            new_line: Some(new_line),
        });
        other += 1;
        new_line += 1;
    }
    out
}

fn marker(kind: LineKind) -> &'static str {
    match kind {
        LineKind::Equal => " ",
        LineKind::Insert => "+",
        LineKind::Delete => "-",
    }
}

fn kind_class(kind: LineKind) -> &'static str {
    match kind {
        LineKind::Equal => "equal",
        LineKind::Insert => "insert",
        LineKind::Delete => "delete",
    }
}

/// Renders a unified diff of the `old` and `new` signals, one row per line with
/// old/new line numbers, a `+`/`-`/space marker, and per-kind styling.
#[component]
pub fn Diff(#[prop(into)] old: Signal<String>, #[prop(into)] new: Signal<String>) -> impl IntoView {
    view! {
        <div class="musaic-diff">
            {move || {
                diff_lines(&old.get(), &new.get())
                    .into_iter()
                    .map(|line| {
                        let old_number = line
                            .old_line
                            .map(|value| value.to_string())
                            .unwrap_or_default();
                        let new_number = line
                            .new_line
                            .map(|value| value.to_string())
                            .unwrap_or_default();
                        view! {
                            <div class=format!("musaic-diff-line {}", kind_class(line.kind))>
                                <span class="musaic-diff-num">{old_number}</span>
                                <span class="musaic-diff-num">{new_number}</span>
                                <span class="musaic-diff-marker">{marker(line.kind)}</span>
                                <span class="musaic-diff-text">{line.text}</span>
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{LineKind, diff_lines};

    #[test]
    fn identical_text_is_all_equal() {
        let lines = diff_lines("a\nb\nc", "a\nb\nc");
        assert!(lines.iter().all(|line| line.kind == LineKind::Equal));
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn detects_insert_and_delete() {
        let lines = diff_lines("a\nb\nc", "a\nc\nd");
        let inserts = lines.iter().filter(|l| l.kind == LineKind::Insert).count();
        let deletes = lines.iter().filter(|l| l.kind == LineKind::Delete).count();
        assert_eq!(deletes, 1);
        assert_eq!(inserts, 1);
        assert!(
            lines
                .iter()
                .any(|l| l.kind == LineKind::Equal && l.text == "a")
        );
    }
}
