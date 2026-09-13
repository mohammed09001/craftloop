//! Platform-neutral constraint solver interface for the Craft Loop shared
//! engineering core.
//!
//! Execution 01, Phase 11, Task 078. Authority: Engine Contract 10.
//!
//! This crate defines *only* the interface and a non-iterating reference
//! implementation ([`ResidualChecker`]) -- deliberately, per this phase's
//! own title ("Technical Spike"). No real numeric solver backend is chosen
//! or wired in here; that is Task 083's decision record plus Phase 12's
//! integration work. See `execution-evidence/solver-evaluations/` for the
//! `ezpz` spike and decision record this interface's vocabulary was
//! informed by.

pub mod constraint;
pub mod residual;
pub mod result;
pub mod solver;
pub mod variable;

pub use constraint::GeometricConstraint;
pub use residual::residual;
pub use result::{ConstraintDiagnostic, SolveResult, SolveStatus};
pub use solver::{ConstraintRequest, ConstraintSolver, ResidualChecker};
pub use variable::{PointVariables, Variable, VariableId};
