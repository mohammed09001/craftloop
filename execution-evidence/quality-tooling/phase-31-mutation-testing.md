# Mutation/Adversarial Testing on Critical Logic

Execution 01, Phase 31, Task 227. Recorded 2026-09-14. Tool: `cargo-mutants` 27.1.0
(`cargo install cargo-mutants --locked`).

## Scope

"Where tooling permits" -- `cargo-mutants` works, but each mutant requires
a full rebuild-and-test cycle, so a whole-workspace run was not
proportionate given this sandbox's tight disk budget this phase (a
`cargo clean` reclaiming 15GB was already needed once in Phase 30; see
that phase's evidence). Two small, genuinely critical, pure-logic files
were chosen instead of a workspace-wide sweep: `craftloop-dimension`'s
`feasible_range.rs` (Article 27's triangle-inequality rule, the exact
engine Phase 30's Task 215 scenario exercises end to end) and
`craftloop-consistency`'s `geometry_validation.rs` (the degenerate-
geometry detector every `SemanticEntity::Primitive` insertion can
trigger). Both are short, dependency-light, and encode real geometric/
consistency rules this execution's own No-Hallucination Contract and
Article 27/Article 4 depend on being correct -- exactly "critical logic"
in the task's own sense.

## Run 1: `craftloop-dimension/src/feasible_range.rs`

```
cargo mutants -p craftloop-dimension -f "crates/craftloop-dimension/src/feasible_range.rs"
```

First pass: **35 mutants tested in 88s: 1 missed, 20 caught, 14 unviable.**

```
MISSED   crates/craftloop-dimension/src/feasible_range.rs:39:44: replace > with >= in TriangleSideRange::for_two_fixed_sides in 1s build + 0s test
```

**Real gap, not a false positive**: column 44 on that line is the
*second* comparison, `b > 0.0` -- the pre-existing test
`non_positive_or_non_finite_side_lengths_are_rejected` only ever varied
the *first* parameter `a` (always with a valid `b = 5.0`), so a mutant
silently accepting `b == 0.0` (or, by the same gap, a negative/NaN/
infinite `b`) passed every test unnoticed. Fixed: added
`a_zero_or_negative_or_non_finite_side_length_is_rejected`, covering the
second parameter symmetrically (`crates/craftloop-dimension/src/feasible_range.rs`).

Second pass, after the fix: **35 mutants tested in 73s: 21 caught, 14
unviable, 0 missed.**

## Run 2: `craftloop-consistency/src/geometry_validation.rs`

```
cargo mutants -p craftloop-consistency -f "crates/craftloop-consistency/src/geometry_validation.rs"
```

First pass: **20 mutants tested in 54s: 5 missed, 13 caught, 2 unviable.**

```
MISSED   :37:23: replace < with <= in validate_primitive_geometry   (line's own degeneracy check)
MISSED   :67:17: replace / with * in validate_primitive_geometry    (shoelace area /2.0)
MISSED   :62:45: replace % with / in validate_primitive_geometry    (shoelace index wraparound)
MISSED   :63:31: replace - with + in validate_primitive_geometry    (shoelace cross-term subtraction)
MISSED   :68:21: replace < with <= in validate_primitive_geometry   (rectangle's own degeneracy check)
```

**All five real gaps**, root-caused by writing the fix tests, not
guessed: every existing degenerate-rectangle fixture used
origin-anchored, coordinate-symmetric points (e.g. `(0,0)`, `(2,0)`,
`(4,0)`, `(2,~0)`), which made several of the *wrong* shoelace formulas
coincidentally also evaluate to a near-zero result -- the mutant escaped
detection not because the test suite was sparse, but because the
specific fixture geometry chosen happened to be degenerate under both
the correct and several incorrect formulas at once. Two additional
mutants (the `<`/`<=` boundary checks) were missed simply because no
test exercised the exact boundary value, only comfortably-above/below
cases.

Fixed with four new tests (`crates/craftloop-consistency/src/geometry_validation.rs`):
- `a_line_exactly_at_the_tolerance_boundary_is_not_flagged` /
  `a_rectangle_exactly_at_the_tolerance_boundary_is_not_flagged` -- exact
  boundary-value coverage for both `<` checks, using a "kite" quadrilateral
  (`(0,0)`, `(1,0)`, `(2,0)`, `(1,height)`) whose shoelace sum reduces to
  exactly `2 * height` (doubling and halving are always exact in
  IEEE754), so the computed area is bit-identical to
  `Tolerances::committed().point_coincidence`, not merely close to it.
- `a_thin_rectangle_with_area_just_below_tolerance_is_still_flagged` --
  a true area of exactly `tolerance / 2` (still using the exact "kite"
  construction), which a `/ 2.0` -> `* 2.0` mutant would report as `4x`
  too large and therefore wrongly non-degenerate.
- `a_collapsed_quadrilateral_away_from_the_origin_is_still_flagged_degenerate`
  -- the same collinear-plus-tiny-perturbation shape the pre-existing
  test already used, translated away from the origin to non-zero,
  asymmetric coordinates, which is exactly what makes the `%`-index and
  `-`-subtraction mutants stop coincidentally agreeing with the correct
  formula.

One real, useful side-finding while building the fix: `RelationalRectangle::from_corners`
has its own separate per-edge coincidence-length check (a different
validation layer than `geometry_validation.rs`'s own area check), which
rejected a first attempt at a boundary-exact test that used a "thin
sliver" rectangle shape (one very short edge) -- fixed by switching to
the "kite" shape (small area, but every edge near length 1), which
exercises the intended area-boundary logic without tripping the
unrelated edge-length gate.

Second pass, after the fix: **20 mutants tested in 54s: 18 caught, 2
unviable, 0 missed.**

## What "unviable" means

A mutant that does not compile at all (e.g. producing a type error) --
`cargo-mutants` reports these separately from "missed," since they
represent a code path so structurally load-bearing that no reasonable
mutation even builds, not a test gap. 14/20 and 2/20 unviable mutants in
the two runs above reflect how much of both files' logic is
type/structure-constrained by construction, not a weakness in the
methodology.
