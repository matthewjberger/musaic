use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

use crate::command::use_commands;

#[derive(Clone, PartialEq, Eq)]
struct Combo {
    modifier: bool,
    alt: bool,
    shift: bool,
    key: String,
}

fn parse_combo(text: &str) -> Combo {
    let mut combo = Combo {
        modifier: false,
        alt: false,
        shift: false,
        key: String::new(),
    };
    for token in text.split('+') {
        let token = token.trim();
        match token.to_lowercase().as_str() {
            "ctrl" | "control" | "cmd" | "command" | "meta" | "super" | "mod" => {
                combo.modifier = true;
            }
            "alt" | "option" => combo.alt = true,
            "shift" => combo.shift = true,
            "" => {}
            other => combo.key = other.to_string(),
        }
    }
    combo
}

fn parse_binding(text: &str) -> Vec<Combo> {
    text.split_whitespace().map(parse_combo).collect()
}

fn event_combo(event: &KeyboardEvent) -> Combo {
    Combo {
        modifier: event.ctrl_key() || event.meta_key(),
        alt: event.alt_key(),
        shift: event.shift_key(),
        key: event.key().to_lowercase(),
    }
}

fn combo_matches(binding: &Combo, event: &Combo) -> bool {
    binding.modifier == event.modifier
        && binding.alt == event.alt
        && (!binding.shift || event.shift)
        && binding.key == event.key
}

fn sequence_matches(binding: &[Combo], pending: &[Combo]) -> bool {
    binding.len() == pending.len()
        && binding
            .iter()
            .zip(pending)
            .all(|(left, right)| combo_matches(left, right))
}

fn sequence_prefixes(binding: &[Combo], pending: &[Combo]) -> bool {
    binding.len() > pending.len()
        && binding
            .iter()
            .zip(pending)
            .all(|(left, right)| combo_matches(left, right))
}

fn is_bare_modifier(key: &str) -> bool {
    matches!(key, "control" | "shift" | "alt" | "meta")
}

fn editable_focus() -> bool {
    let Some(element) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.active_element())
    else {
        return false;
    };
    let tag = element.tag_name().to_lowercase();
    if matches!(tag.as_str(), "input" | "textarea" | "select") {
        return true;
    }
    element
        .dyn_ref::<web_sys::HtmlElement>()
        .is_some_and(|element| element.is_content_editable())
}

pub fn pretty_binding(binding: &str) -> String {
    binding
        .split_whitespace()
        .map(|combo| {
            combo
                .split('+')
                .map(|token| match token.trim().to_lowercase().as_str() {
                    "ctrl" | "control" | "cmd" | "command" | "meta" | "super" | "mod" => {
                        "Ctrl".to_string()
                    }
                    "alt" | "option" => "Alt".to_string(),
                    "shift" => "Shift".to_string(),
                    other if other.chars().count() == 1 => other.to_uppercase(),
                    other => {
                        let mut chars = other.chars();
                        match chars.next() {
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                            None => String::new(),
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join("+")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

const CHORD_RESET_MS: i32 = 900;

#[component]
pub fn KeymapProvider(children: Children) -> impl IntoView {
    let registry = use_commands();
    let pending = StoredValue::new(Vec::<Combo>::new());
    let generation = StoredValue::new(0u32);

    let handle = window_event_listener(leptos::ev::keydown, move |event: KeyboardEvent| {
        let current = event_combo(&event);
        if is_bare_modifier(&current.key) {
            return;
        }

        let bindings = registry
            .commands_untracked()
            .into_iter()
            .filter_map(|action| {
                action
                    .keybinding
                    .as_ref()
                    .map(|binding| (action.id.clone(), parse_binding(binding)))
            })
            .collect::<Vec<_>>();
        if bindings.is_empty() {
            return;
        }

        let editing = editable_focus();
        if editing && !current.modifier && !current.alt && pending.with_value(Vec::is_empty) {
            return;
        }

        let mut sequence = pending.get_value();
        sequence.push(current);

        if let Some((id, _)) = bindings
            .iter()
            .find(|(_, binding)| sequence_matches(binding, &sequence))
        {
            event.prevent_default();
            pending.set_value(Vec::new());
            registry.run(id);
            return;
        }

        let has_prefix = bindings
            .iter()
            .any(|(_, binding)| sequence_prefixes(binding, &sequence));
        if has_prefix {
            event.prevent_default();
            pending.set_value(sequence);
            schedule_reset(pending, generation);
            return;
        }

        let last = sequence.pop().expect("sequence was just pushed");
        let restart = vec![last];
        if bindings
            .iter()
            .any(|(_, binding)| sequence_prefixes(binding, &restart))
        {
            event.prevent_default();
            pending.set_value(restart);
            schedule_reset(pending, generation);
        } else {
            pending.set_value(Vec::new());
        }
    });

    on_cleanup(move || handle.remove());

    children()
}

fn schedule_reset(pending: StoredValue<Vec<Combo>>, generation: StoredValue<u32>) {
    let current = generation.get_value().wrapping_add(1);
    generation.set_value(current);
    set_timeout(
        move || {
            if generation.get_value() == current {
                pending.set_value(Vec::new());
            }
        },
        std::time::Duration::from_millis(CHORD_RESET_MS as u64),
    );
}

#[cfg(test)]
mod tests {
    use super::{
        Combo, combo_matches, parse_binding, pretty_binding, sequence_matches, sequence_prefixes,
    };

    fn event(modifier: bool, key: &str) -> Combo {
        Combo {
            modifier,
            alt: false,
            shift: false,
            key: key.to_string(),
        }
    }

    #[test]
    fn modifier_combos_parse_and_match() {
        let binding = parse_binding("Ctrl+K");
        assert_eq!(binding.len(), 1);
        assert!(combo_matches(&binding[0], &event(true, "k")));
        assert!(!combo_matches(&binding[0], &event(false, "k")));
    }

    #[test]
    fn chord_sequences_match_and_prefix() {
        let binding = parse_binding("g d");
        let step_one = vec![event(false, "g")];
        let full = vec![event(false, "g"), event(false, "d")];
        assert!(sequence_prefixes(&binding, &step_one));
        assert!(sequence_matches(&binding, &full));
        assert!(!sequence_matches(&binding, &step_one));
    }

    #[test]
    fn pretty_binding_is_readable() {
        assert_eq!(pretty_binding("mod+shift+p"), "Ctrl+Shift+P");
        assert_eq!(pretty_binding("g d"), "G D");
    }
}
