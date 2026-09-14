//! Task 215 — Invalid-dimension scenario.
//!
//! Execution 01, Phase 30, Task 215. Reproduces MCP Article 27's own
//! worked example verbatim ("If a triangle has two sides of 30 and 50
//! units... 300 cannot form this triangle") end to end through the real
//! domain engine (`craftloop_dimension::TriangleSideRange`, Phase 13,
//! Tasks 097-098, plus `DimensionStore::edit_driving_value`, Phase 10)
//! and confirms the last valid state remains -- an impossible value is
//! rejected outright, never silently clamped or partially applied.

use craftloop_dimension::{
    DimensionKind, DimensionRole, DimensionStore, DimensionTarget, SemanticDimension,
    TriangleSideRange,
};
use craftloop_ids::{CraftLoopId, DimensionId, PrimitiveId};

#[test]
fn article_27s_worked_example_is_rejected_and_the_last_valid_value_survives() {
    // Article 23/27's exact numbers: two fixed sides of 30 and 50.
    let range = TriangleSideRange::for_two_fixed_sides(30.0, 50.0).unwrap();
    assert_eq!(range.min, 20.0);
    assert_eq!(range.max, 80.0);
    assert!(
        !range.is_feasible(300.0),
        "300 must not form a triangle with sides 30 and 50 (Article 27's own claim)"
    );

    // A driving dimension for the third side, wired to that exact
    // feasible range -- a real engine integration, not a hand-checked
    // boolean the test computes and discards.
    let dimension_id = DimensionId::new();
    let dimension = SemanticDimension::new(
        dimension_id,
        DimensionKind::Linear,
        DimensionRole::Driving,
        DimensionTarget::Single(PrimitiveId::new()),
        40.0, // a genuinely valid third side for a 30/50 triangle
    )
    .unwrap()
    .with_feasible_range(range.min, range.max)
    .unwrap();

    let mut store = DimensionStore::new();
    store.insert_dimension(dimension).unwrap();
    assert_eq!(store.dimension(dimension_id).unwrap().value(), 40.0);

    // Reproduce the impossible value: rejected, with the dimension left
    // completely unchanged (Task 075's own atomicity guarantee).
    let result = store.edit_driving_value(dimension_id, 300.0);
    assert!(
        result.is_err(),
        "300 must be rejected, not silently applied"
    );
    assert_eq!(
        store.dimension(dimension_id).unwrap().value(),
        40.0,
        "the last valid state must remain exactly as it was before the rejected edit"
    );

    // A genuinely feasible edit within (20, 80) still succeeds normally
    // -- proving the rejection above is about feasibility, not a general
    // inability to edit this dimension at all.
    let previous = store.edit_driving_value(dimension_id, 60.0).unwrap();
    assert_eq!(previous, 40.0);
    assert_eq!(store.dimension(dimension_id).unwrap().value(), 60.0);
}

#[test]
fn the_degenerate_boundary_values_are_excluded_by_triangle_side_range_itself() {
    // The type's own doc comment flags this distinction explicitly:
    // `TriangleSideRange` is a strictly-open interval (a triangle with a
    // side exactly at the sum or difference of the other two has zero
    // area and is not really a triangle) -- confirmed directly here so
    // this scenario's evidence backs the claim, not just repeats it.
    let range = TriangleSideRange::for_two_fixed_sides(30.0, 50.0).unwrap();
    assert!(!range.is_feasible(20.0));
    assert!(!range.is_feasible(80.0));
    assert!(range.is_feasible(20.0001));
    assert!(range.is_feasible(79.9999));
}
