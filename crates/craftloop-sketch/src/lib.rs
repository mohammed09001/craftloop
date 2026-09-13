//! Constraint engine integration for the Craft Loop shared engineering
//! core.
//!
//! Execution 01, Phase 12 ("Constraint Engine Integration"), Tasks
//! 084-092. Authority: Engine Contract 10.
//!
//! `craftloop-constraint` (Phase 11) defined a backend-neutral vocabulary
//! and interface but chose no real solver and bound to no real geometry.
//! This crate closes both gaps: [`ezpz_adapter::EzpzSolver`] implements
//! `craftloop_constraint::ConstraintSolver` against the real `ezpz` crate
//! (Task 083's decision record authorized this, pinned at exactly
//! `ezpz = "=0.2.29"`), and [`sketch::Sketch`] binds
//! `craftloop-constraint`'s variable-level vocabulary to real
//! `craftloop-recognition::BeautifiedPrimitive` geometry via
//! [`point_ref::PointRef`] and [`constraint_kind::SketchConstraintKind`].
//!
//! Tasks 089 (tangency) and 090 (symmetry) were conditionally gated on
//! spike evidence Phase 11's original spike had not gathered; both were
//! exercised in a spike extension before this crate was written (see
//! `execution-evidence/solver-evaluations/solver-decision-record.md`'s
//! addendum) and approved unconditionally, so both are implemented here
//! without further hedging.

pub mod constraint_kind;
pub mod ezpz_adapter;
pub mod intent;
pub mod point_ref;
pub mod provenance;
pub mod sketch;
pub mod state;

pub use constraint_kind::SketchConstraintKind;
pub use ezpz_adapter::EzpzSolver;
pub use point_ref::PointRef;
pub use provenance::ConstraintProvenance;
pub use sketch::{ConstraintOutcome, Sketch};
pub use state::DegreesOfFreedomState;
