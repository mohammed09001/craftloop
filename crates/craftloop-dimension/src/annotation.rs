//! Visible dimension annotations.
//!
//! Execution 01, Phase 10, Task 071. Authority: Engine Contract 09; MCP
//! Article 21, Article 499 "Term: Dimension Annotation".
//!
//! An annotation is purely presentational: where a dimension's value
//! renders and whether it is currently shown. It carries no engineering
//! meaning of its own -- deleting or hiding one is exactly what Task 076
//! tests never touches the underlying [`crate::SemanticDimension`].

use craftloop_geometry::Point2;
use craftloop_ids::{DimensionAnnotationId, DimensionId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DimensionAnnotation {
    pub id: DimensionAnnotationId,
    pub dimension_id: DimensionId,
    /// Where the annotation renders, in the same page/engineering space as
    /// the geometry it annotates (Phase 07's `PageLayoutTransform`
    /// separation still applies: this is not a page-layout position).
    pub position: Point2,
    pub visible: bool,
}

impl DimensionAnnotation {
    pub fn new(id: DimensionAnnotationId, dimension_id: DimensionId, position: Point2) -> Self {
        Self {
            id,
            dimension_id,
            position,
            visible: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::CraftLoopId;

    #[test]
    fn a_new_annotation_is_visible_by_default() {
        let annotation = DimensionAnnotation::new(
            DimensionAnnotationId::new(),
            DimensionId::new(),
            Point2::ORIGIN,
        );
        assert!(annotation.visible);
    }

    #[test]
    fn serialization_round_trips() {
        let annotation = DimensionAnnotation::new(
            DimensionAnnotationId::new(),
            DimensionId::new(),
            Point2::new(3.0, 4.0),
        );
        let json = serde_json::to_string(&annotation).unwrap();
        let back: DimensionAnnotation = serde_json::from_str(&json).unwrap();
        assert_eq!(annotation, back);
    }
}
