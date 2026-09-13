# Phase 10 — Dimension Semantic Model — Evidence

Recorded: 2026-09-13

## Crate created

`crates/craftloop-dimension` (Engine Contract 09), depending on
`craftloop-errors`, `craftloop-ids`, `craftloop-geometry`. Added
`DimensionAnnotationId` to `craftloop-ids` and
`DomainError::Dimension`/`DimensionErrorKind` to `craftloop-errors`.
`craftloop-document`'s `SemanticEntity` gained a `Dimension(SemanticDimension)`
variant, closing the "dimensions... Phase 10" deferral noted in the Phase 07
evidence.

## Tasks 070–077

| Task | Module | Summary |
|---|---|---|
| 070 Semantic dimension IDs/types | `kind.rs`, `dimension.rs` | `DimensionKind` (Linear/Angular/Radius/Diameter); `SemanticDimension` carries no presentation fields at all. |
| 071 Visible dimension annotations | `annotation.rs`, `store.rs` | `DimensionAnnotation` referencing its dimension purely by `DimensionId`; `DimensionStore::annotations_for` proves a dimension can have zero, one, or many. |
| 072 Driving/reference state | `role.rs` | `DimensionRole::{Driving, Reference}` with `is_directly_editable()`. |
| 073 Derived/shared/bounded state | `role.rs`, `dimension.rs` | `DimensionRole::{Derived, Shared, Bounded}` completing one 5-variant role enum (matching MCP Article 24's own single-title grouping); `feasible_range` on `SemanticDimension` for `Bounded`. |
| 074 Dimension target references | `target.rs` | `DimensionTarget::{Single, Pair}` binds to `PrimitiveId`s, never screen/page coordinates — the binding survives any presentation-layer transform untouched. |
| 075 Dimension edit transaction | `store.rs::edit_driving_value` | Atomic: role-check, value-validity, and feasible-range checks all happen before anything is written; a rejected edit leaves the dimension provably unchanged (tested directly). Explicitly documented as the seam a real constraint-solver feasibility check (Phase 12) slots into — none exists yet, none is claimed. |
| 076 Dimension visibility tests | `store.rs` | Hiding or deleting an annotation is proven, by test, to never touch the semantic dimension or its other annotations; only an explicit `remove_dimension` call cascades to annotations. |
| 077 Dimension serialization tests | `tests/serialization.rs` | Proves semantic and presentation state persist independently: a lone `SemanticDimension`'s JSON contains no annotation fields, a lone `DimensionAnnotation`'s JSON contains no dimension fields (only its ID), and `DimensionStore` round-trips as two sibling top-level maps. |

## Design note: one `DimensionRole` enum, not five booleans

Article 24 titles all five states ("Driving, Derived, Shared, Bounded, and
Reference Dimensions") together, and they are mutually exclusive in
practice — a dimension is never simultaneously driving and derived. Modeling
this as one enum rather than independent flags makes the invalid combination
unrepresentable rather than merely undocumented, and is recorded here as the
deliberate choice it was, per the Prompt Engineering Contract's expectation
that non-obvious design decisions get a stated reason.

## Commands and results

```
cargo build -p craftloop-dimension
cargo test -p craftloop-dimension                    # 26/26 unit tests (all passed first run)
cargo test -p craftloop-dimension --test serialization   # 4/4 (all passed first run)
cargo build --workspace                               # clean after wiring SemanticEntity::Dimension
cargo fmt --all -- --check                            # clean after cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings # clean, no findings
cargo test --workspace                                # 378/378 tests passing workspace-wide
```

Full output: `test-reports/phase-10-cargo-test.txt`,
`test-reports/phase-10-cargo-clippy.txt`.

## Deferred (explicitly, not silently)

- Real solver-feasibility validation inside `edit_driving_value` — no
  constraint solver exists yet (Phase 11-12); the seam is built, the check
  is not.
- View-label/suggestion/conflict `SemanticEntity` variants — still Phases
  20 and 12-14 respectively, unchanged from the Phase 07 deferral.
- Dimension annotations as a `SemanticEntity`/`Page` concept — deliberately
  kept inside `craftloop-dimension`'s own `DimensionStore` rather than
  duplicated into the document's page storage; no task asked for the
  latter.

## Phase Gate

- All eight tasks (070–077) represented in repository code with passing
  tests; every test passed on first run this phase (no regressions to
  chase), a rarity worth noting rather than omitting.
- `cargo test --workspace`: 378/378 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no constraint-solver logic fabricated ahead of
  Phase 11-12; no view/orthographic logic ahead of Phase 20.
- Proceeding to Phase 11 automatically.
