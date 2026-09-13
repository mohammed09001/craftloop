//! Dimension target references.
//!
//! Execution 01, Phase 10, Task 074. Authority: Engine Contract 09; MCP
//! Article 23 "Dimensions Are Relationships, Not Labels".
//!
//! A dimension binds to `PrimitiveId`s (stable geometric identity from
//! `craftloop-ids`/Phase 02), never to a screen or page coordinate. This is
//! the concrete mechanism behind "dimensions are relationships, not
//! labels": the dimension's meaning survives a redraw, a pan/zoom, or a
//! page-layout move (Phase 07's `PageLayoutTransform`) untouched, because
//! none of those operations change what a `PrimitiveId` refers to.

use craftloop_ids::PrimitiveId;
use serde::{Deserialize, Serialize};

/// What a dimension measures, expressed purely in terms of stable
/// primitive identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DimensionTarget {
    /// A property of one primitive (a circle's radius/diameter, an arc's
    /// included angle).
    Single(PrimitiveId),
    /// A relationship between two primitives (the distance between two
    /// points/lines, the angle between two segments).
    Pair(PrimitiveId, PrimitiveId),
}

impl DimensionTarget {
    /// Every primitive this target references, for callers that need to
    /// check "does this dimension touch primitive X" without matching on
    /// the variant themselves.
    pub fn primitive_ids(&self) -> Vec<PrimitiveId> {
        match self {
            DimensionTarget::Single(a) => vec![*a],
            DimensionTarget::Pair(a, b) => vec![*a, *b],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::CraftLoopId;

    #[test]
    fn single_target_reports_exactly_one_primitive() {
        let id = PrimitiveId::new();
        let target = DimensionTarget::Single(id);
        assert_eq!(target.primitive_ids(), vec![id]);
    }

    #[test]
    fn pair_target_reports_both_primitives_in_order() {
        let a = PrimitiveId::new();
        let b = PrimitiveId::new();
        let target = DimensionTarget::Pair(a, b);
        assert_eq!(target.primitive_ids(), vec![a, b]);
    }

    #[test]
    fn serialization_round_trips() {
        let target = DimensionTarget::Pair(PrimitiveId::new(), PrimitiveId::new());
        let json = serde_json::to_string(&target).unwrap();
        let back: DimensionTarget = serde_json::from_str(&json).unwrap();
        assert_eq!(target, back);
    }
}
