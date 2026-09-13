# Phase 02 — Geometry Mathematics and Primitive Kernel — Evidence

Recorded: 2026-09-13

## Crate created

`crates/craftloop-geometry` (added to workspace `members`). Depends only on
`serde` and `craftloop-errors`; dev-depends on `serde_json` and `proptest 1`.
No platform/UI dependency, satisfying Engine Contract 03's authority boundary.

## Tasks 013–020

| Task | Module | Summary |
|---|---|---|
| 013 Point/vector primitives | `point.rs` | `Point2`, `Vector2`; distance, dot, cross, normalize, lerp, perpendicular, operator overloads. |
| 014 Line segment geometry | `segment.rs` | `Segment2`, `SegmentIntersection`; length, direction, closest point/`t`, distance, hit-test, segment-segment intersection (point/parallel/collinear/none). |
| 015 Circle geometry | `circle.rs` | `Circle2` (constructor rejects non-positive/non-finite radius via `DomainError`), `CircleIntersection`; boundary distance, contains, circle-circle intersection (none/tangent/two-points/coincident), circle-segment intersection. |
| 016 Arc geometry | `arc.rs` | `Arc2` using **start angle + signed sweep angle** (not start/end) to remove direction/short-vs-long-way ambiguity; evaluate, arc length, `contains_angle` (handles wraparound), bounds via axis-extreme-angle inclusion. |
| 017 Ellipse representation | `ellipse.rs` | `Ellipse2`; deliberately minimal — evaluate (parametric outline) and exact closed-form rotated bounding box only, no conic algebra/intersection, per the task's own instruction not to overbuild. |
| 018 Rectangle as relational geometry | `rectangle.rs` | `RelationalRectangle` = four ordered corner points, not a rigid `{x,y,w,h}` struct. `from_corners` rejects only a degenerate (coincident-endpoint) edge — it does **not** enforce right angles, so the future constraint solver (Phase 12) can express rectangularity as ordinary segment constraints instead of the kernel silently deciding it. `is_axis_respecting_rectangle` is a separate read-only check. |
| 019 Geometric tolerance service | `tolerance.rs` | `Tolerances` with two named profiles: `committed()` (tight, exact) and `recognition()` (loose, ink-derived; explicitly marked NOT YET CALIBRATED against real device data, pending Phase 06 Task 047). `Default` resolves to `committed()`. |
| 020 Property-based geometry tests | `tests/property_tests.rs` | 14 `proptest` properties covering the four invariant classes the task names: nonnegative lengths, intersection symmetry (segment-segment, circle-circle, point-distance), reversible transforms (translate+inverse, double-reverse, double-negate), tolerance behavior (inside/outside coincidence radius, recognition ⊇ committed). |

## Bug found and fixed via the Loop Engineering Contract

`Circle2::intersect_segment` initially computed `f = center.vector_to(a).negated()`
instead of `f = center.vector_to(a)` (i.e. `a - center`), an off-by-sign error
in the quadratic-formula setup. Three unit tests
(`segment_through_circle_center_intersects_at_two_diametrically_opposed_points`,
`segment_entirely_outside_circle_does_not_intersect`,
`segment_that_would_intersect_the_infinite_line_but_stops_short_does_not_intersect`)
caught this on first run. Fixed by removing the erroneous `.negated()`; all
three pass on re-run. This is exactly the "Fail → Implement → Verify" loop
the Loop Engineering Contract requires, not a case skipped past.

## Commands and results

```
cargo build -p craftloop-geometry        # clean, 31 packages incl. proptest/rand
cargo test -p craftloop-geometry         # 59/59 unit tests (1 real bug caught+fixed, see above)
cargo test -p craftloop-geometry --test property_tests   # 14/14 properties (256 cases each by default)
cargo fmt --all -- --check               # found reflow-only diffs in 8 files, fixed with cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings    # clean
cargo test --workspace                   # 107/107 tests total across the whole workspace
```

Full output: `test-reports/phase-02-cargo-test.txt`,
`test-reports/phase-02-cargo-clippy.txt`.

## Deferred (explicitly, not silently)

- Segment-segment collinear-overlap *extent* (how much two collinear
  segments overlap) is not computed — `SegmentIntersection::Collinear`
  reports the case without a range. No Version 1 consumer needs the extent
  yet; added when one does, with a test driving its exact semantics.
- Ellipse-ellipse and ellipse-line intersection: not implemented per Task
  017's explicit instruction not to overbuild conic algebra without a
  Version 1 use.
- `Tolerances::recognition()` values are placeholders pending Phase 06 Task
  047's false-positive/false-negative benchmark against real stroke data.
- Canonical internal unit (mm vs. abstract) is not chosen yet — Phase 09,
  Task 062. This crate's `f64` coordinates are unit-agnostic by design.

## Phase Gate

- All eight tasks (013–020) represented in repository code with passing
  tests, including one genuine bug caught and fixed by the tests
  themselves, not by inspection.
- `cargo test --workspace`: 107/107 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no recognition/ink code, no UI dependency, no
  unit-system decision forced early.
- Proceeding to Phase 03 automatically.
