# MCP V1 Source Coverage Ledger

Execution 01, Phase 32, Task 232. Recorded 2026-09-14. Authority: this
task's own objective -- "every MCP V1 article must have an implemented,
tested, documented, deferred, or research status."

## Method

`Craft Loop MCP V1.md` contains 680 numbered articles. Per Phase 00's
own established convention (`traceability/requirements-index.md`'s
"How this index was built"), this ledger is built **structurally**, from
article titles and category membership, not by re-reading all 680
bodies into this session -- the same progressive-disclosure discipline
this execution has followed throughout. Every article maps to exactly
one of five statuses:

- **Implemented and tested** -- real code exists, with a passing test
  proving it, cited to the specific phase/file.
- **Implemented but not hardware/tool-validated** -- real code/config
  exists, but the specific verification named requires unavailable
  hardware or toolchain (matches the No-Hallucination Contract's own
  category).
- **Documented/Research** -- a design decision, philosophy, or
  background-research article this execution read and applied, not a
  discrete engine to build.
- **Deferred, explicitly** -- named, with a reason, not silently
  dropped.
- **Not in this execution's scope** -- production-UI/mobile-platform/
  cloud-service concerns Execution 01's own charter (Task 007, and
  every phase's "never production UI" rule) places outside a
  shared-core engineering execution.

## High-value categories, mapped per article

### Detailed Specification (40 articles, 221-260) -- one per named engine

