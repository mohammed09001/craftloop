//! End-to-End Engine Integration scenarios.
//!
//! Execution 01, Phase 30. No production code lives here -- every task
//! (213-221) is a `tests/*.rs` integration test that wires together the
//! real domain crates the way a real editing session would, using each
//! crate's own real public API (never a mock or a duplicated
//! reimplementation of domain logic). See each test file's own doc
//! comment for which task it satisfies and which engine boundaries it
//! crosses.
