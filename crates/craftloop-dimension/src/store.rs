//! Dimension storage, atomic driving-value edits, and annotation
//! visibility.
//!
//! Execution 01, Phase 10, Tasks 075-076. Authority: Engine Contract 09
//! ("Never silently drop confirmed constraints" -- applies equally to
//! never silently dropping a confirmed dimension when only its
//! presentation changes).
//!
//! **Task 075, "solver-validated":** no constraint solver exists yet
//! (Phase 11-12). `edit_driving_value` validates everything this layer
//! *can* validate today -- the role check (only `Driving` is editable),
//! the value's own domain validity, and its feasible range if one is set
//! -- and is structured so a solver feasibility check slots in as one more
//! validation step before the value is committed, without changing this
//! function's atomicity contract: either the whole edit succeeds, or
//! nothing about the dimension changes.

use std::collections::BTreeMap;

use craftloop_errors::{DimensionErrorKind, DomainError, DomainResult};
use craftloop_ids::{DimensionAnnotationId, DimensionId};
use serde::{Deserialize, Serialize};

use crate::annotation::DimensionAnnotation;
use crate::dimension::SemanticDimension;

/// Semantic dimensions and their presentation annotations, kept as two
/// independent maps (Task 077: "persist semantic and presentation state
/// independently") -- each serializes and deserializes on its own; neither
/// is nested inside the other.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DimensionStore {
    dimensions: BTreeMap<DimensionId, SemanticDimension>,
    annotations: BTreeMap<DimensionAnnotationId, DimensionAnnotation>,
}

