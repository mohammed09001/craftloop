//! Execution 03, Phase 03, Task 020: proof that `wasm32-unknown-unknown`
//! runs a genuine shared-core domain operation, not just a trivial
//! arithmetic function. `solve_demo_sketch` builds a real
//! [`craftloop_sketch::Sketch`], stores a line primitive that is *not*
//! horizontal, applies a real `Horizontal` constraint to it, and solves
//! with the real `ezpz`/`faer`-backed [`EzpzSolver`] -- the single
//! highest-risk dependency in this crate's whole chain per the Phase 03
//! compatibility audit (`execution-evidence/execution-03/
//! phase-03-wasm-compatibility-audit.md`). If this compiles and the
//! solved line's endpoints actually end up at equal Y, the solver's real
//! numerical code ran on `wasm32-unknown-unknown`, not a stub.
//!
//! Phase 04's real session API lives in [`session`]; this module keeps
//! its original narrow proof rather than folding it into that session,
//! since it is still useful as a minimal, dependency-light compile/run
//! smoke check independent of the full session surface.

pub mod session;
pub mod types;

pub use session::CraftLoopSession;

use craftloop_geometry::{Point2, Segment2};
use craftloop_ids::{ConstraintId, CraftLoopId, PrimitiveId};
use craftloop_recognition::{Beautified, BeautifiedPrimitive};
use craftloop_sketch::{ConstraintProvenance, EzpzSolver, Sketch, SketchConstraintKind};
use wasm_bindgen::prelude::*;

/// Builds a two-point line whose endpoints do not share a Y coordinate,
/// constrains it `Horizontal`, solves, and returns the solved sketch as
/// canonical JSON (the same serialization
/// [`craftloop_document::persistence::save_document_atomically`] uses
/// natively -- see that module's `wasm32-unknown-unknown` doc note for
/// why this crate calls `craftloop_serialization` directly instead).
///
/// Errors surface as a rejected `Promise`-free `Result<String, JsValue>`
/// rather than panicking, since a panic across the wasm boundary leaves
/// the module in an unrecoverable trapped state for the rest of the page
/// session (Article 13: "never continue silently").
#[wasm_bindgen]
pub fn solve_demo_sketch() -> Result<String, JsValue> {
    let mut sketch = Sketch::new();

    let line_id = PrimitiveId::new();
    sketch.insert_primitive(
        line_id,
        Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(
                Point2::new(0.0, 0.0),
                Point2::new(10.0, 4.0),
            )),
            displacement: 0.0,
        },
    );

    let constraint_id = ConstraintId::new();
    sketch
        .add_constraint(
            constraint_id,
            SketchConstraintKind::Horizontal(line_id),
            ConstraintProvenance::UserCreated,
        )
        .map_err(|err| JsValue::from_str(&format!("add_constraint failed: {err}")))?;

    let mut solver = EzpzSolver::new();
    let outcome = sketch.solve(&mut solver);

    let solved = sketch
        .primitive(line_id)
        .ok_or_else(|| JsValue::from_str("solved primitive vanished"))?;

    #[derive(serde::Serialize)]
    struct DemoResult<'a> {
        solve_status: String,
        solved_primitive: &'a Beautified,
    }

    craftloop_serialization::to_canonical_json(&DemoResult {
        solve_status: format!("{:?}", outcome.status),
        solved_primitive: solved,
    })
    .map_err(|err| JsValue::from_str(&format!("serialization failed: {err}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs under the native test target too (not just
    /// `wasm-bindgen-test`), so `cargo test -p craftloop-web-bridge`
    /// catches a broken demo without needing a browser. The
    /// wasm-specific proof (this same function, actually compiled to
    /// `wasm32-unknown-unknown` and run inside real Chromium via
    /// `wasm-bindgen`'s generated glue) is manual for now -- see
    /// `execution-evidence/execution-03/phase-03-wasm-compatibility-audit.md`
    /// for the exact commands and the real browser output. A permanent,
    /// CI-runnable `wasm-bindgen-test` harness is Phase 18's job
    /// (Article 51/Task 18x, cross-platform build hardening), not
    /// duplicated ad hoc here.
    #[test]
    fn demo_sketch_actually_solves_to_horizontal() {
        let json = solve_demo_sketch().expect("demo sketch should solve");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["solve_status"], "Solved");

        let primitive = &value["solved_primitive"]["primitive"]["Line"];
        let y0 = primitive["a"]["y"].as_f64().unwrap();
        let y1 = primitive["b"]["y"].as_f64().unwrap();
        assert!(
            (y0 - y1).abs() < 1e-6,
            "expected a horizontal line after solving, got y0={y0} y1={y1}"
        );
    }
}
