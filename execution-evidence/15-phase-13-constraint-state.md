# Phase 13 — Constraint State and Feasible Geometry — Evidence

Recorded: 2026-09-13

## Task 093 — Degree-of-freedom interpretation

Extended `craftloop_constraint::ConstraintSolver` with a new default-empty
trait method, `underconstrained_variables` (`crates/craftloop-constraint/
src/solver.rs`): the raw, backend-reported freedom data a higher domain
layer translates into user-facing state, never solver jargon by default.
`EzpzSolver` (`crates/craftloop-sketch/src/ezpz_adapter.rs`) implements it
using `ezpz::solve_analysis`'s own `FreedomAnalysis::underconstrained()` --
a real, already-existing solver capability, not a hand-rolled DOF-count
heuristic (the same "prefer a demonstrated capability over reinventing
one" judgment Phase 11's solver evaluation made).

`crates/craftloop-sketch/src/state.rs`'s new `DegreesOfFreedomState` enum
(Unknown/Free/PartiallyConstrained/FullyDetermined/Conflicting, MCP
Article 28's vocabulary minus the dimension-role states already owned by
`craftloop-dimension`) and `Sketch::degrees_of_freedom` compute this per
primitive.

**Real finding**: the first version of the `underconstrained_variables`
test placed a distance-constrained point exactly on the x-axis from its
anchor (4.0, 0.0) and asserted both its x and y components would be
reported free; only y came back. Investigated and found correct, not a
bug: freedom analysis is a *local* (linearized) judgment, and at that
exact axis-aligned point the only locally-free direction (the circle's
tangent) is purely vertical. Off that degenerate initial guess (matching
`ezpz`'s own documented example), both components correctly show up.
Fixed the test to use a non-degenerate guess and documented the caveat in
both the test and `state.rs`.

## Task 094 — Under-constrained state

`DegreesOfFreedomState::is_healthy_for_exploration()` returns `true` for
`Free`/`PartiallyConstrained`/`FullyDetermined`, `false` only for
`Conflicting` -- Article 29's "under-constrained is not necessarily an
error" made directly testable rather than left as a design intention.

## Task 095 — Fully determined state

`DegreesOfFreedomState::FullyDetermined`: every point of a primitive
present in `underconstrained_variables`' complement. Tested with a line
anchored at both endpoints via two `Fixed` constraints (Task 093's own
new `SketchConstraintKind::Fixed`, actually Phase 12 infrastructure,
exercised here).

## Task 096 — Redundant constraint detection

`SketchConstraintKind::is_equivalent` (`constraint_kind.rs`): literal
duplicate detection, argument-order-normalized for genuinely symmetric
relationships (`Parallel`, `Perpendicular`, `EqualLength`, `EqualRadius`,
`Concentric`, `CircleTangentToCircle`, `Coincident`, `Symmetric`'s `a`/`b`)
but not for asymmetric ones (`LineTangentToCircle`'s line/circle are
distinct roles). `Sketch::add_constraint` now returns
`DomainResult<ConstraintOutcome>` (`Added` or `Redundant { existing }`,
with a plain-language `explain()`) instead of `DomainResult<()>` --
Article 312's "should not necessarily create an error" is the return
type, not a special-cased error variant. Deliberately narrower than
general symbolic redundancy (noticing two lines are *already* forced
parallel through an unrelated chain) -- that needs graph/symbolic
reasoning no task in this phase asks for; literal duplicate rejection is
exactly Task 096's stated objective.

## Task 097/098 — Bounded dimension state / triangle feasibility

`crates/craftloop-dimension/src/feasible_range.rs` (new):
`TriangleSideRange::for_two_fixed_sides(a, b)` computes the strictly-open
feasible range `(|a-b|, a+b)` for a triangle's third side, and
`triangle_third_side_is_feasible(a, b, c)`. Deliberately kept separate
from `SemanticDimension::with_feasible_range`'s *inclusive* `[min, max]`
(Phase 10) rather than silently reconciling the inclusive/exclusive
mismatch -- documented as a real distinction a caller wiring this into a
`Bounded` dimension must handle explicitly.

Task 098's exact regression: `triangle_third_side_is_feasible(30.0, 50.0,
300.0)` is `false` -- MCP Article 27's own worked example ("300 cannot
form this triangle while the other two sides remain 30 and 50"), plus
boundary-value coverage (20.0 and 80.0 both infeasible/degenerate,
20.0001/79.9999 feasible, equal-side and invalid-input cases).

## Task 099 — State-to-diagnostic mapping

Split across the two crates that actually own each half of the named
vocabulary, rather than one new unified enum duplicating either:
- `DegreesOfFreedomState::explain()` (craftloop-sketch): "free",
  "conflicting" (plus Unknown/PartiallyConstrained/FullyDetermined).
- `DimensionRole::explain()` (craftloop-dimension): "derived", "shared",
  "bounded" (plus Driving/Reference).

Both checked by a test asserting no jargon terms ("degree of freedom",
"solver", "driving dimension") leak into the explanation text -- Article
99's "prefer 'Depth is still unknown' over 'One degree of freedom
remains'" made mechanically enforced, not just aspirational.

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 474/474 passing (24 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean (1 finding fixed: type_complexity)
```

Full output: `test-reports/phase-13-cargo-test.txt`,
`test-reports/phase-13-cargo-clippy.txt`,
`test-reports/phase-13-cargo-fmt.txt`.

## Deferred, explicitly

- General symbolic constraint redundancy (beyond literal duplicates) --
  Task 096's stated objective is duplicate avoidance, not full graph
  reasoning.
- Wiring `TriangleSideRange` into `SemanticDimension::with_feasible_range`
  automatically -- the inclusive/exclusive boundary mismatch needs a
  deliberate design decision (widen slightly? reject at the boundary
  regardless?) no task this phase asks for; left as a documented, real gap
  for whichever later phase actually wires dimension roles into live
  constraint solving.
- Multiple-solution/mirrored-branch handling (Article 310) -- no task in
  this phase names it; the solver already "preserves the branch closest
  to the current geometry" by construction (every solve warm-starts from
  current primitive state), but no explicit flip/alternate-solution action
  exists yet.

## Phase Gate

- All six tasks (093-099, note there is no 100 skipped -- the phase has
  Tasks 093 through 099) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 474/474 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: the backend-neutral `ConstraintSolver` trait
  gained one default-empty method (any existing implementer, including
  `ResidualChecker`, compiles unchanged); no new crate dependency was
  added anywhere.
- Implemented and tested: DOF state classification, redundant-constraint
  detection, triangle-inequality feasible ranges (exact MCP worked
  example + boundaries), jargon-free state explanations.
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 14 automatically.
