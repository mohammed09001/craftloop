//! Deterministic, UI-independent 2D geometry kernel for the Craft Loop
//! shared engineering core.
//!
//! Execution 01, Phase 02. Authority: Engine Contract 03 ("Points,
//! segments, circles, arcs, ellipses, relational rectangles, tolerance";
//! "deterministic and UI-independent"); MCP Article 95 "Vector Geometry
//! Engine".
//!
//! This crate has no dependency on any platform/UI crate and never will:
//! see the Engine Contract 03 review question ("Could a future Android or
//! iPad adapter use this behavior without importing Windows UI concepts?").

pub mod arc;
pub mod bounds;
pub mod circle;
pub mod ellipse;
pub mod point;
pub mod rectangle;
pub mod segment;
pub mod tolerance;

pub use arc::Arc2;
pub use bounds::Bounds2;
pub use circle::{Circle2, CircleIntersection};
pub use ellipse::Ellipse2;
pub use point::{Point2, Vector2};
pub use rectangle::RelationalRectangle;
pub use segment::{Segment2, SegmentIntersection};
pub use tolerance::Tolerances;
