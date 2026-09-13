//! Ink Command Language and the shared Command Bus for the Craft Loop
//! shared engineering core.
//!
//! Execution 01, Phase 19, Tasks 133-142. Authority: MCP Article 46
//! ("Ink Command Language"), Articles 236-239.

pub mod bus;
pub mod command;
pub mod confirmation;
pub mod ephemeral;
pub mod grammar;
pub mod risk;

pub use bus::CommandBus;
pub use command::{
    Command, CommandAction, CommandNamespace, CommandSource, RiskLevel, UndoMetadata,
};
pub use confirmation::{
    evaluate_confirmation, is_recent_enough_to_confirm, ConfirmationEvidence, ConfirmationOutcome,
    RECENT_COMMAND_WINDOW_SECONDS,
};
pub use ephemeral::{dispose_of_confirmed_command_ink, EphemeralInkDisposition};
pub use grammar::{is_valid_in_namespace, resolve, GrammarMatch};
pub use risk::{may_execute, requires_confirmation};