impl DimensionStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_dimension(&mut self, dimension: SemanticDimension) -> DomainResult<()> {
        if self.dimensions.contains_key(&dimension.id) {
            return Err(DomainError::Dimension {
                kind: DimensionErrorKind::DuplicateId,
                detail: format!("dimension {:?} already exists", dimension.id),
            });
        }
        self.dimensions.insert(dimension.id, dimension);
        Ok(())
    }

    pub fn dimension(&self, id: DimensionId) -> Option<&SemanticDimension> {
        self.dimensions.get(&id)
    }

    /// Remove a dimension **and** every annotation that presented it. This
    /// is the one operation that is allowed to take the semantic dimension
    /// away -- an explicit, deliberate deletion, never a side effect of
    /// hiding or deleting just one annotation (Task 076).
    pub fn remove_dimension(
        &mut self,
        id: DimensionId,
    ) -> Option<(SemanticDimension, Vec<DimensionAnnotation>)> {
        let dimension = self.dimensions.remove(&id)?;
        let orphaned_ids: Vec<DimensionAnnotationId> = self
            .annotations
            .iter()
            .filter(|(_, a)| a.dimension_id == id)
            .map(|(aid, _)| *aid)
            .collect();
        let orphaned: Vec<DimensionAnnotation> = orphaned_ids
            .into_iter()
            .filter_map(|aid| self.annotations.remove(&aid))
            .collect();
        Some((dimension, orphaned))
    }

    /// Atomically change a `Driving` dimension's value (Task 075). Fails
    /// -- with the dimension left completely unchanged -- if the
    /// dimension does not exist, its role does not permit direct edits, or
    /// the new value is invalid (including outside a feasible range).
    /// Returns the previous value on success, so a caller building undo
    /// history has it for free.
    pub fn edit_driving_value(&mut self, id: DimensionId, new_value: f64) -> DomainResult<f64> {
        let dimension = self
            .dimensions
            .get_mut(&id)
            .ok_or_else(|| DomainError::Dimension {
                kind: DimensionErrorKind::UnknownDimension,
                detail: format!("no dimension with id {id:?}"),
            })?;

        if !dimension.role.is_directly_editable() {
            return Err(DomainError::Dimension {
                kind: DimensionErrorKind::NotEditable,
                detail: format!(
                    "dimension {id:?} has role {:?}, which is not directly editable",
                    dimension.role
                ),
            });
        }

        let previous = dimension.value();
        dimension.set_value_unchecked(new_value)?;
        Ok(previous)
    }

    pub fn add_annotation(&mut self, annotation: DimensionAnnotation) -> DomainResult<()> {
        if !self.dimensions.contains_key(&annotation.dimension_id) {
            return Err(DomainError::Dimension {
                kind: DimensionErrorKind::UnknownDimension,
                detail: format!(
                    "annotation references unknown dimension {:?}",
                    annotation.dimension_id
                ),
            });
        }
        if self.annotations.contains_key(&annotation.id) {
            return Err(DomainError::Dimension {
                kind: DimensionErrorKind::DuplicateId,
                detail: format!("annotation {:?} already exists", annotation.id),
            });
        }
        self.annotations.insert(annotation.id, annotation);
        Ok(())
    }

    pub fn annotations_for(&self, dimension_id: DimensionId) -> Vec<&DimensionAnnotation> {
        self.annotations
            .values()
            .filter(|a| a.dimension_id == dimension_id)
            .collect()
    }

    /// Hide (not delete) an annotation. The dimension and every other
    /// annotation of it are untouched (Task 076).
    pub fn hide_annotation(&mut self, id: DimensionAnnotationId) -> DomainResult<()> {
        let annotation = self
            .annotations
            .get_mut(&id)
            .ok_or_else(|| DomainError::Dimension {
                kind: DimensionErrorKind::UnknownDimension,
                detail: format!("no annotation with id {id:?}"),
            })?;
        annotation.visible = false;
        Ok(())
    }

    /// Delete one annotation. The dimension it presented (and any other
    /// annotation of it) is untouched (Task 076) -- this only removes this
    /// one presentation, never the semantic dimension.
    pub fn remove_annotation(&mut self, id: DimensionAnnotationId) -> Option<DimensionAnnotation> {
        self.annotations.remove(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kind::DimensionKind;
    use crate::role::DimensionRole;
    use crate::target::DimensionTarget;
    use craftloop_geometry::Point2;
    use craftloop_ids::{CraftLoopId, PrimitiveId};

    fn driving_dimension() -> SemanticDimension {
        SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            25.0,
        )
        .unwrap()
    }

    fn reference_dimension() -> SemanticDimension {
        SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Reference,
            DimensionTarget::Single(PrimitiveId::new()),
            25.0,
        )
        .unwrap()
    }

    #[test]
    fn editing_a_driving_dimension_succeeds_and_returns_the_previous_value() {
        let mut store = DimensionStore::new();
        let dim = driving_dimension();
        let id = dim.id;
        store.insert_dimension(dim).unwrap();

        let previous = store.edit_driving_value(id, 40.0).unwrap();
        assert_eq!(previous, 25.0);
        assert_eq!(store.dimension(id).unwrap().value(), 40.0);
    }

    #[test]
    fn editing_a_reference_dimension_is_rejected_and_leaves_it_unchanged() {
        let mut store = DimensionStore::new();
        let dim = reference_dimension();
        let id = dim.id;
        store.insert_dimension(dim).unwrap();

        let result = store.edit_driving_value(id, 999.0);
        assert!(matches!(
            result,
            Err(DomainError::Dimension {
                kind: DimensionErrorKind::NotEditable,
                ..
            })
        ));
        assert_eq!(store.dimension(id).unwrap().value(), 25.0);
    }

    #[test]
    fn an_invalid_new_value_is_rejected_and_leaves_the_dimension_unchanged() {
        let mut store = DimensionStore::new();
        let dim = driving_dimension();
        let id = dim.id;
        store.insert_dimension(dim).unwrap();

        assert!(store.edit_driving_value(id, -5.0).is_err());
        assert_eq!(
            store.dimension(id).unwrap().value(),
            25.0,
            "a rejected edit must not partially apply"
        );
    }

    #[test]
    fn editing_an_unknown_dimension_is_a_structured_error() {
        let mut store = DimensionStore::new();
        assert!(store.edit_driving_value(DimensionId::new(), 1.0).is_err());
    }

    #[test]
    fn hiding_an_annotation_does_not_touch_the_dimension_or_other_annotations() {
        let mut store = DimensionStore::new();
        let dim = driving_dimension();
        let id = dim.id;
        store.insert_dimension(dim).unwrap();

        let a1 = DimensionAnnotation::new(DimensionAnnotationId::new(), id, Point2::ORIGIN);
        let a2 = DimensionAnnotation::new(DimensionAnnotationId::new(), id, Point2::new(1.0, 1.0));
        store.add_annotation(a1.clone()).unwrap();
        store.add_annotation(a2.clone()).unwrap();

        store.hide_annotation(a1.id).unwrap();

        assert!(
            store.dimension(id).is_some(),
            "hiding an annotation must never delete the dimension"
        );
        assert_eq!(
            store.annotations_for(id).len(),
            2,
            "the other annotation must be untouched"
        );
        let hidden = store
            .annotations_for(id)
            .into_iter()
            .find(|a| a.id == a1.id)
            .unwrap();
        assert!(!hidden.visible);
    }

    #[test]
    fn deleting_an_annotation_does_not_delete_the_dimension() {
        let mut store = DimensionStore::new();
        let dim = driving_dimension();
        let id = dim.id;
        store.insert_dimension(dim).unwrap();

        let annotation = DimensionAnnotation::new(DimensionAnnotationId::new(), id, Point2::ORIGIN);
        let annotation_id = annotation.id;
        store.add_annotation(annotation).unwrap();

        let removed = store.remove_annotation(annotation_id);
        assert!(removed.is_some());
        assert!(
            store.dimension(id).is_some(),
            "deleting an annotation must never delete the semantic dimension"
        );
        assert!(store.annotations_for(id).is_empty());
    }

    #[test]
    fn a_dimension_can_have_zero_one_or_many_annotations() {
        let mut store = DimensionStore::new();
        let dim = driving_dimension();
        let id = dim.id;
        store.insert_dimension(dim).unwrap();

        assert_eq!(store.annotations_for(id).len(), 0);

        store
            .add_annotation(DimensionAnnotation::new(
                DimensionAnnotationId::new(),
                id,
                Point2::ORIGIN,
            ))
            .unwrap();
        assert_eq!(store.annotations_for(id).len(), 1);

        store
            .add_annotation(DimensionAnnotation::new(
                DimensionAnnotationId::new(),
                id,
                Point2::new(5.0, 5.0),
            ))
            .unwrap();
        assert_eq!(store.annotations_for(id).len(), 2);
    }

    #[test]
    fn explicitly_removing_a_dimension_cascades_to_its_annotations() {
        let mut store = DimensionStore::new();
        let dim = driving_dimension();
        let id = dim.id;
        store.insert_dimension(dim).unwrap();
        store
            .add_annotation(DimensionAnnotation::new(
                DimensionAnnotationId::new(),
                id,
                Point2::ORIGIN,
            ))
            .unwrap();

        let (_, orphaned) = store.remove_dimension(id).unwrap();
        assert_eq!(orphaned.len(), 1);
        assert!(store.dimension(id).is_none());
        assert!(store.annotations_for(id).is_empty());
    }

    #[test]
    fn an_annotation_cannot_reference_an_unknown_dimension() {
        let mut store = DimensionStore::new();
        let annotation = DimensionAnnotation::new(
            DimensionAnnotationId::new(),
            DimensionId::new(),
            Point2::ORIGIN,
        );
        assert!(store.add_annotation(annotation).is_err());
    }

    #[test]
    fn inserting_a_dimension_with_a_duplicate_id_is_rejected() {
        let mut store = DimensionStore::new();
        let dim = driving_dimension();
        let id = dim.id;
        store.insert_dimension(dim).unwrap();

        let duplicate = SemanticDimension::new(
            id,
            DimensionKind::Angular,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            1.0,
        )
        .unwrap();
        assert!(store.insert_dimension(duplicate).is_err());
    }
}
