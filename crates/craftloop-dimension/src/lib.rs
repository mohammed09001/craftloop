//! Semantic dimension model for the Craft Loop shared engineering core.
//!
//! Execution 01, Phase 10. Authority: Engine Contract 09 ("Presentation
//! labels are not the semantic source of truth").
//!
//! No platform UI dependency; depends only on `craftloop-errors`,
//! `craftloop-ids`, `craftloop-geometry`.

pub mod annotation;
pub mod dimension;
pub mod feasible_range;
pub mod kind;
pub mod role;
pub mod store;
pub mod target;

pub use annotation::DimensionAnnotation;
pub use dimension::SemanticDimension;
pub use feasible_range::{triangle_third_side_is_feasible, TriangleSideRange};
pub use kind::DimensionKind;
pub use role::DimensionRole;
pub use store::DimensionStore;
pub use target::DimensionTarget;
