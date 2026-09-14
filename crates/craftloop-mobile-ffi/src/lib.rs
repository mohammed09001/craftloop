//! Android/iPad FFI boundary.
//!
//! Execution 01, Phase 28, Task 201 (Android/UniFFI-Kotlin); Phase 29,
//! Task 207 reuses this same crate for Swift, since UniFFI generates
//! both language bindings from one Rust interface definition. Authority:
//! Engine Contract 27 ("Adapters translate platform input into the
//! shared normalized model; the shared core never imports a platform
//! SDK"); MCP Article 4 (no fabricated capability).
//!
//! **The one rule this crate exists to enforce**: Kotlin/Jetpack
//! Ink/SwiftUI/PencilKit concepts stop here. No domain crate
//! (`craftloop-input`, `craftloop-ink`, `craftloop-command`, ...) knows
//! this crate -- or a mobile platform -- exists; `craftloop-mobile-ffi`
//! is a one-way translation layer *on top of* them. Every `Ffi*` type
//! here is a plain, FFI-safe mirror of a real domain type (no lifetimes,
//! no `&'static str`, no generics UniFFI cannot cross the boundary with)
//! with an explicit `From`/`TryFrom` conversion -- never the domain type
//! re-exported directly, so a domain type can change shape without
//! silently breaking every mobile binding, and a UniFFI-only concept
//! (like the C-safe FFI enum tag) can never leak into the domain crates
//! it does not need it in.
//!
//! Two representative boundaries are implemented, not an exhaustive
//! mirror of every domain type (Task 201's own scope: "map only
//! normalized inputs/commands/data", not "mirror everything"):
//! - **Input**: [`FfiPointerSample`]/[`validate_stroke`] -- Task 204's
//!   Jetpack Ink mapping target (in-progress/final strokes, pressure,
//!   tilt, source, capability flags) crossing into a real domain call
//!   (`craftloop_ink::Stroke::new`), proving the boundary is a genuine
//!   bridge, not just a data-shape exercise.
//! - **Command**: [`FfiCommandNamespace`]/[`resolve_command`] -- Article
//!   237's command grammar (Phase 19), reachable from a mobile client the
//!   same way the Windows harness reaches it today.

uniffi::setup_scaffolding!();

pub mod session;

use craftloop_command::grammar;
use craftloop_command::{CommandAction, CommandNamespace, GrammarMatch};
use craftloop_ids::{CraftLoopId, StrokeId};
use craftloop_ink::Stroke;
use craftloop_input::{InputCapabilities, PointerButtons, PointerSample, PointerSource};

pub use session::*;

// ---------------------------------------------------------------------
// Input boundary
// ---------------------------------------------------------------------

/// Mirrors `craftloop_input::PointerSource`. Kept as its own FFI enum
/// (Kotlin/Swift get a real sealed enum, not a raw integer) rather than
/// exposing the domain enum directly across the boundary.
#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiPointerSource {
    SimulatedMouse,
    Stylus,
    Touch,
}

impl From<PointerSource> for FfiPointerSource {
    fn from(source: PointerSource) -> Self {
        match source {
            PointerSource::SimulatedMouse => FfiPointerSource::SimulatedMouse,
            PointerSource::Stylus => FfiPointerSource::Stylus,
            PointerSource::Touch => FfiPointerSource::Touch,
        }
    }
}

impl From<FfiPointerSource> for PointerSource {
    fn from(source: FfiPointerSource) -> Self {
        match source {
            FfiPointerSource::SimulatedMouse => PointerSource::SimulatedMouse,
            FfiPointerSource::Stylus => PointerSource::Stylus,
            FfiPointerSource::Touch => PointerSource::Touch,
        }
    }
}

/// One normalized pointer sample crossing the FFI boundary -- Task 204's
/// mapping target for Jetpack Ink's `MutableStrokeInputBatch`/finished
/// `Stroke` data (position, pressure, tilt, source) and
/// `InputCapabilities`/`PointerButtons`, flattened into plain FFI-safe
/// fields rather than nested records, since UniFFI can cross either
/// shape equally well and a flat record is simpler generated Kotlin/
/// Swift to read.
#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiPointerSample {
    pub x: f64,
    pub y: f64,
    pub timestamp_seconds: f64,
    pub pressure: Option<f64>,
    pub tilt_x_deg: Option<f64>,
    pub tilt_y_deg: Option<f64>,
    pub source: FfiPointerSource,
    pub button_primary: bool,
    pub button_secondary: bool,
    pub button_barrel: bool,
    /// Real, queried capability flags (Article 4: never fabricated) --
    /// see `craftloop_input::InputCapabilities`'s own doc comment. An
    /// Android/iPad adapter must set these from an actual platform
    /// capability query, never a guess.
    pub capability_pressure: bool,
    pub capability_tilt: bool,
    pub capability_hover: bool,
    pub capability_palm_rejection: bool,
    pub capability_eraser: bool,
}

