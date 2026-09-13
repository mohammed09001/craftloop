//! Command confirmation gesture contract and temporal disambiguation.
//!
//! Execution 01, Phase 19, Tasks 138-139. Authority: MCP Article 238
//! ("The detection system should evaluate: Whether the enclosed content
//! is recent. Whether the enclosed content is recognized as a valid
//! command. Whether the circle encloses most of the command bounds.
//! Whether the stroke itself looks like a deliberate enclosure. Whether
//! the current context would interpret the same circle as geometry.
//! Whether the user has disabled circle-to-command behavior.").
//!
//! Every field on [`ConfirmationEvidence`] is exactly one of Article
//! 238's six named checks -- no speculative seventh signal.

use serde::{Deserialize, Serialize};

/// Task 139: how recent is "recent"? Article 238 names the question
/// ("is the enclosed content recent") without a number; this constant is
/// this engine's own documented, uncalibrated answer -- named honestly as
/// a placeholder pending real usability testing, matching this
/// workspace's established pattern for such constants (e.g.
/// `craftloop-geometry::Tolerances::recognition`).
pub const RECENT_COMMAND_WINDOW_SECONDS: f64 = 3.0;

/// Task 139: is `command_timestamp` recent enough, relative to
/// `gesture_timestamp`, to plausibly be what the enclosing gesture is
/// confirming -- as opposed to the gesture lassoing older content that
/// happens to sit nearby? A negative interval (the "command" is somehow
/// after the gesture) is never recent.
pub fn is_recent_enough_to_confirm(
    command_timestamp_seconds: f64,
    gesture_timestamp_seconds: f64,
) -> bool {
    let elapsed = gesture_timestamp_seconds - command_timestamp_seconds;
    (0.0..=RECENT_COMMAND_WINDOW_SECONDS).contains(&elapsed)
}

/// Article 238's six evaluated signals, gathered by the caller (ink
/// recognition, stroke geometry, context/settings) before calling
/// [`evaluate_confirmation`]. This type does not compute any of them
/// itself -- deliberately: "is this stroke recent," "does the enclosure
/// ratio clear a bar," and "would this context treat a circle as
/// geometry" are each already-real questions other engines in this
/// workspace answer (Phase 06 recognition, this module's own
/// `is_recent_enough_to_confirm`, and Phase 16's `IntentCategory`
/// respectively) -- this module's job is only to combine the six
/// answers into one outcome, not to re-derive them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfirmationEvidence {
    pub content_is_recent: bool,
    pub content_is_a_valid_command: bool,
    /// Fraction (0.0-1.0) of the command's own bounds the enclosing
    /// gesture actually covers.
    pub enclosure_ratio: f64,
    pub enclosure_looks_deliberate: bool,
    /// Would the *current context* (namespace/tool) treat this same
    /// stroke as ordinary geometry instead of a confirmation gesture?
    pub context_would_treat_as_geometry: bool,
    pub circle_to_command_disabled_by_user: bool,
}

/// Article 238's three-tier outcome ("If confidence is high... medium...
/// low").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfirmationOutcome {
    /// A low-risk command can execute.
    Execute,
    /// A small command preview can appear -- not yet executed.
    Preview,
    /// The circle remains ink, or selection behavior applies.
    RemainsInk,
}

const HIGH_ENCLOSURE_RATIO: f64 = 0.7;
const MEDIUM_ENCLOSURE_RATIO: f64 = 0.4;

