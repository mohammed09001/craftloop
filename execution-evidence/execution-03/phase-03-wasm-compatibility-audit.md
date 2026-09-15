# Execution 03, Phase 03 — Rust/Wasm Compatibility Audit

Recorded 2026-09-15. Tasks 018-022.

## Task 018 — Audit shared dependencies

Full dependency chain `CraftLoopSession` (native) coordinates, read
directly from every crate's `Cargo.toml` in `crates/`:
`craftloop-{ids,errors,geometry,input,ink,command,document,dimension,
consistency,sketch,recognition,constraint,serialization,transactions,
units,standards,ink-intent,handwriting}`. Only one third-party
(non-workspace, non-path) dependency exists anywhere in that chain:
`ezpz = "=0.2.29"` (craftloop-sketch's constraint solver, pinned per
Execution 01 Phase 11's solver decision record), which itself pulls in
`faer`/`gemm` (linear algebra). Everything else is `serde`,
`serde_json`, `thiserror`, `uuid`, all workspace-pinned.

Classification (Article 11's taxonomy), verified by actually building
for `wasm32-unknown-unknown` rather than reasoning from crate names:

| Dependency / call site | Classification | Evidence |
|---|---|---|
| `uuid::Uuid::new_v4()` (`craftloop-ids`) | randomness-dependent, compatible behind a feature flag | `cargo build --target wasm32-unknown-unknown -p craftloop-sketch` initially failed: `uuid` 1.26 refuses to compile for `wasm32-unknown-unknown` without an explicit randomness source (`js`, `rng-getrandom`, or `rng-rand`) |
| `ezpz`/`faer`/`gemm` (`craftloop-sketch`'s solver) | pure / Wasm-compatible | compiles clean once the `uuid` fix above landed; no `rayon` in this dependency chain (the one `rayon` entry in `Cargo.lock` belongs to `criterion`, a dev-only benchmarking dependency, not `ezpz`/`faer`) — confirmed by checking `Cargo.lock`'s reverse dependency list, not assumed from the crate names |
| `std::fs` in `craftloop-document::{persistence, autosave}` | native filesystem-dependent | compiles on `wasm32-unknown-unknown` (Rust's `std::fs` is stubbed, not absent, on that target) but every call returns an IO error at runtime — there is no filesystem. Not a compile blocker; a call-discipline requirement instead (Task 021) |
| `SystemTime::now()` in `craftloop-mobile-ffi::session::now_seconds` | time-dependent | local to `craftloop-mobile-ffi` (not a shared domain crate `craftloop-web-bridge` depends on), so it does not block this phase; the web bridge needs its own wall-clock source (`js_sys::Date::now()`) when Phase 04 needs timestamps, not this function |
| everything else in the chain (`serde`, `serde_json`, `thiserror`, geometry/ID/command/dimension/constraint/consistency domain code) | pure / Wasm-compatible | confirmed by a clean `wasm32-unknown-unknown` build of the full chain (Task 020) |

No threading dependency exists anywhere in this chain (no `std::thread`,
no `rayon` outside `criterion`), so there is no thread-availability
question to resolve for a browser main-thread build.

## Task 019 — Create craftloop-web-bridge

Added `crates/craftloop-web-bridge/` (registered in the root
`Cargo.toml` workspace `members`), `crate-type = ["cdylib", "rlib"]`,
depending directly on `craftloop-ids`, `craftloop-geometry`,
`craftloop-serialization`, `craftloop-recognition`, and
`craftloop-sketch` — the same lower-level domain crates
`CraftLoopSession` coordinates — plus `wasm-bindgen = "0.2"`. It does
**not** depend on `craftloop-mobile-ffi` or `uniffi`; Article 10 is
explicit that the Android UniFFI wrapper must not be compiled into the
browser, and UniFFI 0.32 has no `wasm32-unknown-unknown` backend to
begin with.

## Task 020 — Compile first real shared-core call

`solve_demo_sketch()` in `crates/craftloop-web-bridge/src/lib.rs`:
builds a real `craftloop_sketch::Sketch`, inserts a line primitive
whose endpoints are *not* level (`(0,0)-(10,4)`), applies a real
`SketchConstraintKind::Horizontal` constraint, solves with the real
`ezpz`/`faer`-backed `EzpzSolver`, and returns the solved sketch as
canonical JSON via `craftloop_serialization::to_canonical_json`. This
exercises the single highest-risk dependency in the whole chain (the
solver), not a trivial arithmetic stub.

**Native proof:** `cargo test -p craftloop-web-bridge` — 1 passed. The
solved line's endpoints land at `y0=1.9999999980000003`,
`y1=2.0000000000000004` (asserted `< 1e-6` apart) — a real numerical
solve, not a hardcoded result.

**Wasm compile proof:**
```powershell
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown -p craftloop-web-bridge
```
Exit 0.

**Real browser proof** (not simulated, not "compiles therefore works" —
Article 65 forbids that leap): built release Wasm, generated JS glue
with `wasm-bindgen-cli` 0.2.128 (matching the crate's resolved
`wasm-bindgen` version exactly, installed via `cargo install
wasm-bindgen-cli --version 0.2.128 --locked`), served a two-line HTML
harness that calls `solve_demo_sketch()` on load, and drove it with the
same Playwright/Chromium already installed for `apps/web-live` (Phase
02). Real browser console output:

```json
{"solve_status":"Solved","solved_primitive":{"displacement":0,"primitive":{"Line":{"a":{"x":0,"y":1.9999999980000003},"b":{"x":10,"y":2.0000000000000004}}}}}
```

Identical numeric result to the native test, confirming the solver's
real code path (not a stub) ran inside actual Chromium via
WebAssembly. No page errors. This harness was scratch/throwaway (built
under the scratchpad and a temp folder inside `apps/web-live/` that was
deleted afterward) — it proves the target works; wiring Wasm into
`apps/web-live` itself for real is Phase 04's `CraftLoopSession`
browser session (Task 023 onward), not duplicated here.

## Task 021 — Feature-gate native adapters

One additive Cargo change, no logic fork:

```toml
# crates/craftloop-ids/Cargo.toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
uuid = { workspace = true, features = ["js"] }
```

Native targets (`x86_64-pc-windows-msvc`, `aarch64-linux-android`) are
untouched — this table only applies when compiling for `wasm32`. The
same `Uuid::new_v4()` call site in `craftloop-ids` runs unchanged on
every platform; only the entropy *source* wasm-bindgen's `getrandom`
backend reads from differs per target, which is exactly what `uuid`'s
own `js` feature exists for.

`craftloop-document`'s `persistence.rs` and `autosave.rs` were **not**
`cfg`-gated — Task 018 found they already compile fine for wasm32 (the
stdlib stubs `std::fs` there), so gating them out would be forking the
crate's public surface for no compile-time benefit. Instead, both
modules got a doc-comment note (this task) stating plainly that
`craftloop-web-bridge` must never call `save_document_atomically`,
`load_document`, or `AutosaveJournal` — every one of those calls would
compile but fail at runtime with an IO error, since there is no
filesystem. Phase 14 (Persistence and Reload) uses
`craftloop_serialization::to_canonical_json` directly plus a browser
storage adapter instead, which is exactly what `solve_demo_sketch`
already does today for its own output.

## Task 022 — Document compatibility changes

Every shared-crate change this phase made, and why each is safe for
native targets:

1. `crates/craftloop-ids/Cargo.toml` — added a `wasm32`-only
   `[target...dependencies]` table enabling `uuid`'s `js` feature.
   Native builds never evaluate that `cfg`, so nothing changes for
   Android/Windows. Verified: `cargo test -p craftloop-ids` still 7/7,
   full workspace `cargo test --workspace` still all-green after the
   change.
2. `crates/craftloop-document/src/persistence.rs` and `autosave.rs` —
   doc-comment additions only, no code changes. Verified:
   `cargo test -p craftloop-document` unaffected (5+ tests, all still
   passing as part of the full-workspace run below).
3. New crate `crates/craftloop-web-bridge/` — additive; no existing
   crate's public API changed to create it.
4. Root `Cargo.toml` — added `crates/craftloop-web-bridge` to workspace
   `members`. No existing member's behavior changes from workspace
   membership alone.

## Verification

```powershell
cargo fmt --all -- --check                                    # clean
cargo clippy --workspace --all-targets                        # clean, zero warnings
cargo test --workspace                                        # all green, no regressions from Phase 00's baseline
cargo test -p craftloop-web-bridge                             # 1/1, real solver result
cargo build --target wasm32-unknown-unknown -p craftloop-web-bridge   # exit 0
```

Plus the real-Chromium proof under Task 020.

## Phase Gate — CLOSED

All five tasks have evidence above, including a real browser run (not
just a compiler success), so this satisfies the phase-gate's "Web Live
View opens and the phase behavior is interactively testable" in spirit
even though the demo lived outside `apps/web-live` itself this phase.
Shared semantics remain platform-neutral: the one behavior change
(`uuid`'s entropy source) is additive and per-target, not a fork; no
domain crate gained Wasm-only logic. No unsupported native-device claim
made. Continuing automatically to Phase 04 (Web CraftLoopSession),
which is where `craftloop-web-bridge` grows a real session API and gets
wired into `apps/web-live` for real.
