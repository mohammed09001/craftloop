//! Contextual command grammar: namespace vocabularies, full-word
//! matching, shortest-unique-prefix resolution, and ambiguous-prefix
//! handling.
//!
//! Execution 01, Phase 19, Tasks 134-137. Authority: MCP Article 237
//! ("The Notebook context might include: Pen. Eraser. Select. Sketch.
//! Orthographic. The Sketch context might include: Line. Circle. Arc.
//! Rectangle. Dimension. Exit Sketch. The Orthographic context might
//! include: Add View. Label View. Link. Resolve. Dimension.").
//!
//! Word lists and the one curated synonym (`Ortho` -> `Orthographic`,
//! Article 237's own example) are the exact vocabulary the article
//! names -- not a speculative expansion. "Synonyms increase recognition
//! burden. They should be intentionally curated": exactly one synonym
//! exists, deliberately, rather than an open-ended alias table.

use crate::command::{CommandAction, CommandNamespace};

/// One vocabulary entry: the word as written, and which action it
/// resolves to. `word` is always stored lowercase; matching normalizes
/// input to lowercase too (Article 236's "Normalize handwriting" stage).
struct VocabularyEntry {
    word: &'static str,
    action: CommandAction,
}

fn vocabulary(namespace: CommandNamespace) -> &'static [VocabularyEntry] {
    match namespace {
        CommandNamespace::Notebook => &[
            VocabularyEntry {
                word: "pen",
                action: CommandAction::Pen,
            },
            VocabularyEntry {
                word: "eraser",
                action: CommandAction::Eraser,
            },
            VocabularyEntry {
                word: "select",
                action: CommandAction::Select,
            },
            VocabularyEntry {
                word: "sketch",
                action: CommandAction::Sketch,
            },
            VocabularyEntry {
                word: "orthographic",
                action: CommandAction::Orthographic,
            },
            // Article 237's own curated synonym example.
            VocabularyEntry {
                word: "ortho",
                action: CommandAction::Orthographic,
            },
        ],
        CommandNamespace::Sketch => &[
            VocabularyEntry {
                word: "line",
                action: CommandAction::Line,
            },
            VocabularyEntry {
                word: "circle",
                action: CommandAction::Circle,
            },
            VocabularyEntry {
                word: "arc",
                action: CommandAction::Arc,
            },
            VocabularyEntry {
                word: "rectangle",
                action: CommandAction::Rectangle,
            },
            VocabularyEntry {
                word: "dimension",
                action: CommandAction::Dimension,
            },
            VocabularyEntry {
                word: "exit sketch",
                action: CommandAction::ExitSketch,
            },
        ],
        CommandNamespace::Orthographic => &[
            VocabularyEntry {
                word: "add view",
                action: CommandAction::AddView,
            },
            VocabularyEntry {
                word: "label view",
                action: CommandAction::LabelView,
            },
            VocabularyEntry {
                word: "link",
                action: CommandAction::Link,
            },
            VocabularyEntry {
                word: "resolve",
                action: CommandAction::Resolve,
            },
            VocabularyEntry {
                word: "dimension",
                action: CommandAction::Dimension,
            },
        ],
    }
}

/// The result of resolving recognized text against a namespace's
/// grammar. Task 137's core rule -- "Never guess" -- is why `Ambiguous`
/// exists as its own variant rather than this function returning
/// `Option<CommandAction>` and picking arbitrarily on a tie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarMatch {
    /// Task 135: an exact vocabulary word (or curated synonym).
    Exact(CommandAction),
    /// Task 136: a prefix that is unique among every word in this
    /// namespace.
    UniquePrefix(CommandAction),
    /// Task 137: a prefix matching more than one word -- the caller must
    /// request more input or show candidates, never guess. Candidates
    /// are the distinct matching words, in the namespace's own vocabulary
    /// order.
    Ambiguous(Vec<&'static str>),
    /// No word or prefix in this namespace matches at all.
    NoMatch,
}

/// Task 142: is `action` actually part of `namespace`'s own grammar? The
/// Command Bus uses this as a structural validity check independent of
/// how the command was resolved (ink, toolbar, keyboard, ...) -- a
/// toolbar button firing `CommandAction::Line` while the active
/// namespace is `Notebook` is just as invalid as a misrecognized ink
/// stroke would be.
pub fn is_valid_in_namespace(action: CommandAction, namespace: CommandNamespace) -> bool {
    vocabulary(namespace)
        .iter()
        .any(|entry| entry.action == action)
}