impl From<&FfiPointerSample> for PointerSample {
    fn from(sample: &FfiPointerSample) -> Self {
        PointerSample {
            position: craftloop_geometry_compat_point(sample.x, sample.y),
            timestamp_seconds: sample.timestamp_seconds,
            pressure: sample.pressure,
            tilt_x_deg: sample.tilt_x_deg,
            tilt_y_deg: sample.tilt_y_deg,
            source: sample.source.into(),
            buttons: PointerButtons {
                primary: sample.button_primary,
                secondary: sample.button_secondary,
                barrel: sample.button_barrel,
            },
            capabilities: InputCapabilities {
                pressure: sample.capability_pressure,
                tilt: sample.capability_tilt,
                hover: sample.capability_hover,
                palm_rejection: sample.capability_palm_rejection,
                eraser: sample.capability_eraser,
            },
        }
    }
}

/// Small local helper so this module's one geometry construction site
/// stays obviously correct without importing `craftloop-geometry`'s
/// whole surface for a single `Point2::new` call.
fn craftloop_geometry_compat_point(x: f64, y: f64) -> craftloop_geometry::Point2 {
    craftloop_geometry::Point2::new(x, y)
}

/// The result of trying to build a real domain stroke from FFI samples
/// -- crossing back out, proving this boundary is a two-way bridge into
/// the real domain core (`craftloop_ink::Stroke::new`'s own validation:
/// non-empty, single consistent `PointerSource`), not merely a
/// type-shape exercise.
#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct FfiStrokeValidation {
    pub is_valid: bool,
    pub sample_count: u32,
    pub reason: Option<String>,
}

#[uniffi::export]
pub fn validate_stroke(samples: Vec<FfiPointerSample>) -> FfiStrokeValidation {
    let sample_count = samples.len() as u32;
    let domain_samples: Vec<PointerSample> = samples.iter().map(PointerSample::from).collect();
    match Stroke::new(StrokeId::new(), domain_samples) {
        Ok(stroke) => FfiStrokeValidation {
            is_valid: true,
            sample_count: stroke.len() as u32,
            reason: None,
        },
        Err(err) => FfiStrokeValidation {
            is_valid: false,
            sample_count,
            reason: Some(err.to_string()),
        },
    }
}

// ---------------------------------------------------------------------
// Command boundary
// ---------------------------------------------------------------------

/// Mirrors `craftloop_command::CommandNamespace` (Article 237).
#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiCommandNamespace {
    Notebook,
    Sketch,
    Orthographic,
}

impl From<FfiCommandNamespace> for CommandNamespace {
    fn from(namespace: FfiCommandNamespace) -> Self {
        match namespace {
            FfiCommandNamespace::Notebook => CommandNamespace::Notebook,
            FfiCommandNamespace::Sketch => CommandNamespace::Sketch,
            FfiCommandNamespace::Orthographic => CommandNamespace::Orthographic,
        }
    }
}

/// Mirrors `craftloop_command::CommandAction` -- the full flat
/// vocabulary across every namespace (Article 237).
#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiCommandAction {
    Pen,
    Eraser,
    Select,
    Sketch,
    Orthographic,
    Line,
    Circle,
    Arc,
    Rectangle,
    Dimension,
    ExitSketch,
    AddView,
    LabelView,
    Link,
    Resolve,
}

impl From<CommandAction> for FfiCommandAction {
    fn from(action: CommandAction) -> Self {
        match action {
            CommandAction::Pen => FfiCommandAction::Pen,
            CommandAction::Eraser => FfiCommandAction::Eraser,
            CommandAction::Select => FfiCommandAction::Select,
            CommandAction::Sketch => FfiCommandAction::Sketch,
            CommandAction::Orthographic => FfiCommandAction::Orthographic,
            CommandAction::Line => FfiCommandAction::Line,
            CommandAction::Circle => FfiCommandAction::Circle,
            CommandAction::Arc => FfiCommandAction::Arc,
            CommandAction::Rectangle => FfiCommandAction::Rectangle,
            CommandAction::Dimension => FfiCommandAction::Dimension,
            CommandAction::ExitSketch => FfiCommandAction::ExitSketch,
            CommandAction::AddView => FfiCommandAction::AddView,
            CommandAction::LabelView => FfiCommandAction::LabelView,
            CommandAction::Link => FfiCommandAction::Link,
            CommandAction::Resolve => FfiCommandAction::Resolve,
        }
    }
}

