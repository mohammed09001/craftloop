//! Projection convention, default layout mapping, Orthographic
//! Readiness, blocker classification, and the Ortho command transition.
//!
//! Execution 01, Phase 21, Tasks 150-156. Authority: MCP Article 33
//! "Orthographic Readiness"; Article 234 "Detailed Specification of the
//! Orthographic Readiness Engine"; Articles 488-490 "Projection
//! Convention"/"First-Angle Projection"/"Third-Angle Projection".

use std::collections::BTreeMap;

use craftloop_command::{Command, CommandAction};
use craftloop_errors::{CommandErrorKind, DomainError, DomainResult};
use craftloop_geometry::Vector2;
use craftloop_ids::PrimitiveId;
use craftloop_recognition::BeautifiedPrimitive;

use crate::page_layout::PageLayoutTransform;
use crate::view::{PrincipalViewIdentity, ViewBlock};

/// Task 150: exactly Article 488's two named conventions -- no
/// speculative third option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionConvention {
    FirstAngle,
    ThirdAngle,
}

/// Task 151: the default page-space offset for `identity`, relative to
/// the `Front` view, under `convention`.
///
/// This is a deliberately simplified default mapping, not a claim of
/// certified drafting-standard compliance (Article 235's own caution:
/// "Version 1 should avoid claiming full standards compliance unless the
/// implemented subset has been reviewed professionally"; Article 489:
/// "Craft Loop should follow the selected standards profile rather than
/// inventing its own arrangement" -- a real, profile-driven arrangement
/// is the Standards Engine's job, Phase 25, not this phase's). What this
/// function *does* honestly guarantee, and what Task 151 actually asks
/// for, is that the two conventions produce genuinely different, and
/// internally consistent, arrangements: third-angle places `Top` above
/// and `Right`/`Back` to the right of `Front` (the common US/ANSI
/// convention); first-angle mirrors both axes (the common ISO
/// convention). `+y` is treated as "up" in page space by this function's
/// own convention -- a rendering-layer detail, not asserted here as a
/// universal one.
pub fn default_layout_offset(
    identity: PrincipalViewIdentity,
    convention: ProjectionConvention,
    spacing: f64,
) -> Vector2 {
    let sign = match convention {
        ProjectionConvention::ThirdAngle => 1.0,
        ProjectionConvention::FirstAngle => -1.0,
    };
    match identity {
        PrincipalViewIdentity::Front => Vector2::ZERO,
        PrincipalViewIdentity::Top => Vector2::new(0.0, sign * spacing),
        PrincipalViewIdentity::Right => Vector2::new(sign * spacing, 0.0),
        PrincipalViewIdentity::Back => Vector2::new(sign * 2.0 * spacing, 0.0),
    }
}

/// Task 151/155: compute the full default layout for every identity
/// *other than* `Front`, positioned relative to `front_layout` (the
/// source view's own, unmoved page position -- Task 155's "do not
/// visually teleport the user away from the sketch"). Returns a fresh
/// map every call: this function is the source of truth for what the
/// layout *should* be, not a cached/stored value a caller could let
/// drift (Task 151's "without making page layout the source of truth").
pub fn default_orthographic_layout(
    front_layout: PageLayoutTransform,
    convention: ProjectionConvention,
    spacing: f64,
) -> BTreeMap<PrincipalViewIdentity, PageLayoutTransform> {
    [
        PrincipalViewIdentity::Top,
        PrincipalViewIdentity::Right,
        PrincipalViewIdentity::Back,
    ]
    .into_iter()
    .map(|identity| {
        let offset = default_layout_offset(identity, convention, spacing);
        let combined = Vector2::new(
            front_layout.offset.x + offset.x,
            front_layout.offset.y + offset.y,
        );
        (identity, PageLayoutTransform::translated(combined))
    })
    .collect()
}

/// Task 152: Article 33's five graded readiness levels, in increasing
/// order (`Ord` follows declaration order intentionally, so a caller can
/// compare "at least Link Ready" with `>=`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrthographicReadiness {
    /// Recognizable geometry exists.
    DraftReady,
    /// The source view has an explicit view identity.
    IdentityReady,
    /// Enough structure exists to create orthographic relationships.
    LinkReady,
    /// Critical unknowns required by current relationships have been
    /// supplied.
    Resolved,
    /// The drawing has sufficient constraints for predictable editing.
    Constrained,
}

