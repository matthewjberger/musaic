use leptos::prelude::*;

#[derive(Clone, PartialEq)]
enum Inline {
    Text(String),
    Bold(String),
    Italic(String),
    Code(String),
    Link { text: String, url: String },
}

#[derive(Clone, PartialEq)]
enum Block {
    Heading(u8, Vec<Inline>),
    Paragraph(Vec<Inline>),
    Code(String),
    List {
        ordered: bool,
        items: Vec<Vec<Inline>>,
    },
    Quote(Vec<Inline>),
    Rule,
}

fn parse_inline(source: &str) -> Vec<Inline> {
    let chars: Vec<char> = source.chars().collect();
    let count = chars.len();
    let mut runs: Vec<Inline> = Vec::new();
    let mut plain = String::new();
    let mut index = 0;

    let flush = |plain: &mut String, runs: &mut Vec<Inline>| {
        if !plain.is_empty() {
            runs.push(Inline::Text(std::mem::take(plain)));
        }
    };

    while index < count {
        let rest: String = chars[index..].iter().collect();
        if rest.starts_with("**")
            && let Some(end) = find_delim(&chars, index + 2, "**")
        {
            flush(&mut plain, &mut runs);
            runs.push(Inline::Bold(chars[index + 2..end].iter().collect()));
            index = end + 2;
        } else if (chars[index] == '*' || chars[index] == '_')
            && let Some(end) = find_char(&chars, index + 1, chars[index])
        {
            flush(&mut plain, &mut runs);
            runs.push(Inline::Italic(chars[index + 1..end].iter().collect()));
            index = end + 1;
        } else if chars[index] == '`'
            && let Some(end) = find_char(&chars, index + 1, '`')
        {
            flush(&mut plain, &mut runs);
            runs.push(Inline::Code(chars[index + 1..end].iter().collect()));
            index = end + 1;
        } else if chars[index] == '['
            && let Some((text, url, next)) = parse_link(&chars, index)
        {
            flush(&mut plain, &mut runs);
            runs.push(Inline::Link { text, url });
            index = next;
        } else {
            plain.push(chars[index]);
            index += 1;
        }
    }
    flush(&mut plain, &mut runs);
    runs
}

fn find_char(chars: &[char], start: usize, target: char) -> Option<usize> {
    (start..chars.len()).find(|&index| chars[index] == target)
}

fn find_delim(chars: &[char], start: usize, delim: &str) -> Option<usize> {
    let delim: Vec<char> = delim.chars().collect();
    let mut index = start;
    while index + delim.len() <= chars.len() {
        if chars[index..index + delim.len()] == delim[..] {
            return Some(index);
        }
        index += 1;
    }
    None
}

fn parse_link(chars: &[char], start: usize) -> Option<(String, String, usize)> {
    let text_end = find_char(chars, start + 1, ']')?;
    if text_end + 1 >= chars.len() || chars[text_end + 1] != '(' {
        return None;
    }
    let url_end = find_char(chars, text_end + 2, ')')?;
    let text: String = chars[start + 1..text_end].iter().collect();
    let url: String = chars[text_end + 2..url_end].iter().collect();
    Some((text, url, url_end + 1))
}

fn parse(source: &str) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            index += 1;
        } else if let Some(fence) = trimmed.strip_prefix("```") {
            let _ = fence;
            index += 1;
            let mut code = String::new();
            while index < lines.len() && !lines[index].trim_start().starts_with("```") {
                code.push_str(lines[index]);
                code.push('\n');
                index += 1;
            }
            index += 1;
            blocks.push(Block::Code(code));
        } else if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            blocks.push(Block::Rule);
            index += 1;
        } else if let Some(level) = heading_level(trimmed) {
            let text = trimmed[level as usize..].trim();
            blocks.push(Block::Heading(level, parse_inline(text)));
            index += 1;
        } else if is_unordered(trimmed) || is_ordered(trimmed) {
            let ordered = is_ordered(trimmed);
            let mut items = Vec::new();
            while index < lines.len() {
                let item = lines[index].trim();
                if ordered && is_ordered(item) {
                    items.push(parse_inline(strip_ordered(item)));
                } else if !ordered && is_unordered(item) {
                    items.push(parse_inline(item[2..].trim()));
                } else {
                    break;
                }
                index += 1;
            }
            blocks.push(Block::List { ordered, items });
        } else if let Some(quote) = trimmed.strip_prefix("> ") {
            blocks.push(Block::Quote(parse_inline(quote)));
            index += 1;
        } else {
            let mut paragraph = String::new();
            while index < lines.len() && !lines[index].trim().is_empty() {
                let current = lines[index].trim();
                if heading_level(current).is_some()
                    || current.starts_with("```")
                    || is_unordered(current)
                    || is_ordered(current)
                {
                    break;
                }
                if !paragraph.is_empty() {
                    paragraph.push(' ');
                }
                paragraph.push_str(current);
                index += 1;
            }
            blocks.push(Block::Paragraph(parse_inline(&paragraph)));
        }
    }
    blocks
}