/// Resolve `input` (already-recognized, normalized-by-the-caller text --
/// e.g. `craftloop-handwriting`'s `TextCandidate::text`) against
/// `namespace`'s grammar.
pub fn resolve(input: &str, namespace: CommandNamespace) -> GrammarMatch {
    let normalized = input.trim().to_lowercase();
    if normalized.is_empty() {
        return GrammarMatch::NoMatch;
    }
    let words = vocabulary(namespace);

    // Task 135: full-word matching is the stable fallback -- checked
    // first, so a short vocabulary word that also happens to be a prefix
    // of a longer one (none currently in this grammar, but the rule
    // should hold regardless) always resolves to itself exactly.
    if let Some(entry) = words.iter().find(|entry| entry.word == normalized) {
        return GrammarMatch::Exact(entry.action);
    }

    // Task 136/137: prefix matching.
    let matches: Vec<&VocabularyEntry> = words
        .iter()
        .filter(|entry| entry.word.starts_with(&normalized))
        .collect();
    match matches.len() {
        0 => GrammarMatch::NoMatch,
        1 => GrammarMatch::UniquePrefix(matches[0].action),
        _ => {
            let mut candidates: Vec<&'static str> =
                matches.iter().map(|entry| entry.word).collect();
            candidates.dedup();
            GrammarMatch::Ambiguous(candidates)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_full_word_match_is_exact_even_when_it_is_also_a_prefix_of_something_else() {
        assert_eq!(
            resolve("select", CommandNamespace::Notebook),
            GrammarMatch::Exact(CommandAction::Select)
        );
    }

    #[test]
    fn matching_is_case_insensitive() {
        assert_eq!(
            resolve("PEN", CommandNamespace::Notebook),
            GrammarMatch::Exact(CommandAction::Pen)
        );
    }

    #[test]
    fn a_curated_synonym_resolves_to_the_same_action_as_the_full_word() {
        assert_eq!(
            resolve("ortho", CommandNamespace::Notebook),
            GrammarMatch::Exact(CommandAction::Orthographic)
        );
    }

    #[test]
    fn an_unambiguous_prefix_resolves_via_shortest_unique_prefix() {
        // "sk" only matches "sketch" in the Notebook namespace.
        assert_eq!(
            resolve("sk", CommandNamespace::Notebook),
            GrammarMatch::UniquePrefix(CommandAction::Sketch)
        );
    }

    #[test]
    fn an_ambiguous_prefix_never_guesses_and_names_every_candidate() {
        // "s" matches both "select" and "sketch" in the Notebook
        // namespace -- a genuine collision.
        let ambiguous = resolve("s", CommandNamespace::Notebook);
        assert!(matches!(ambiguous, GrammarMatch::Ambiguous(_)));
        if let GrammarMatch::Ambiguous(candidates) = ambiguous {
            assert!(candidates.contains(&"select"));
            assert!(candidates.contains(&"sketch"));
        }
    }

    #[test]
    fn a_prefix_of_exactly_one_word_is_unique_not_ambiguous() {
        // "line" is the only Sketch word starting with "l".
        assert_eq!(
            resolve("l", CommandNamespace::Sketch),
            GrammarMatch::UniquePrefix(CommandAction::Line)
        );
    }

    #[test]
    fn a_word_from_a_different_namespace_does_not_match() {
        assert_eq!(
            resolve("circle", CommandNamespace::Notebook),
            GrammarMatch::NoMatch
        );
    }

    #[test]
    fn empty_input_never_matches_anything() {
        assert_eq!(
            resolve("", CommandNamespace::Notebook),
            GrammarMatch::NoMatch
        );
        assert_eq!(
            resolve("   ", CommandNamespace::Notebook),
            GrammarMatch::NoMatch
        );
    }

    #[test]
    fn dimension_is_valid_in_both_sketch_and_orthographic_as_the_same_action() {
        assert_eq!(
            resolve("dimension", CommandNamespace::Sketch),
            GrammarMatch::Exact(CommandAction::Dimension)
        );
        assert_eq!(
            resolve("dimension", CommandNamespace::Orthographic),
            GrammarMatch::Exact(CommandAction::Dimension)
        );
    }

    #[test]
    fn every_namespaces_vocabulary_has_no_internal_duplicate_words() {
        for namespace in [
            CommandNamespace::Notebook,
            CommandNamespace::Sketch,
            CommandNamespace::Orthographic,
        ] {
            let words: Vec<&str> = vocabulary(namespace).iter().map(|e| e.word).collect();
            let mut sorted = words.clone();
            sorted.sort();
            sorted.dedup();
            assert_eq!(
                words.len(),
                sorted.len(),
                "duplicate word in {namespace:?}'s vocabulary"
            );
        }
    }
}
