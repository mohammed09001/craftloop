# Phase 18 — Dimension Association Engine — Evidence

Recorded: 2026-09-13

Built inside the existing `crates/craftloop-dimension` crate (new
`association.rs` module) rather than a new crate: this phase produces a
`DimensionTarget`, a type that crate already owns, and needed no
dependency `craftloop-dimension` didn't already have except
`craftloop-recognition` (for `BeautifiedPrimitive`/`Confidence`, both
already-established, reused types).

## Tasks 125/126 — Explicit selection and dimension-guide association

`associate()`'s first two evidence tiers: an explicit selection or a
dimension-guide-bound target, if present and in-scope, short-circuits
ranking entirely at maximum (explicit selection, `Confidence::ONE`) or
near-maximum (guide binding, `0.95`) confidence. Tested including the
case where both are present -- explicit selection still wins.

## Task 127 — Proximity ranking, never sole authority

Reuses `craftloop_recognition::confidence::confidence_from_residual`
(Phase 06) rather than a duplicate decay function. "Never sole authority"
is enforced structurally, not by convention: the only code path that
produces a `ProximityAndOrientation` result always blends proximity with
orientation (`proximity * 0.7 + orientation * 0.3`) -- there is no
function that returns a proximity-only score.
`proximity_alone_never_produces_a_result_the_engine_cannot_also_explain_with_orientation`
proves a distance-zero placement still doesn't reach full confidence when
orientation evidence is neutral.

## Task 128 — Directional evidence

`orientation_alignment_score`: `|sin(angle)|` between the text's offset
from a line/rectangle's center and the primitive's own direction -- `1.0`
for a conventional perpendicular-offset placement, `0.0` for text sitting
directly along the primitive's own axis. Circles/arcs (no preferred
orientation) get an honest neutral `0.5` rather than a fabricated
direction.
`a_perpendicular_offset_can_outrank_a_closer_but_in_line_offset` is a
real, worked geometric case (confidence 0.888 vs 0.658, computed and
verified by hand before writing the test) proving directional evidence
can outrank raw distance, not just tie-break it.

## Task 129 — Scope to View Block

Scoped by `craftloop_ids::ViewId` -- the concrete `ViewBlock` type this
belongs to is Phase 20's job and does not exist yet, but the stable ID
type already does (Phase 01), which is enough to express real,
non-speculative scoping now. A candidate outside the active view is
excluded *before* proximity/orientation ever run (a hard filter, not
evidence that could be outweighed), including the edge case of a
candidate with no view at all once a view is active.

## Task 130 — Medium-confidence target preview

`AssociationResult::is_committable`/`needs_preview`, reusing the same
`0.6`/`0.3` threshold convention established in Phase 16 (redefined
locally since this crate doesn't depend on `craftloop-ink-intent`, a
different domain -- same numeric value, same Article 96 justification).

## Task 131 — Reassignment transaction

`DimensionStore::reassign_target` (in the existing `store.rs`, matching
`edit_driving_value`'s established atomic-and-returns-the-previous-value
shape): moves a dimension to a new target without touching its value,
role, kind, or annotations, and without ever deleting/recreating it.

## Task 132 — Wrong-target regression suite

`tests/wrong_target_regression.rs`: two genuinely ambiguous, equally
plausible lines are both surfaced with a non-committable top confidence
rather than the engine silently picking one; an explicit selection
prevents a visually-closer wrong candidate from ever being proposed; and
an end-to-end scenario simulates a dimension that already committed to
the wrong target and corrects it via `reassign_target`, verifying the
value (42.5), role, and annotation all survive the fix untouched.

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 561/561 passing (17 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
```

Full output: `test-reports/phase-18-cargo-test.txt`,
`test-reports/phase-18-cargo-clippy.txt`,
`test-reports/phase-18-cargo-fmt.txt`.

## Deferred, explicitly

- Binding this engine to a concrete `ViewBlock` type -- Phase 20's job;
  `ViewId` scoping already in place is forward-compatible with it.
- Calibrating `PROXIMITY_SCALE_MM` against real device/page-scale data --
  named honestly as an uncalibrated placeholder, matching
  `Tolerances::recognition()`'s own precedent from Phase 02.
- Wiring `associate()`'s output automatically into `craftloop-handwriting`'s
  routing output (Phase 17) or into a live `craftloop-sketch::Sketch` --
  no task this phase asks for that end-to-end integration; each engine
  remains independently usable and tested, the same layering choice every
  phase since Phase 12 has made.

## Phase Gate

- All eight tasks (125-132) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 561/561 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no new crate; the one new dependency
  (`craftloop-recognition`) was already depended on transitively by nothing
  circular (it does not depend on `craftloop-dimension`).
- Implemented and tested: every evidence tier (selection, guide,
  proximity, orientation), view scoping, preview/commit thresholds,
  atomic reassignment, and the wrong-target regression suite.
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 19 automatically.
