# PaperKit Decision Boundary

Execution 01, Phase 29, Task 210. Recorded 2026-09-13. Authority: MCP
Article 128 "Document Model"; Engine Contract 15 (Document: "Confirmed
semantics survive recognition-model changes").

**Confidence note**: as with `pencilkit-mapping.md`, PaperKit API details
below reflect training knowledge (PaperKit is a newer, less
extensively-documented Apple framework than PencilKit as of this
assistant's training data), not a live docs fetch. Re-verify before real
integration.

## Decision

**PaperKit, if used at all, is scoped strictly to generic document
markup/annotation presentation -- it never becomes, wraps, or replaces
this product's engineering geometry model.** Every primitive, dimension,
constraint, and view this product reasons about continues to live
exclusively in the shared Rust core (`craftloop-geometry`,
`craftloop-dimension`, `craftloop-sketch`, `craftloop-document`), exactly
as Engine Contract 15 and Article 128 already require independent of any
platform SDK.

## Why this boundary is necessary to state explicitly

PaperKit (Apple's markup/annotation framework, built on top of PencilKit)
is the kind of convenient, higher-level SDK that could *look* like a
shortcut to a working document/annotation model on iPad -- generic
freeform markup, text, shapes, and page layout out of the box. That
convenience is exactly the risk Task 210's objective names: "do not let
it become the engineering geometry model." If a future iPad adapter
reached for PaperKit's own shape/annotation primitives to represent, say,
a dimension line or a constraint, this product would end up with
Apple-owned data structures as its engineering source of truth on one
platform and Rust-owned structures on every other platform -- a direct
violation of Engine Contract 15's "confirmed semantics survive
recognition-model changes" (a PaperKit SDK version bump could silently
reshape or deprecate the very types holding engineering meaning) and of
this whole execution's foundational rule (Task 007, Phase 01) that the
shared core is the single source of truth across every adapter.

## Where PaperKit is legitimately usable, if adopted

Only for concerns that are genuinely presentation/markup, not
engineering semantics -- and only if a future phase's actual UI work
finds real value in it (this record authorizes *scope*, not *adoption*;
no phase through 29 has a task asking to integrate PaperKit at all):

- Freeform annotation/markup on top of an already-exported page (e.g.
  marking up a printed-style export for review), analogous to how a PDF
  annotation tool works on a static page image -- never on the live
  engineering document.
- General notes composition UI chrome (text formatting, page furniture),
  distinct from `craftloop_document::Note` (Phase 07), which remains the
  one real, persisted, cross-platform note entity.

## What must never happen

- A `PKDrawing`/PaperKit markup object standing in for a
  `SemanticEntity::Primitive` or `SemanticDimension` (Phase 06/10).
  Geometry recognition/beautification stays exactly where Phase 06 built
  it -- platform-independent, tested without any Apple SDK.
- Document save/load going through a PaperKit-native serialization format
  instead of `craftloop-document`'s persistence (Phase 07/08). Every
  platform's adapter, Android and iPad alike, converges on the same
  `Document`/`Page`/`SemanticEntity` shape before anything is saved.
- Constraint/dimension state (Phase 10-13) represented in any
  PaperKit-owned type. The FFI boundary this phase confirmed
  (`crates/craftloop-mobile-ffi`, Task 207) is the only sanctioned path
  from platform UI into engineering state, on any platform.

## Relationship to PencilKit

Distinct decision from `pencilkit-mapping.md` (Task 209): PencilKit
supplies raw stroke *input* (position/pressure/tilt), which this
document's sibling maps onto `FfiPointerSample` and which legitimately
does cross into the shared core via `validate_stroke`. PaperKit is a
*presentation/composition* layer built on top of that input, one level
further from raw sensor data and correspondingly further from anything
that should touch engineering semantics.
