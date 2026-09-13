//! Command risk levels and confirmation policy.
//!
//! Execution 01, Phase 19, Task 141. Authority: MCP Article 236/238
//! ("Low-risk tool changes can execute directly; destructive commands
//! require stronger confirmation.").

use crate::command::RiskLevel;
use crate::confirmation::ConfirmationOutcome;

/// Does `risk` require an explicit confirmation gesture before
/// executing, or can it run directly once merely recognized? Matches
/// Task 141's exact two-tier framing: only `Low` risk skips
/// confirmation.
pub fn requires_confirmation(risk: RiskLevel) -> bool {
    risk != RiskLevel::Low
}

/// Task 138/141 combined: given a command's risk and the confirmation
/// evidence outcome already computed (`confirmation.rs`), may this
/// specific command actually execute right now?
///
/// - `Low` risk executes on recognition alone (`ConfirmationOutcome`
///   from a lower-risk gesture reading is not required) -- but if a
///   confirmation *was* evaluated and came back `RemainsInk`, that is
///   evidence the ink was never a command at all, and must still block
///   execution.
/// - `Medium`/`High` risk requires at least `Preview`, and `High`
///   (destructive) requires the strongest signal, `Execute`, exactly
///   like `Medium` -- Article 238 defines only two positive outcomes
///   (`Execute`, `Preview`), so "stronger confirmation" for `High` risk
///   is enforced by this policy layer refusing to auto-execute a
///   `Preview`-only result for it, not by inventing a fourth
///   `ConfirmationOutcome` variant Article 238 never names.
pub fn may_execute(risk: RiskLevel, confirmation: ConfirmationOutcome) -> bool {
    match risk {
        RiskLevel::Low => confirmation != ConfirmationOutcome::RemainsInk,
        RiskLevel::Medium => {
            confirmation == ConfirmationOutcome::Execute
                || confirmation == ConfirmationOutcome::Preview
        }
        RiskLevel::High => confirmation == ConfirmationOutcome::Execute,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_low_risk_skips_the_confirmation_requirement() {
        assert!(!requires_confirmation(RiskLevel::Low));
        assert!(requires_confirmation(RiskLevel::Medium));
        assert!(requires_confirmation(RiskLevel::High));
    }

    #[test]
    fn low_risk_executes_on_a_preview_level_result_too() {
        assert!(may_execute(RiskLevel::Low, ConfirmationOutcome::Preview));
        assert!(may_execute(RiskLevel::Low, ConfirmationOutcome::Execute));
    }

    #[test]
    fn low_risk_still_never_executes_ink_that_was_never_a_command() {
        assert!(!may_execute(
            RiskLevel::Low,
            ConfirmationOutcome::RemainsInk
        ));
    }

    #[test]
    fn medium_risk_accepts_a_preview_level_confirmation() {
        assert!(may_execute(RiskLevel::Medium, ConfirmationOutcome::Preview));
    }

    #[test]
    fn high_risk_destructive_commands_require_full_execute_confidence() {
        assert!(!may_execute(RiskLevel::High, ConfirmationOutcome::Preview));
        assert!(may_execute(RiskLevel::High, ConfirmationOutcome::Execute));
    }

    #[test]
    fn no_risk_level_ever_executes_from_remains_ink() {
        for risk in [RiskLevel::Low, RiskLevel::Medium, RiskLevel::High] {
            assert!(!may_execute(risk, ConfirmationOutcome::RemainsInk));
        }
    }
}
