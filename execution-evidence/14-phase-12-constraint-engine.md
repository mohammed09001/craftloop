# Phase 12 — Constraint Engine Integration — Evidence

Recorded: 2026-09-13

## Spike extension before implementation (Tasks 089/090 gate)

Tasks 089 ("implement tangency if solver quality passes") and 090
("implement symmetry if scope remains safe") are explicitly conditioned on
spike evidence the Phase 11 spike had not gathered (it only exercised
`Fixed`/`Distance`/`Horizontal`/`Vertical`/`HorizontalDistance`/
`VerticalDistance`). Before writing any Phase 12 production code, three
new real scenarios were added to the Phase 11 spike
(`execution-evidence/solver-evaluations/ezpz-spike/src/main.rs`) and run
for real against `ezpz`:

| Scenario | `ezpz` constraint | Result |
|---|---|---|
| Vertical line tangent to a fixed circle | `LineTangentToCircle` | `is_satisfied: true`, converged in 1 iteration |
| Circle externally tangent to a fixed circle | `CircleTangentToCircle` | `is_satisfied: true`, converged in 1 iteration |
| Point reflected across a fixed axis | `Symmetric` | `is_satisfied: true`, converged in 1 iteration |

All three converged to numerically correct answers (~1e-9 error) with no
stability issues or degenerate-input warnings. Recorded as an addendum to
`solver-evaluations/solver-decision-record.md`. **Decision: Tasks 089 and
090 approved for unconditional implementation this phase.** Raw output:
`solver-evaluations/ezpz-spike-output.txt` (regenerated to include the
three new scenarios).

## New crate: `crates/craftloop-sketch`

The real "Constraint Engine Integration" this phase's title names. Three
layers:

1. **`ezpz_adapter.rs`** — `EzpzSolver`, a real implementation of
   `craftloop_constraint::ConstraintSolver` against `ezpz` (pinned
   `= "=0.2.29"` per Task 083's decision record). Translates every
   `GeometricConstraint` variant (all 14, including the four new ones
   below) into `ezpz::Constraint`. One non-trivial translation: `Radius`/
   `LineTangentToCircle`/`CircleTangentToCircle` represent a circle's
   radius as a center-to-boundary-point distance (no dedicated scalar),
   but `ezpz`'s `DatumCircle` requires exactly a scalar `DatumDistance` --
   so the two tangency variants lower to *two* `ezpz::Constraint`s each: a
   synthetic radius variable tied to the real points via
   `ezpz::Constraint::DistanceVar`, plus the tangency constraint itself.
   Diagnostics reuse Phase 11's already-demonstrated mitigation for
   `ezpz`'s missing per-constraint residual API: ignore `ezpz`'s own
   satisfaction bookkeeping entirely and re-evaluate
   `craftloop_constraint::residual` (independent of `ezpz`) at the solved
   values, once per original `ConstraintRequest`.
2. **`point_ref.rs` / `constraint_kind.rs`** — bind the variable-level
   vocabulary to real `craftloop_recognition::BeautifiedPrimitive`
   geometry (`Line`/`Circle`/`Rectangle` -- `Arc` deliberately deferred,
   see below). `SketchConstraintKind` has 11 variants: `Fixed` (anchor
   infrastructure, see below), `Coincident` (084), `Horizontal`/`Vertical`
   (085), `Parallel`/`Perpendicular` (086), `EqualLength`/`EqualRadius`
   (087), `Concentric` (088), `LineTangentToCircle`/`CircleTangentToCircle`
   (089), `Symmetric` (090).
3. **`sketch.rs`** — `Sketch`: owns primitives + constraints (each with
   provenance, Task 091), validates every constraint against real
   primitive kinds at `add_constraint` time (never mid-solve), and
   `solve()` translates + calls a real `ConstraintSolver` + writes the
   solution back into stored primitives ("maintain the relation through
   edits", 084; "stable drag behavior", 085 -- guesses always come from
   current primitive state, so a re-solve after an external edit is a
   natural warm start).

`provenance.rs` — `ConstraintProvenance` (`UserCreated`/
`AcceptedSuggestion`/`Derived`), a small constraint-specific enum
deliberately *not* reusing `craftloop-document::ProvenanceState` (that
type's `Propagated`/`Rejected` describe a document entity's recognition
lifecycle, a different question).

### `Fixed`, infrastructure not named by Tasks 084-090

Every one of Tasks 084-090's relationships is *relative* (two points
coincide, a line is horizontal, two circles share a center...). A system
built only from relative constraints has no unique solution -- there is
nothing to anchor it in space. Task 084's "maintain the relation through
edits" and Task 085's "stable drag behavior" both presuppose an
authoritative point (the one being dragged) while everything else solves
around it. `SketchConstraintKind::Fixed(PointRef, Point2)` (lowering to
two `GeometricConstraint::FixedValue`s, already defined in Phase 11) is
the minimal, necessary infrastructure this requires -- not a new named
relationship, and not scope creep beyond what 084/085 already imply.

