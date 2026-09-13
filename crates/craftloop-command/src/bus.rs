//! The shared Command Bus.
//!
//! Execution 01, Phase 19, Task 142. Authority: MCP Article 239 ("The
//! Command Bus is the unified application action layer... validates
//! whether the command is allowed... creates a user-visible transaction...
//! returns success or structured failure.").
//!
//! Every source channel (toolbar, Ink Command Engine, keyboard,
//! accessibility, future voice/gesture) is required to go through
//! [`CommandBus::submit`] -- there is no second entry point. This is the
//! literal meaning of Task 142's "route every command through the
//! Command Bus": the only way a `Command` (Task 133) becomes part of this
//! engine's recorded history is this one function, regardless of which
//! `CommandSource` produced it.
//!
//! What this phase's bus does *not* do: actually invoke the ~15 other
//! engines this execution has built (the constraint solver, the
//! consistency engine, dimension association...) to perform each
//! command's real domain effect. Article 236 itself draws that boundary
//! ("The engine should not invoke business logic directly... Dispatch
//! through the shared Command Bus") but Task 142's own objective is
//! narrower still: "Toolbar, keyboard, future gesture, and ink command
//! must share one semantic operation" -- proving every source produces
//! the *same* validated, recorded `Command` object is what this task
//! asks for. Wiring each `CommandAction` to the specific engine call it
//! ultimately triggers is real integration work with no task naming it
//! yet, and would require this crate to depend on every domain crate
//! built so far -- exactly the kind of speculative, premature coupling
//! this workspace's conventions forbid.

use craftloop_errors::{CommandErrorKind, DomainError, DomainResult};
use craftloop_ids::CommandId;

use crate::command::Command;
use crate::confirmation::ConfirmationOutcome;
use crate::grammar::is_valid_in_namespace;
use crate::risk::{may_execute, requires_confirmation};

/// A rejected command's outcome, alongside `Err` -- see `submit`'s doc
/// comment for why validation happens before anything is recorded.
#[derive(Debug)]
pub struct CommandBus {
    history: Vec<Command>,
}

impl Default for CommandBus {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandBus {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    /// Validate and record `command`. `confirmation`, when the command's
    /// risk level requires one (`risk::requires_confirmation`), is the
    /// already-evaluated [`ConfirmationOutcome`] (Task 138) -- this
    /// function does not itself gather or judge confirmation evidence,
    /// only enforces the policy once evidence exists.
    ///
    /// Rejects, with nothing recorded, if the action is not valid in the
    /// command's own declared namespace, or if the risk/confirmation
    /// policy is not satisfied. Every command that *is* accepted becomes
    /// part of `history()` in submission order -- the durable,
    /// user-visible transaction record Article 239 asks for.
    pub fn submit(
        &mut self,
        command: Command,
        confirmation: Option<ConfirmationOutcome>,
    ) -> DomainResult<CommandId> {
        if !is_valid_in_namespace(command.action, command.namespace) {
            return Err(DomainError::Command {
                kind: CommandErrorKind::InvalidForNamespace,
                detail: format!(
                    "{:?} is not valid in the {:?} namespace",
                    command.action, command.namespace
                ),
            });
        }

        if requires_confirmation(command.risk) {
            let outcome = confirmation.unwrap_or(ConfirmationOutcome::RemainsInk);
            if !may_execute(command.risk, outcome) {
                return Err(DomainError::Command {
                    kind: CommandErrorKind::NotConfirmed,
                    detail: format!(
                        "{:?}-risk command {:?} was not confirmed strongly enough (got {outcome:?})",
                        command.risk, command.action
                    ),
                });
            }
        }

        let id = command.id;
        self.history.push(command);
        Ok(id)
    }