/// Mirrors `craftloop_command::grammar::GrammarMatch`. `Ambiguous`'s
/// `Vec<&'static str>` becomes owned `Vec<String>` -- a borrowed
/// `'static` slice cannot cross an FFI boundary at all, exactly the kind
/// of domain-shape detail this crate exists to absorb (see the module
/// doc comment) rather than push onto every mobile caller.
#[derive(uniffi::Enum, Debug, Clone, PartialEq, Eq)]
pub enum FfiGrammarMatch {
    Exact(FfiCommandAction),
    UniquePrefix(FfiCommandAction),
    Ambiguous(Vec<String>),
    NoMatch,
}

impl From<GrammarMatch> for FfiGrammarMatch {
    fn from(result: GrammarMatch) -> Self {
        match result {
            GrammarMatch::Exact(action) => FfiGrammarMatch::Exact(action.into()),
            GrammarMatch::UniquePrefix(action) => FfiGrammarMatch::UniquePrefix(action.into()),
            GrammarMatch::Ambiguous(words) => {
                FfiGrammarMatch::Ambiguous(words.into_iter().map(str::to_string).collect())
            }
            GrammarMatch::NoMatch => FfiGrammarMatch::NoMatch,
        }
    }
}

/// Resolve `input` against `namespace`'s command grammar (Article 237).
/// Never guesses: an ambiguous prefix comes back as `FfiGrammarMatch::Ambiguous`
/// with every candidate word, exactly like the Rust-only `grammar::resolve`
/// this wraps.
#[uniffi::export]
pub fn resolve_command(input: String, namespace: FfiCommandNamespace) -> FfiGrammarMatch {
    grammar::resolve(&input, namespace.into()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(x: f64, y: f64, source: FfiPointerSource) -> FfiPointerSample {
        FfiPointerSample {
            x,
            y,
            timestamp_seconds: 0.0,
            pressure: None,
            tilt_x_deg: None,
            tilt_y_deg: None,
            source,
            button_primary: false,
            button_secondary: false,
            button_barrel: false,
            capability_pressure: false,
            capability_tilt: false,
            capability_hover: false,
            capability_palm_rejection: false,
            capability_eraser: false,
        }
    }

    #[test]
    fn a_consistent_stylus_stroke_validates() {
        let samples = vec![
            sample(0.0, 0.0, FfiPointerSource::Stylus),
            sample(1.0, 1.0, FfiPointerSource::Stylus),
        ];
        let result = validate_stroke(samples);
        assert!(result.is_valid);
        assert_eq!(result.sample_count, 2);
        assert!(result.reason.is_none());
    }

    #[test]
    fn an_empty_stroke_is_rejected_with_a_real_domain_reason() {
        let result = validate_stroke(vec![]);
        assert!(!result.is_valid);
        assert_eq!(result.sample_count, 0);
        assert!(result.reason.is_some());
    }

    #[test]
    fn mixed_pointer_sources_in_one_stroke_are_rejected() {
        let samples = vec![
            sample(0.0, 0.0, FfiPointerSource::Stylus),
            sample(1.0, 1.0, FfiPointerSource::Touch),
        ];
        let result = validate_stroke(samples);
        assert!(
            !result.is_valid,
            "a stroke mixing stylus and touch samples is not a single coherent gesture"
        );
        assert!(result.reason.is_some());
    }

    #[test]
    fn an_exact_grammar_word_resolves_through_the_ffi_boundary() {
        let result = resolve_command("pen".to_string(), FfiCommandNamespace::Notebook);
        assert_eq!(result, FfiGrammarMatch::Exact(FfiCommandAction::Pen));
    }

    #[test]
    fn no_match_crosses_the_boundary_as_its_own_variant_not_a_null() {
        let result = resolve_command("zzz".to_string(), FfiCommandNamespace::Notebook);
        assert_eq!(result, FfiGrammarMatch::NoMatch);
    }

    #[test]
    fn an_ambiguous_prefix_carries_every_candidate_word_as_owned_strings() {
        // "s" is ambiguous in the Notebook namespace: both "select" and
        // "sketch" start with it (Article 237's own vocabulary, Phase 19
        // grammar.rs already tests this at the Rust-only level).
        let result = resolve_command("s".to_string(), FfiCommandNamespace::Notebook);
        match result {
            FfiGrammarMatch::Ambiguous(candidates) => {
                assert!(candidates.len() >= 2);
            }
            other => panic!("expected Ambiguous, got {other:?}"),
        }
    }
}