fn heading_level(line: &str) -> Option<u8> {
    let hashes = line
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if (1..=6).contains(&hashes) && line.chars().nth(hashes) == Some(' ') {
        Some(hashes as u8)
    } else {
        None
    }
}

fn is_unordered(line: &str) -> bool {
    line.starts_with("- ") || line.starts_with("* ") || line.starts_with("+ ")
}

fn is_ordered(line: &str) -> bool {
    strip_ordered_prefix(line).is_some()
}

fn strip_ordered_prefix(line: &str) -> Option<usize> {
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    if digits == 0 {
        return None;
    }
    let after = &line[digits..];
    if after.starts_with(". ") {
        Some(digits + 2)
    } else {
        None
    }
}

fn strip_ordered(line: &str) -> &str {
    match strip_ordered_prefix(line) {
        Some(offset) => line[offset..].trim(),
        None => line,
    }
}

fn render_inline(runs: Vec<Inline>) -> AnyView {
    runs.into_iter()
        .map(|run| match run {
            Inline::Text(text) => text.into_any(),
            Inline::Bold(text) => view! { <strong>{text}</strong> }.into_any(),
            Inline::Italic(text) => view! { <em>{text}</em> }.into_any(),
            Inline::Code(text) => view! { <code class="musaic-md-code">{text}</code> }.into_any(),
            Inline::Link { text, url } => view! {
                <a class="musaic-md-link" href=url target="_blank" rel="noreferrer">
                    {text}
                </a>
            }
            .into_any(),
        })
        .collect_view()
        .into_any()
}

fn render_block(block: Block) -> AnyView {
    match block {
        Block::Heading(level, runs) => {
            let inline = render_inline(runs);
            match level {
                1 => view! { <h1 class="musaic-md-h1">{inline}</h1> }.into_any(),
                2 => view! { <h2 class="musaic-md-h2">{inline}</h2> }.into_any(),
                3 => view! { <h3 class="musaic-md-h3">{inline}</h3> }.into_any(),
                _ => view! { <h4 class="musaic-md-h4">{inline}</h4> }.into_any(),
            }
        }
        Block::Paragraph(runs) => {
            view! { <p class="musaic-md-p">{render_inline(runs)}</p> }.into_any()
        }
        Block::Code(code) => {
            view! { <pre class="musaic-md-pre"><code>{code}</code></pre> }.into_any()
        }
        Block::List { ordered, items } => {
            let entries = items
                .into_iter()
                .map(|item| view! { <li>{render_inline(item)}</li> })
                .collect_view();
            if ordered {
                view! { <ol class="musaic-md-list">{entries}</ol> }.into_any()
            } else {
                view! { <ul class="musaic-md-list">{entries}</ul> }.into_any()
            }
        }
        Block::Quote(runs) => {
            view! { <blockquote class="musaic-md-quote">{render_inline(runs)}</blockquote> }
                .into_any()
        }
        Block::Rule => view! { <hr class="musaic-md-rule" /> }.into_any(),
    }
}

#[component]
pub fn Markdown(#[prop(into)] source: Signal<String>) -> impl IntoView {
    view! {
        <div class="musaic-markdown">
            {move || {
                parse(&source.get())
                    .into_iter()
                    .map(render_block)
                    .collect_view()
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{Block, Inline, parse, parse_inline};

    #[test]
    fn parses_headings_and_paragraphs() {
        let blocks = parse("# Title\n\nA paragraph line\nwrapped here.");
        assert!(matches!(blocks[0], Block::Heading(1, _)));
        assert!(matches!(blocks[1], Block::Paragraph(_)));
    }

    #[test]
    fn parses_lists_and_code_fences() {
        let blocks = parse("- one\n- two\n\n```\ncode\n```\n");
        assert!(matches!(blocks[0], Block::List { ordered: false, .. }));
        assert!(matches!(&blocks[1], Block::Code(code) if code.contains("code")));
    }

    #[test]
    fn parses_inline_emphasis_code_and_links() {
        let runs = parse_inline("a **b** and *c* and `d` and [e](http://x)");
        assert!(
            runs.iter()
                .any(|run| matches!(run, Inline::Bold(text) if text == "b"))
        );
        assert!(
            runs.iter()
                .any(|run| matches!(run, Inline::Italic(text) if text == "c"))
        );
        assert!(
            runs.iter()
                .any(|run| matches!(run, Inline::Code(text) if text == "d"))
        );
        assert!(
            runs.iter()
                .any(|run| matches!(run, Inline::Link { url, .. } if url == "http://x"))
        );
    }

    #[test]
    fn parses_ordered_lists() {
        let blocks = parse("1. first\n2. second\n");
        match &blocks[0] {
            Block::List { ordered, items } => {
                assert!(ordered);
                assert_eq!(items.len(), 2);
            }
            _ => panic!("expected ordered list"),
        }
    }
}
