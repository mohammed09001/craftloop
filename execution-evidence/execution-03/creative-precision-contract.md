# Creative Precision Contract

This is the standing design contract for Execution 03 (`apps/web-live`
and its Sketch Mode). Every frontend phase from Phase 02 onward is
checked against this document in addition to its own task-card UI/UX
invariant. Derived from Articles 5-7 and Phase 01's research mapping
(`phase-01-research-to-design-contract.md`); do not re-derive it from
scratch per phase.

## The doctrine

Craft Loop should feel like a sketchbook that understands engineering.
It must not feel like a CAD cockpit hidden inside a notebook. The user
begins with Pen. Precision appears only when needed. Sketch Mode adds
engineering tools to the creative surface instead of replacing it.

## Hard rules (a phase fails review if it violates any of these)

1. **Canvas dominates.** No layout may shrink the drawing surface to
   make room for permanent chrome.
2. **Top-center toolbar only.** No bottom toolbar, no side ribbon, no
   full-width bar. (Contradicts the current Android `Column` layout on
   purpose — Android gets its own parity patch later, Article 52.)
3. **No 3D cube, no feature tree, no permanent property inspector, no
   full-width ribbon, no dense textual toolbar, no persistent wall of
   constraints, no 3D modeling tools** in this execution (Article 7).
4. **Tool families group by icon + popover**, not one button per
   variant (Onshape finding, Task 007).
5. **Constraints shown are selection-filtered**, not an exhaustive
   always-visible list (Shapr3D finding, Task 009).
6. **Construction geometry is visually distinct** (dashed/muted) and
   never contaminates solid/export output (Fusion finding, Task 008).
7. **Draw-and-Hold refinement always produces an explicit, cancelable
   ghost candidate.** Raw ink is retained as a fallback. Never silently
   replace what the user drew (Free2CAD finding, Task 006; Article 27).
8. **Under/fully-defined state is a continuous visible signal**, not a
   one-shot toast (Shapr3D finding, Task 009).
9. **Every primitive, dimension, constraint, and conflict the UI shows
   must come from a real `CraftLoopSession` scene snapshot.** The
   browser never invents or duplicates engineering state (Article 8,
   Article 65's no-hallucination contract).
10. **Pen stays reachable from inside Sketch Mode** (`B`/`Pen` always
    returns to Main Creative; the reverse never traps the user).

## How later phases should use this file

When a phase's task card says "preserve Creative Precision," check the
concrete UI change against the ten rules above, not against a fresh
reading of Procreate/Onshape/etc. If a new situation isn't covered by
an existing rule, extend this file with a new numbered rule and note
which phase/task prompted it, rather than deciding silently.
