//! Dimension presentation policy.
//!
//! Execution 01, Phase 25, Task 183. Authority: MCP Article 235
//! ("Dimension typography rules").
//!
//! `craftloop_dimension::DimensionAnnotation` (Phase 10) already carries
//! no value of its own -- purely a position and a visibility flag. This
//! module adds the one piece that phase did not need yet: typography
//! (text height, and which `LineStyle` the dimension/extension lines
//! render with). [`DimensionTypography`] has no value field either, and
//! [`present`] never copies a dimension's value into anything this
//! module owns -- it always reads `SemanticDimension::value()` fresh at
//! presentation time, so there is no code path by which a stale
//! presentation-layer copy could ever disagree with the real, semantic
//! value.

use craftloop_dimension::{DimensionAnnotation, SemanticDimension};

use crate::line_role::LineRole;
use crate::line_style::{style_for_role, LineStyle};

/// Purely presentational: no dimension value, no engineering meaning.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DimensionTypography {
    pub text_height: f64,
}

impl DimensionTypography {
    pub fn new(text_height: f64) -> Self {
        Self { text_height }
    }
}

/// The fully-resolved rendering of one dimension annotation: its current
/// semantic value (read fresh, by reference, from `dimension` -- never
/// copied into a field this type owns across calls), where it renders,
/// and how. Constructing one is cheap and meant to be done on demand at
/// render/export time, not cached and allowed to drift.
pub struct PresentedDimension<'a> {
    pub value: f64,
    pub position: &'a craftloop_geometry::Point2,
    pub text_height: f64,
    pub dimension_line_style: LineStyle,
    pub extension_line_style: LineStyle,
}

pub fn present<'a>(
    dimension: &SemanticDimension,
    annotation: &'a DimensionAnnotation,
    typography: &DimensionTypography,
) -> PresentedDimension<'a> {
    PresentedDimension {
        value: dimension.value(),
        position: &annotation.position,
        text_height: typography.text_height,
        dimension_line_style: style_for_role(LineRole::DimensionLine),
        extension_line_style: style_for_role(LineRole::ExtensionLine),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_dimension::{DimensionKind, DimensionRole, DimensionTarget};
    use craftloop_geometry::Point2;
    use craftloop_ids::{CraftLoopId, DimensionAnnotationId, DimensionId, PrimitiveId};

    #[test]
    fn presenting_a_dimension_reads_its_real_current_value() {
        let dimension = SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            42.0,
        )
        .unwrap();
        let annotation = DimensionAnnotation::new(
            DimensionAnnotationId::new(),
            dimension.id,
            Point2::new(1.0, 2.0),
        );
        let typography = DimensionTypography::new(3.5);

        let presented = present(&dimension, &annotation, &typography);
        assert_eq!(presented.value, 42.0);
        assert_eq!(*presented.position, Point2::new(1.0, 2.0));
        assert_eq!(presented.text_height, 3.5);
    }

    #[test]
    fn presenting_the_same_dimension_twice_after_an_edit_reflects_the_new_value() {
        let mut dimension = SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            10.0,
        )
        .unwrap();
        let annotation =
            DimensionAnnotation::new(DimensionAnnotationId::new(), dimension.id, Point2::ORIGIN);
        let typography = DimensionTypography::new(3.0);

        let before = present(&dimension, &annotation, &typography);
        assert_eq!(before.value, 10.0);

        // Presentation never caches the value: it must be re-derived,
        // never patched in place.
        dimension = SemanticDimension::new(
            dimension.id,
            DimensionKind::Linear,
            DimensionRole::Driving,
            dimension.target,
            25.0,
        )
        .unwrap();
        let after = present(&dimension, &annotation, &typography);
        assert_eq!(after.value, 25.0);
    }
}
