//! Generates `scenarios/example-line.json`: a minimal, valid scenario
//! fixture usable with the harness's "Load scenario" panel and as a
//! reference for hand-authoring further scenarios.
//!
//! Run with: `cargo run -p windows-harness --example generate_scenario`

use craftloop_geometry::Point2;
use craftloop_input::{
    InputCapabilities, MouseSimulator, PointerButtons, PointerEvent, PointerTrace,
};

fn main() {
    let events = vec![
        PointerEvent::Down(MouseSimulator::sample(
            Point2::new(0.0, 0.0),
            0.0,
            PointerButtons::default(),
        )),
        PointerEvent::Move(MouseSimulator::sample(
            Point2::new(50.0, 10.0),
            0.1,
            PointerButtons::default(),
        )),
        PointerEvent::Move(MouseSimulator::sample(
            Point2::new(100.0, 20.0),
            0.2,
            PointerButtons::default(),
        )),
        PointerEvent::Up(MouseSimulator::sample(
            Point2::new(100.0, 20.0),
            0.3,
            PointerButtons::default(),
        )),
    ];
    let trace = PointerTrace::new("diagonal-line", events);
    trace
        .validate()
        .expect("generated trace must be a valid stroke lifecycle");

    let scenario = serde_json::json!({
        "name": "example-line",
        "description": "A single simulated-mouse stroke drawn diagonally, for exercising the harness's Load scenario panel.",
        "traces": [trace],
    });

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("scenarios/example-line.json");
    let json = craftloop_serialization::to_canonical_json(&scenario).expect("serialize");
    std::fs::write(&path, json).expect("write scenario fixture");
    println!("wrote {}", path.display());

    // Also prove InputCapabilities import is meaningful documentation: the
    // generated events genuinely carry the simulator's honest NONE
    // capabilities, not a fabricated stylus profile.
    let capabilities = MouseSimulator::CAPABILITIES;
    assert_eq!(capabilities, InputCapabilities::NONE);
}
