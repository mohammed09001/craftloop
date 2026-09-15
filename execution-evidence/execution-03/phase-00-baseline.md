# Execution 03, Phase 00 — Re-Audit and Execution Lock

Recorded 2026-09-15. Tasks 001-005.

## Task 001 — Pin current repository state

Repository evidence, inspected directly at commit
`93e5bdb2c30919e9a73832b6759c3ed06553ab74` ("Execution 02, Phase 10:
dimensions/constraints slice, fix a real selection crash"), branch
`main`, tracking `origin/main`, up to date.

**Working-tree state before this phase:** `Craft Loop Execution 01.md`
and `Craft Loop Execution 02 Androi Tablet Engineering Alpha.md` had
been moved (outside this session) into a new `execution/` directory,
and `execution/Craft Loop Execution 03 Web Live View Creative Sketch
Mode.md` had been added there, untracked. No other changes.

**Android toolbar placement confirmed as the doc describes:**
`android/app/src/main/java/com/craftloop/shell/MainActivity.kt` line
244 lays out `Column { InkCanvas(...); ...; PrimaryToolbar(viewModel) }`
— the toolbar renders below the canvas in a vertical column, not
top-center. This is the Article 1 / Article 17 starting point Phase 07
must change (web-side) without touching the native Android layout in
this execution (Article 52's native parity patch is a separate, later
task).

**Current `CraftLoopSession` API surface** (source of truth for the
web bridge to mirror), from `crates/craftloop-mobile-ffi/src/session.rs`:
`new`, `open`, `save`, `scene_snapshot`, `debug_state`, `submit_stroke`,
`accept_recognition`, `create_primitive_line`, `create_primitive_circle`,
`create_primitive_rectangle`, `select`, `clear_selection`,
`delete_selected`, `create_dimension`, `edit_dimension`,
`apply_constraint`, `remove_constraint`, `solve_constraints`,
`assign_view_identity`, `add_geometry_to_view`, `enter_orthographic`,
`propagate_shared_value`, `resolve_conflict`, `undo`, `redo`,
`can_undo`, `can_redo`. Supporting read-model types: `FfiSceneSnapshot`,
`FfiDebugState`, `FfiStrokeOutcome`, `FfiSolveOutcome`,
`FfiPrimitiveSummary`, `FfiDimensionSummary`, `FfiConstraintSummary` (via
`solve_constraints`), `FfiConflictSummary`, `FfiViewBlockSummary`,
`FfiOrthographicSetSummary`. This crate binds Kotlin/Swift through
UniFFI (`uniffi = "=0.32.1"`); UniFFI does not target
`wasm32-unknown-unknown`, so Phase 03's `craftloop-web-bridge` crate
must wrap the same underlying domain crates directly with
`wasm-bindgen` rather than reusing `craftloop-mobile-ffi` as-is. This
is exactly Article 10/11's premise, confirmed against real code rather
than assumed.

**CI:** `.github/workflows/` still runs Rust-only fmt/clippy/test (plus
a macOS compile gate). No web/Wasm CI job exists yet (Phase 18 target).

**Web/Wasm toolchain on this host:** `node v22.20.0`, `npm 10.9.3`
present. `rustup target list --installed` shows
`aarch64-linux-android` and `x86_64-pc-windows-msvc` only —
`wasm32-unknown-unknown` is **not yet installed** (added in Phase 02/03
when the bridge crate is created). No `wasm-pack` on `PATH`; `cargo-ndk`
is present from Execution 02's Android work. This host now has an
Android SDK on `PATH` (`platform-tools`, `cmdline-tools`) that Execution
02's baseline did not have, but per this execution's own metadata no
tablet hardware validation is required, so this is recorded and not
pursued further here.

## Task 002 — Run Rust baseline

Run from repository root:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets
cargo test --workspace
```

Results: `fmt` clean (exit 0). `clippy` clean, zero warnings (workspace
`[lints] warnings = "deny"` policy self-enforces this), build finished
in 18.26s. `test`: all suites green across every crate — unit,
integration/scenario, and doctests, 0 failures (see full log; per-crate
counts include 67, 59, 52, 44, 36×2, 32, 29, 27, 26×2, 17, 15, 14, 9×2,
8, 7×3, 6, 5×2, 4×4, 3×8, 2×6, 1×6 passed, 0 failed throughout). This is
the Execution 03 starting baseline — matches Execution 02's closing
state; no regression since `93e5bdb`.

## Task 003 — Map unfinished Execution 02 work

Ledger from Article 4, verified against current source rather than
assumed:

| Old phase | Status entering Execution 03 | Disposition here |
|---|---|---|
| Phase 11 — View Identity / Orthographic | `assign_view_identity`, `add_geometry_to_view`, `enter_orthographic`, `propagate_shared_value`, `resolve_conflict` exist on `CraftLoopSession` (Rust), but no Android UI renders linked Top/Right View Blocks yet | Completed here visually/semantically in the web harness: Phase 13 |
| Phase 12 — Persistence / lifecycle | `open`/`save` exist on `CraftLoopSession` taking a `path: String`; no browser-side adapter (no filesystem) | Completed here as path-independent serialization + browser (IndexedDB) persistence: Phase 14 |
| Phase 13 — physical S Pen diagnostics | Not started | Deferred; no tablet hardware in this execution |
| Phase 14 — handwriting-to-dimension | `craftloop-handwriting` crate exists as a platform-neutral boundary; no native ML Kit integration | Split here: platform-neutral recognition boundary + browser command/recognition simulation (Phase 15); native ML Kit validation stays deferred |
| Phase 15 — Ink Commands | `resolve_command`/`FfiGrammarMatch` exist in `craftloop-mobile-ffi/src/lib.rs`; no browser Command Bus or simulator | Completed here at Command Bus + interaction-state level: Phases 08, 15 |
| Phase 16 — Android CI | No Android CI job in `.github/workflows/` | Completed here without a tablet: Phase 18 |
| Phase 18 — physical Engineering Alpha acceptance | Not applicable without hardware | Replaced by Web Live Acceptance + shared-core regression acceptance: Phase 20 |
| Phase 19 — final audit | Not started | Completed here as universal-architecture + no-hallucination audit: Phase 20 |

## Task 004 — Create evidence area

Created `execution-evidence/execution-03/` for baseline, research,
architecture, browser, preview, CI, and final-report evidence. This
file is the first entry.

## Task 005 — Execution branch

**Deviation from the literal doc text, recorded per Article 3 ("current
repository evidence" overriding an example instruction) and Article 1
("continues the real repository"):** `git log --all` shows Execution 01
and Execution 02 both committed directly to `main` with no feature
branch ever created or merged (`git branch -a` shows only `main` and
`origin/main` today). Execution 03 follows the same established
convention — phase work lands as direct commits on `main` with
descriptive `Execution 03, Phase NN: ...` messages — rather than
introducing a branching model this repository's own history has never
used. If the user wants an isolated branch instead, that is a one-line
ask (`git checkout -b execution/03-web-live-sketch`) and everything
below still applies unchanged.

## Phase Gate — CLOSED

All five tasks have evidence above. Rust baseline is green, fmt/clippy
clean. No platform leakage detected. Continuing automatically to
Phase 01.

No true blocker for Phase 00-02. Phase 03 (Wasm compatibility audit)
will need `rustup target add wasm32-unknown-unknown` and a Wasm build
tool (`wasm-pack` or `wasm-bindgen-cli`); neither is a blocker, just a
setup step performed when that phase starts.