/// Task 153: Article 234's exact distinction -- "the engine should
/// report blockers separately from incompleteness." A `Blocker`
/// prevents automatic view semantics outright; a `Gap` is merely
/// unresolved information that does not block progression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadinessIssue {
    /// Article 234: "A missing View Identity is a blocker for automatic
    /// view semantics."
    MissingViewIdentity,
    /// Article 234: "A local constraint contradiction may be a blocker
    /// if propagation would amplify the conflict" -- represented here as
    /// degenerate/corrupted local geometry (`craftloop-consistency`,
    /// Phase 14), the one contradiction category already real and
    /// checkable in this workspace today.
    DegenerateGeometry(PrimitiveId),
    /// Article 234: "A missing depth is incompleteness," not a blocker.
    /// Represented generically (this phase does not have a specific
    /// "depth" field to check against -- no task before this one
    /// introduced one -- so this variant is named but not yet
    /// constructed by [`evaluate_readiness`]; a later phase that adds a
    /// concrete "missing depth" check constructs it, without needing a
    /// new type).
    UnresolvedDimension,
}

impl ReadinessIssue {
    /// Article 234's own rule made directly checkable: does this issue
    /// block automatic view semantics, or is it merely incompleteness a
    /// caller may progress past?
    pub fn is_blocker(&self) -> bool {
        !matches!(self, ReadinessIssue::UnresolvedDimension)
    }
}

/// Task 152/153/156: evaluate `view`'s readiness against its member
/// geometry. Never requires full dimensioning (Article 234: "It should
/// not require full dimensioning") -- a view with an identity, real
/// geometry, and zero dimensions still reaches `LinkReady` (Task 156).
pub fn evaluate_readiness(
    view: &ViewBlock,
    geometry: &BTreeMap<PrimitiveId, BeautifiedPrimitive>,
) -> (OrthographicReadiness, Vec<ReadinessIssue>) {
    let mut issues = Vec::new();

    let has_geometry = !view.geometry_members.is_empty();
    if !has_geometry {
        // No structured geometry at all: nothing further to evaluate.
        return (OrthographicReadiness::DraftReady, issues);
    }

    for &primitive_id in &view.geometry_members {
        if let Some(primitive) = geometry.get(&primitive_id) {
            if craftloop_consistency::validate_primitive_geometry(primitive_id, primitive).is_some()
            {
                issues.push(ReadinessIssue::DegenerateGeometry(primitive_id));
            }
        }
    }

    if !view.has_established_identity() {
        issues.push(ReadinessIssue::MissingViewIdentity);
    }

    // `MissingViewIdentity` is itself a blocker, so a blocker-free result
    // already implies an established identity -- no separate identity
    // check is needed here.
    let has_blocker = issues.iter().any(ReadinessIssue::is_blocker);
    let level = if has_blocker {
        OrthographicReadiness::DraftReady
    } else {
        // Task 156: identity established, real geometry present, no
        // blockers -- Link Ready, with zero dimensions required.
        OrthographicReadiness::LinkReady
    };
    (level, issues)
}

