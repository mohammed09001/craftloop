//! Task 219 — Ink-command scenario.
//!
//! Execution 01, Phase 30, Task 219. Recognizes a deterministic command
//! fixture (`FixtureHandwritingRecognizer`, Phase 17), resolves it
//! against the real command grammar (Phase 19, Article 237), confirms it
//! through the real Article 238 confirmation policy, routes it through
//! `CommandBus` (Phase 19), and applies the accepted command to update a
//! small stand-in "harness state" the same way a real adapter's state
//! module would react to `CommandBus::history()` -- this crate cannot
//! import `apps/windows-harness`'s own private `state` module (a
//! different binary crate), so the wiring pattern is reproduced
//! honestly rather than the exact private struct.

use std::collections::BTreeMap;

use craftloop_command::grammar::{resolve, GrammarMatch};
use craftloop_command::{
    evaluate_confirmation, Command, CommandAction, CommandBus, CommandNamespace, CommandSource,
    ConfirmationEvidence, ConfirmationOutcome, RiskLevel, UndoMetadata,
};
use craftloop_handwriting::{FixtureHandwritingRecognizer, HandwritingRecognizer, TextCandidate};
use craftloop_ids::{CommandId, CraftLoopId, StrokeId};
use craftloop_ink::Stroke;
use craftloop_input::{MouseSimulator, PointerButtons};
use craftloop_recognition::Confidence;

/// A minimal stand-in for a real adapter's tool/mode state (mirrors the
/// shape `apps/windows-harness`'s own state module reacts to
/// `CommandBus::history()` with), local to this test so it can prove the
/// wiring pattern without importing a different binary crate's private
/// module.
#[derive(Debug, PartialEq)]
enum ToyHarnessMode {
    Notebook,
    Sketch,
}

fn apply_accepted_commands(commands: &[Command]) -> ToyHarnessMode {
    let mut mode = ToyHarnessMode::Notebook;
    for command in commands {
        if command.action == CommandAction::Sketch {
            mode = ToyHarnessMode::Sketch;
        }
    }
    mode
}

#[test]
fn a_recognized_sketch_command_is_confirmed_routed_and_updates_harness_state() {
    // 1. Deterministic command-word fixture.
    let stroke = Stroke::new(
        StrokeId::new(),
        vec![MouseSimulator::sample(
            craftloop_geometry::Point2::ORIGIN,
            0.0,
            PointerButtons::default(),
        )],
    )
    .unwrap();
    let mut recognizer = FixtureHandwritingRecognizer::new().with_fixture(
        std::slice::from_ref(&stroke),
        vec![TextCandidate::new("sketch", Confidence::new(0.95))],
    );
    let candidates = recognizer.recognize(std::slice::from_ref(&stroke));
    let text = &candidates[0].text;

    // 2. Real command grammar (Article 237).
    let grammar_match = resolve(text, CommandNamespace::Notebook);
    let action = match grammar_match {
        GrammarMatch::Exact(action) | GrammarMatch::UniquePrefix(action) => action,
        other => panic!("expected a real match for \"sketch\", got {other:?}"),
    };
    assert_eq!(action, CommandAction::Sketch);

    // 3. Real confirmation policy (Article 238) -- every signal
    //    deliberately positive, so this exercises the "confirm" step
    //    for real rather than trivially.
    let confirmation = evaluate_confirmation(ConfirmationEvidence {
        content_is_recent: true,
        content_is_a_valid_command: true,
        enclosure_ratio: 0.9,
        enclosure_looks_deliberate: true,
        context_would_treat_as_geometry: false,
        circle_to_command_disabled_by_user: false,
    });
    assert_eq!(confirmation, ConfirmationOutcome::Execute);

    // 4. Route through the real Command Bus.
    let command = Command {
        id: CommandId::new(),
        action,
        source: CommandSource::InkCommand,
        namespace: CommandNamespace::Notebook,
        parameters: BTreeMap::new(),
        risk: RiskLevel::Medium,
        timestamp_seconds: 1.0,
        undo: UndoMetadata::undoable("Entered Sketch mode"),
    };
    let mut bus = CommandBus::new();
    let command_id = bus.submit(command, Some(confirmation)).unwrap();
    assert_eq!(bus.history().len(), 1);
    assert_eq!(bus.history()[0].id, command_id);

    // 5. Update harness state from the accepted command history.
    let mode = apply_accepted_commands(bus.history());
    assert_eq!(mode, ToyHarnessMode::Sketch);
}

#[test]
fn an_unconfirmed_medium_risk_command_never_reaches_the_bus_history() {
    // The same command, but confirmation evidence says the ink remains
    // ordinary geometry (e.g. the active context would already treat it
    // as such) -- the bus must reject it outright, and harness state
    // must never update from a rejected command.
    let confirmation = evaluate_confirmation(ConfirmationEvidence {
        content_is_recent: true,
        content_is_a_valid_command: true,
        enclosure_ratio: 0.9,
        enclosure_looks_deliberate: true,
        context_would_treat_as_geometry: true,
        circle_to_command_disabled_by_user: false,
    });
    assert_eq!(confirmation, ConfirmationOutcome::RemainsInk);

    let command = Command {
        id: CommandId::new(),
        action: CommandAction::Sketch,
        source: CommandSource::InkCommand,
        namespace: CommandNamespace::Notebook,
        parameters: BTreeMap::new(),
        risk: RiskLevel::Medium,
        timestamp_seconds: 1.0,
        undo: UndoMetadata::undoable("Entered Sketch mode"),
    };
    let mut bus = CommandBus::new();
    assert!(bus.submit(command, Some(confirmation)).is_err());
    assert!(bus.history().is_empty());
    assert_eq!(
        apply_accepted_commands(bus.history()),
        ToyHarnessMode::Notebook
    );
}
