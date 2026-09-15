# Execution 03, Phase 01 — Research-to-Design Contract

Recorded 2026-09-15. Tasks 006-011.

Source: Articles 5-7 of `execution/Craft Loop Execution 03 Web Live
View Creative Sketch Mode.md`, which already convert the scholarly and
competitive research into Craft Loop rules. This phase's job is to
turn those rules into concrete, checkable engineering and UI
constraints that later phases can be graded against, not to re-run the
literature review from scratch.

## Task 006 — Scholarly findings → backend/interaction constraints

| Source | Finding | Concrete constraint for this execution |
|---|---|---|
| SketchGraphs | Sketches are relational: primitives + explicit constraints, not pixels | `craftloop-web-bridge` must expose primitive/constraint/dimension IDs and relations from `CraftLoopSession`'s real scene snapshot; the browser never invents its own geometry graph (Article 8) |
| Vitruvion | Design intent (primitives + constraints) must survive edits | Undo/redo, edit-dimension, and re-solve must all round-trip through the same shared `CraftLoopSession`, never through client-only React state mutation |
| Free2CAD | Freehand input may *suggest* structured ops, but the user stays in control and deterministic geometry validates the result | Draw-and-Hold (Phase 10) always produces a ghost candidate the user confirms/cancels explicitly; raw ink is preserved as fallback (Task 086) — never silent auto-replacement |
| OpenAlex-style research discovery | Cite research provenance rather than asserting unsourced ML behavior | Article 60's ML boundary: no black-box handwriting/ML claim without a matching test or explicit "deferred" label (Article 65) |

## Task 007 — Onshape Sketch UX

Recorded: entering Sketch is a dedicated contextual mode with its own
toolbar; tools are grouped into families (Line/Arc/Circle/Rectangle
etc.) behind a single icon with a popover rather than one row per
variant; the toolbar stays near the top of the canvas; keyboard
shortcuts (e.g. `L` for line) work while the pointer is over the
canvas; dimensions and constraints attach directly to selected
geometry rather than living in a separate always-open panel.

Applied here: Article 25 (Grouped Tool Families), Article 46 (Keyboard
Shortcuts), Article 94/095 task pair (geometry-adjacent dimension
input) all mirror this directly.

## Task 008 — Fusion Sketch UX

Recorded: "Sketch" is a distinct palette entered/exited explicitly;
grid and snap are Sketch-scoped, not global chrome; construction
geometry is visually distinct (dashed/muted) and excluded from solid
export; dimension and constraint visibility can be toggled without
deleting them.

Applied here: Article 32 (Construction Geometry), Article 34 (Grid and
Precision), Article 31 (Constraint Visibility), Task 101 (visibility
toggles).

## Task 009 — Shapr3D Sketch UX

Recorded: constraints offered to the user are filtered to what the
current selection can actually take (no permanent wall of every
constraint type); direct manipulation (drag a point, see the solve
happen) is preferred over modal dialogs; under-defined vs. fully-defined
state is a visible, continuous signal, not a one-time toast.

Applied here: Article 30 (Adaptive Constraints), Article 99/100 task
pair (adaptive constraint popover + solver feedback), Article 57
(Constraint Micro-Feedback).

## Task 010 — Creative sketch apps (Procreate, Concepts, Linea, Morpholio Trace)

Recorded: QuickShape-style hold-to-refine turns a rough stroke into a
clean primitive without leaving the drawing gesture; precision tools
are reachable but not default-visible chrome; UI chrome is minimized so
the canvas/drawing dominates the frame; a creative app can still carry
CAD-grade intelligence (Morpholio Trace) without looking like CAD.

Applied here: Article 27 (Draw-and-Hold Refinement), Article 56
(Creative Surface Rules), Article 7 itself (Design Doctrine).

## Task 011 — Creative Precision contract

Written to `execution-evidence/execution-03/creative-precision-contract.md`.
This is the artifact every subsequent frontend phase (02 onward) is
checked against, alongside each phase's own UI/UX invariant line.

## Phase Gate — CLOSED

All six tasks have evidence above; the contract artifact exists.
Nothing here required a Rust/Wasm/frontend test since Phase 01 is
research synthesis, not behavior change — Article 81's own task
template still applies but this phase's "test-first rule" is satisfied
vacuously (no deterministic behavior changed). No platform-semantic
fork introduced. Continuing automatically to Phase 02.