    /// Every accepted command, in submission order -- the shared,
    /// source-agnostic history Task 142 exists to guarantee.
    pub fn history(&self) -> &[Command] {
        &self.history
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::{CommandAction, CommandNamespace, CommandSource, RiskLevel, UndoMetadata};
    use craftloop_ids::CraftLoopId;
    use std::collections::BTreeMap;

    fn command(
        action: CommandAction,
        namespace: CommandNamespace,
        source: CommandSource,
        risk: RiskLevel,
    ) -> Command {
        Command {
            id: CommandId::new(),
            action,
            source,
            namespace,
            parameters: BTreeMap::new(),
            risk,
            timestamp_seconds: 0.0,
            undo: UndoMetadata::undoable("test command"),
        }
    }

    #[test]
    fn a_valid_low_risk_command_is_accepted_without_confirmation() {
        let mut bus = CommandBus::new();
        let cmd = command(
            CommandAction::Pen,
            CommandNamespace::Notebook,
            CommandSource::Toolbar,
            RiskLevel::Low,
        );
        assert!(bus.submit(cmd, None).is_ok());
        assert_eq!(bus.history().len(), 1);
    }

    #[test]
    fn an_action_not_valid_in_its_declared_namespace_is_rejected_and_not_recorded() {
        let mut bus = CommandBus::new();
        let cmd = command(
            CommandAction::Circle,
            CommandNamespace::Notebook,
            CommandSource::InkCommand,
            RiskLevel::Low,
        );
        let result = bus.submit(cmd, None);
        assert!(matches!(
            result,
            Err(DomainError::Command {
                kind: CommandErrorKind::InvalidForNamespace,
                ..
            })
        ));
        assert!(bus.history().is_empty());
    }

    #[test]
    fn a_high_risk_command_without_confirmation_is_rejected_and_not_recorded() {
        let mut bus = CommandBus::new();
        let cmd = command(
            CommandAction::Eraser,
            CommandNamespace::Notebook,
            CommandSource::InkCommand,
            RiskLevel::High,
        );
        let result = bus.submit(cmd, None);
        assert!(matches!(
            result,
            Err(DomainError::Command {
                kind: CommandErrorKind::NotConfirmed,
                ..
            })
        ));
        assert!(bus.history().is_empty());
    }

    #[test]
    fn a_high_risk_command_with_a_full_confirmation_is_accepted() {
        let mut bus = CommandBus::new();
        let cmd = command(
            CommandAction::Eraser,
            CommandNamespace::Notebook,
            CommandSource::InkCommand,
            RiskLevel::High,
        );
        let result = bus.submit(cmd, Some(ConfirmationOutcome::Execute));
        assert!(result.is_ok());
        assert_eq!(bus.history().len(), 1);
    }

    #[test]
    fn every_source_channel_shares_the_same_history_task_142() {
        let mut bus = CommandBus::new();
        for source in [
            CommandSource::Toolbar,
            CommandSource::InkCommand,
            CommandSource::Keyboard,
            CommandSource::Accessibility,
            CommandSource::Voice,
            CommandSource::Gesture,
        ] {
            let cmd = command(
                CommandAction::Select,
                CommandNamespace::Notebook,
                source,
                RiskLevel::Low,
            );
            bus.submit(cmd, None).unwrap();
        }
        assert_eq!(bus.history().len(), 6);
        // All six distinct sources are present in one shared record.
        let sources: std::collections::BTreeSet<_> = bus
            .history()
            .iter()
            .map(|c| format!("{:?}", c.source))
            .collect();
        assert_eq!(sources.len(), 6);
    }

    #[test]
    fn history_preserves_submission_order() {
        let mut bus = CommandBus::new();
        bus.submit(
            command(
                CommandAction::Pen,
                CommandNamespace::Notebook,
                CommandSource::Toolbar,
                RiskLevel::Low,
            ),
            None,
        )
        .unwrap();
        bus.submit(
            command(
                CommandAction::Select,
                CommandNamespace::Notebook,
                CommandSource::Keyboard,
                RiskLevel::Low,
            ),
            None,
        )
        .unwrap();
        assert_eq!(bus.history()[0].action, CommandAction::Pen);
        assert_eq!(bus.history()[1].action, CommandAction::Select);
    }
}
