//! Semantic dimension kinds.
//!
//! Execution 01, Phase 10, Task 070. Authority: Engine Contract 09
//! (Semantic Dimensions); MCP Article 21 "Semantic Dimensions and Visible
//! Dimension Annotations".

use serde::{Deserialize, Serialize};

/// What geometric relationship a dimension expresses. Independent of how
/// (or whether) it is presented visually -- see `annotation.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DimensionKind {
    /// A distance, stored in millimeters (`craftloop-units`'s canonical
    /// length unit).
    Linear,
    /// An angle, stored in radians (`craftloop-geometry`'s convention).
    Angular,
    Radius,
    Diameter,
}

impl DimensionKind {
    /// Whether this kind's canonical value is an angle (radians) rather
    /// than a length (millimeters).
    pub fn is_angular(&self) -> bool {
        matches!(self, DimensionKind::Angular)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_angular_reports_itself_as_angular() {
        assert!(DimensionKind::Angular.is_angular());
        assert!(!DimensionKind::Linear.is_angular());
        assert!(!DimensionKind::Radius.is_angular());
        assert!(!DimensionKind::Diameter.is_angular());
    }

    #[test]
    fn serialization_round_trips_for_every_kind() {
        for kind in [
            DimensionKind::Linear,
            DimensionKind::Angular,
            DimensionKind::Radius,
            DimensionKind::Diameter,
        ] {
            let json = serde_json::to_string(&kind).unwrap();
            let back: DimensionKind = serde_json::from_str(&json).unwrap();
            assert_eq!(kind, back);
        }
    }
}
