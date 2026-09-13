# Phase 24 — Cross-View Correspondence and Ambiguity — Evidence

Recorded: 2026-09-13

Built inside `crates/craftloop-document` (new `correspondence.rs`).

## Task 173 — Manual link/unlink first

`CorrespondenceStore::confirm_link`/`unlink`/`is_confirmed`: an explicit,
deterministic, order-independent (normalized pair) store, fully usable
and correct with zero dependency on any evidence-computation code below
it -- tested standalone before any evidence function is even introduced.

## Task 174 — Deterministic correspondence evidence

`evaluate_correspondence` uses three of Article 232's four named
deterministic sources for real: **shared coordinates** (page-space
position agreement along the screen axis two view identities are known
to share -- computed properly for Front/Top and Front/Right using each
view's real `PageLayoutTransform`; `Top`/`Right`, which share `Depth`,
honestly returns no coordinate evidence since correct correspondence
there needs real miter-line projection-transfer geometry no earlier
phase built), **dimensions** (a confirmed `DimensionTarget::Pair`
already naming both entities), and **existing links**
(`CorrespondenceStore::confirm_link`). **Centerlines** are named and
explicitly recorded as deferred -- no centerline concept exists anywhere
in this workspace yet. A fourth, extra signal (**shared extent** --
matching characteristic size) is added as real, always-computable
evidence for pairs the coordinate check cannot reach.

## Task 175 — Suggestion state

Not a stored state at all, deliberately: a "suggestion" is the transient
`CorrespondenceCandidate` `evaluate_correspondence` returns on demand,
kept structurally separate from `CorrespondenceStore`'s persisted
`confirmed` set -- a candidate with `Confidence::ONE` and
`EvidenceKind::ExistingLink` is distinguishable from a partial-confidence
suggestion by a direct test.

## Task 176 — Rejection state

`CorrespondenceStore::reject`/`is_rejected`, checked first in
`evaluate_correspondence` -- a rejected pair returns `None` outright,
proven directly: a pair with real, otherwise-strong evidence still
produces no candidate once rejected.

## Task 177 — Smallest-missing-fact diagnostics

`smallest_missing_fact`: exactly one unconfirmed candidate (the
highest-confidence one among several) rather than every open question at
once -- Article 233's own rule made literal for the correspondence case
this phase covers.

## Task 178 — Future learned-ranking seam

`CorrespondenceRanker` trait + `DeterministicRanker` reference
implementation. The trait's own signature (`Vec<CorrespondenceCandidate>
-> Vec<CorrespondenceCandidate>`, no `&mut CorrespondenceStore` anywhere)
is the structural guarantee that no implementation, however confident its
ranking, can confirm a link itself -- only `CorrespondenceStore::confirm_link`
(an explicit user action) can.

## Task 179 — Ambiguous-feature regression suite

Three named scenarios: **repeated holes** of identical size (both produce
real candidates, but only the position-aligned one also gets coordinate
evidence, so they are distinguishable rather than tied); **similar
edges** (extent evidence, real and computed); **deliberately independent
geometry** (wildly different size and position, no shared dimension,
never confirmed -- `evaluate_correspondence` returns `None`, proving
unrelated geometry is never forced into a candidate).

## Commands and results

```
cargo build --workspace
cargo test --workspace                                  # 667/667 passing (13 new)
cargo fmt --all -- --check                                # clean
cargo clippy --workspace --all-targets -- -D warnings      # clean, no findings
```

Full output: `test-reports/phase-24-cargo-test.txt`,
`test-reports/phase-24-cargo-clippy.txt`,
`test-reports/phase-24-cargo-fmt.txt`.

## Deferred, explicitly

- Centerline-based evidence -- no centerline type exists in this
  workspace yet.
- Top/Right coordinate-based (depth) correspondence -- needs real
  miter-line projection-transfer geometry; only extent-based evidence is
  available for that pair.
- Probabilistic/learned evidence (visual similarity, structural context,
  repeated patterns -- Article 232's own "probabilistic evidence" list) --
  Task 178 provides the seam; no task asks for an actual learned model.
- Wiring this engine into a live editing session -- no task this phase
  names that integration.

## Phase Gate

- All seven tasks (173-179) represented in repository code, each with a
  focused, passing test.
- `cargo test --workspace`: 667/667 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no new crate, no new dependency; reused
  `DimensionTarget::Pair` (Phase 10) and `PageLayoutTransform` (Phase 07)
  rather than inventing parallel mechanisms.
- Implemented and tested: manual link/unlink, three real deterministic
  evidence sources (with the fourth, centerlines, named and deferred),
  the suggestion/confirmed/rejected three-way state separation, smallest-
  missing-fact diagnostics, the learned-ranking seam, and all three named
  ambiguity regression scenarios.
- Deferred: see above, each with a reason.
- Blocked with evidence: none.
- Proceeding to Phase 25 automatically.