/// Task 138: combine Article 238's six signals into one outcome. Any
/// disqualifying signal (not recent, not a valid command, the user
/// disabled this behavior, or the context would read this as geometry)
/// forces `RemainsInk` outright -- these are treated as hard
/// disqualifiers, not merely negative evidence to be outweighed by a
/// good enclosure ratio, matching Article 238's own framing of them as
/// binary checks rather than scored ones.
pub fn evaluate_confirmation(evidence: ConfirmationEvidence) -> ConfirmationOutcome {
    if evidence.circle_to_command_disabled_by_user
        || evidence.context_would_treat_as_geometry
        || !evidence.content_is_recent
        || !evidence.content_is_a_valid_command
    {
        return ConfirmationOutcome::RemainsInk;
    }

    if evidence.enclosure_looks_deliberate && evidence.enclosure_ratio >= HIGH_ENCLOSURE_RATIO {
        ConfirmationOutcome::Execute
    } else if evidence.enclosure_ratio >= MEDIUM_ENCLOSURE_RATIO {
        ConfirmationOutcome::Preview
    } else {
        ConfirmationOutcome::RemainsInk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_evidence() -> ConfirmationEvidence {
        ConfirmationEvidence {
            content_is_recent: true,
            content_is_a_valid_command: true,
            enclosure_ratio: 0.9,
            enclosure_looks_deliberate: true,
            context_would_treat_as_geometry: false,
            circle_to_command_disabled_by_user: false,
        }
    }

    #[test]
    fn high_confidence_evidence_executes() {
        assert_eq!(
            evaluate_confirmation(full_evidence()),
            ConfirmationOutcome::Execute
        );
    }

    #[test]
    fn a_partial_but_deliberate_enclosure_only_previews() {
        let mut evidence = full_evidence();
        evidence.enclosure_ratio = 0.5;
        assert_eq!(
            evaluate_confirmation(evidence),
            ConfirmationOutcome::Preview
        );
    }

    #[test]
    fn a_barely_touching_enclosure_stays_ink() {
        let mut evidence = full_evidence();
        evidence.enclosure_ratio = 0.1;
        assert_eq!(
            evaluate_confirmation(evidence),
            ConfirmationOutcome::RemainsInk
        );
    }

    #[test]
    fn stale_content_never_executes_even_with_a_perfect_enclosure() {
        let mut evidence = full_evidence();
        evidence.content_is_recent = false;
        assert_eq!(
            evaluate_confirmation(evidence),
            ConfirmationOutcome::RemainsInk
        );
    }

    #[test]
    fn content_that_is_not_a_valid_command_never_executes() {
        let mut evidence = full_evidence();
        evidence.content_is_a_valid_command = false;
        assert_eq!(
            evaluate_confirmation(evidence),
            ConfirmationOutcome::RemainsInk
        );
    }

    #[test]
    fn a_context_that_would_read_this_as_geometry_takes_priority_over_a_perfect_enclosure() {
        // This is exactly Article 238's own worked scenario: a context
        // where the same circle is ordinary geometry must never be
        // hijacked into a command just because the enclosure looks
        // deliberate.
        let mut evidence = full_evidence();
        evidence.context_would_treat_as_geometry = true;
        assert_eq!(
            evaluate_confirmation(evidence),
            ConfirmationOutcome::RemainsInk
        );
    }

    #[test]
    fn a_user_disabled_setting_is_respected_regardless_of_evidence_quality() {
        let mut evidence = full_evidence();
        evidence.circle_to_command_disabled_by_user = true;
        assert_eq!(
            evaluate_confirmation(evidence),
            ConfirmationOutcome::RemainsInk
        );
    }

    // --- Task 139: temporal disambiguation ---------------------------------

    #[test]
    fn a_gesture_immediately_after_the_command_is_recent() {
        assert!(is_recent_enough_to_confirm(10.0, 10.5));
    }

    #[test]
    fn a_gesture_long_after_the_command_is_not_recent() {
        assert!(!is_recent_enough_to_confirm(10.0, 20.0));
    }

    #[test]
    fn a_gesture_around_much_older_content_is_never_treated_as_confirmation() {
        // Separates "confirming a recent command" from "lassoing older
        // content" -- Task 139's exact objective.
        assert!(!is_recent_enough_to_confirm(0.0, 100.0));
    }

    #[test]
    fn a_gesture_before_the_command_is_never_recent() {
        assert!(!is_recent_enough_to_confirm(10.0, 5.0));
    }
}
