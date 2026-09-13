//! Stable, strongly typed domain identifiers for the Craft Loop shared
//! engineering core.
//!
//! Execution 01, Phase 01, Task 008. Authority: Engine Contracts 01-30
//! (every contract requires public domain types independent of UI/platform
//! concepts); MCP V1 Article 129 "Stable Identifiers".
//!
//! Each identifier wraps a [`uuid::Uuid`] so that:
//! - identity survives serialization/deserialization and cross-view/
//!   cross-platform round trips (Article 129);
//! - identity is never derived from screen position, array index, or
//!   presentation order, so moving/relayouting an entity never changes what
//!   it *is* (Article 130, Separation of Geometry and Presentation);
//! - identifiers created independently (e.g. on different platforms before a
//!   sync exists) do not collide, unlike a shared counter would.
//!
//! Production code must never construct one of these from a raw integer.
//! [`CraftLoopId::from_u128`] exists only so deterministic fixtures/tests can
//! build reproducible identifiers without depending on random generation.

use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Behavior shared by every strongly typed Craft Loop domain identifier.
pub trait CraftLoopId:
    Copy + Clone + Eq + PartialEq + Ord + PartialOrd + std::hash::Hash + fmt::Debug + fmt::Display
{
    /// Generate a fresh, randomly sourced identifier.
    ///
    /// This is the only production constructor. It is intentionally
    /// non-deterministic: identity must not depend on call order or process
    /// state, only on genuine uniqueness.
    fn new() -> Self;

    /// Construct a deterministic identifier from a fixed value.
    ///
    /// Reserved for tests and fixtures (see `craftloop-test-support`) where
    /// reproducible IDs make assertions readable. Never call this from
    /// production domain logic.
    fn from_u128(value: u128) -> Self;

    /// The all-zero sentinel value. Useful as an explicit "no id yet"
    /// placeholder that is still type-correct, never as a real identity.
    fn nil() -> Self;

    /// Expose the underlying UUID, e.g. for diagnostic export (Engine
    /// Contract 26).
    fn as_uuid(&self) -> Uuid;
}

macro_rules! define_id {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl CraftLoopId for $name {
            fn new() -> Self {
                Self(Uuid::new_v4())
            }

            fn from_u128(value: u128) -> Self {
                Self(Uuid::from_u128(value))
            }

            fn nil() -> Self {
                Self(Uuid::nil())
            }

            fn as_uuid(&self) -> Uuid {
                self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

define_id!(
    StrokeId,
    "Identifies one raw ink stroke (Engine Contract 02)."
);
define_id!(
    PrimitiveId,
    "Identifies one structured geometric primitive: point, segment, circle, \
     arc, ellipse, or relational rectangle (Engine Contract 03)."
);
define_id!(
    DimensionId,
    "Identifies one semantic dimension, independent of its visible \
     annotation(s) (Engine Contract 09; MCP Article 21)."
);
define_id!(
    ConstraintId,
    "Identifies one constraint instance tracked by the solver (Engine \
     Contract 10)."
);
define_id!(
    ViewId,
    "Identifies one orthographic view block with its own identity and local \
     coordinate frame (Engine Contract 18; MCP Article 30)."
);
define_id!(
    OrthographicSetId,
    "Identifies one group of linked views describing a single design state \
     (Engine Contract 19; MCP Article 327)."
);
define_id!(
    TransactionId,
    "Identifies one atomic, undoable domain transaction (Engine Contract 14)."
);
define_id!(
    ConflictId,
    "Identifies one structured conflict object raised by the consistency \
     engine (Engine Contract 12; MCP Article 134)."
);
define_id!(
    SuggestionId,
    "Identifies one AI-originated suggestion that remains distinct from \
     confirmed engineering truth until accepted (MCP Article 5, Article 384)."
);
define_id!(
    NoteId,
    "Identifies one freeform note (Engine Contract 15 document model; MCP \
     Article 67 \"Notes Around Engineering Views\"). Added in Phase 07 \
     alongside the document model that first needs to store notes."
);
define_id!(
    PageId,
    "Identifies one page/canvas within a notebook document (Engine \
     Contract 15; MCP Article 65 \"Pages and Infinite Paper\"). Added in \
     Phase 07 alongside the document model that first needs to store pages."
);
define_id!(
    DimensionAnnotationId,
    "Identifies one visible presentation of a semantic dimension (Engine \
     Contract 09; MCP Article 499 \"Term: Dimension Annotation\"). Distinct \
     from `DimensionId`: one semantic dimension can have zero, one, or \
     several annotations, each independently identified so hiding/deleting \
     one never implies anything about the semantic dimension or its other \
     annotations. Added in Phase 10 alongside the dimension model."
);
define_id!(
    CommandId,
    "Identifies one command object dispatched through the shared Command \
     Bus (MCP Article 239), regardless of source channel (toolbar, Ink \
     Command Engine, keyboard, accessibility, future voice/gesture). \
     Added in Phase 19 alongside the Ink Command Language."
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn new_ids_are_unique_across_many_calls() {
        let mut seen = HashSet::new();
        for _ in 0..10_000 {
            let id = StrokeId::new();
            assert!(seen.insert(id), "StrokeId::new() produced a duplicate");
        }
    }

    #[test]
    fn distinct_id_types_do_not_implicitly_convert() {
        // This is a compile-time property: the following line intentionally
        // does not exist because it would not type-check:
        // let _: PrimitiveId = StrokeId::new();
        // The test below just documents/exercises that both types exist and
        // are independently constructible.
        let stroke = StrokeId::new();
        let primitive = PrimitiveId::new();
        assert_ne!(stroke.as_uuid(), Uuid::nil());
        assert_ne!(primitive.as_uuid(), Uuid::nil());
    }

    #[test]
    fn deterministic_construction_is_reproducible() {
        let a = DimensionId::from_u128(42);
        let b = DimensionId::from_u128(42);
        assert_eq!(a, b);
        assert_eq!(a.to_string(), b.to_string());
    }

    #[test]
    fn nil_is_the_all_zero_sentinel() {
        assert_eq!(ConflictId::nil().as_uuid(), Uuid::nil());
    }

    #[test]
    fn ordering_is_total_and_stable_for_canonical_output() {
        // Engine Contract 15/Task 010 requires canonical ordering where
        // deterministic output matters (e.g. diagnostic export). IDs must
        // support a total order so callers can sort collections by ID.
        let mut ids: Vec<ViewId> = (0..50u128).rev().map(ViewId::from_u128).collect();
        ids.sort();
        let expected: Vec<ViewId> = (0..50u128).map(ViewId::from_u128).collect();
        assert_eq!(ids, expected);
    }

    #[test]
    fn serialization_round_trips_and_is_transparent() {
        let id = TransactionId::from_u128(7);
        let json = serde_json::to_string(&id).expect("serialize");
        // #[serde(transparent)] means the wire format is just the UUID
        // string, not a wrapper object -- required so document JSON stays
        // stable across schema-adjacent refactors (Task 010).
        assert_eq!(json, "\"00000000-0000-0000-0000-000000000007\"");
        let back: TransactionId = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(id, back);
    }

    #[test]
    fn debug_format_includes_type_name_for_diagnostics() {
        let id = SuggestionId::from_u128(1);
        let debug = format!("{:?}", id);
        assert!(debug.starts_with("SuggestionId("));
    }
}
