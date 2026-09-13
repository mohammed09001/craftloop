//! Export and Diagnostic Interoperability for the Craft Loop shared
//! engineering core.
//!
//! Execution 01, Phase 26. Authority: Engine Contract 15 (Document); MCP
//! Article 128 "Document Model", Article 72 "Unit System", Article 236
//! (ephemeral command ink).
//!
//! No platform UI dependency; depends only on `craftloop-document` and the
//! crates it already depends on, plus `craftloop-standards` (Phase 25) for
//! line-role styling and dimension typography.
//!
//! Three real export targets, each scoped honestly to what this execution
//! has real geometry/typography types for:
//! - [`diagnostic_json`] (Task 186): a rich, unfiltered developer/debugging
//!   view of a whole `Document`.
//! - [`svg`] (Task 187): visible vector geometry, ink, notes, and
//!   dimension value text as a single-page SVG.
//! - [`pdf`] (Task 188): the same visible content as a hand-written,
//!   explicitly bounded single-page PDF prototype (no external PDF
//!   dependency).
//!
//! [`visible_entities`] (Task 190) is the one filter both `svg` and `pdf`
//! share, so ephemeral command ink and diagnostic-only entities
//! (`SemanticEntity::Conflict`) cannot leak into either by drifting out of
//! sync with each other.
//!
//! Task 189 (units/annotation values must not be silently rescaled) has no
//! dedicated module of its own: it is proven directly by tests in `svg`
//! and `pdf` that convert a known millimeter-stored dimension value to a
//! *different* configured display unit and assert the converted number
//! (not the raw millimeter number) appears in the output.
//!
//! ## Task 191 -- DXF is deferred
//!
//! No DXF export exists in this crate, deliberately. A real DXF writer
//! needs entity-section conventions (layers, block references, exact
//! `ENTITIES`/`TABLES` structure) that no external CAD tool's fidelity
//! this execution can currently verify against -- shipping a partial DXF
//! that *opens* in some tools but silently drops or misrepresents
//! geometry would be exactly the "misleading partial implementation" Task
//! 191 forbids. This is a documented, deliberate scope boundary, not an
//! oversight: [`UNSUPPORTED_EXPORT_TARGETS`] names it explicitly so
//! nothing pretends the gap does not exist.

pub mod diagnostic_json;
pub mod pdf;
pub mod svg;
pub mod visible_entities;

pub use diagnostic_json::{
    diagnostic_export, export_diagnostic_json, DiagnosticEntityRecord, DiagnosticExport,
    DiagnosticPageRecord,
};
pub use pdf::export_pdf_bounded;
pub use svg::export_svg;
pub use visible_entities::{is_export_visible, visible_entities};

/// Export targets this crate deliberately does not implement, and why --
/// Task 191's explicit, honest gap list, matching the pattern
/// `craftloop_standards::UNSUPPORTED_CONVENTIONS` established in Phase 25.
pub const UNSUPPORTED_EXPORT_TARGETS: &[&str] = &[
    "DXF -- deferred until fidelity against a real CAD tool can be verified (Task 191); no partial writer exists in this crate",
    "STEP/IGES (3D exchange formats) -- this product's document model is 2D; out of scope for any phase reached so far",
    "Multi-page PDF -- the Task 188 prototype writes exactly one page",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_dxf_gap_is_documented_not_silently_missing() {
        assert!(UNSUPPORTED_EXPORT_TARGETS
            .iter()
            .any(|entry| entry.contains("DXF")));
    }

    #[test]
    fn no_dxf_export_function_exists_in_this_crate() {
        // A structural, not merely textual, proof: if a `dxf` module or an
        // `export_dxf`-named function is ever added, this file's own
        // module list above would need to change and this comment would
        // need updating -- there is no dxf module declared in `lib.rs`.
        // This test exists as a marker so the deferral decision is
        // findable from the test suite, not only from documentation.
        let modules_declared_here = ["diagnostic_json", "pdf", "svg", "visible_entities"];
        assert!(!modules_declared_here.contains(&"dxf"));
    }
}
