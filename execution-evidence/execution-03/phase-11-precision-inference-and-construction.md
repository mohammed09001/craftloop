# Execution 03, Phase 11 — Precision Inference and Construction

Recorded 2026-09-15. Tasks 087-093.

## Backend: construction is real, persisted `Document` state

Unlike Ellipse (Phase 09), construction geometry needed no new
`BeautifiedPrimitive` variant, so it was built as real backend state
rather than deferred:

- `Document.construction_primitives: BTreeSet<PrimitiveId>`
  (`#[serde(default)]` for backward-compatible deserialization of
  older saved documents), with `is_construction`/
  `construction_primitives()`/`set_construction` accessors
  (`crates/craftloop-document/src/document.rs`).
- `DocumentChange::SetConstructionFlag { id, previous, new }` --
  `apply()`/`invert()` arms in `history.rs`, verified real,
  undoable/redoable, and JSON-round-trippable by
  `set_construction_flag_is_real_undoable_redoable_state`.
- `CraftLoopSession::set_construction(primitive_id, flag)` in
  `craftloop-web-bridge`: validates the primitive exists, reads the
  current flag, commits exactly one `DocumentChange` through the same
  `DocumentHistory` every other mutation uses -- no direct mutation of
  `self.document`. Not routed through the Command Bus (Article 237's
  curated `CommandAction` vocabulary is deliberately not extended for
  this, same reasoning as Arc/Ellipse in Phase 09).
- `WebPrimitiveSummary.is_construction: bool` exposes it in the scene
  snapshot.

Confirmed low-risk before building: `DocumentChange`'s `apply()`/
`invert()` exhaustive matches exist *only* in `history.rs` (two arms
needed), unlike `BeautifiedPrimitive`'s matches across seven-plus
crates -- a full workspace `cargo build` (all 22 crates + native
`craftloop-mobile-ffi` + `windows-harness`) compiled clean in one pass
after the change.

## Task 087/089 — Inference candidates, conservative policy

`src/canvas/inference.ts`'s `snapPoint(point, primitives, tolerance)`
is pure geometry over the real scene snapshot already provides:
endpoints/midpoint for a Line, center for a Circle/Arc, corners for a
Rectangle, falling back to the nearest 50-unit grid intersection, or
the original point unchanged if nothing is within tolerance. Real
geometry wins over the grid when both are in range.

Task 089's "conservative auto-constraint policy" is satisfied by
construction, not by a runtime check: `snapPoint`'s return type is
`{ point, guide }` -- there is no code path through which it could
return or imply a `SketchConstraintKind`. Landing a new endpoint
exactly on an existing one is a numeric coincidence the solver cannot
distinguish from a real `Coincident` constraint, but the module never
manufactures one; only the existing, explicit Constraint toolbar
action (Task 073, Phase 09) ever calls `applyConstraint`. A dedicated
test (`does not add or suggest any constraint`) asserts the result
object has exactly the `point`/`guide` keys.

## Task 088 — Ephemeral guides

`InkCanvas.tsx`'s `redraw()`: while a shape tool drags and Snap is on,
the live-preview's defining "last" point is converted to world space,
run through `snapPoint` against the real primitives, converted back to
screen space, and used for *both* the rubber-band preview shape *and*
a small dashed violet indicator circle drawn at the snapped point. The
indicator is drawn fresh every frame from the current drag state and
is never stored anywhere -- it disappears the instant the guide is no
longer relevant (the next frame with no snap, or stroke end), matching
"removed when no longer relevant" without any separate cleanup logic.

## Task 090/091 — Construction geometry UI

`GeometryLayer.tsx`'s `PrimitiveShape`: `primitive.is_construction`
renders a muted gray (`#9c9c98`) stroke with a looser dash pattern than
a Draw-and-Hold candidate, plus a `data-construction` attribute for
tests -- selection and candidate states still take visual priority, so
a selected or pending-confirm primitive never reads as inert
reference geometry.

`Workspace.tsx`'s `handleToggleConstruction`: an action (not a
persistent drawing mode, matching Dimension/Constraint's pattern) that
calls the real `session.setConstruction` for every currently selected
primitive, flipping each to the opposite of the *first* selected
primitive's current state -- a mixed selection becomes uniformly
construction on one click rather than left in an ambiguous mixed
state. Disabled with nothing selected (`constructionEnabled =
selectedPrimitives.length > 0`).

## Task 092/093 — Grid and snap controls

`Workspace.tsx` owns one `snapEnabled` boolean (both `CanvasStack` and
`Toolbar` need to agree on it, so it lives at the one place both
already share their real state). It is a genuinely independent toggle
-- `toolRegistry.ts`'s `snap` entry stays `kind: 'toggle'` but
`Toolbar.tsx` special-cases it outside the mutually-exclusive
`activeTool` radio group Pen/Select/Eraser/Line/Arc/Circle/Rectangle/
Construction share, so it can be on (or off) simultaneously with any
drawing tool.

`BackgroundGrid.tsx` gained a `visible` prop (default `true`, so
Creative mode's always-on grid is unchanged) that `CanvasStack` wires
directly to `snapEnabled` in Sketch2D -- Task 092's "optional subtle
grid" reads as part of the one precision feature rather than an
independent decoration.

`CanvasStack.tsx`'s shape-tool `handleStrokeComplete` branch applies
the *same* `snapPoint` call (same tolerance constant,
`SNAP_TOLERANCE_SCREEN_PX`, exported once from `inference.ts` so
`InkCanvas`'s preview and `CanvasStack`'s final creation call can never
drift apart) to the drag's defining world-space point before calling
`createShapeFromDrag` -- the primitive that actually gets created
always matches what the live preview showed.

## Test-first evidence

**Frontend unit** (`npm run test`): 49 passed, 0 failed (40 carried
over from Phase 10, 7 new `inference.test.ts` tests covering endpoint/
midpoint/center snapping, grid fallback, no-snap-in-range, geometry-
over-grid precedence, and the "no constraint" invariant, plus 2 new
`Toolbar.test.tsx` tests for the Construction and Snap buttons).

**Real browser** (`npm run test:e2e`): 24 passed, 0 failed (21 carried
over + 3 new `precision-inference.spec.ts` tests): with Snap on by
default, a second line dragged to within tolerance of an existing
line's real endpoint is created landing *exactly* on that endpoint
(`x2`/`y2` match the first line's, not the raw drag coordinate) --
proof the snap changes what the real `CraftLoopSession` actually
persists, not just what is painted; toggling Snap off makes the same
drag land at the raw coordinate instead, and the background grid
disappears with it; Construction toggles a real `data-construction`
attribute on the selected primitive on and back off through the real
`session.setConstruction` call.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean
cargo clippy --workspace --all-targets --all-features                # clean
cargo test --workspace                                              # all green (22 web-bridge, 26 windows-harness, + all domain crates)
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build (release wasm)
npm run test                                                        # 49/49
npm run test:e2e                                                    # 24/24, real Chromium
npm run lint                                                        # oxlint clean
```

## Phase Gate — CLOSED

All seven tasks have evidence above. Backend invariant held: real
engineering state (`construction_primitives`) lives in
`craftloop-document`, exposed through `craftloop-web-bridge` with no
duplicate model on the TypeScript side beyond the DTO mirror
`sceneTypes.ts` already maintains for every other field. Universal
invariant held: `inference.ts` is pure TypeScript geometry with no
DOM/Canvas/React coupling, testable in isolation (and is). No
Ellipse/CommandBus scope creep. Continuing automatically to Phase 12
(Dimensions and Adaptive Constraints UX).
