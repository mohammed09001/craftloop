//! Centralized geometric tolerance service.
//!
//! Execution 01, Phase 02, Task 019. Authority: MCP Article 207 "Numerical
//! Tolerance", Article 74 "Snap System".
//!
//! Forbidden shortcut this module exists to close: scattering ad hoc
//! `1e-6`-style epsilon literals across every geometry function, which makes
//! it impossible to reason about or retune tolerance behavior in one place.
//!
//! Two tolerance *profiles* are distinguished on purpose, because they
//! answer different questions:
//!
//! - [`Tolerances::committed`] — "are these two already-confirmed geometric
//!   facts consistent with each other?" Used by the constraint solver
//!   (Phase 11–13) and consistency engine (Phase 14) once geometry is
//!   semantic, not raw ink. Tight, because a committed dimension is supposed
//!   to be exact.
//! - [`Tolerances::recognition`] — "is this noisy, hand-drawn ink close
//!   enough to a candidate primitive to suggest it?" Used by the recognition
//!   engine (Phase 06). Deliberately looser, and explicitly **not**
//!   calibrated against real device data yet: Task 047 (Phase 06) benchmarks
//!   false-positive/false-negative behavior against real stroke samples and
//!   may retune these constants. Treating today's placeholder values as
//!   final would violate the No-Hallucination Contract.
//!
//! Conflating the two profiles would let ink-noise-scale slop leak into a
//! constraint solve, or let an overly strict exact-equality check reject
//! ordinary hand-drawn geometry -- both are real defects this separation
//! prevents.

use serde::{Deserialize, Serialize};

/// A named set of epsilon values used throughout the geometry kernel.
///
/// All distances are in the same abstract "document units" the geometry
/// kernel itself is agnostic to (Phase 09, Task 062 chooses the canonical
/// internal unit and how it maps to real-world millimeters/inches).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tolerances {
    /// Two points are the same point if their distance is below this.
    pub point_coincidence: f64,
    /// Two directions are parallel if the absolute value of their unit-
    /// vector cross product is below this.
    pub parallel: f64,
    /// Two directions are perpendicular if the absolute value of their
    /// unit-vector dot product is below this.
    pub perpendicular: f64,
    /// Two scalar lengths/angles are equal if their difference is below
    /// this.
    pub length_equality: f64,
    /// Maximum distance at which pointer input snaps to a candidate
    /// geometric feature (Article 74, Snap System).
    pub snap_distance: f64,
    /// Convergence threshold the constraint solver uses to decide a
    /// solution is acceptable (Phase 11).
    pub solver_convergence: f64,
}

impl Tolerances {
    /// Tight tolerances for already-committed, semantic geometry. Anything
    /// that fails a `committed` check should generally be surfaced as a
    /// `DomainError` rather than silently accepted.
    pub const fn committed() -> Self {
        Self {
            point_coincidence: 1e-9,
            parallel: 1e-9,
            perpendicular: 1e-9,
            length_equality: 1e-9,
            snap_distance: 0.0,
            solver_convergence: 1e-9,
        }
    }

    /// Loose tolerances for interpreting noisy, hand-drawn ink.
    ///
    /// NOT YET CALIBRATED against real device/stroke data -- see Phase 06,
    /// Task 047 ("Benchmark false-positive behavior"). These are
    /// placeholder values chosen to be self-evidently "loose relative to
    /// `committed`," not a claim about real handwriting accuracy.
    pub const fn recognition() -> Self {
        Self {
            point_coincidence: 0.02,
            parallel: 0.05,
            perpendicular: 0.05,
            length_equality: 0.02,
            snap_distance: 0.15,
            solver_convergence: 1e-6,
        }
    }
}

impl Default for Tolerances {
    /// Defaults to the strict `committed` profile: callers must opt in to
    /// the loose `recognition` profile explicitly rather than accidentally
    /// inheriting it.
    fn default() -> Self {
        Self::committed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognition_profile_is_strictly_looser_than_committed() {
        let committed = Tolerances::committed();
        let recognition = Tolerances::recognition();
        assert!(recognition.point_coincidence > committed.point_coincidence);
        assert!(recognition.parallel > committed.parallel);
        assert!(recognition.perpendicular > committed.perpendicular);
        assert!(recognition.length_equality > committed.length_equality);
        assert!(recognition.snap_distance > committed.snap_distance);
    }

    #[test]
    fn default_is_the_strict_profile_not_the_loose_one() {
        assert_eq!(Tolerances::default(), Tolerances::committed());
    }

    #[test]
    fn committed_snap_distance_is_zero_because_committed_geometry_never_snaps() {
        // Snapping is an input-interpretation behavior (Article 74); once
        // geometry is committed there is nothing left to "snap" to.
        assert_eq!(Tolerances::committed().snap_distance, 0.0);
    }
}
