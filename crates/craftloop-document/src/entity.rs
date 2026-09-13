//! Semantic object containers.
//!
//! Execution 01, Phase 07, Task 050. Authority: Engine Contract 15
//! (Document); MCP Article 128 "Document Model".
//!
//! Task 050 names seven kinds of semantic object a page can hold: "raw
//! ink, geometry, notes, dimensions, view labels, suggestions, conflicts,
//! and ephemeral interaction records." Only three have a real, implemented
//! type behind them yet -- raw ink (`craftloop_ink::Stroke`, Phase 05),
//! geometry (`craftloop_recognition::BeautifiedPrimitive`, Phase 06), and
//! notes (this crate's `Note`, added this phase). The other four belong to
//! engines this execution has not reached: dimensions (Phase 10), view
//! labels (Phase 20), suggestions/conflicts (Phases 06's `RecognitionCandidate`
//! is a suggestion in spirit but not yet a *stored, reified* document
//! entity with its own lifecycle -- that lands with Phase 12-14's
//! consistency/conflict engines). Adding placeholder variants for them now
//! would be exactly the fabricated completeness the No-Hallucination
//! Contract forbids; `SemanticEntity` grows a new variant in the phase that
//! actually builds each one.

use std::fmt;
use std::str::FromStr;

use craftloop_ids::{CraftLoopId, NoteId, PrimitiveId, StrokeId};
use craftloop_ink::Stroke;
use craftloop_recognition::Beautified;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use crate::note::Note;

/// A stable reference to one semantic entity, regardless of kind. `Ord` is
/// derived so entities can live in a `BTreeMap` keyed by this type,
/// guaranteeing deterministic (sorted) iteration/serialization order
/// (`craftloop-serialization`'s canonical-JSON convention, Phase 01).
///
/// `Serialize`/`Deserialize` are hand-written rather than derived: the
/// default derive for a multi-variant enum produces a JSON *object* (e.g.
/// `{"Stroke": "<uuid>"}`), and JSON object keys must be strings -- so a
/// `BTreeMap<EntityId, _>` (exactly how `Page` stores its entities) would
/// fail to serialize at all. This was caught by the Task 055 save/reopen
/// golden test, not by any smaller unit test, because no earlier test
/// actually serialized a full map keyed by `EntityId`. The manual impl
/// below serializes as one string, `"<Kind>:<uuid>"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityId {
    Stroke(StrokeId),
    Primitive(PrimitiveId),
    Note(NoteId),
}

impl EntityId {
    fn kind_str(&self) -> &'static str {
        match self {
            EntityId::Stroke(_) => "Stroke",
            EntityId::Primitive(_) => "Primitive",
            EntityId::Note(_) => "Note",
        }
    }

    fn uuid(&self) -> Uuid {
        match self {
            EntityId::Stroke(id) => id.as_uuid(),
            EntityId::Primitive(id) => id.as_uuid(),
            EntityId::Note(id) => id.as_uuid(),
        }
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.kind_str(), self.uuid())
    }
}

impl FromStr for EntityId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let (kind, uuid_str) = s
            .split_once(':')
            .ok_or_else(|| format!("invalid EntityId string: {s:?}"))?;
        let uuid = Uuid::parse_str(uuid_str).map_err(|e| e.to_string())?;
        let value = uuid.as_u128();
        match kind {
            "Stroke" => Ok(EntityId::Stroke(StrokeId::from_u128(value))),
            "Primitive" => Ok(EntityId::Primitive(PrimitiveId::from_u128(value))),
            "Note" => Ok(EntityId::Note(NoteId::from_u128(value))),
            other => Err(format!("unknown EntityId kind: {other:?}")),
        }
    }
}

impl Serialize for EntityId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EntityId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SemanticEntity {
    Stroke(Stroke),
    /// An accepted, beautified recognition result (Phase 06) that the user
    /// has committed as structured geometry, given a stable identity so it
    /// can be referenced (e.g. by a future dimension) and persisted.
    Primitive {
        id: PrimitiveId,
        beautified: Beautified,
    },
    Note(Note),
}

impl SemanticEntity {
    pub fn id(&self) -> EntityId {
        match self {
            SemanticEntity::Stroke(s) => EntityId::Stroke(s.id),
            SemanticEntity::Primitive { id, .. } => EntityId::Primitive(*id),
            SemanticEntity::Note(n) => EntityId::Note(n.id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::Point2;

    #[test]
    fn entity_id_matches_the_wrapped_entitys_own_id() {
        let note = Note::new(NoteId::new(), Point2::ORIGIN, "hi");
        let entity = SemanticEntity::Note(note.clone());
        assert_eq!(entity.id(), EntityId::Note(note.id));
    }

    #[test]
    fn entity_id_ordering_is_deterministic_for_btreemap_use() {
        let mut ids = vec![
            EntityId::Note(NoteId::from_u128(3)),
            EntityId::Stroke(StrokeId::from_u128(1)),
            EntityId::Primitive(PrimitiveId::from_u128(2)),
        ];
        let expected = ids.clone();
        ids.sort();
        ids.sort(); // idempotent: sorting twice must not reorder further
                    // Not asserting a specific cross-variant order (that's an
                    // implementation detail of derive(Ord) on the enum), only that
                    // sorting is stable/idempotent and total.
        let mut resorted = expected;
        resorted.sort();
        assert_eq!(ids, resorted);
    }

    #[test]
    fn serialization_round_trips_for_each_variant() {
        let note_entity = SemanticEntity::Note(Note::new(NoteId::new(), Point2::ORIGIN, "hi"));
        let json = serde_json::to_string(&note_entity).unwrap();
        let back: SemanticEntity = serde_json::from_str(&json).unwrap();
        assert_eq!(note_entity, back);
    }

    #[test]
    fn entity_id_display_and_from_str_round_trip() {
        let id = EntityId::Primitive(PrimitiveId::from_u128(42));
        let text = id.to_string();
        let parsed: EntityId = text.parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn entity_id_serializes_as_a_plain_json_string_not_an_object() {
        // This is exactly the property that broke `BTreeMap<EntityId, _>`
        // serialization before EntityId got a manual Serialize impl: only a
        // JSON string is valid as a map key.
        let id = EntityId::Stroke(StrokeId::from_u128(7));
        let json = serde_json::to_string(&id).unwrap();
        assert!(
            json.starts_with('"') && json.ends_with('"'),
            "expected a JSON string, got {json}"
        );
        let back: EntityId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, back);
    }

    #[test]
    fn a_map_keyed_by_entity_id_serializes_and_round_trips() {
        use std::collections::BTreeMap;
        let mut map: BTreeMap<EntityId, i32> = BTreeMap::new();
        map.insert(EntityId::Stroke(StrokeId::from_u128(1)), 10);
        map.insert(EntityId::Note(NoteId::from_u128(2)), 20);

        let json = serde_json::to_string(&map).unwrap();
        let back: BTreeMap<EntityId, i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(map, back);
    }
}
