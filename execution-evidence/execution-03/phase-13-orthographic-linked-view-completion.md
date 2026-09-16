# Execution 03, Phase 13 — Orthographic Linked-View Completion

Recorded 2026-09-16. Tasks 102-112.

## No backend work needed

Unlike every prior phase, this one required zero changes to
`craftloop-document`, `craftloop-web-bridge`, or any other crate.
`CraftLoopSession::assign_view_identity`/`add_geometry_to_view`/
`enter_orthographic`/`propagate_shared_value`/`resolve_conflict` (all
Phase 04) and `WebViewBlockSummary`/`WebOrthographicSetSummary` (both
already in every scene snapshot) were already real, complete, and
covered end-to-end by `golden_alpha_journey_end_to_end_through_json_round_trip`
-- this phase's entire job was building the browser UI that finally
exercises that already-tested backend surface, which is why the
Playwright test below is deliberately structured as the exact same
journey that Rust test proves, just driven through the real DOM
instead of direct method calls.

## Task 102-105 — Real View Blocks, conditional Back

`orthographic/OrthographicPanel.tsx` (new): when no
`OrthographicSet` exists yet, offers "Assign selection as Front View,"
which chains three real calls
(`assignViewIdentity(undefined, Front)` → `addGeometryToView(frontId,
selectedIds)` → `enterOrthographic(frontId)`) -- exactly
`golden_alpha_journey`'s own sequence. `enterOrthographic` returns the
real ids of every view it actually created; the panel then renders
whatever `session.snapshot.view_blocks`/`orthographic_sets` report,
nothing else. Back only appears because the real engine produced one
for this case -- there is no Back-specific code path that could
fabricate one when the engine didn't.

## Task 106 — 2D engineering regions

`orthographic/ViewBlockCard.tsx`: each view is a flat labeled panel
(identity, readiness badge, a small 2D preview, blockers, two axis
fields) laid out in a wrapping row -- no 3D viewport, no camera/
orbit language anywhere. The mini preview reuses the exact same
`GeometryLayer` component the main Sketch2D canvas uses (via the
existing `fitToBounds` viewport helper), so a view block's rendering
is never a second, divergent shape-drawing implementation.

## Task 107 — Real readiness/unresolved state

`orthographic/orthographicSelectors.ts`'s `readinessTone`/
`isAxisUnresolved` read straight from `WebViewBlockSummary.readiness`/
`.unresolved_axes` -- both already real, already-tested backend
fields. `ViewBlockCard` shows an axis field's placeholder as
"unresolved" (and its geometry preview as "No geometry assigned yet")
only when the real data says so; Top and Right start with zero
geometry members and visibly show that, rather than guessing at
content the engine never produced.

## Task 108/112 — Shared-axis entry, no permanent master view

Every `ViewBlockCard`'s two relevant axis fields (Width/Height for
Front/Back, Width/Depth for Top, Depth/Height for Right -- mirroring
the real, static `craftloop_document::multiview::axes_for_identity`
table as a documented, deliberately-safe constant, since it is
projection-convention data, not user/engineering state) call the same
`onCommitAxis`, which `OrthographicPanel` wires to
`session.propagateSharedValue(thatBlock'sOwnId, axis, value)` -- Front,
Top, and Right all go through the identical code path with their own
real view id. The e2e test proves this concretely by entering Depth
from **Top**, not Front, and observing it propagate to Right.

## Task 109/110/111 — Atomic propagation, real conflict, real resolution

`propagateSharedValue`'s real return value (`{Propagated:
{affected_views}}` or `{Conflict: {conflict_id}}`) drives the UI
directly -- no separate polling/correlation step was needed here
(unlike Phase 12's dimension-edit conflict, this call returns the
conflict id synchronously). A `Conflict` outcome renders the real
`evidence` and real `allowed_resolutions` inline; resolving calls the
real `session.resolveConflict`, and the affected views' axis fields
update from the next real snapshot.

**Real bug found and fixed:** `AxisField`'s numeric input seeded its
local text state once at mount and never re-synced when its `value`
prop changed later -- so Right's Depth field stayed blank even after
`propagateSharedValue` from Top updated the real bound dimension. This
is precisely Task 109's own point (a view's field must update from
another view's edit), so it was caught immediately by the e2e test,
not shipped. Fixed with a `useEffect` re-seeding the field whenever
the real value prop changes.

## Test-first evidence

**Frontend unit** (`npm run test`): 58 passed (50 at Phase 12's close
+ 7 new `orthographicSelectors.test.ts` tests + 1 new `Toolbar.test.tsx`
test for the View toggle).

**Real browser** (`npm run test:e2e`): 29 passed, 0 failed (28 carried
over + 1 new `orthographic-views.spec.ts`, plus `toolbar.spec.ts`'s
deferred-tools test updated now that View is real): draws and selects
a line in Sketch2D, returns to Creative, opens the real Orthographic
panel, assigns Front and enters Orthographic (real Front/Top/Right/Back
blocks, Front showing the real line geometry, Top starting empty),
enters Depth 40 from Top and confirms it reaches Right, enters a
contradictory Depth 999 from Right and confirms the real conflict
evidence and `ReplaceAndPropagate` resolution appear, resolves it, and
confirms 999 now appears back on Top -- proving propagation is real,
symmetric, and conflict-aware end to end.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean, no Rust changed this phase
cargo clippy --workspace --all-targets --all-features                # clean
cargo test --workspace                                              # all green, no regressions
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build (release wasm)
npm run test                                                        # 58/58
npm run test:e2e                                                    # 29/29, real Chromium
npm run lint                                                        # oxlint clean
```

## Phase Gate — CLOSED

All eleven tasks have evidence above. Backend invariant held
absolutely: no crate outside `apps/web-live` changed this phase --
every real capability already existed, tested, since Phase 04. UI/UX
invariant held: the toolbar's top-center anchor is unchanged; View
follows the same independent-toggle pattern Snap/Show All already
established, and the panel itself uses flat 2D regions with no 3D
viewport language (Task 106). Universal invariant held: every new file
is plain React/SVG in `apps/web-live`. Continuing automatically to
Phase 14 (Persistence and Reload).
