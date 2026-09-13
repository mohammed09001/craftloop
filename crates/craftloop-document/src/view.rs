//! View Blocks and Orthographic Sets.
//!
//! Execution 01, Phase 20, Tasks 143-149. Authority: Engine Contract 18;
//! MCP Article 30 "Orthographic View Identity".
//!
//! `page_layout.rs` (Phase 07) already established the type-level
//! boundary this phase builds inside: "moving a view block on the page
//! cannot change its internal engineering dimensions." A [`ViewBlock`]
//! only ever stores *membership* (which `PrimitiveId`s/
//! `DimensionAnnotationId`s belong to it) plus its own
//! [`PageLayoutTransform`] -- never a copy of any entity's own
//! coordinates -- so that invariant holds here by the same construction,
//! not a new mechanism.

use std::collections::BTreeSet;

use craftloop_errors::{DocumentErrorKind, DomainError, DomainResult};
use craftloop_ids::{DimensionAnnotationId, OrthographicSetId, PrimitiveId, ViewId};
use serde::{Deserialize, Serialize};

use crate::page_layout::PageLayoutTransform;

/// Task 144: the initial supported principal identities. Article 30 also
/// names `Left`/`Bottom` but explicitly marks them "if later supported"
/// -- deferred, not silently omitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrincipalViewIdentity {
    Front,
    Top,
    Right,
    Back,
}

/// One view block (Task 143): semantic identity, local coordinate frame
/// (via `layout`), geometry membership, and annotation membership.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewBlock {
    pub id: ViewId,
    /// Task 145: kept separate from whether the identity's text label is
    /// currently shown -- `None` means no identity has been established
    /// at all yet (Article 30: "The application should not enter linked
    /// orthographic behavior if it cannot establish a source view
    /// identity"), which is a different state from "identity established
    /// but its label is hidden."
    identity: Option<PrincipalViewIdentity>,
    label_visible: bool,
    pub geometry_members: BTreeSet<PrimitiveId>,
    pub annotation_members: BTreeSet<DimensionAnnotationId>,
    pub layout: PageLayoutTransform,
}

impl ViewBlock {
    pub fn new(id: ViewId) -> Self {
        Self {
            id,
            identity: None,
            label_visible: true,
            geometry_members: BTreeSet::new(),
            annotation_members: BTreeSet::new(),
            layout: PageLayoutTransform::IDENTITY,
        }
    }

    pub fn identity(&self) -> Option<PrincipalViewIdentity> {
        self.identity
    }

    /// Task 144: establish (or change) this view's principal identity.
    pub fn set_identity(&mut self, identity: PrincipalViewIdentity) {
        self.identity = Some(identity);
    }

    /// Article 30's gate: has a source view identity actually been
    /// established?
    pub fn has_established_identity(&self) -> bool {
        self.identity.is_some()
    }

    pub fn is_label_visible(&self) -> bool {
        self.label_visible
    }

    /// Task 145: hide the visible identity label. The semantic identity
    /// itself is untouched -- `identity()` still returns exactly what it
    /// did before.
    pub fn hide_label(&mut self) {
        self.label_visible = false;
    }

    pub fn show_label(&mut self) {
        self.label_visible = true;
    }

    /// Task 148: move this block in page space. Only `layout` changes --
    /// membership (and, by construction, every member entity's own
    /// stored engineering coordinates, which this type never holds a
    /// copy of) is untouched.
    pub fn move_to(&mut self, layout: PageLayoutTransform) {
        self.layout = layout;
    }

    /// Task 149: a **linked** duplicate shares this view's exact
    /// geometry/annotation membership (the same `PrimitiveId`s/
    /// `DimensionAnnotationId`s) -- an edit to that shared geometry is
    /// visible from both views, because both views point at the same
    /// underlying entities. Identity and label visibility carry over
    /// too, since a linked duplicate is presenting the *same* design
    /// state from a new page position.
    pub fn duplicate_linked(&self, new_id: ViewId, layout: PageLayoutTransform) -> ViewBlock {
        ViewBlock {
            id: new_id,
            identity: self.identity,
            label_visible: self.label_visible,
            geometry_members: self.geometry_members.clone(),
            annotation_members: self.annotation_members.clone(),
            layout,
        }
    }

    /// Task 149: an **independent** duplicate starts with no geometry or
    /// annotation membership at all, and no identity. Silently copying
    /// membership here would alias the "independent" copy onto the same
    /// entities as the original -- exactly what "independent" must not
    /// mean -- and this type has no way to *clone* a primitive into a
    /// genuinely new entity (that requires `PrimitiveId` allocation and
    /// geometry duplication, a `Page`/`Document`-level operation, not
    /// this type's job). Populating an independent copy's own geometry
    /// is therefore left as an explicit, separate step for the caller.
    pub fn duplicate_independent(&self, new_id: ViewId, layout: PageLayoutTransform) -> ViewBlock {
        ViewBlock::new(new_id).with_layout(layout)
    }

