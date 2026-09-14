# Unsafe Code and FFI Audit

Execution 01, Phase 31, Task 228. Recorded 2026-09-14. Authority: MCP
Article 4 (no fabricated certainty); this execution's own "shared core
stays platform-independent" boundary (Task 007, Phase 01), now extended
through the real FFI boundary Phase 28/29 built.

## Finding

**Zero `unsafe` blocks exist anywhere in this workspace's own code.**
Verified directly, not assumed:

```
grep -rn "unsafe" crates/**/*.rs apps/**/*.rs android/**/*.kt
# (also checked with the Grep tool across crates/, apps/, android/)
-> No matches found.
```

This covers every domain crate, `apps/windows-harness`, and
`crates/craftloop-mobile-ffi` -- the one crate this audit specifically
targeted ("before mobile bindings," per the task's own wording), since
it is the crate closest to a real FFI boundary. Its `#[uniffi::export]`/
`uniffi::setup_scaffolding!()` proc macros generate the actual
unsafe C-ABI glue UniFFI needs (pointer/repr(C) handling, etc.), but
that generated code lives inside the `uniffi`/`uniffi_core` library
crates themselves (external dependencies, not part of this workspace) --
confirmed by the fact this crate compiles cleanly under the
`unsafe_code = "forbid"` lint added below, which would fail to compile
if any `unsafe` token appeared in `craftloop-mobile-ffi`'s own expanded
source.

## Enforcement, not just observation

Per the task's explicit "minimize unsafe Rust" framing, this finding is
now self-enforced rather than a one-time grep result that could silently
regress: root `Cargo.toml` gained

```toml
[workspace.lints.rust]
unsafe_code = "forbid"
```

alongside the existing `warnings = "deny"` policy (Phase 27, Task 192),
inherited by every crate that opts in via `[lints]\nworkspace = true`
(all 22 workspace members). Verified by a full `cargo build --workspace
--all-targets` and `cargo clippy --workspace --all-targets -- -D
warnings` after adding the lint: both succeed, confirming the lint is
real (would fail the build on any unsafe code) and that no crate --
including `craftloop-mobile-ffi` -- currently needs an override.

## What "document every necessary unsafe boundary" means here, today

There is no boundary to document yet, honestly: no unsafe code exists.
The lint's own comment (in `Cargo.toml`) states the forward-looking
plan explicitly, so a future contributor does not have to rediscover
it: if a crate ever needs a real, reviewed unsafe boundary (most
plausibly `craftloop-mobile-ffi`, if a future phase adds real
cross-compiled Android/iOS cdylib-loading glue that cannot be expressed
safely), that crate overrides the workspace policy with a crate-level
`#![allow(unsafe_code)]` carrying its own doc comment justifying exactly
why -- documented at the boundary itself when it is first introduced,
not preemptively invented here where it would be speculative.

## Scope note

This audit covers this workspace's own source only. It does not audit
`unsafe` code inside external dependencies (`uniffi`, `ezpz`, `eframe`/
`egui`/`winit`, etc.) -- auditing a third-party dependency's own internal
safety is a different, much larger undertaking (crate-by-crate security
review) that no task in this phase asks for, and this record does not
claim to have done it.
