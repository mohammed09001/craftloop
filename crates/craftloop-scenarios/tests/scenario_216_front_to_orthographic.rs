//! Task 216 — Front-to-Orthographic scenario.
//!
//! Execution 01, Phase 30, Task 216. Creates a labeled Front view with
//! real geometry and *zero* dimensions, and confirms it reaches linked
//! orthographic mode successfully -- MCP Article 234's explicit rule
//! ("should not require full dimensioning") proven end to end through
//! `evaluate_readiness` (Phase 21) and `transition_to_orthographic`
//! (Phase 21), not merely asserted.

use std::collections::BTreeMap;

use craftloop_command::{
    Command, CommandAction, CommandNamespace, CommandSource, RiskLevel, UndoMetadata,
};
use craftloop_document::{
    default_orthographic_layout, evaluate_readiness, transition_to_orthographic,
    OrthographicReadiness, OrthographicSet, PrincipalViewIdentity, ProjectionConvention, ViewBlock,
};
use craftloop_geometry::{Point2, Segment2};
use craftloop_ids::{CommandId, CraftLoopId, OrthographicSetId, PrimitiveId, ViewId};
use craftloop_recognition::BeautifiedPrimitive;

#[test]
fn a_front_view_with_geometry_and_no_dimensions_reaches_link_ready_and_enters_linked_mode() {
    // A labeled Front view with one real primitive, zero dimensions.
    let mut front = ViewBlock::new(ViewId::new());
    front.set_identity(PrincipalViewIdentity::Front);
    let primitive_id = PrimitiveId::new();
    front.geometry_members.insert(primitive_id);

    let mut geometry = BTreeMap::new();
    geometry.insert(
        primitive_id,
        BeautifiedPrimitive::Line(Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 0.0))),
    );

    let (readiness, issues) = evaluate_readiness(&front, &geometry);
    assert_eq!(
        readiness,
        OrthographicReadiness::LinkReady,
        "identity + real geometry + zero dimensions must still reach LinkReady (Article 234)"
    );
    assert!(
        issues.iter().all(|i| !i.is_blocker()),
        "no blocker should exist for a clean, identified view with no dimensions at all"
    );

    // Enter linked mode: an Orthographic command transitions the Front
    // view into a real set of derived-view layouts.
    let command = Command {
        id: CommandId::new(),
        action: CommandAction::Orthographic,
        source: CommandSource::Toolbar,
        namespace: CommandNamespace::Notebook,
        parameters: BTreeMap::new(),
        risk: RiskLevel::Medium,
        timestamp_seconds: 0.0,
        undo: UndoMetadata::undoable("Entered Orthographic mode"),
    };
    let layouts =
        transition_to_orthographic(&command, &front, ProjectionConvention::ThirdAngle, 50.0)
            .expect("transitioning a ready Front view to Orthographic must succeed");
    assert!(
        !layouts.is_empty(),
        "a real projection convention must produce at least one derived view layout"
    );

    // Sanity: the layouts match what the pure layout function alone
    // would produce -- transition_to_orthographic adds no hidden extra
    // behavior beyond validating the command and delegating.
    let expected =
        default_orthographic_layout(front.layout, ProjectionConvention::ThirdAngle, 50.0);
    assert_eq!(layouts, expected);

    // Build the real OrthographicSet: Front plus one derived Top view,
    // both coexisting without identity collision.
    let mut set = OrthographicSet::new(OrthographicSetId::new());
    set.add_view(&front, &[]).unwrap();

    let top_layout = layouts[&PrincipalViewIdentity::Top];
    let mut top = ViewBlock::new(ViewId::new());
    top.set_identity(PrincipalViewIdentity::Top);
    top.move_to(top_layout);
    set.add_view(&top, &[&front]).unwrap();

    assert_eq!(set.views().len(), 2);
}