    fn with_layout(mut self, layout: PageLayoutTransform) -> Self {
        self.layout = layout;
        self
    }
}

/// Task 146: a group of linked views describing one design state.
/// "Multiple alternatives can coexist on one page" (Task 146's own
/// objective) means multiple, independent `OrthographicSet`s may exist
/// side by side -- this type only groups the views *within* one set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrthographicSet {
    pub id: OrthographicSetId,
    views: BTreeSet<ViewId>,
}

impl OrthographicSet {
    pub fn new(id: OrthographicSetId) -> Self {
        Self {
            id,
            views: BTreeSet::new(),
        }
    }

    pub fn views(&self) -> &BTreeSet<ViewId> {
        &self.views
    }

    /// Task 147: reject adding `view` if it would give this set two
    /// members sharing the same established identity (e.g. two `Front`
    /// views describing the same design state is a contradiction, not a
    /// legitimate alternative -- alternatives belong in a *different*
    /// `OrthographicSet`, per Task 146). Views with no identity yet
    /// (`None`) never collide with each other or with anything else --
    /// uniqueness only applies "where required" (Task 147's own
    /// wording), i.e. once an identity actually exists to conflict over.
    pub fn add_view(
        &mut self,
        view: &ViewBlock,
        existing_views: &[&ViewBlock],
    ) -> DomainResult<()> {
        if let Some(identity) = view.identity() {
            let collides = existing_views
                .iter()
                .filter(|other| self.views.contains(&other.id) && other.id != view.id)
                .any(|other| other.identity() == Some(identity));
            if collides {
                return Err(DomainError::Document {
                    kind: DocumentErrorKind::DuplicateViewIdentity,
                    detail: format!("this set already has a view with identity {identity:?}"),
                });
            }
        }
        self.views.insert(view.id);
        Ok(())
    }

