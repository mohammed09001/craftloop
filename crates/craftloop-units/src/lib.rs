//! Canonical units and engineering numeric parsing for the Craft Loop
//! shared engineering core.
//!
//! Execution 01, Phase 09. Authority: Engine Contract 07 (Engineering
//! Parser).
//!
//! No platform UI dependency; depends only on `craftloop-errors`. Every
//! parse failure in this crate returns `craftloop_errors::DomainError::Parser`,
//! which always carries the original raw `input` string alongside a typed
//! `kind` (Task 069) -- see `tests/invalid_numeric_diagnostics.rs` for the
//! cross-cutting proof of that property.

pub mod angle;
pub mod length;
pub mod length_unit;
pub mod numeric;
pub mod radius_diameter;

pub use angle::parse_angle_degrees;
pub use length::{parse_length, ParsedLength};
pub use length_unit::LengthUnit;
pub use numeric::{parse_decimal, parse_decimal_auto, DecimalLocale};
pub use radius_diameter::{parse_radial, RadialCandidate, RadialKind};
