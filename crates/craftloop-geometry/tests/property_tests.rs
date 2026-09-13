//! Property-based geometry tests.
//!
//! Execution 01, Phase 02, Task 020. Authority: MCP Article 206 "Geometry
//! Property Testing".
//!
//! Covers the four invariant classes Task 020 names explicitly:
//! nonnegative lengths, intersection symmetry, reversible transforms, and
//! tolerance behavior. Kept as a crate-level integration test (rather than
//! inside each module) because these properties are cross-cutting, not
//! specific to one primitive.

use proptest::prelude::*;

use craftloop_geometry::{Circle2, Point2, Segment2, Tolerances, Vector2};

/// Coordinates and lengths are bounded to a "reasonable document" range so
/// generated cases exercise real geometric behavior rather than floating
/// point overflow/precision edge cases, which are not what Task 020 is
/// testing for.
fn coordinate() -> impl Strategy<Value = f64> {
    -1000.0f64..1000.0
}

fn positive_length() -> impl Strategy<Value = f64> {
    0.01f64..1000.0
}

fn point() -> impl Strategy<Value = Point2> {
    (coordinate(), coordinate()).prop_map(|(x, y)| Point2::new(x, y))
}

fn vector() -> impl Strategy<Value = Vector2> {
    (coordinate(), coordinate()).prop_map(|(x, y)| Vector2::new(x, y))
}

fn nondegenerate_segment() -> impl Strategy<Value = Segment2> {
    (point(), point())
        .prop_filter("segment must not be degenerate", |(a, b)| {
            a.distance_to(*b) > Tolerances::committed().point_coincidence * 10.0
        })
        .prop_map(|(a, b)| Segment2::new(a, b))
}

fn circle() -> impl Strategy<Value = Circle2> {
    (point(), positive_length()).prop_map(|(center, radius)| Circle2::new(center, radius).unwrap())
}

proptest! {
    // --- Nonnegative lengths -------------------------------------------

    #[test]
    fn segment_length_is_always_nonnegative(a in point(), b in point()) {
        let s = Segment2::new(a, b);
        prop_assert!(s.length() >= 0.0);
    }

    #[test]
    fn point_distance_is_always_nonnegative(a in point(), b in point()) {
        prop_assert!(a.distance_to(b) >= 0.0);
    }

    #[test]
    fn vector_length_is_always_nonnegative(v in vector()) {
        prop_assert!(v.length() >= 0.0);
    }

    #[test]
    fn circle_distance_to_boundary_is_always_nonnegative(c in circle(), p in point()) {
        prop_assert!(c.distance_to_boundary(p) >= 0.0);
    }

    // --- Intersection symmetry ------------------------------------------

    #[test]
    fn segment_intersection_discriminant_is_order_independent(
        s1 in nondegenerate_segment(),
        s2 in nondegenerate_segment(),
    ) {
        let tol = Tolerances::committed();
        let forward = s1.intersect(&s2, &tol);
        let backward = s2.intersect(&s1, &tol);
        prop_assert_eq!(
            std::mem::discriminant(&forward),
            std::mem::discriminant(&backward)
        );
    }

    #[test]
    fn circle_intersection_discriminant_is_order_independent(c1 in circle(), c2 in circle()) {
        let tol = Tolerances::committed();
        let forward = c1.intersect_circle(&c2, &tol);
        let backward = c2.intersect_circle(&c1, &tol);
        prop_assert_eq!(
            std::mem::discriminant(&forward),
            std::mem::discriminant(&backward)
        );
    }

    #[test]
    fn point_distance_is_symmetric(a in point(), b in point()) {
        prop_assert!((a.distance_to(b) - b.distance_to(a)).abs() < 1e-9);
    }

    // --- Reversible transforms -------------------------------------------

    #[test]
    fn translate_then_inverse_translate_returns_the_original_point(p in point(), offset in vector()) {
        let moved = p.translated(offset);
        let back = moved.translated(offset.negated());
        prop_assert!(back.distance_to(p) < 1e-6);
    }

    #[test]
    fn segment_reversed_twice_is_the_original_segment(a in point(), b in point()) {
        let s = Segment2::new(a, b);
        let twice = s.reversed().reversed();
        prop_assert_eq!(twice.a, s.a);
        prop_assert_eq!(twice.b, s.b);
    }

    #[test]
    fn reversing_a_segment_preserves_its_length(a in point(), b in point()) {
        let s = Segment2::new(a, b);
        prop_assert!((s.length() - s.reversed().length()).abs() < 1e-9);
    }

    #[test]
    fn negating_a_vector_twice_returns_the_original(v in vector()) {
        let twice = v.negated().negated();
        prop_assert!((twice.x - v.x).abs() < 1e-9);
        prop_assert!((twice.y - v.y).abs() < 1e-9);
    }

    // --- Tolerance behavior ------------------------------------------------

    #[test]
    fn points_within_the_coincidence_tolerance_are_coincident(
        p in point(),
        angle in 0.0f64..std::f64::consts::TAU,
        fraction in 0.0f64..0.99,
    ) {
        let tol = Tolerances::committed();
        // Place the second point strictly inside the tolerance radius.
        let offset = Vector2::new(angle.cos(), angle.sin()).scaled(tol.point_coincidence * fraction);
        let nearby = p.translated(offset);
        prop_assert!(p.is_coincident_with(nearby, &tol));
    }

    #[test]
    fn points_beyond_the_coincidence_tolerance_are_not_coincident(
        p in point(),
        angle in 0.0f64..std::f64::consts::TAU,
        multiple in 1.01f64..10.0,
    ) {
        let tol = Tolerances::committed();
        let offset = Vector2::new(angle.cos(), angle.sin()).scaled(tol.point_coincidence * multiple);
        let far = p.translated(offset);
        prop_assert!(!p.is_coincident_with(far, &tol));
    }

    #[test]
    fn recognition_tolerance_accepts_everything_committed_tolerance_accepts(
        p in point(),
        angle in 0.0f64..std::f64::consts::TAU,
        fraction in 0.0f64..0.99,
    ) {
        let committed = Tolerances::committed();
        let recognition = Tolerances::recognition();
        let offset = Vector2::new(angle.cos(), angle.sin()).scaled(committed.point_coincidence * fraction);
        let nearby = p.translated(offset);
        if p.is_coincident_with(nearby, &committed) {
            prop_assert!(p.is_coincident_with(nearby, &recognition));
        }
    }
}
