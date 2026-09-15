# Execution 03, Phase 05 — Scene Snapshot Enrichment

Recorded 2026-09-15. Tasks 035-042.

## Task 035 — Audit snapshot completeness

Phase 04's `WebSceneSnapshot` (a direct port of native `FfiSceneSnapshot`)
was missing everything a real canvas/badge UI needs beyond a bounding
box: real per-primitive coordinates, dimension target anchors,
**constraints entirely** (native's own snapshot never lists them
either — a real gap, not something this phase regressed), view-block
readiness/shared-axis state, and full conflict-resolution metadata.
Listed precisely, then closed one task at a time below.

## Task 036 — Add geometry summaries

`WebPrimitiveSummary` gained a `geometry:
craftloop_recognition::BeautifiedPrimitive` field, serialized as-is
(`{"Line":{"a":{...},"b":{...}}}`, `{"Circle":{"center":...,
"radius":...}}`, `{"Arc":{...}}`, `{"Rectangle":{"corners":[...]}}`) —
real coordinates for every one of the four primitive kinds that
actually exist. Kept the existing `min_x`/`max_x`/etc. bounding box too
(still needed for hit-testing/fit-to-content, per Phase 04's own
carried-over rationale).

**No `Ellipse`.** The doc's task wording says "Segment/Circle/Arc/
Ellipse/Rectangle," but `craftloop_recognition::BeautifiedPrimitive`
has exactly four variants (`Line`, `Circle`, `Arc`, `Rectangle`) — no
`Ellipse` exists anywhere in the shared core. Repository-first context
(Article 81: "example ... are guidance, not authority over the
repository") and Article 65's no-hallucination contract both apply:
fabricating an `Ellipse` field here would expose a capability the
engine does not have. Recorded as an honest gap, not silently dropped.

## Task 037 — Add dimension summaries

`WebDimensionSummary` gained `target_primitive_ids: Vec<String>`, from
the real `DimensionTarget::primitive_ids()`.

## Task 038 — Add constraint summaries

New `WebConstraintSummary { id, label, primitive_ids }` and a new
`constraints` field on `WebSceneSnapshot`. Required a small additive
change to the shared `craftloop-sketch` crate itself: `Sketch` only
exposed `constraint(id)` (single lookup), no way to list every stored
constraint. Added `Sketch::constraints() -> impl Iterator<Item =
(ConstraintId, &SketchConstraintKind, &ConstraintProvenance)>`
(`crates/craftloop-sketch/src/sketch.rs`), a pure read accessor over
the same `BTreeMap` — no semantic change, no mutation path added. One
new native test, `constraints_lists_every_stored_constraint`
(`cargo test -p craftloop-sketch`: 68/68 including it).

`label` is the constraint kind's own `Debug` rendering rather than a
second hand-maintained name for each of its eleven variants — honest
and always in sync; a prettier label is Phase 09-12's presentation
concern. `primitive_ids` is deduplicated (a `BTreeSet` before
collecting to `Vec<String>`): a single primitive can supply more than
one `PointRef` to the same constraint (`Horizontal(line)` references
that line's start *and* end), and a real test caught this
(`primitive_ids: [line_id, line_id]` before the fix) rather than being
reasoned out in advance.

## Task 039 — Add View Block summaries

`WebViewBlockSummary` gained `geometry_member_ids: Vec<String>` (not
just a count), `readiness: WebOrthographicReadiness` (a new enum
mirroring the real, 5-level `OrthographicReadiness`), `blockers:
Vec<String>` (the real blocking `ReadinessIssue`s from the same
`evaluate_readiness` call `enterOrthographic` itself uses), and
`axis_bindings`/`unresolved_axes` from the real
`MultiviewGraph::bindings_for_view`/`unresolved_axes`. Deliberately
excludes `PageLayoutTransform` (pixel/canvas layout): Article 38 puts
computing the visual 2D engineering-region layout in the frontend, not
in this read model.

## Task 040 — Add Orthographic summaries

Shared-axis state lives per-view (Task 039's `axis_bindings`/
`unresolved_axes`), matching the real domain API's own shape
(`MultiviewGraph` operates per-`ViewId`, not per-`OrthographicSet`) —
`WebOrthographicSetSummary` itself stays `{id, view_ids}`, which is all
an `OrthographicSet` actually owns.

## Task 041 — Add conflict summaries

`WebConflictSummary` gained `severity` (new `WebSeverity`, including
the `Blocker` variant the compiler caught missing on the first build),
`affected_entities`, `existing_truth`, `proposed_truth`, `evidence`,
and `allowed_resolutions: Vec<WebResolutionChoice>` — every field the
real `craftloop_consistency::Conflict` carries.

## Task 042 — Keep read-model discipline

`sceneSnapshot()` returns an owned JSON `String`; there is no
`#[wasm_bindgen]` setter or mutable-reference path back into
`self.document` anywhere in this crate. Proven with a real test
(`scene_snapshot_is_a_disconnected_copy_not_a_mutable_handle`):
mutate the parsed snapshot value's `primitives`/`revision` fields, take
a fresh `sceneSnapshot()` call, and confirm the real state is
untouched — a JS caller could do the exact same `JSON.parse` +
field-mutation and it would be equally inert.

## Test-first evidence

`cargo test -p craftloop-web-bridge`: **15 passed, 0 failed** (up from
Phase 04's 9). Six new tests, one per enrichment: real geometry
coordinates, dimension anchors, constraint listing (with the
deduplication bug caught and fixed), view-block readiness/axis
bindings (exercised through a real Orthographic + shared-depth-
propagation flow, not a hand-built fixture), conflict resolution
metadata, and the read-model-discipline proof.

`cargo test -p craftloop-sketch`: **68 passed, 0 failed** (67 + 1 new).

## Verification

```powershell
cargo fmt --all -- --check                                          # clean
cargo clippy --workspace --all-targets                              # clean, zero warnings
cargo test --workspace                                              # all green, no regressions
cargo test -p craftloop-web-bridge                                  # 15/15
cargo test -p craftloop-sketch                                      # 68/68
cargo build --target wasm32-unknown-unknown -p craftloop-web-bridge # exit 0
```

## Phase Gate — CLOSED

All eight tasks have evidence above. Shared semantics remain
platform-neutral: the one shared-crate change (`Sketch::constraints()`)
is a pure additive read accessor, not a new mutation path or a
duplicated model. No unsupported native-device claim made; the missing
`Ellipse` capability is stated plainly rather than fabricated.
Continuing automatically to Phase 06 (Browser Canvas Stack), which is
where this enriched snapshot actually gets rendered for the first
time.
