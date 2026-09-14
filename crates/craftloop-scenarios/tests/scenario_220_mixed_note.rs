//! Task 220 — Mixed-note scenario.
//!
//! Execution 01, Phase 30, Task 220. Ordinary notes that happen to
//! contain numbers and command-vocabulary *substrings* (e.g. "pens"
//! contains "pen", a real `CommandAction`) must remain notes, never be
//! misinterpreted as commands. Proven against the real command grammar
//! (Phase 19, `craftloop_command::grammar::resolve`) rather than
//! asserted: `resolve` matches a whole trimmed, lowercased input against
//! whole vocabulary words or their unique prefixes, never a substring
//! search, so a full sentence naturally falls through to `NoMatch` --
//! this scenario exists to confirm that word-boundary safety holds for
//! real, not to re-implement it.

use craftloop_command::{CommandNamespace, GrammarMatch};
use craftloop_document::{Document, Note, SemanticEntity};
use craftloop_geometry::Point2;
use craftloop_ids::{CraftLoopId, NoteId};

#[test]
fn a_note_containing_numbers_and_a_command_word_substring_resolves_as_no_match() {
    let note_text = "Buy 5 pens and 3 rulers today";

    // The command grammar must not fire on this sentence just because
    // "pens" contains the real vocabulary word "pen" (Notebook's Pen
    // tool) as a substring.
    let result = craftloop_command::resolve(note_text, CommandNamespace::Notebook);
    assert_eq!(
        result,
        GrammarMatch::NoMatch,
        "a full sentence must never resolve as a command merely because a vocabulary word appears inside a longer word"
    );

    // A bare numeric-looking prefix inside the sentence is not enough to
    // misroute it as a dimension either -- this scenario is specifically
    // about the sentence staying a *note*, not about numeric routing,
    // which Task 214 already covers for genuine standalone numeric
    // input.
    let mut document = Document::new("Scenario 220", 0.0);
    let page_id = document.active_page().unwrap();
    let note = Note::new(NoteId::new(), Point2::new(0.0, 0.0), note_text);
    document
        .page_mut(page_id)
        .unwrap()
        .insert(SemanticEntity::Note(note))
        .unwrap();

    let stored = document
        .page(page_id)
        .unwrap()
        .entities()
        .find_map(|e| match e {
            SemanticEntity::Note(n) => Some(n),
            _ => None,
        })
        .expect("the note must be stored as a Note entity, not converted into anything else");
    assert_eq!(stored.text, note_text);
}

#[test]
fn every_notebook_vocabulary_word_embedded_in_a_longer_word_still_falls_through_to_no_match() {
    // Systematic version of the scenario above: for every real Notebook
    // command word, a sentence containing it only as a substring of a
    // longer word must never resolve as that command.
    let cases = [
        ("The pencil is on the desk", "pen"),
        ("selection bias is a real concern", "select"),
        ("sketchy plans for tomorrow", "sketch"),
        ("erasers wear out fast", "eraser"),
    ];
    for (sentence, embedded_word) in cases {
        assert!(
            sentence.contains(embedded_word),
            "test setup error: {sentence:?} should contain {embedded_word:?}"
        );
        let result = craftloop_command::resolve(sentence, CommandNamespace::Notebook);
        assert_eq!(
            result,
            GrammarMatch::NoMatch,
            "sentence {sentence:?} must not resolve as a command"
        );
    }
}
