# Phase 23 — Orthographic Relationship Engine — Evidence

Recorded: 2026-09-13

Built inside `crates/craftloop-document` (new `propagation.rs`), on top
of Phase 22's `MultiviewGraph` and Phase 10's `DimensionStore`.
`craftloop-consistency` gained one new `ConflictKind::CrossViewMismatch`
variant (Task 171) -- an extension of Phase 14's existing schema, not a
new mechanism.

## Tasks 165-167 — Deterministic shared-width/height/depth propagation

One function, `propagate_confirmed_value`, not three: because
`MultiviewGraph` already represents sharing as multiple views binding the
*same* `DimensionId`, "propagation" is simply editing that one dimension
through the real `DimensionStore::edit_driving_value` (Phase 10) --
nothing is copied to bound views, they already read the current value.
Tested three times, once per axis pair Article 35 names (Front/Top for
width, Front/Right for height, Top/Right for depth), because each is a
real, separately evidenced requirement even though the implementation is
shared. A fourth test proves propagation never touches a view bound to
an unrelated dimension.

## Task 168 — Conservative Back extent sharing

`share_back_extent_from_front` refuses two ways to invent an unseen back
feature: it errors if `Front` has no resolved binding for the axis yet
(nothing to share), and it inherits `MultiviewGraph::bind_axis`'s own
rejection of an axis `Back`'s identity does not consume (`Depth`).

## Task 169 — Projection guides

`ProjectionGuide`/`projection_guides`: purely computed from the graph's
current bindings, never stored, never a real geometric constraint.
Tested that two independent calls with the same inputs return equal
results and never mutate the graph -- the guide is transient by
construction, not merely by convention.

## Task 170 — View-local vs. shared geometry distinction

A direct test: two views' `geometry_members` sets stay completely
independent even when populated with visually-coincidental content,
because `MultiviewGraph` has no mechanism anywhere that inspects geometry
to auto-create a binding -- only explicit `bind_axis` calls create
sharing.

## Task 171 — Cross-view conflict generation

`propose_shared_value` reproduces MCP Article 37's own worked example
exactly: width confirmed at 100mm in Front, then 130mm proposed in Top,
produces a `Conflict { kind: CrossViewMismatch, .. }` rather than a
second `DimensionId` -- verified directly that `Top`'s binding still
resolves to the original dimension afterward. A matching re-confirmation
(same value, within committed tolerance) correctly produces no conflict.

## Task 172 — No-permanent-master-view invariant

`propagate_confirmed_value` never special-cases which view initiated an
edit -- there is no "master" parameter or code path anywhere in this
module. Tested directly: binding `Top` before `Front` (reversed from
every other test in this phase) produces an identical, fully-propagated
result.

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 654/654 passing (15 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
```

Full output: `test-reports/phase-23-cargo-test.txt`,
`test-reports/phase-23-cargo-clippy.txt`,
`test-reports/phase-23-cargo-fmt.txt`.

## Deferred, explicitly

- Wiring this engine into a live editing session (calling
  `propagate_confirmed_value`/`propose_shared_value` automatically when a
  user edits a dimension annotation in a real view) -- no task this phase
  names that integration; both functions are real and tested standalone.
- Resolving a `CrossViewMismatch` conflict through
  `craftloop_consistency::resolve` end-to-end with an actual
  `ReplaceAndPropagate` effect on the shared dimension -- Task 171 asks
  only for conflict *generation*, and Phase 14's `resolve` already handles
  generic resolution transactions; connecting `ReplaceAndPropagate`'s
  specific effect (calling `propagate_confirmed_value` with the proposed
  value) to conflict resolution is a future integration no task names yet.

## Phase Gate

- All eight tasks (165-172) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 654/654 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no new crate; `craftloop-consistency` gained
  one enum variant, reusing its existing `Conflict` schema rather than a
  parallel one.
- Implemented and tested: shared-extent propagation for all three named
  axis pairs, conservative Back sharing, transient projection guides,
  view-local/shared geometry separation, Article 37's exact conflict
  scenario, and the no-master-view invariant.
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 24 automatically.