### `Concentric`/`EqualRadius` deliberately do not get new
`GeometricConstraint` variants

`Concentric` is numerically identical to `Coincident` on two centers;
`EqualRadius` is numerically identical to `EqualLength` on two
center/point-on-circle pairs. Both lower to the existing variant rather
than duplicating it at the backend-neutral layer -- the domain distinction
(what the user asked for, what provenance/undo remembers) lives only in
`SketchConstraintKind`.

### Deferred, explicitly

- **Arc points** (`Arc2::start_point()`/`end_point()`): derived from
  `center`/`radius`/`start_angle`/`sweep_angle`, not stored. Writing a
  solved point back would need inverse-angle math no task in this phase
  asks for. No 084-090 task mentions arcs. Recorded as a real, safe-to-defer
  gap, not a silent omission.
- **Internal circle tangency** (one circle inside another): `ezpz`
  supports it via `CircleSide::Interior`, but no Version 1 scenario needs
  it and the spike only exercised external tangency. `GeometricConstraint::
  CircleTangentToCircle`'s doc comment records this explicitly.
- Solver-priority-based provenance weighting (e.g. `UserCreated`
  outranking `Derived` during solve): Task 091 asks only to *distinguish*
  provenance as data, not to change solve behavior. Not implemented.

## RED-GREEN findings (Task 092)

Writing the invalid/conflict-case tests surfaced one genuine, worth-recording
finding, exactly like Phase 11's sign-convention discovery: a test asserting
`Horizontal(line)` + `Vertical(line)` together would be unsatisfiable
**failed** -- correctly. Both conditions are simultaneously satisfiable by
collapsing the line to a single point (`a == b`), which is a real,
consistent (if degenerate) solution, not a contradiction. The test was
corrected to assert the true property (`craftloop-sketch/src/sketch.rs`,
`horizontal_and_vertical_together_are_not_a_contradiction_they_collapse_the_line_to_a_point`),
and a genuinely unsatisfiable case (two different `Fixed` values on the same
point) was added alongside it to still cover Task 092's "invalid/conflict
cases" requirement.

Other invalid/conflict cases covered: unknown-primitive rejection,
wrong-primitive-kind rejection (`Horizontal` on a circle, `Concentric` on a
line), duplicate-`ConstraintId` rejection, unknown-`ConstraintId` removal
rejection, and a genuinely contradictory `Fixed`/`Fixed` pair reported
`Unsatisfied` (never a panic, never silently `Solved`).

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 450/450 passing (49 new)
cargo fmt --all -- --check                               # clean
cargo clippy --workspace --all-targets -- -D warnings     # clean, no findings
```

Full output: `test-reports/phase-12-cargo-test.txt`,
`test-reports/phase-12-cargo-clippy.txt`,
`test-reports/phase-12-cargo-fmt.txt`,
`solver-evaluations/ezpz-spike-output.txt` (regenerated),
`solver-evaluations/solver-decision-record.md` (addendum).

## Phase Gate

- All nine tasks (084-092, including the two conditionally-gated ones)
  represented in repository code, backed by real solved systems against
  the real `ezpz` backend -- not the reference `ResidualChecker` alone.
- `cargo test --workspace`: 450/450 passing (49 new: 12 in
  `craftloop-constraint` for the 4 new `GeometricConstraint` variants, 1 in
  `craftloop-errors` for `SketchErrorKind`, 36 in the new `craftloop-sketch`
  crate). `cargo fmt --check` and `cargo clippy -D warnings`: clean.
- No scope boundary crossed: `craftloop-constraint` (the backend-neutral
  interface) gained only vocabulary, never an `ezpz` dependency; the real
  `ezpz` binding lives entirely in the new `craftloop-sketch` crate, which
  is where Task 083's decision record authorized it to land.
- Implemented and tested: coincident/horizontal/vertical/parallel/
  perpendicular/equal-length/equal-radius/concentric/tangency/symmetry, all
  solved against the real `ezpz` backend with passing assertions on the
  resulting geometry, not just "the solver returned `Solved`".
- Implemented but not hardware-validated: none this phase (no
  platform-specific code was touched).
- Deferred: arc point references, internal circle tangency,
  provenance-weighted solve priority -- all named above with reasons.
- Blocked with evidence: none.
- Proceeding to Phase 13 automatically.
