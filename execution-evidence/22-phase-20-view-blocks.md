# Phase 20 — View Blocks and Orthographic Sets — Evidence

Recorded: 2026-09-13

Built inside the existing `crates/craftloop-document` crate (new
`view.rs` module) rather than a new crate: `page_layout.rs`'s own Phase
07 doc comment predicted exactly this ("View blocks with their own local
coordinate frame do not exist until Phase 20 ... Phase 20 gives each view
block one of these for real"), and `ViewId`/`OrthographicSetId` already
existed in `craftloop-ids` since Phase 01.

## Task 143 — View Block model

`ViewBlock { id, identity, label_visible, geometry_members,
annotation_members, layout }` -- exactly the five things the objective
names. Membership is tracked by ID (`BTreeSet<PrimitiveId>`/
`BTreeSet<DimensionAnnotationId>`), never by copying any entity's own
coordinates, so `page_layout.rs`'s "moving a view cannot change internal
engineering dimensions" invariant holds by the same construction that
module already established, not a new mechanism.

## Task 144 — Principal View Identity

`PrincipalViewIdentity::{Front,Top,Right,Back}` -- exactly Article 30's
four initial identities. `Left`/`Bottom` are Article 30's own "if later
supported" -- deferred, not silently omitted.

## Task 145 — Visible label vs. semantic identity

`identity` and `label_visible` are two independent fields;
`hide_label()`/`show_label()` only ever touch `label_visible`. Tested
directly: hiding the label after establishing `Front` leaves
`identity()` still returning `Some(Front)`.

## Task 146 — Orthographic Set

`OrthographicSet { id, views: BTreeSet<ViewId> }`: a group of linked
views. "Multiple alternatives can coexist on one page" is realized as
multiple independent `OrthographicSet`s, each free to have its own
`Front` -- tested directly.

## Task 147 — Identity uniqueness within a set

`OrthographicSet::add_view` rejects a view whose established identity
already exists among the set's current members, leaving the set
unchanged on rejection. Views with no identity yet (`None`) never
collide with each other -- uniqueness is scoped to "where required"
(Task 147's own wording), i.e. only once an identity exists to conflict
over.

## Task 148 — View movement in page space

`ViewBlock::move_to` only ever reassigns `layout`; a test builds a view
with real geometry membership and an established identity, moves it, and
asserts both are untouched -- the same "external, non-mutating
transform" proof `page_layout.rs` already demonstrated, now exercised
through a real `ViewBlock`.

## Task 149 — Duplicate/copy policies

`duplicate_linked` (shares the exact same `PrimitiveId`/
`DimensionAnnotationId` membership, identity, and label visibility -- an
edit to shared geometry is visible from both views because both merely
reference the same entities) versus `duplicate_independent` (starts with
empty membership and no identity -- silently copying membership would
have aliased the "independent" copy onto the original's own entities,
exactly what independence must not mean). `duplicate_independent`
explicitly does not attempt to clone primitives into new entities --
that requires `PrimitiveId` allocation and geometry duplication, a
`Page`/`Document`-level operation no task in this phase names.

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 611/611 passing (13 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
```

Full output: `test-reports/phase-20-cargo-test.txt`,
`test-reports/phase-20-cargo-clippy.txt`,
`test-reports/phase-20-cargo-fmt.txt`.

## Deferred, explicitly

- `Left`/`Bottom` principal identities -- Article 30's own "if later
  supported."
- Wiring `ViewBlock`/`OrthographicSet` into `Page`/`Document` as a real
  stored, persisted entity (the way `Conflict` was wired into
  `SemanticEntity` in Phase 14) -- no task in this phase names that
  integration; Phase 18's dimension-association engine already
  anticipated `ViewId`-based scoping without needing the concrete type,
  and this phase's types are ready to be wired in whenever a task asks
  for it.
- Actual primitive-cloning for `duplicate_independent` -- named and
  reasoned about above, genuinely out of this type's scope.

## Phase Gate

- All seven tasks (143-149) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 611/611 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no new crate, no new dependency; reused
  `ViewId`/`OrthographicSetId` (Phase 01) and `PageLayoutTransform`
  (Phase 07) rather than inventing parallel types.
- Implemented and tested: the full view block model, principal identity,
  label/identity separation, orthographic sets with identity uniqueness,
  page-space movement, and both duplication policies.
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 21 automatically.