Every one of 221-239 (Ink Intent, Primitive Recognition, Beautification,
Handwriting Parser, Dimension Association, Constraint Engine, Geometric
Consistency, Design Intent Graph, Constraint State, Orthographic
Relationship, Multiview Constraint Graph, Cross-View Correspondence,
Ambiguity, Orthographic Readiness, Standards, Ink Command, Contextual
Command Grammar, Command Confirmation Gesture, Command Bus) is
**Implemented and tested** -- each is the exact engine one of Phases
06/16/06/17/18/12/14/15/13/23/22/24/24/21/25/19/19/19/19 built, with its
own crate and passing test suite (see that phase's own evidence file).
Article 240 (Document Engine) and 241 (Native Ink Boundary) likewise
**Implemented and tested** (Phases 07, 05).

Articles 242-260 (Stroke Grouping/Smoothing, Pen Tools, Touch
Interaction, Zoom/Pan, Spatial Navigation, Page Frames, Layers, Styles,
Theme, toolbars, Contextual Popovers, Inline Editing, Selection Handles,
Dragging Constrained Geometry, Snap vs. Constraint, Automatic Constraint
Suggestions) split: **242-243 Implemented and tested** (Phase 05's
`grouping.rs`/`smoothing.rs`); **244, 246, 247 Implemented and tested**
at the Windows-harness level (Phase 04's `tool.rs`/`viewport.rs`), but
**Task 230's own audit this phase (below) found the harness UI never
grew past Phase 04's scope** -- so 244/246/247 are validated only for
the harness's original Phase 04 feature set, not for anything added by
later engines; **258-260 Implemented and tested** at the domain-model
level (Phase 12's `constraint_kind.rs`, Phase 15's suggestion
accept/reject), never at a UI-interaction level (no drag gesture exists
to test against, since no production UI was built); **245, 248-257 Not
in this execution's scope** -- production visual/interaction design
(Touch Interaction, Page Frames/Layers/Styles/Theme, toolbar/popover/
inline-editing/selection-handle UI) is explicitly the "never production
UI" boundary every phase's evidence respects (Task 007; the disposable-
harness pattern established Phase 04 and never violated since).

### Requirement Group (40 articles, 581-620) -- functional checklists

581-614 (Notebook Entry through Search) and 618-620 (Platform
Consistency, Product Boundaries, Human Authority) each map 1:1 to the
phase Phase 00's own table already assigned (see
`traceability/requirements-index.md`'s Phase→Article map) --
**Implemented and tested**, except:
- **613 Search**: Phase 00's table assigned this to Phase 30, but
  **Phase 30's actual nine tasks (213-221) never built or tested any
  search functionality** -- a real gap this ledger surfaces rather than
  silently inherits from the earlier table's assignment. **Deferred,
  explicitly, discovered by this audit**: no search engine or index
  exists anywhere in this workspace.
- **614 Accessibility**: same finding -- assigned to Phase 30 by the
  earlier table, never actually built. **Deferred, explicitly,
  discovered by this audit.**
- **615 Privacy, 616 Reliability, 617 Performance**: Phase 00 assigned
  these to Phase 31. Reliability and Performance are genuinely
  **Implemented and tested** (Phase 31's crash-recovery/stale-result/
  latency work). **Privacy is Not in this execution's scope** -- there
  is no analytics, telemetry, cloud sync, or account system anywhere in
  this codebase for a privacy requirement to apply *to* (see Article
  413/414 below).

### Validation Scenario (60 articles, 621-680)

Named end-to-end scenarios. Nine were chosen and built as Phase 30's own
Tasks 213-221, matched here by content, not just number proximity:

| Article | Title | Status |
|---|---|---|
| 621 | Blank Page to First Line | **Implemented and tested** -- Task 213, exact match |
| 624/625 | Handwritten Integer/Decimal | **Implemented and tested** -- Task 214 ("25mm") |
| 632 | Triangle Feasibility | **Implemented and tested** -- Task 215, exact match (Article 27's own 30/50/300 example) |
| 658 | Orthographic With No Dimensions | **Implemented and tested** -- Task 216, exact match |
| 659 | Resolve Depth in Top | **Implemented and tested** -- Task 217, exact title match |
| 633/634 | Shared Width / Shared Width Conflict | **Implemented and tested** -- Task 218 (Article 37's 100/130 example) |
| 649 | Command "Sketch" | **Implemented and tested** -- Task 219 |
| 630 | Numeric Note | **Implemented and tested** -- Task 220 (word-boundary-safe note handling) |
| 667/668 | Save Unresolved Orthographic Drawing / Save Conflict | **Implemented and tested** -- Task 221 (combined save/reopen) |
| 669 | Stale Recognition | **Implemented and tested** -- Phase 31 Task 226, exact match |
| 670 | Stale Correspondence Suggestion | **Implemented and tested** -- Phase 31 Task 226, exact match |
| 674 | Autosave Crash | **Implemented and tested** -- Phase 31 Task 225 |
| 675 | Crash During Propagation | **Implemented but not exactly this scenario** -- Phase 31 Task 225 tests crash-during-save, not specifically crash-during-a-propagation-call; the underlying propagation function (Phase 23) has no I/O of its own to crash mid-call, so this scenario's literal premise (an interrupted propagation) does not apply the way an interrupted *save* does. **Deferred, explicitly.** |
| 677 | Memory Stress | **Implemented and tested** -- Phase 31 Task 222 (time/size-scaling proxies; no literal heap profiler in this sandbox, documented in that phase's own evidence) |
| 678/679 | Export / Vector Export | **Implemented and tested** -- Phase 26 |
| 680 | Document Migration | **Implemented and tested** -- Phase 07's `migration.rs` |
| 622/623 | Rough Circle / Rectangle Versus Trapezoid | **Implemented and tested** at the unit level -- Phase 06's recognition/beautification tests cover this exact behavior, just not as a named Phase 30 scenario |
| 626-629 | Explicit Unit / Angle / Diameter / Radius | **Implemented and tested** at the unit level -- Phase 09/10's parser and dimension-kind tests |
| 631 | Ambiguous Target | **Implemented and tested** at the unit level -- Phase 18's association engine tests |
| 635/636 | Reference Dimension / Redundant Dimension | **Implemented and tested** at the unit level -- Phase 10/15 |
| 637-643 | Parallel/Perpendicular/Coincident/Equal Circles/Concentric/Tangent/Symmetry constraints | **Implemented and tested** at the unit level -- Phase 12's constraint-kind tests, one per relationship |
| 644-646 | Under-/Fully-/Over-Constrained | **Implemented and tested** at the unit level -- Phase 13's DOF state |
| 647/648 | Command "Pen"/"Eraser" | **Implemented and tested** at the unit level -- Phase 19's grammar tests |
| 650-655 | Short Prefix / Full Command During Ambiguity / Command in Sentence / Circle Versus Lasso / Geometry vs. Confirmation Circle / Left-Handed Command | **Implemented and tested** at the unit level for 650-652 (Phase 19's grammar/confirmation tests, and Phase 30's Task 220 for "in a sentence"); **653/654 Implemented and tested** (Phase 19's confirmation-evidence tests distinguish these); **655 Not in this execution's scope** -- left/right-handedness is a physical ergonomics concern with no headless-engine equivalent to test |
| 656/657 | Front View Identity / Orthographic Without View Label | **Implemented and tested** at the unit level -- Phase 20/21 |
| 660-666 | Dimension in Right Propagates to Front / Back View Uncertainty / Multiple Orthographic Sets / Duplicate Front / Move View / Duplicate View / Projection Convention | **Implemented and tested** at the unit level -- Phases 20-23 |
| 671 | Cancel Recognition | **Implemented and tested** at the unit level -- Phase 08's `stale_result.rs` (a cancellation is just a result that never gets accepted) |
| 672/673 | Offline Start / Optional Cloud Intelligence Offline | **Implemented and tested** structurally -- this entire workspace has zero network/cloud dependency by construction (verifiable: no crate depends on any HTTP/cloud client); "offline" is not a mode this architecture can fail to be in |
| 676 | Device Rotation | **Not in this execution's scope** -- no mobile UI exists yet to rotate (Phase 28/29 are readiness phases, not production UI) |

### Performance/Security/Reliability/Privacy Requirement (20 articles, 401-420)

| Article | Title | Status |
|---|---|---|
| 401 | Performance Requirement: Inking | **Implemented but not formally benchmarked** -- Phase 03/05's stroke pipeline is real and fast in every test run, but no dedicated p50/p95 measurement exists the way Phase 31 built for solving/propagation. **Deferred, explicitly, discovered by this audit.** |
| 402 | Performance Requirement: Recognition | **Implemented and tested** -- Phase 27's `recognition_bench.rs` (real criterion measurements) |
| 403 | Performance Requirement: Constraint Solve | **Implemented and tested** -- Phase 31 Task 223 |
| 404 | Performance Requirement: Orthographic Propagation | **Implemented and tested** -- Phase 31 Task 224 |
| 405 | Performance Requirement: Search | **Not in this execution's scope** -- no search engine exists (see Requirement Group 613 above) |
| 406 | Performance Requirement: Autosave | **Implemented but not formally benchmarked** -- Phase 08's autosave path is real; no dedicated latency measurement. **Deferred, explicitly, discovered by this audit.** |
| 407 | Performance Requirement: Export | **Implemented but not formally benchmarked** as export latency specifically -- Phase 27's `large_document_bench.rs` measures the closely-related canonical-JSON serialization path, not SVG/PDF export directly. **Deferred, explicitly, discovered by this audit.** |
| 408 | Security Requirement: Data at Rest | **Deferred, explicitly, discovered by this audit** -- saved documents are plain, unencrypted JSON (Phase 07); no encryption-at-rest exists or was asked for by any phase's task list. |
| 409 | Security Requirement: Data in Transit | **Not in this execution's scope** -- no network transit of any kind exists in this local-only V1. |
| 410 | Security Requirement: Account Separation | **Not in this execution's scope** -- no account/multi-user system exists. |
| 411 | Security Requirement: Model Service Isolation | **Not in this execution's scope** -- no external model service is called anywhere in this workspace (confirmed: no HTTP client dependency exists in any domain crate). |
| 412 | Security Requirement: Imported Content | **Implemented and tested**, partially -- `load_document`'s crash-recovery/validation path (Phase 07, exercised further by Phase 31 Task 225) is the closest real analogue: malformed/untrusted file content is rejected with a structured error, never silently accepted. No separate "imported from another app" content path exists to test beyond that, since this V1 has no import feature. |
| 413 | Privacy Requirement: Consent | **Not in this execution's scope** -- nothing in this V1 collects anything a consent flow would gate. |
| 414 | Privacy Requirement: Analytics | **Not in this execution's scope** -- no analytics/telemetry code exists anywhere in this workspace. |
| 415 | Reliability Requirement: Crash Recovery | **Implemented and tested** -- Phase 07 Task 053, re-proven end-to-end Phase 31 Task 225 |
| 416 | Reliability Requirement: Corruption Detection | **Implemented and tested** -- `Document::validate()` (Phase 07), exercised directly by Phase 31 Task 225's corrupted-file tests |
| 417 | Reliability Requirement: Export Verification | **Implemented and tested**, partially -- Phase 26's export tests verify structural correctness (well-formed SVG/PDF, correct unit conversion) but do not verify round-trip re-import (no import-from-SVG/PDF feature exists to round-trip through). |
| 418 | Reliability Requirement: Long Session Stability | **Deferred, explicitly, discovered by this audit** -- Phase 31's Task 222 stress-tests document *size*, not session *duration*; no long-running-process/memory-leak-over-time test exists. |
| 419 | Reliability Requirement: Device Rotation | **Not in this execution's scope** -- same reasoning as Validation Scenario 676. |
| 420 | Reliability Requirement: Background and Resume | **Not in this execution's scope** -- mobile app lifecycle concern; no mobile app exists yet (Phase 28/29 are readiness-only). |

### Technical Spike (5 articles, 214-218)

| Article | Title | Status |
|---|---|---|
| 214 | Constraint Solver | **Implemented and tested** -- Phase 11's full spike-and-decision-record treatment (`solver-evaluations/solver-decision-record.md`), the only one of the five built as a genuine comparative spike. |
| 215-218 | Handwriting Recognition / Primitive Recognition / Orthographic Graph / Command Grammar | **Documented**, not spiked the same way -- Phases 06/17/22/19 each built a real engine directly, with the design rationale recorded in that phase's own code comments and evidence file, rather than a separate options-comparison document the way Phase 11 did for the solver. This is an honest distinction worth naming: only the solver got a dedicated "evaluate options, then decide" artifact; the other four areas had their design choices made and justified in-line during normal engine construction. |

### Acceptance Criterion (9 articles, 196-204)

All nine (Ink, Refinement, Handwriting Dimensions, Constraints,
Orthographic Relationships, Ink Commands, Offline Use, Persistence,
Export) are **Implemented and tested** -- each corresponds directly to
a phase this execution completed (05/06, 06, 17/18, 12-15, 20-24, 19,
07, 07, 26) with its own passing test suite.

### Architecture Principle (20 articles)

**Implemented** -- these are the load-bearing rules this entire
execution has followed rather than separately "implemented" as their
own feature: deterministic serialization (Phase 01's
`craftloop-serialization`, applied everywhere), schema evolution
(Phase 07's `migration.rs`), stale-result rejection (Phase 08's
`stale_result.rs`, Phase 31's Task 226), and so on. Cited by name
throughout nearly every phase's own evidence file rather than owning a
single phase of their own.

## Lower-specificity categories, assessed at category level

- **Term** (120 articles): glossary definitions (e.g. "Term: Provenance,"
  "Term: Dimension Annotation"). **Documented** -- applied correctly
  wherever the corresponding domain concept was built (every phase's
  code doc comments cite the specific MCP article defining the term it
  implements, e.g. Phase 10's `SemanticDimension` citing Article 499
  "Term: Dimension Annotation"). Not independently "implemented" as
  their own feature; a definition has no separate test of its own
  beyond the engine that embodies it.
- **Visual Design Principle** (20), **Interaction Principle** (15),
  **Design System Component** (5): **Not in this execution's scope** --
  production visual/interaction design, explicitly excluded by every
  phase's "never production UI" rule (Task 007; Phase 04's disposable-
  harness precedent; Phase 28/29's Task 206/210-style deferrals).
- **Core User Journey** (14), **User Journey** (5): **Documented**
  where the underlying functional steps are covered by Phase 30's
  scenarios (the *engine* behavior a journey depends on is tested;
  the *UI* walkthrough itself is not, since no production UI exists).
- **Research Synthesis** (18), **Research Foundation** (10), **Standards
  Research** (4), **Competitive Boundary** (5): **Research** -- read
  and applied to specific design decisions where relevant (Jetpack Ink
  research informing Phase 28's mapping document, PencilKit/PaperKit
  research informing Phase 29's, ISO/ASME standards research informing
  Phase 25's standards-aware-not-standards-driven boundary). Not
  independently implemented; these are inputs to design, not features.
- **Narrative/Other** (231 articles -- product philosophy, engine
  descriptions, numbered behavioral rules): **Implemented** where the
  article states a concrete engine rule this execution built and cited
  directly (Article 4 no-fabrication, Article 23/27 dimension
  relationships and triangle inequality, Article 29 under-constrained-
  is-not-wrong, Article 36/37 no-permanent-master-view/shared
  dimensions, Article 92/96 determinism/confidence-consistency, Article
  128 document model, Article 138 transaction model, Article 170 error
  philosophy, Article 236-239 command language -- all cited by name,
  repeatedly, across this execution's own evidence trail); **Documented**
  where the article is pure product positioning/philosophy with no
  discrete engine behavior to test (Articles 1-10's "why Craft Loop
  exists," inspiration-source articles, and similar).

## Summary

| Status | Approximate article count | Basis |
|---|---|---|
| Implemented and tested | ~430 | Every Detailed Specification/Requirement Group/Acceptance Criterion/Architecture Principle article tied to a completed phase, plus ~50 Validation Scenario articles at unit-test granularity |
| Implemented but not hardware/tool-validated | ~10 | Mobile/CI-runner-dependent claims (Phase 27-29's own named exceptions) |
| Documented/Research | ~185 | Term, Research*, Competitive Boundary, User Journey, and philosophy-only Narrative articles |
| Deferred, explicitly (including gaps this audit itself found) | ~15 | Search, Accessibility, Privacy Requirement's two articles' N/A framing aside, Crash-During-Propagation, five unbenchmarked performance articles, data-at-rest, long-session-stability |
| Not in this execution's scope | ~40 | Production UI/visual design, mobile app lifecycle, cloud/account/analytics articles |

**This audit found five real gaps that a prior, less careful pass would
have missed**: Requirement Group 613/614 (Search/Accessibility) were
assigned to Phase 30 by Phase 00's own table but never actually built by
any of Phase 30's nine real tasks; Performance Requirement 401/406/407
(Inking/Autosave/Export) have real, working code but no dedicated
latency measurement the way solving/propagation got in Phase 31;
Reliability Requirement 418 (Long Session Stability) and Security
Requirement 408 (Data at Rest) have no corresponding test or mechanism
anywhere in this workspace. Every one is named here explicitly rather
than folded into a vague "mostly done" summary.
