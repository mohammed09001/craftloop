# Phase 22 — Multiview Constraint Graph — Evidence

Recorded: 2026-09-13

Built inside `crates/craftloop-document` (new `multiview.rs`). Core
design decision: a shared axis variable is not a new value type -- it is
a `craftloop_dimension::DimensionId` whose role is already `Shared`
(Phase 10). Multiple views binding to the *same* `DimensionId` is what
makes them share a value; there is nothing to copy or keep synchronized,
so Article 36's "Dimension anywhere. Resolve everywhere" and Article
231's "a change in a shared dimension should update only affected
subgraphs" both fall out of the representation itself.

## Task 157/158 — Shared axis/extent variables and view-to-variable mapping

`SharedAxis::{Width,Height,Depth}` and `axes_for_identity`: exactly
Article 35's worked example (Front: width+height; Top: width+depth;
Right: depth+height), with `Back` treated as sharing `Front`'s plane (not
in the article's own example; the most direct, non-speculative reading
consistent with its logic). No 3D solid is constructed anywhere.

## Task 159 — Shared dimension references

`MultiviewGraph::bind_axis` lets multiple views reference the identical
`DimensionId` for their respective axis -- tested with Front and Top both
bound to the same width.

## Task 160 — Graph dependency queries

`affected_views(dimension)` ("which views depend on this width?") and
`bindings_for_view(view)` ("which dimensions control this feature?") --
Article 231's own two named example queries, both real and tested.

## Task 161 — Incremental propagation

Not a separate mechanism: since views only ever hold a reference
(`DimensionId`) to a shared value rather than a copy, editing that value
through `DimensionStore` never has to "propagate" through this graph at
all -- every bound view already points at the current value. What this
task's atomicity requirement (`bind_axis` fails clean, changing nothing,
if the axis isn't valid for the view's identity) and the incremental-
recompute query (`affected_views`) both guarantee is tested directly:
binding one view/dimension pair never touches an unrelated view's own
bindings.

## Task 162 — Unresolved variables

`unresolved_axes(view)`: exactly which of a view's own consumed axes have
no binding yet -- explicit, queryable state (Article 38: "An unresolved
dimension is not an error"), not a value a caller has to infer from
absence.

## Task 163 — Unlink semantics

`unlink_axis(view, axis)` removes exactly one binding; a test proves a
second view still bound to the same `DimensionId` is completely
unaffected, and unlinking an unbound axis is a harmless no-op.

## Task 164 — Propagation/reopen tests

`the_graph_serializes_and_reloads_with_every_shared_link_intact`.

## A genuine, honestly-investigated persistence bug (the same class as
Phase 07's `EntityId`)

The first version of `MultiviewGraph` derived `Serialize`/`Deserialize`
directly on a `BTreeMap<(ViewId, SharedAxis), DimensionId>` -- a
tuple-keyed map, which `serde_json` cannot serialize at all (JSON object
keys must be strings), the *exact* failure mode `EntityId` hit in Phase
07. Caught immediately by Task 164's own round-trip test on first run,
not by inspection. Fixed the same way `EntityId` was fixed: hand-written
`Serialize`/`Deserialize` that stores the graph as a flat list of
`{view, axis, dimension}` records instead of a map with a composite key.
An additional regression test (`an_empty_graph_round_trips_too`) locks
this in alongside the original.

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 639/639 passing (14 new), re-run 3x clean
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
```

Full output: `test-reports/phase-22-cargo-test.txt`,
`test-reports/phase-22-cargo-clippy.txt`,
`test-reports/phase-22-cargo-fmt.txt`.

## Deferred, explicitly

- Confidence metadata, user-authorship metadata, suggestion states,
  conflict states as first-class graph node attributes (Article 35's own
  list names all of these) -- this phase's task list (157-164) only asks
  for the shared-axis/variable-binding subset of that list; the rest
  belongs to whichever later phase actually needs it (conflict states
  likely connect to Phase 14's `Conflict`, suggestion states to Phase
  15's provenance machinery, both already real elsewhere in this
  workspace and reusable rather than duplicated when that phase arrives).
- "Later feature dimensions" beyond width/height/depth (Task 157's own
  wording) -- no concrete one is named by any task yet.
- Wiring `MultiviewGraph` into a live `Document`/`Page` as a persisted
  field -- no task in this phase names that integration; the type is
  serializable and ready for it.

## Phase Gate

- All eight tasks (157-164) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 639/639 passing, confirmed stable across
  repeated runs. `cargo fmt --check` and `cargo clippy -D warnings`:
  clean.
- No scope boundary crossed: no new crate, no new dependency; reused
  `craftloop-dimension`'s existing `Shared` role and `DimensionId` rather
  than inventing a parallel shared-value mechanism.
- Implemented and tested: shared axis vocabulary and view mapping, shared
  dimension references, both named dependency queries, atomic/incremental
  binding, explicit unresolved-axis state, unlink semantics, and
  save/reopen persistence (after fixing the tuple-key serialization bug
  found by that very test).
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 23 automatically.
