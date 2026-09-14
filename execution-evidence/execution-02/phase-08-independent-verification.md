# Execution 02, Phase 08 — Independent Verification

Recorded 2026-09-14, by the coordinating session directly, after
commit `325a625` (Phase 08: pan, zoom, selection, deletion,
fit-to-content). Given the stylus-gate saga in Phase 07 (two false
"verified" claims before a real fix), every Phase 08 claim was
independently re-checked here from a fresh app launch, reusing no
subagent-produced screenshots as evidence.

## Method and results

1. `adb shell am force-stop com.craftloop.shell && adb shell am start -n com.craftloop.shell/.MainActivity` --
   confirmed via `dumpsys activity activities` that the app actually
   resumed (`topResumedActivity`/`ResumedActivity` both show
   `MainActivity`) before trusting any screenshot.
2. **Baseline** (`phase-08-independent-verify-baseline2.png`):
   `pointer=none revision=0 txCount=0 selected=0 primitives=0
   zoom=1.00 pan=(0, 0)`.
3. **Touch-gate + pan, combined test**
   (`phase-08-independent-verify-after-pan.png`): one
   `adb shell input touchscreen swipe 700 400 1400 700 300` (finger
   swipe) produced `revision=0 txCount=0` (**unchanged** -- no ink
   created) **and** `pan=(692, 297)` (changed from `(0, 0)` -- real
   pan applied). This is the first time in this project a single test
   simultaneously proves the touch-gate holds *and* new functionality
   works, rather than needing separate passes.
4. **Primitive creation** (`phase-08-independent-verify-testline2.png`):
   tapping the temporary `TestLine` button produced
   `revision=1 txCount=1 primitives=1 canUndo=true` -- a real
   committed transaction, not a UI-only change.
5. **Undo** (`phase-08-independent-verify-undo.png`): tapping `Undo`
   produced `revision=2 txCount=0 primitives=0 canUndo=false
   canRedo=true` -- the primitive was actually removed via a real
   inverse transaction (not just hidden), and redo became available,
   consistent with `DocumentHistory`'s real semantics proven at the
   Rust layer since Phase 04.

## Result

**PASS, independently confirmed**: the narrowed touch-gate holds, pan
works, primitive creation and undo both drive real `CraftLoopSession`
transactions end to end, all on the connected Samsung tablet, from a
fresh launch, verified without reusing subagent-reported evidence.

## Not independently re-verified here (stated, not silently assumed)

- Pinch-to-zoom: `adb shell input` cannot synthesize real multi-touch,
  matching the fork's own stated limitation.
- Selection-via-stylus-tap and fit-to-content: not re-exercised in
  this pass; the fork's own evidence file documents its own testing
  of these. Given the touch-gate/pan/undo spot-checks above all
  matched the fork's claims exactly, no separate reason exists here to
  distrust the remaining claims, but they were not personally
  re-clicked by this session.
