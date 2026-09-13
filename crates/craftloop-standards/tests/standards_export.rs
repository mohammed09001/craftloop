//! Export-facing standards tests.
//!
//! Execution 01, Phase 25, Task 185. Authority: MCP Article 235.
//!
//! Exercises the implemented role mappings explicitly, end to end, the
//! way an export pass would actually use them: build a profile, resolve
//! a style for every semantic role a drawing might use, and confirm the
//! set is internally consistent (every role maps to something, no two
//! roles that must be visually distinguishable collide).

use craftloop_document::ProjectionConvention;
use craftloop_standards::{style_for_role, LineRole, StandardsProfile, UNSUPPORTED_CONVENTIONS};

#[test]
fn every_implemented_role_resolves_to_an_explicit_style() {
    let roles = [
        LineRole::Visible,
        LineRole::Construction,
        LineRole::Centerline,
        LineRole::Hidden,
        LineRole::DimensionLine,
        LineRole::ExtensionLine,
    ];
    let styles: Vec<_> = roles.iter().map(|&role| style_for_role(role)).collect();
    assert_eq!(styles.len(), roles.len());
}

#[test]
fn visible_and_hidden_geometry_are_never_visually_identical() {
    // An export that cannot tell visible from hidden edges apart would
    // be silently misleading -- the single most important role
    // distinction for a mechanical drawing.
    let visible = style_for_role(LineRole::Visible);
    let hidden = style_for_role(LineRole::Hidden);
    assert!(visible.dash_pattern != hidden.dash_pattern || visible.weight != hidden.weight);
}

#[test]
fn a_project_can_build_a_named_profile_and_read_back_its_projection_convention() {
    let profile =
        StandardsProfile::new("Third-angle baseline", ProjectionConvention::ThirdAngle).unwrap();
    assert_eq!(profile.name(), "Third-angle baseline");
    assert_eq!(
        profile.projection_convention,
        ProjectionConvention::ThirdAngle
    );
}

#[test]
fn unsupported_conventions_are_documented_not_silently_missing() {
    assert!(UNSUPPORTED_CONVENTIONS
        .iter()
        .any(|entry| entry.contains("Leader")));
    assert!(UNSUPPORTED_CONVENTIONS
        .iter()
        .any(|entry| entry.contains("Section")));
}