/// Task 154: one transaction, regardless of whether `command` came from
/// the toolbar or the Ink Command Language -- both are just a `Command`
/// with `action == CommandAction::Orthographic` by the time this
/// function sees them (Phase 19's `CommandBus` already unifies the
/// source channel; this function is the "single semantic operation"
/// Task 142 named, applied to this one specific transition). Computes
/// the default layout for the new orthographic views without moving
/// `source` itself (Task 155).
pub fn transition_to_orthographic(
    command: &Command,
    source: &ViewBlock,
    convention: ProjectionConvention,
    spacing: f64,
) -> DomainResult<BTreeMap<PrincipalViewIdentity, PageLayoutTransform>> {
    if command.action != CommandAction::Orthographic {
        return Err(DomainError::Command {
            kind: CommandErrorKind::InvalidForNamespace,
            detail: format!(
                "transition_to_orthographic requires an Orthographic command, got {:?}",
                command.action
            ),
        });
    }
    Ok(default_orthographic_layout(
        source.layout,
        convention,
        spacing,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_command::{CommandNamespace, CommandSource, RiskLevel, UndoMetadata};
    use craftloop_geometry::{Point2, Segment2};
    use craftloop_ids::{CommandId, CraftLoopId, ViewId};
    use craftloop_recognition::Beautified;
    use std::collections::BTreeMap as Map;

    // --- Task 150/151: projection convention and default layout -----------

    #[test]
    fn front_never_moves_under_either_convention() {
        assert_eq!(
            default_layout_offset(
                PrincipalViewIdentity::Front,
                ProjectionConvention::ThirdAngle,
                100.0
            ),
            Vector2::ZERO
        );
        assert_eq!(
            default_layout_offset(
                PrincipalViewIdentity::Front,
                ProjectionConvention::FirstAngle,
                100.0
            ),
            Vector2::ZERO
        );
    }

    #[test]
    fn third_angle_places_top_above_and_right_to_the_right() {
        let top = default_layout_offset(
            PrincipalViewIdentity::Top,
            ProjectionConvention::ThirdAngle,
            100.0,
        );
        let right = default_layout_offset(
            PrincipalViewIdentity::Right,
            ProjectionConvention::ThirdAngle,
            100.0,
        );
        assert_eq!(top, Vector2::new(0.0, 100.0));
        assert_eq!(right, Vector2::new(100.0, 0.0));
    }

    #[test]
    fn switching_convention_changes_the_layout_task_666() {
        // MCP Article 666's own validation scenario: choose first-angle,
        // verify default layout, switch to third-angle, verify the
        // layout changes.
        let third = default_layout_offset(
            PrincipalViewIdentity::Top,
            ProjectionConvention::ThirdAngle,
            100.0,
        );
        let first = default_layout_offset(
            PrincipalViewIdentity::Top,
            ProjectionConvention::FirstAngle,
            100.0,
        );
        assert_ne!(third, first);
    }

    #[test]
    fn default_orthographic_layout_never_repositions_the_source_view() {
        let front_layout = PageLayoutTransform::translated(Vector2::new(50.0, 25.0));
        let layout =
            default_orthographic_layout(front_layout, ProjectionConvention::ThirdAngle, 100.0);
        // Front is not even a key in the output -- it is never
        // repositioned by this function (Task 155).
        assert!(!layout.contains_key(&PrincipalViewIdentity::Front));
        assert_eq!(layout.len(), 3);
    }

    #[test]
    fn the_computed_layout_is_positioned_relative_to_the_sources_actual_position() {
        let front_layout = PageLayoutTransform::translated(Vector2::new(1000.0, 500.0));
        let layout =
            default_orthographic_layout(front_layout, ProjectionConvention::ThirdAngle, 100.0);
        let top = layout[&PrincipalViewIdentity::Top];
        assert_eq!(top.offset, Vector2::new(1000.0, 600.0));
    }

    // --- Task 152/156: readiness levels, including zero-dimension ---------

    fn line_primitive() -> Beautified {
        Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(
                Point2::new(0.0, 0.0),
                Point2::new(10.0, 0.0),
            )),
            displacement: 0.0,
        }
    }

    #[test]
    fn an_empty_view_is_only_draft_ready() {
        let view = ViewBlock::new(ViewId::new());
        let (level, issues) = evaluate_readiness(&view, &Map::new());
        assert_eq!(level, OrthographicReadiness::DraftReady);
        assert!(issues.is_empty());
    }

    #[test]
    fn geometry_without_an_identity_is_draft_ready_with_a_blocker() {
        let mut view = ViewBlock::new(ViewId::new());
        let primitive_id = PrimitiveId::new();
        view.geometry_members.insert(primitive_id);
        let mut geometry = Map::new();
        geometry.insert(primitive_id, line_primitive().primitive);
        let (level, issues) = evaluate_readiness(&view, &geometry);
        assert_eq!(level, OrthographicReadiness::DraftReady);
        assert!(issues.contains(&ReadinessIssue::MissingViewIdentity));
        assert!(issues[0].is_blocker());
    }

    #[test]
    fn a_labeled_view_with_real_geometry_and_zero_dimensions_reaches_link_ready_task_156() {
        // Task 156's exact scenario: a structured, labeled view with no
        // dimensions at all must still be allowed into the linked
        // workflow.
        let mut view = ViewBlock::new(ViewId::new());
        view.set_identity(PrincipalViewIdentity::Front);
        let primitive_id = PrimitiveId::new();
        view.geometry_members.insert(primitive_id);
        let mut geometry = Map::new();
        geometry.insert(primitive_id, line_primitive().primitive);

        let (level, issues) = evaluate_readiness(&view, &geometry);
        assert_eq!(level, OrthographicReadiness::LinkReady);
        assert!(
            issues.is_empty(),
            "zero dimensions must not itself be reported as an issue: {issues:?}"
        );
    }

    // --- Task 153: blocker vs. incompleteness classification --------------

    #[test]
    fn missing_view_identity_is_classified_as_a_blocker() {
        assert!(ReadinessIssue::MissingViewIdentity.is_blocker());
    }

    #[test]
    fn an_unresolved_dimension_is_classified_as_incompleteness_not_a_blocker() {
        assert!(!ReadinessIssue::UnresolvedDimension.is_blocker());
    }

    #[test]
    fn degenerate_geometry_is_classified_as_a_blocker() {
        assert!(ReadinessIssue::DegenerateGeometry(PrimitiveId::new()).is_blocker());
    }

    #[test]
    fn degenerate_member_geometry_prevents_link_ready_even_with_an_identity() {
        let mut view = ViewBlock::new(ViewId::new());
        view.set_identity(PrincipalViewIdentity::Front);
        let primitive_id = PrimitiveId::new();
        view.geometry_members.insert(primitive_id);
        let degenerate = Beautified {
            primitive: BeautifiedPrimitive::Line(Segment2::new(Point2::ORIGIN, Point2::ORIGIN)),
            displacement: 0.0,
        };
        let mut geometry = Map::new();
        geometry.insert(primitive_id, degenerate.primitive);

        let (level, issues) = evaluate_readiness(&view, &geometry);
        assert_eq!(level, OrthographicReadiness::DraftReady);
        assert!(issues
            .iter()
            .any(|i| matches!(i, ReadinessIssue::DegenerateGeometry(_))));
    }

    // --- Task 154: Ortho command transition --------------------------------

    fn orthographic_command(source: CommandSource) -> Command {
        Command {
            id: CommandId::new(),
            action: CommandAction::Orthographic,
            source,
            namespace: CommandNamespace::Notebook,
            parameters: Map::new(),
            risk: RiskLevel::Medium,
            timestamp_seconds: 0.0,
            undo: UndoMetadata::undoable("Entered Orthographic mode"),
        }
    }

    #[test]
    fn a_toolbar_and_an_ink_command_orthographic_transition_produce_the_identical_layout() {
        let source = ViewBlock::new(ViewId::new());
        let toolbar_command = orthographic_command(CommandSource::Toolbar);
        let ink_command = orthographic_command(CommandSource::InkCommand);

        let from_toolbar = transition_to_orthographic(
            &toolbar_command,
            &source,
            ProjectionConvention::ThirdAngle,
            100.0,
        )
        .unwrap();
        let from_ink = transition_to_orthographic(
            &ink_command,
            &source,
            ProjectionConvention::ThirdAngle,
            100.0,
        )
        .unwrap();

        assert_eq!(
            from_toolbar, from_ink,
            "both source channels must produce the same transition"
        );
    }

    #[test]
    fn a_non_orthographic_command_is_rejected() {
        let source = ViewBlock::new(ViewId::new());
        let mut command = orthographic_command(CommandSource::Toolbar);
        command.action = CommandAction::Pen;
        let result =
            transition_to_orthographic(&command, &source, ProjectionConvention::ThirdAngle, 100.0);
        assert!(result.is_err());
    }
}
