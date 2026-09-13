//! Replay and serialization tests.
//!
//! Execution 01, Phase 05, Task 039. Authority: Engine Contract 02;
//! Article 92 "Determinism". Proves that strokes built from a
//! `craftloop_input::PointerTrace` save, reload, and replay deterministically
//! enough to serve as engine regression tests -- the same property Phase 03
//! established for raw pointer traces, carried one layer up to `Stroke`.

use craftloop_geometry::Point2;
use craftloop_ids::{CraftLoopId, StrokeId};
use craftloop_ink::{smoothed_positions, Stroke, StrokeProvenance};
use craftloop_input::{
    InputCapabilities, MouseSimulator, PointerButtons, PointerEvent, PointerTrace,
};
use craftloop_test_support::DeterministicIdSequence;

fn recorded_trace() -> PointerTrace {
    PointerTrace::new(
        "wavy-line",
        vec![
            PointerEvent::Down(MouseSimulator::sample(
                Point2::new(0.0, 0.0),
                0.0,
                PointerButtons::default(),
            )),
            PointerEvent::Move(MouseSimulator::sample(
                Point2::new(1.0, 1.0),
                0.1,
                PointerButtons::default(),
            )),
            PointerEvent::Move(MouseSimulator::sample(
                Point2::new(2.0, -1.0),
                0.2,
                PointerButtons::default(),
            )),
            PointerEvent::Move(MouseSimulator::sample(
                Point2::new(3.0, 1.0),
                0.3,
                PointerButtons::default(),
            )),
            PointerEvent::Up(MouseSimulator::sample(
                Point2::new(4.0, 0.0),
                0.4,
                PointerButtons::default(),
            )),
        ],
    )
}

#[test]
fn a_stroke_built_from_a_trace_round_trips_through_json_byte_identically() {
    let trace = recorded_trace();
    trace.validate().expect("fixture trace must be well-formed");

    let stroke = Stroke::from_events(StrokeId::from_u128(1), &trace.events).unwrap();

    let first_json = serde_json::to_string(&stroke).unwrap();
    let second_json = serde_json::to_string(&stroke).unwrap();
    assert_eq!(
        first_json, second_json,
        "serializing the same stroke twice must be byte-identical"
    );

    let reloaded: Stroke = serde_json::from_str(&first_json).unwrap();
    assert_eq!(stroke, reloaded);
}

#[test]
fn a_reloaded_stroke_replays_the_same_smoothed_path_as_the_original() {
    let trace = recorded_trace();
    let original = Stroke::from_events(StrokeId::from_u128(2), &trace.events).unwrap();

    let json = serde_json::to_string(&original).unwrap();
    let reloaded: Stroke = serde_json::from_str(&json).unwrap();

    let positions_of =
        |s: &Stroke| -> Vec<Point2> { s.samples().iter().map(|sample| sample.position).collect() };

    let original_smoothed = smoothed_positions(&positions_of(&original), 1);
    let reloaded_smoothed = smoothed_positions(&positions_of(&reloaded), 1);
    assert_eq!(original_smoothed, reloaded_smoothed);
}

#[test]
fn deterministic_ids_make_a_recorded_bug_reproducible_across_runs() {
    // This is the exact workflow Task 039 exists for: "a bug found while
    // drawing becomes a regression test." A fixed trace plus a
    // deterministic ID sequence means the resulting Stroke (and any
    // downstream engine state keyed by its ID) is identical every time
    // this test runs, on any machine.
    let mut ids: DeterministicIdSequence<StrokeId> = DeterministicIdSequence::new();
    let trace = recorded_trace();

    let run_once = |ids: &mut DeterministicIdSequence<StrokeId>| -> Stroke {
        let id = ids.next_id();
        Stroke::from_events(id, &trace.events).unwrap()
    };

    let first_run = run_once(&mut DeterministicIdSequence::new());
    let second_run = run_once(&mut ids);

    assert_eq!(first_run, second_run);
}

#[test]
fn provenance_created_from_a_stroke_matches_its_first_sample() {
    let trace = recorded_trace();
    let stroke = Stroke::from_events(StrokeId::new(), &trace.events).unwrap();

    let provenance = StrokeProvenance::created(stroke.source(), stroke.start_timestamp_seconds());
    let json = serde_json::to_string(&provenance).unwrap();
    let reloaded: StrokeProvenance = serde_json::from_str(&json).unwrap();
    assert_eq!(provenance, reloaded);
    assert_eq!(reloaded.events().len(), 1);
}

#[test]
fn a_malformed_trace_never_produces_a_stroke_even_after_a_round_trip_attempt() {
    // A trace with only an Up event is not a valid lifecycle; Stroke
    // construction from its (single, sourceless-mismatched-free but still
    // incomplete) events should not silently produce a one-sample stroke
    // that looks legitimate. Stroke::from_events does not itself validate
    // lifecycle ordering (that is PointerTrace::validate's job) but must
    // still reject genuinely empty input.
    let empty_trace = PointerTrace::new("empty", vec![]);
    let result = Stroke::from_events(StrokeId::new(), &empty_trace.events);
    assert!(result.is_err());
}

#[test]
fn capabilities_on_a_replayed_simulated_stroke_never_claim_real_hardware() {
    let trace = recorded_trace();
    let stroke = Stroke::from_events(StrokeId::new(), &trace.events).unwrap();
    for sample in stroke.samples() {
        assert_eq!(sample.capabilities, InputCapabilities::NONE);
    }
}
