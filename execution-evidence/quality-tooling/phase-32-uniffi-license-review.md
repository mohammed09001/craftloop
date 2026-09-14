# UniFFI License Review

Execution 01, Phase 32, Task 231 (independent review's finding). Recorded
2026-09-14. Authority: this task's own objective ("review... dependency
licenses"), same treatment `solver-evaluations/solver-decision-record.md`
gave `ezpz`'s license in Phase 11.

## Why this record exists

The independent, fresh-context code review run this phase (Task 231)
found a real, specific gap: `uniffi` (pinned `=0.32.1`,
`crates/craftloop-mobile-ffi/Cargo.toml`, adopted Phase 28) is the one
dependency in this workspace that is not MIT/Apache-2.0-permissive, yet
unlike `ezpz` -- which got a full license review as part of its own
decision record -- no equivalent review existed anywhere in
`execution-evidence/` for `uniffi`. This record closes that gap.

## License

Confirmed directly via `cargo metadata --format-version 1`, not taken on
faith from documentation:

```
uniffi                  0.32.1  |  MPL-2.0
uniffi_bindgen          0.32.1  |  MPL-2.0
uniffi_core             0.32.1  |  MPL-2.0
uniffi_internal_macros  0.32.1  |  MPL-2.0
uniffi_macros           0.32.1  |  MPL-2.0
uniffi_meta             0.32.1  |  MPL-2.0
uniffi_pipeline         0.32.1  |  MPL-2.0
uniffi_udl              0.32.1  |  MPL-2.0
```

Every `uniffi_*` crate in the dependency tree is **MPL-2.0** (Mozilla
Public License 2.0). Its own direct dependency chain (`weedle2` 5.0.0:
MIT; `askama`/`askama_derive`/`askama_macros`/`askama_parser` 0.16.1:
`MIT OR Apache-2.0`) introduces nothing further non-permissive.

## What MPL-2.0 actually requires

MPL-2.0 is a **weak, file-level copyleft** license -- meaningfully
different from GPL/AGPL/LGPL's project-wide or dynamic-linking-triggered
copyleft, and different from the "no non-permissive dependency at all"
bar `ezpz`'s own decision record happened to clear:

- Copyleft applies only to the **licensed file itself** ("Covered
  Software"). Modifying and redistributing a file from `uniffi`'s own
  source would require releasing *that file's* modifications under
  MPL-2.0 (or a compatible license) -- it does not extend to
  `craftloop-mobile-ffi`'s own source files, which are separate files
  this project wrote from scratch and owns outright.
- **Unmodified use as a library dependency does not trigger any
  copyleft obligation at all.** This project does not fork, patch, or
  vendor `uniffi`'s source -- it depends on it as an ordinary
  `crates.io` library the same way it depends on `serde` or `thiserror`.
  MPL-2.0 explicitly permits this (Section 3.3, "Distribution of
  Executable Form") without imposing source-disclosure obligations on
  the combined/larger work, unlike GPL's "derivative work" framing.
- MPL-2.0 is compatible with combining into a proprietary/`UNLICENSED`
  project (this workspace's own `license.workspace = true` resolves to
  `UNLICENSED` per root `Cargo.toml`) -- the license itself states this
  explicitly (Section 3.3) as a design goal distinguishing it from
  GPL-family licenses.

## Decision

**No action required; the dependency is safe to keep as-is.** MPL-2.0's
file-level, non-viral copyleft does not propagate any obligation onto
this project's own source given ordinary (unmodified) library usage,
and it is explicitly designed to be combinable with a proprietary
codebase. This matches Phase 28's own original adoption rationale (real,
working UniFFI Kotlin/Swift bindings proven end to end in Phases 28-29)
without needing any license-driven reconsideration.

## What would change this

If a future phase ever needed to **fork or directly modify** `uniffi`'s
own source (rather than depending on it as published), the modified
files specifically would need to stay MPL-2.0-licensed (or be
republished under a compatible license) when distributed -- a real but
narrow obligation, not triggered by this project's current, ordinary
dependency usage.
