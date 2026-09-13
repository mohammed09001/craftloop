//! Normalized pointer input, capability descriptors, stroke lifecycle
//! events, and deterministic playback for the Craft Loop shared engineering
//! core.
//!
//! Execution 01, Phase 03. Authority: Engine Contract 01 (Normalized
//! Input); MCP Article 582 "Requirement Group: Pen Input".
//!
//! No platform UI types cross into this crate (Engine Contract 01's
//! authority boundary): `apps/windows-harness` (Phase 04) will depend on
//! this crate, never the reverse.

pub mod capabilities;
pub mod disclaimer;
pub mod lifecycle;
pub mod mouse_simulator;
pub mod sample;
pub mod trace;

pub use capabilities::InputCapabilities;
pub use disclaimer::{SIMULATOR_DISCLAIMER, SIMULATOR_LIMITATIONS};
pub use lifecycle::{validate_sequence, PointerEvent, StrokeLifecycleValidator};
pub use mouse_simulator::MouseSimulator;
pub use sample::{PointerButtons, PointerSample, PointerSource};
pub use trace::PointerTrace;
