//! Simulator limitation disclaimer text.
//!
//! Execution 01, Phase 03, Task 026. Authority: No-Hallucination Contract
//! ("must never claim real Apple Pencil behavior was tested on Windows");
//! MCP Article 6 "Pen-Native, Not Merely Pen-Compatible".
//!
//! This crate has no UI of its own -- the Windows harness UI that actually
//! renders a disclaimer banner/log line is Phase 04's job
//! (`apps/windows-harness`). What belongs here, now, is the single
//! authoritative source of *what the disclaimer says*, so Phase 04 (and any
//! later diagnostic export, per Engine Contract 26) quotes one consistent,
//! tested string instead of every call site inventing its own wording that
//! could drift out of honesty over time.

/// One-line disclaimer suitable for a persistent UI banner or a startup log
/// line. Deliberately names the specific hardware capabilities this
/// simulation cannot validate, rather than a vague "for testing only".
pub const SIMULATOR_DISCLAIMER: &str = "Windows mouse-as-pen simulation: does NOT validate real Apple Pencil or Android stylus pressure, tilt, hover, or palm rejection.";

/// The specific capability-by-capability limitations, for a detail
/// panel/log rather than a one-line banner.
pub const SIMULATOR_LIMITATIONS: &[&str] = &[
    "Position: real mouse coordinates.",
    "Pressure: constant placeholder value, not hardware-sensed.",
    "Tilt: unavailable.",
    "Hover: unavailable.",
    "Palm rejection: unavailable.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disclaimer_explicitly_names_apple_pencil_and_stylus_so_it_cannot_be_mistaken_for_them() {
        let lower = SIMULATOR_DISCLAIMER.to_lowercase();
        assert!(lower.contains("simulat"));
        assert!(lower.contains("pencil"));
        assert!(lower.contains("stylus"));
        assert!(lower.contains("not"));
    }

    #[test]
    fn limitations_list_is_nonempty_and_mentions_pressure_and_tilt() {
        assert!(!SIMULATOR_LIMITATIONS.is_empty());
        let joined = SIMULATOR_LIMITATIONS.join(" ").to_lowercase();
        assert!(joined.contains("pressure"));
        assert!(joined.contains("tilt"));
    }
}