    pub fn remove_view(&mut self, id: ViewId) -> bool {
        self.views.remove(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::Vector2;
    use craftloop_ids::CraftLoopId;

    // --- Task 143: view block model -----------------------------------

    #[test]
    fn a_new_view_block_starts_with_no_identity_and_empty_membership() {
        let view = ViewBlock::new(ViewId::new());
        assert!(!view.has_established_identity());
        assert!(view.geometry_members.is_empty());
        assert!(view.annotation_members.is_empty());
        assert_eq!(view.layout, PageLayoutTransform::IDENTITY);
    }

    #[test]
    fn geometry_and_annotation_membership_are_tracked_independently() {
        let mut view = ViewBlock::new(ViewId::new());
        let primitive = PrimitiveId::new();
        let annotation = DimensionAnnotationId::new();
        view.geometry_members.insert(primitive);
        view.annotation_members.insert(annotation);
        assert!(view.geometry_members.contains(&primitive));
        assert!(view.annotation_members.contains(&annotation));
        assert_eq!(view.geometry_members.len(), 1);
        assert_eq!(view.annotation_members.len(), 1);
    }

    // --- Task 144: principal view identity ------------------------------

    #[test]
    fn every_initial_principal_identity_can_be_set_and_read_back() {
        for identity in [
            PrincipalViewIdentity::Front,
            PrincipalViewIdentity::Top,
            PrincipalViewIdentity::Right,
            PrincipalViewIdentity::Back,
        ] {
            let mut view = ViewBlock::new(ViewId::new());
            view.set_identity(identity);
            assert_eq!(view.identity(), Some(identity));
            assert!(view.has_established_identity());
        }
    }

    // --- Task 145: visible label vs. semantic identity --------------------

    #[test]
    fn hiding_the_label_never_erases_the_identity() {
        let mut view = ViewBlock::new(ViewId::new());
        view.set_identity(PrincipalViewIdentity::Front);
        view.hide_label();
        assert!(!view.is_label_visible());
        assert_eq!(view.identity(), Some(PrincipalViewIdentity::Front));
        assert!(view.has_established_identity());
    }

    #[test]
    fn showing_the_label_again_does_not_change_identity_either() {
        let mut view = ViewBlock::new(ViewId::new());
        view.set_identity(PrincipalViewIdentity::Top);
        view.hide_label();
        view.show_label();
        assert!(view.is_label_visible());
        assert_eq!(view.identity(), Some(PrincipalViewIdentity::Top));
    }

    // --- Task 146/147: orthographic sets and identity uniqueness ----------

    #[test]
    fn two_views_with_different_identities_join_the_same_set_freely() {
        let mut front = ViewBlock::new(ViewId::new());
        front.set_identity(PrincipalViewIdentity::Front);
        let mut top = ViewBlock::new(ViewId::new());
        top.set_identity(PrincipalViewIdentity::Top);

        let mut set = OrthographicSet::new(OrthographicSetId::new());
        set.add_view(&front, &[]).unwrap();
        set.add_view(&top, &[&front]).unwrap();
        assert_eq!(set.views().len(), 2);
    }

    #[test]
    fn a_second_front_in_the_same_set_is_rejected() {
        let mut front_a = ViewBlock::new(ViewId::new());
        front_a.set_identity(PrincipalViewIdentity::Front);
        let mut front_b = ViewBlock::new(ViewId::new());
        front_b.set_identity(PrincipalViewIdentity::Front);

        let mut set = OrthographicSet::new(OrthographicSetId::new());
        set.add_view(&front_a, &[]).unwrap();
        let result = set.add_view(&front_b, &[&front_a]);
        assert!(matches!(
            result,
            Err(DomainError::Document {
                kind: DocumentErrorKind::DuplicateViewIdentity,
                ..
            })
        ));
        assert_eq!(set.views().len(), 1, "the rejected view must not be added");
    }

    #[test]
    fn multiple_unidentified_views_never_collide_with_each_other() {
        let a = ViewBlock::new(ViewId::new());
        let b = ViewBlock::new(ViewId::new());
        let mut set = OrthographicSet::new(OrthographicSetId::new());
        set.add_view(&a, &[]).unwrap();
        set.add_view(&b, &[&a]).unwrap();
        assert_eq!(set.views().len(), 2);
    }

    #[test]
    fn two_different_sets_may_each_have_their_own_front_view_task_146() {
        // "Multiple alternatives can coexist on one page": a second Front
        // describing a different design-state alternative is fine, as
        // long as it is in a different OrthographicSet.
        let mut front_a = ViewBlock::new(ViewId::new());
        front_a.set_identity(PrincipalViewIdentity::Front);
        let mut front_b = ViewBlock::new(ViewId::new());
        front_b.set_identity(PrincipalViewIdentity::Front);

        let mut set_a = OrthographicSet::new(OrthographicSetId::new());
        set_a.add_view(&front_a, &[]).unwrap();
        let mut set_b = OrthographicSet::new(OrthographicSetId::new());
        set_b.add_view(&front_b, &[]).unwrap();

        assert_eq!(set_a.views().len(), 1);
        assert_eq!(set_b.views().len(), 1);
    }

    // --- Task 148: view movement in page space -----------------------------

    #[test]
    fn moving_a_view_block_changes_only_its_layout_not_its_membership() {
        let mut view = ViewBlock::new(ViewId::new());
        let primitive = PrimitiveId::new();
        view.geometry_members.insert(primitive);
        view.set_identity(PrincipalViewIdentity::Front);

        view.move_to(PageLayoutTransform::translated(Vector2::new(500.0, -200.0)));

        assert_eq!(view.layout.offset, Vector2::new(500.0, -200.0));
        assert!(view.geometry_members.contains(&primitive));
        assert_eq!(view.identity(), Some(PrincipalViewIdentity::Front));
    }

    // --- Task 149: duplicate/copy policies ----------------------------------

    #[test]
    fn a_linked_duplicate_shares_the_exact_same_membership_and_identity() {
        let mut original = ViewBlock::new(ViewId::new());
        let primitive = PrimitiveId::new();
        original.geometry_members.insert(primitive);
        original.set_identity(PrincipalViewIdentity::Right);

        let new_id = ViewId::new();
        let copy = original.duplicate_linked(
            new_id,
            PageLayoutTransform::translated(Vector2::new(100.0, 0.0)),
        );

        assert_eq!(copy.id, new_id);
        assert_ne!(copy.id, original.id);
        assert_eq!(copy.geometry_members, original.geometry_members);
        assert_eq!(copy.identity(), Some(PrincipalViewIdentity::Right));
        assert_eq!(copy.layout.offset, Vector2::new(100.0, 0.0));
    }

    #[test]
    fn an_independent_duplicate_starts_with_no_membership_and_no_identity() {
        let mut original = ViewBlock::new(ViewId::new());
        original.geometry_members.insert(PrimitiveId::new());
        original.set_identity(PrincipalViewIdentity::Back);

        let copy = original.duplicate_independent(ViewId::new(), PageLayoutTransform::IDENTITY);

        assert!(
            copy.geometry_members.is_empty(),
            "an independent copy must not alias the original's geometry"
        );
        assert!(
            !copy.has_established_identity(),
            "an independent copy must not silently inherit identity"
        );
    }

    #[test]
    fn linked_and_independent_duplication_are_visibly_different_outcomes() {
        let mut original = ViewBlock::new(ViewId::new());
        original.geometry_members.insert(PrimitiveId::new());

        let linked = original.duplicate_linked(ViewId::new(), PageLayoutTransform::IDENTITY);
        let independent =
            original.duplicate_independent(ViewId::new(), PageLayoutTransform::IDENTITY);

        assert_eq!(linked.geometry_members.len(), 1);
        assert_eq!(independent.geometry_members.len(), 0);
    }
}
