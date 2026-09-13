# Phase 15 — Design Intent Graph — Evidence

Recorded: 2026-09-13

## Design decision: no new parallel model

MCP Article 26 defines design intent as "the set of relationships the
user expects to remain true when the drawing changes." That is exactly
what `craftloop-sketch::Sketch`'s stored constraints
(`SketchConstraintKind` + `ConstraintProvenance`, Phase 12) already are:
a confirmed relationship the solver re-enforces on every `solve()` call,
tagged with who asserted it. Rather than invent a second "intent
relation" record duplicating that storage, Phase 15 adds a new module,
`crates/craftloop-sketch/src/intent.rs`, that queries and curates the
*existing* constraint graph. This was a real design decision, not an
assumption -- the alternative (a parallel `IntentRelation` type) was
considered and rejected as exactly the duplicated-truth shortcut this
workspace's conventions forbid.

## Task 107 — Intent relation records

Satisfied by the existing `Sketch` constraint store itself: every
`add_constraint` call already records a persistent relationship with
provenance. What was missing, and what this phase adds, is proof that it
stays put across edits (Task 112, below) and query access to it (Task
111).

## Task 108 — Observed coincidence vs. intended relation

`Sketch::observed_length_coincidences()`: lines whose current lengths are
merely close (within `Tolerances::recognition().length_equality`) but
have no stored `EqualLength` constraint. Purely advisory data for a
future suggestion engine -- a dedicated test
(`near_equal_length_lines_with_no_constraint_are_reported_as_observed_only`)
proves that observing a coincidence never itself creates a constraint.

## Task 109 — Bind accepted suggestions into intent

`Sketch::accept_suggestion(id, kind)`: identical to `add_constraint` with
`ConstraintProvenance::AcceptedSuggestion`, proven to be enforced by the
real `ezpz` solver exactly like a user-drawn relationship (the solver
never branches on provenance) via
`an_accepted_suggestion_is_enforced_by_solve_exactly_like_a_user_created_constraint`.
Accepting also clears any prior rejection of the same relationship.

## Task 110 — Preserve rejected suggestions

`Sketch` gained a `rejected_suggestions: Vec<SketchConstraintKind>` field
plus `reject_suggestion`/`is_suggestion_rejected`, reusing Task 096's
`is_equivalent` so a re-proposed relationship (in any argument order for
symmetric kinds) is recognized as "already declined" rather than
duplicated or re-nagging.

## Task 111 — Query intent dependencies

`Sketch::constraints_touching(primitive)` ("which relations control this
entity") and `Sketch::affected_primitives(primitive)` (transitive BFS/DFS
over shared constraints -- "what would changing this entity affect"),
tested including a three-primitive chain proving transitivity and an
isolated primitive proving an empty result rather than a panic.

## Task 112 — Edit-propagation tests

Three tests prove confirmed intent is never silently dropped: re-solving
five times in a row leaves the stored constraint and its
`constraints_touching` entry intact each time; inserting an unrelated new
primitive does not disturb an existing relation's provenance; removing
one constraint never removes a different one. All pass because
`Sketch::solve` (Phase 12) never touches `self.constraints` at all -- only
`apply_solution` writes back primitive geometry.

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 514/514 passing (14 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
```

Full output: `test-reports/phase-15-cargo-test.txt`,
`test-reports/phase-15-cargo-clippy.txt`,
`test-reports/phase-15-cargo-fmt.txt`.

## Deferred, explicitly

- Persisting the intent graph (constraints + rejected suggestions)
  through `craftloop-document`'s save/reopen path, the way Conflicts were
  wired in Phase 14 -- no task in this phase names document persistence
  specifically (unlike Task 106 for conflicts last phase), and `Sketch`
  itself is not yet a document-held object at all (that wiring decision
  belongs to whichever later phase actually integrates the constraint
  engine into the live document, not this one). `SketchConstraintKind`/
  `ConstraintProvenance`/`PointRef` are already `Serialize`/`Deserialize`
  in anticipation of that, so no rework is needed when it happens.
- Automatically feeding `observed_length_coincidences()` into a real
  suggestion UI/engine -- Phase 06's recognition engine and this query are
  now both real, but no task connects them yet; that is a future
  suggestion-engine phase's job.

## Phase Gate

- All six tasks (107-112) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 514/514 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no new crate, no new dependency; everything
  builds on Phase 12's existing `Sketch`/constraint storage.
- Implemented and tested: intent queries, coincidence-vs-intent
  separation, accepted-suggestion enforcement, rejection memory,
  edit-propagation durability.
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 16 automatically.
