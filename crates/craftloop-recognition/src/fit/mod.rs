//! Primitive candidate fitters (Tasks 040-043).

pub mod arc;
pub mod circle;
pub mod line;
pub mod rectangle;

pub use arc::{fit_arc, AmbiguityFlag, ArcCandidate};
pub use circle::{fit_circle, CircleCandidate};
pub use line::{fit_line, LineCandidate};
pub use rectangle::{fit_rectangle, RectangleCandidate};
