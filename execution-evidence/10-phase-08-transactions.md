# Phase 08 — Undo, Redo, Transactions, and Event History — Evidence

Recorded: 2026-09-13

## Modules added

All within `crates/craftloop-document` (which already existed from Phase
07; this phase is where Phase 01's generic `craftloop-transactions`
primitives finally meet a real domain type). New dependency:
`craftloop-transactions`. New modules: `history.rs`, `provenance.rs`,
`stale_result.rs`; extended `document.rs` with `revision`/`provenance`
fields and accessors.

## Tasks 056–061

| Task | Module | Summary |
|---|---|---|
| 056 Command transactions | `history.rs::DocumentHistory::commit` | A `Vec<DocumentChange>` commits as one `craftloop_transactions::Transaction` — one visible action, however many entities it touches. |
| 057 Inverse operations / state deltas | `history.rs::DocumentChange::invert` | **State-delta strategy chosen and documented**: `RemoveEntity`/`SetProvenance` snapshot the value being overwritten at record time, so inversion is a pure data swap, not a synthesized inverse function. "Cross-view propagation" (no real views exist until Phase 20) is stood in for by a transaction spanning two *pages* at once, documented as the closest real analog available now. |
| 058 Redo determinism | `history.rs::DocumentHistory::redo` | Redo replays the original forward `DocumentChange`s exactly — pure data application, no recognition or any other nondeterministic step re-runs. |
| 059 Provenance states | `provenance.rs`, `document.rs` | `ProvenanceState` (the seven outcomes named verbatim: UserCreated/Suggested/Accepted/Modified/Derived/Propagated/Rejected), tracked per-entity on `Document` and mutated through the same transaction/undo mechanism via `DocumentChange::SetProvenance`. |
| 060 Stale async result rejection | `stale_result.rs::AsyncResult<T>` | Captures `Document::revision()` at creation; `accept_if_fresh` discards the value if the document's revision moved on. No async runtime exists yet (honestly noted); this is the freshness-check primitive any future background producer must go through. |
| 061 Crash-safe transaction tests | `tests/crash_safe_transactions.rs` | A transaction with a step engineered to fail partway (referencing an unknown page) is proven to leave **zero** partial changes — not just "eventually consistent," but literally: the page is empty, no transaction enters history, prior unrelated commits are untouched, and undo afterward still correctly reverts the last real commit. |

## A real bug caught and fixed before it ever compiled

The first draft of `DocumentChange::SetProvenance` had `new: ProvenanceState`
(non-optional) and computed its inverse as
`previous.unwrap_or(ProvenanceState::UserCreated)` when `previous` was
`None`. That is wrong: undoing "first-ever classification" should restore
*no classification at all*, not fabricate `UserCreated` as an invented
default. Caught during self-review of the initial design (before the crate
even built), fixed by making both `previous` and `new` `Option<ProvenanceState>`
so the inverse can express "clear the entry" via `Document::clear_provenance`
instead of inventing a state. The existing
`set_provenance_undoes_to_the_prior_state_including_none` test exercises
exactly this path (its captured `previous` is `None`, since it runs before
any provenance was ever set).

## Commands and results

```
cargo build -p craftloop-document
cargo test -p craftloop-document                       # 53/53 unit tests
cargo test -p craftloop-document --test crash_safe_transactions   # 3/3
cargo fmt --all -- --check                              # clean after cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings   # clean, no findings
cargo test --workspace                                  # 342/342 tests passing workspace-wide
```

Full output: `test-reports/phase-08-cargo-test.txt`,
`test-reports/phase-08-cargo-clippy.txt`.

## Deferred (explicitly, not silently)

- Real multi-view propagation testing — no view blocks exist until Phase
  20; the multi-page transaction test is the closest available analog and
  is documented as such in `history.rs`.
- Real async recognition producing `AsyncResult` values — no background
  recognition pipeline exists yet (Phase 16+); only the freshness-check
  primitive is built now.
- Persistence-level crash simulation (killing the process mid-write) beyond
  what Phase 07 already covers via corrupted-file recovery tests — Task 061
  is about transaction atomicity in memory, which is what is tested here;
  Phase 07's `persistence.rs` tests already cover on-disk atomicity.

## Phase Gate

- All six tasks (056–061) represented in repository code with passing
  tests, including one design bug caught and fixed before first compile.
- `cargo test --workspace`: 342/342 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no view/orthographic logic fabricated ahead of
  Phase 20; no async runtime invented ahead of a real producer needing one.
- Proceeding to Phase 09 automatically.
