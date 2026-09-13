//! Geometric Consistency Engine for the Craft Loop shared engineering
//! core.
//!
//! Execution 01, Phase 14, Tasks 100-106. Authority: MCP Article 27
//! "Geometric Consistency Engine".
//!
//! Deliberately does not depend on `craftloop-document`: this crate
//! validates the lower-level types (`BeautifiedPrimitive`,
//! `SemanticDimension`, solver `SolveResult`s, raw parser input) directly,
//! the same way `craftloop-sketch` (Phase 12) operates on
//! `BeautifiedPrimitive` rather than requiring a whole `Document`. Task
//! 106's persistence requirement is satisfied the other direction:
//! `craftloop-document` depends on this crate for the `Conflict` type,
//! exactly as it already depends on `craftloop-dimension` for
//! `SemanticDimension` and `craftloop-recognition` for `Beautified`.

pub mod conflict;
pub mod dimension_validation;
pub mod geometry_validation;
pub mod resolution;
pub mod unit_validation;

pub use conflict::{Conflict, ConflictKind, ConflictStatus, ResolutionChoice};
pub use dimension_validation::validate_dimension_against_solve;
pub use geometry_validation::validate_primitive_geometry;
pub use resolution::resolve;
pub use unit_validation::validate_unit_consistency;
