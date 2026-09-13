# Phase 14 — Geometric Consistency Engine — Evidence

Recorded: 2026-09-13

## New crate: `crates/craftloop-consistency`

Deliberately does not depend on `craftloop-document`: validates the
lower-level types directly (`BeautifiedPrimitive`, `SemanticDimension`,
`SolveResult`, raw parser input), the same layering choice
`craftloop-sketch` (Phase 12) made. `craftloop-document` depends on this
crate for the `Conflict` type (Task 106), not the other way around.

## Task 100 — Conflict object schema

`conflict.rs`: `Conflict` (id, `ConflictKind`, `severity` -- reusing
`craftloop_errors::Severity` rather than a duplicate scale --
`affected_entities`, `existing_truth`, `proposed_truth`, `evidence`,
`resolution_choices`, `status`), `ConflictKind` (one variant per validator
below), `ResolutionChoice` (Task 104's five choices, matching Article
27/311/312 exactly, no speculative extras), `ConflictStatus`
(`Unresolved`/`Resolved{choice}` -- Task 105's invariant made a type: a
conflict is never silently discarded, only recorded as resolved).

## Task 101 — Local geometry consistency

`geometry_validation.rs`: `validate_primitive_geometry`. Most degeneracy
is already unconstructable (`Circle2::new`/`Arc2::new`, Phase 02, already
reject non-positive radius/zero sweep) -- the real, remaining gaps are
`Segment2` (no validation at all) and `RelationalRectangle::from_corners`
(only rejects a coincident adjacent-corner pair, not a merely
near-zero-area quadrilateral). Both checked against
`Tolerances::committed().point_coincidence` (that module's own doc
comment names this crate as its Phase 14 consumer). A test proves a valid
`Circle2` can never be flagged, since it cannot be constructed degenerate
in the first place.

## Task 102 — Dimension/constraint consistency

`dimension_validation.rs`: `validate_dimension_against_solve` coordinates
the three layers Article 23 names (parser, dimension model, solver) --
only a `Driving` dimension (which *asserts* a value) can conflict with an
unsatisfied solver diagnostic for the constraint it drives;
`Reference`/`Derived`/`Shared`/`Bounded` dimensions report geometry rather
than asserting it, so an unsatisfied constraint under one of those is
already `craftloop_sketch::DegreesOfFreedomState::Conflicting`
(Phase 13) -- a geometry fact, not a dimension-specific conflict this
module would otherwise be fabricating. A dedicated Task 105 test proves
all four non-driving roles never produce a conflict here even when the
underlying constraint is genuinely unsatisfied.

## Task 103 — Unit consistency

`unit_validation.rs`: `validate_unit_consistency` re-parses a value's
*own recorded raw input* and compares to the stored canonical value --
catching silent drift between what the user actually typed and what ended
up stored, the one thing Phase 09's parse-time "explicit unit always
wins" guarantee cannot catch on its own (that guarantee is about a single
parse; this is about a stored value staying honest to its own origin
afterward).

## Task 104 — Conflict resolution transactions

`resolution.rs`: `resolve(&mut Conflict, ResolutionChoice)`. Atomic and
one-time (mirroring `craftloop-transactions`' commit-once philosophy):
rejects (without mutating anything) a second resolution attempt or a
choice the conflict itself never offered. Added two new
`ConsistencyErrorKind` variants (`AlreadyResolved`, `ChoiceNotOffered`) --
the first real use of `DomainError::Consistency` anywhere in the
workspace; it existed since Phase 01 but had no caller until this phase.

## Task 105 — Separate unresolved from invalid

Not a separate module -- a constraint every validator above is tested
against directly: `DegreesOfFreedomState::{Free,PartiallyConstrained,
Unknown}` (Phase 13) never reach a `Conflict`; only `Circle`/`Arc`
primitives (which cannot be constructed degenerate) prove the geometry
validator's restraint; non-driving dimension roles prove the dimension
validator's restraint. `ConflictStatus`'s own shape (Task 100) is the
type-level enforcement: nothing in this crate can silently erase a
conflict's existence, only record how it was resolved.

## Task 106 — Conflict persistence tests

`craftloop-document` gained `SemanticEntity::Conflict(Conflict)` and
`EntityId::Conflict(ConflictId)` -- reusing the exact save/load/autosave
machinery every other entity kind already has (Phase 07-08), not a new
persistence mechanism. New `tests/conflict_persistence.rs`: an unresolved
conflict referencing a valid primitive survives save/reopen with the
primitive untouched and the conflict's every field (including `Unresolved`
status) intact; a resolved conflict's resolution choice survives; repeated
round-trips do not drift.

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 500/500 passing (26 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
```

Full output: `test-reports/phase-14-cargo-test.txt`,
`test-reports/phase-14-cargo-clippy.txt`,
`test-reports/phase-14-cargo-fmt.txt`.

## Deferred, explicitly

- Document-wide orchestration ("run every validator over a whole page and
  collect the results into stored `Conflict` entities automatically") --
  no task in this phase names a single "check everything" entry point;
  each validator is a real, tested function a future caller (a later
  phase's editing/UI-adjacent layer) invokes at the point an edit actually
  happens, matching how `craftloop-sketch::Sketch::solve` itself is
  invoked by a caller rather than run on a timer.
- Wiring `validate_dimension_against_solve`'s `constraint_id` parameter
  automatically from a `SemanticDimension`'s `DimensionTarget` -- that
  binding (which `ConstraintId` a given dimension drives inside a live
  `Sketch`) does not exist yet anywhere in the workspace; no task in this
  phase asks for it, and inventing it speculatively would be scope creep.
- Cross-view dimension conflicts, incompatible view labels (Article 27's
  own list includes both) -- both require Phase 20's view-block
  infrastructure, which does not exist yet.

## Phase Gate

- All seven tasks (100-106) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 500/500 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: the new crate depends only on already-existing
  crates; `craftloop-document` gained one new entity variant using its
  existing persistence path, not a new one.
- Implemented and tested: conflict schema, degenerate-geometry detection,
  dimension/solver mismatch detection, unit-drift detection, atomic
  resolution transactions, unresolved-vs-invalid separation, save/reopen
  persistence.
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 15 automatically.
