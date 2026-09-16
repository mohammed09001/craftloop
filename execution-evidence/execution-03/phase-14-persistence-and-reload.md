# Execution 03, Phase 14 — Persistence and Reload

Recorded 2026-09-16. Tasks 113-118.

## Task 113 — Path-independent serialization: already done

No new work needed. `CraftLoopSession::to_json`/`from_json`
(`craftloop-web-bridge`, Phase 04) already exist precisely because
`wasm32-unknown-unknown` has no filesystem -- the module's own doc
comment already states "No `open(path)`/`save(path)`... `to_json`/
`from_json` replace them (Task 034/Article 43: path-independent
serialization)." This phase's real job was building the browser-side
adapter that finally calls them for something other than a unit test.

## Task 114 — IndexedDB adapter

`persistence/documentStore.ts` (new): a thin, promise-based IndexedDB
wrapper storing exactly one blob -- the real
`CraftLoopSession.toJson()` output -- under a single fixed key.
IndexedDB rather than `localStorage`: a real document's JSON can
exceed `localStorage`'s practical size ceiling, and IndexedDB's async
API is the standard, honest answer. Deliberately untested by Vitest
(jsdom has no real IndexedDB); proven instead by a real Playwright
test against Chromium's actual implementation, matching this project's
established preference for real-browser proof over a fake that would
only agree with itself.

## Task 115 — New/Open/Save harness flow

Three real actions added to `useCraftLoopSession`:
- `saveNow`: writes the current real `session.toJson()` to IndexedDB
  immediately -- the toolbar's `Save` button, un-deferred this phase.
- `newDocument`: constructs a genuinely fresh, empty
  `CraftLoopSession` (via a new `craftLoopSession.ts::createSession`,
  which shares the one-time Wasm `init()` call but always builds a new
  session instance) and makes it the active one -- like most document
  apps' "New," this does not delete the last real saved document, so
  `Open Last Saved` can still recover it afterward.
- `openLastSaved`: discards any unsaved in-memory state and rebuilds
  the session from the real last-saved JSON via
  `CraftLoopSession.fromJson`.

Both New and Open live in the toolbar's `More` overflow menu -- its
first real content ever (Phase 07's own doc comment had left it
disabled specifically because "nothing is currently being hidden from
either toolbar"; now something is).

## Task 116 — Autosave

A `useEffect` in `useCraftLoopSession` debounces (`AUTOSAVE_DEBOUNCE_MS
= 800`) on `snapshot.revision` -- the real `Document::revision()`,
which the backend only advances on a committed `DocumentChange`. This
is the precise mechanism the task's own wording asks for ("debounce
after committed semantic transactions, never pointer moves") without
any manual bookkeeping: selection changes, workspace-mode switches,
and raw pointer movement never touch `Document::revision()` at all
(confirmed by Phase 08's existing "mode switching never touches
document history" invariant and selection's documented ephemeral-state
status), so the effect simply never fires for any of them.

## Task 117 — Reload acceptance

`useCraftLoopSession`'s init effect now checks IndexedDB after the
Wasm module is ready: if a document was saved on a previous visit, it
is loaded via `CraftLoopSession.fromJson` and used as the tab's real
session instead of a fresh empty one. Proven by a real Playwright
`page.reload()` in `tests/e2e/persistence.spec.ts`: draw a line, wait
past the debounce with no explicit Save, reload, and the line is still
there.

## Task 118 — UI preferences stay separate

Structurally guaranteed, not merely tested: every ephemeral UI flag
(`activeTool`, `snapEnabled`, `showAllAnnotations`, `orthographicOpen`,
popover states) lives as plain React `useState` in `Workspace.tsx`/
its children -- none of it ever passes through
`CraftLoopSession.toJson()`, which only ever serializes the real
`Document`/`DocumentHistory`. There is no code path by which
`documentStore.ts` could see any of it. A real e2e test still proves
the *observable* consequence: toggling Snap off, reloading, and
confirming it comes back to its default rather than the pre-reload
value.

## Real bug found and fixed

`ViewBlockCard.tsx`'s `AxisField` (Phase 13) seeded its editable text
state once at mount from its `value` prop and never resynced --
harmless for that phase's own e2e test (which happened to check the
field immediately after a fresh render) but surfaced as an oxlint
`react(set-state-in-effect)` warning once inspected here. Fixed by
switching from a resync `useEffect` to the React-recommended pattern:
keying the field on its own real bound value so React remounts (and
correctly reseeds) it exactly when that value changes from outside,
never on an unrelated re-render.

## Test-first evidence

**Frontend unit** (`npm run test`): 60 passed (58 at Phase 13's close
+ 2 new `Toolbar.test.tsx` tests for Save/More; `documentStore.ts`
itself is deliberately not unit-tested -- see Task 114 above).

**Real browser** (`npm run test:e2e`): 32 passed, 0 failed (29 carried
over + 3 new `persistence.spec.ts` tests, plus `toolbar.spec.ts`'s
stale "Save/More disabled" assertion updated now that both are real):
a committed line survives an autosave-then-reload with no explicit
Save; explicit Save + New (real empty history, `Undo` disabled) +
Open Last Saved round-trips a document through the toolbar UI; Snap's
toggle state does not survive a reload while the geometry drawn
alongside it does.

## Verification

```powershell
cargo fmt --all -- --check                                          # clean, no Rust changed this phase
cargo clippy --workspace --all-targets --all-features                # clean
cargo test --workspace                                              # all green, no regressions
cd apps/web-live
npm run build                                                       # tsc -b (strict) + vite build (release wasm)
npm run test                                                        # 60/60
npm run test:e2e                                                    # 32/32, real Chromium
npm run lint                                                        # oxlint clean
```

## Phase Gate — CLOSED

All six tasks have evidence above. Backend invariant held: no crate
outside `apps/web-live` changed -- `to_json`/`from_json` already
existed. UI/UX invariant held: Save/More join the toolbar's existing
real tools without changing its shape; the More menu reuses the exact
dropdown pattern Constraint already established (Phase 09). Universal
invariant held: `IDBDatabase`/`indexedDB` types are confined to
`apps/web-live/src/persistence/documentStore.ts`, nowhere near a
platform-neutral crate. Continuing automatically to Phase 15 (Ink
Command and Command-Simulator Completion).
