# Requirement Traceability Index — Phase 00, Task 006

Recorded: 2026-09-13. Maintained incrementally: each phase/task closure should
add a row-level note (file + test) under the matching phase below rather than
re-deriving this table from scratch.

## How this index was built

`Craft Loop MCP V1.md` contains 680 numbered Articles. Reading all 680 into
every task's context would violate the Context Engineering Contract's
progressive-disclosure rule. Instead this index was built structurally, by
grepping article titles (cheap) rather than bodies (expensive), then matching
three well-organized sub-ranges onto the 33 Execution 01 phases:

- **Articles 1–220**: product constitution, engine narrative articles, UX/
  interaction articles, architecture/platform articles, Version 1 scope,
  acceptance criteria, testing, research spikes.
- **Articles 221–260**: `Detailed Specification of <Engine>` — one article per
  named engine, closely aligned to Engine Contracts 01–30.
- **Articles 581–620**: `Requirement Group: <Topic>` — a deliberately
  phase-shaped functional checklist (40 articles).
- **Articles 621–680**: `Validation Scenario: <Topic>` — end-to-end scenarios
  for the completion gate (Phase 32) and integration testing (Phase 30).
- **Articles 300–340 / 621–680**: edge cases and validation scenarios, to be
  pulled in as concrete test fixtures when the matching phase is implemented,
  not read upfront.

A task implementing phase N should read only: the Engine Contract(s) listed,
the Requirement Group article(s) listed, the Detailed Specification article(s)
listed, and any specific edge-case/validation-scenario articles named in its
own evidence notes — not the full MCP document.

## Phase → Article/Contract map

| Phase | Engine Contract(s) | Requirement Group (581–620) | Detailed Spec (221–260) | Core articles |
|---|---|---|---|---|
| 00 Repository Reconnaissance | — | 619 Product Boundaries | — | 121–124 (V1 scope/non-goals), 620 Human Authority |
| 01 Workspace & Core Skeleton | 01 | — | 240 Document Engine (schema shape only) | 88 Shared Engineering Core, 93 Engine Boundaries |
| 02 Geometry Mathematics | 03 | 587 Lines, 588 Circles, 589 Arcs | — | 95 Vector Geometry Engine, 130 Separation of Geometry/Presentation, 207 Numerical Tolerance |
| 03 Input Abstraction | 01 | 582 Pen Input | 241 Native Ink Boundary | 87 Cross-Platform Principle, 127 Device Capability Matrix |
| 04 Windows Harness | 27 | 581 Notebook Entry, 583 Navigation, 608 Toolbar | 244 Pen Tools, 246 Zoom/Pan, 247 Spatial Navigation | 89 Platform-Specific Presentation, Front End Skill "Windows Harness Frontend Rules" |
| 05 Raw Ink Model | 02 | 584 Ink Preservation | 242 Stroke Grouping, 243 Stroke Smoothing | 15 Raw Ink and Structured Geometry Must Coexist, 94 Ink Engine |
| 06 Recognition & Beautification | 04, 05 | 585 Primitive Recognition, 586 Refinement | 222 Primitive Recognition Engine, 223 Beautification Engine | 18 Primitive Recognition, 19 Beautification, 96 Recognition Confidence, 97 Human Correction as Signal |
| 07 Document Model & Persistence | 15 | 610 Persistence, 611 Offline | 240 Document Engine, 248 Page Frames | 14 Semantic Paper, 81 Local-First, 82 Autosave, 128 Document Model |
| 08 Undo/Redo/Transactions | 14 | 609 Undo | — | 79 Undo and Redo, 80 Event History and Authorship, 138 Interaction Transaction Model |
| 09 Units & Numeric Parsing | 07 | 591 Engineering Numeric Parsing | — | 72 Unit System |
| 10 Dimension Semantic Model | 09 | 593 Linear, 594 Angular, 595 Radius/Diameter | — | 21 Semantic Dimensions, 23 Dimensions Are Relationships, 24 Driving/Derived/Shared/Bounded |
| 11 Constraint Solver Spike | 10 | 596 Basic Constraints | 226 Constraint Engine (interface only) | 214 Technical Spike: Constraint Solver, 25 Constraint Engine |
| 12 Constraint Engine Integration | 10 | 596 Basic Constraints, 597 Constraint Suggestions | 226 Constraint Engine, 258 Dragging Constrained Geometry, 259 Snap vs Constraint, 260 Auto Constraint Suggestions | 25 Constraint Engine, 98 Constraint Solver, 99 Solver Feedback |
| 13 Constraint State & Feasible Geometry | 11 | — | 229 Constraint State Engine | 28 Constraint State Engine, 29 Under-Constrained ≠ Wrong, 136 Feasible Ranges |
| 14 Geometric Consistency Engine | 12 | 598 Geometric Conflict | 227 Geometric Consistency Engine | 27 Geometric Consistency Engine, 134 Conflict Model, 135 Conflict Resolution |
| 15 Design Intent Graph | 13 | — | 228 Design Intent Graph | 26 Design Intent |
| 16 Ink Intent Classification | — (feeds 04/05) | — | 221 Ink Intent Engine | 17 Ink Intent Classification |
| 17 Handwriting Adapter Boundary | 06 | 590 Handwriting | 224 Engineering Handwriting Parser | 20 Engineering Handwriting |
| 18 Dimension Association Engine | 08 | 592 Dimension Association | 225 Dimension Association Engine | 22 Dimension Association |
| 19 Ink Command Language | 16, 17 | 606 Ink Commands, 607 Command Safety | 236 Ink Command Engine, 237 Contextual Command Grammar, 238 Confirmation Gesture, 239 Command Bus | 45–58 (command/gesture articles) |
| 20 View Blocks & Orthographic Sets | 18, 19 | 599 View Identity, 601 Orthographic Set | — | 30 Orthographic View Identity, 66 View Blocks, 327 Orthographic Set |
| 21 Projection Convention & Readiness | 20 | 600 Orthographic Entry, 605 Orthographic Layout | 234 Orthographic Readiness Engine | 32 Entering Orthographic Mode, 33 Orthographic Readiness, 42 First/Third-Angle |
| 22 Multiview Constraint Graph | 21 | 602 Shared Dimensions | 231 Multiview Constraint Graph | 35 Multiview Constraint Graph, 37 Shared Dimensions, 38 Unresolved Dimensions |
| 23 Orthographic Relationship Engine | 22 | — | 230 Orthographic Relationship Engine | 34 Orthographic Intelligence Is Not an Image Generator |
| 24 Cross-View Correspondence & Ambiguity | 23, 24 | 603 Unresolved Values, 604 Cross-View Correspondence | 232 Cross-View Correspondence Engine, 233 Ambiguity Engine | 39 Ambiguity, 40 Cross-View Correspondence, 41 Back and Hidden Information |
| 25 Standards-Aware Representation | 25 | — | 235 Standards Engine | 43 Standards-Aware Not Standards-Driven, 279–282 Standards Research |
| 26 Export & Diagnostic Interop | 26 | 612 Export | — | 151–154 Export/PDF/SVG/DXF |
| 27 Quality Tooling & CI | — | — | — | 205 Testing Pyramid, 206 Geometry Property Testing, 208 Gesture Testing, 209 Orthographic Testing |
| 28 Android Adapter Readiness | 28 | 618 Platform Consistency (Android half) | — | 126 Android Tablets as First-Class Platform, 264 Jetpack Ink |
| 29 iPad Adapter Readiness | 29 | 618 Platform Consistency (iPad half) | — | 125 iPad as Reference Platform, 262 PencilKit, 263 PaperKit |
| 30 End-to-End Engine Integration | 30 | 613 Search, 614 Accessibility | — | 621–680 Validation Scenarios (pull specific ones as integration fixtures) |
| 31 Performance, Reliability, Hardening | — | 615 Privacy, 616 Reliability, 617 Performance | — | 91 Performance Budget, 401–420 Performance/Security/Reliability Requirements |
| 32 Completion Gate | — | 619, 620 (re-verify) | — | 121 V1 Product Goal, 220 Definition of a Usable Version 1, 196–204 Acceptance Criteria |

## Engine Contract quick index (from Execution 01, lines 434–1001)

01 Normalized Input · 02 Raw Ink · 03 Geometry Kernel · 04 Recognition ·
05 Beautification · 06 Handwriting Adapter · 07 Engineering Parser ·
08 Dimension Association · 09 Semantic Dimensions · 10 Constraint Solver ·
11 Constraint State · 12 Consistency · 13 Design Intent · 14 Transactions ·
15 Document · 16 Command Bus · 17 Ink Commands · 18 View Blocks ·
19 Orthographic Sets · 20 Projection Convention · 21 Multiview Graph ·
22 Orthographic Engine · 23 Correspondence · 24 Ambiguity · 25 Standards ·
26 Export · 27 Windows Harness · 28 Android Adapter · 29 iPad Adapter ·
30 Machine Intelligence Seam.

## Per-phase evidence log

Populated as each phase closes. See `execution-evidence/test-reports/` for
command output and `execution-evidence/architecture-decisions/` for ADRs.

### Phase 00 — closed 2026-09-13
- Evidence: this file; `00-repository-evidence-map.md`; `01-immutable-inputs.md`
  (+ `.sha256`); `02-baseline-commands.md`.
- Tests: pre-existing `windows-simulator` suite re-run as baseline (5/5 pass,
  see `02-baseline-commands.md`). No new domain code in this phase, so no new
  domain test was required — Task 001–006 are documentation/process tasks.
- Branch: `execution-01/phase-00` created per Task 003 (never committed
  directly to `main`).

### Phase 01 — closed 2026-09-13
- Evidence: `03-phase-01-workspace.md`; `test-reports/phase-01-cargo-test.txt`;
  `test-reports/phase-01-cargo-clippy.txt`.
- Code: `Cargo.toml` (workspace), `crates/craftloop-ids`,
  `crates/craftloop-errors`, `crates/craftloop-serialization`,
  `crates/craftloop-transactions`, `crates/craftloop-test-support`,
  `apps/README.md` (placeholder boundary doc).
- Tests: 34/34 passing across 5 crates (`cargo test --workspace`).
- Lint/format: `cargo fmt --all -- --check` and
  `cargo clippy --workspace --all-targets -- -D warnings` both clean after
  one real clippy finding was fixed (`should_implement_trait` on
  `DeterministicIdSequence::next`, renamed to `next_id`).

### Phase 02 — closed 2026-09-13
- Evidence: `04-phase-02-geometry.md`; `test-reports/phase-02-cargo-test.txt`;
  `test-reports/phase-02-cargo-clippy.txt`.
- Code: `crates/craftloop-geometry` (`point.rs`, `segment.rs`, `circle.rs`,
  `arc.rs`, `ellipse.rs`, `rectangle.rs`, `tolerance.rs`, `bounds.rs`,
  `tests/property_tests.rs`).
- Tests: 107/107 passing workspace-wide (59 geometry unit + 14 property +
  34 from Phase 01). One real bug (sign error in
  `Circle2::intersect_segment`) caught by tests and fixed in-loop.
- Lint/format: both clean.

### Phase 03 — closed 2026-09-13
- Evidence: `05-phase-03-input.md`; `test-reports/phase-03-cargo-test.txt`;
  `test-reports/phase-03-cargo-clippy.txt`.
- Code: `crates/craftloop-input` (`sample.rs`, `capabilities.rs`,
  `mouse_simulator.rs`, `lifecycle.rs`, `trace.rs`, `disclaimer.rs`); added
  `DomainError::Input`/`InputErrorKind` to `craftloop-errors`.
- Tests: 139/139 passing workspace-wide (32 new). No bugs found on first
  run for this phase's own code.
- Lint/format: both clean (1 clippy finding fixed: `clone_on_copy`).

### Phase 04 — closed 2026-09-13
- Evidence: `06-phase-04-windows-harness.md`;
  `test-reports/phase-04-cargo-test.txt`;
  `test-reports/phase-04-cargo-clippy.txt`;
  `test-reports/phase-04-harness-launch.log`.
- Code: `apps/windows-harness` (`main.rs`, `app.rs`, `viewport.rs`,
  `tool.rs`, `state.rs`, `scenario.rs`, `diagnostic_export.rs`,
  `examples/generate_scenario.rs`, `scenarios/example-line.json`).
- Tests: 165/165 passing workspace-wide (26 new). App launches without
  panicking under a timeout; interactive/visual verification explicitly
  deferred to a human (no display in this environment).
- Lint/format: both clean.

### Phase 05 — closed 2026-09-13
- Evidence: `07-phase-05-raw-ink.md`; `test-reports/phase-05-cargo-test.txt`;
  `test-reports/phase-05-cargo-clippy.txt`.
- Code: `crates/craftloop-ink` (`stroke.rs`, `grouping.rs`, `smoothing.rs`,
  `spatial.rs`, `provenance.rs`, `tests/replay_and_serialization.rs`); added
  `DomainError::Ink`/`InkErrorKind` to `craftloop-errors`.
- Tests: 198/198 passing workspace-wide (33 new). 2 real production bugs
  (endpoint preservation, symmetric-window averaging in `smoothing.rs`) and
  1 test-construction bug caught and fixed in-loop.
- Lint/format: both clean (1 clippy finding fixed: `len_without_is_empty`).

### Phase 06 — closed 2026-09-13
- Evidence: `08-phase-06-recognition.md`;
  `test-reports/phase-06-cargo-test.txt`;
  `test-reports/phase-06-cargo-clippy.txt`.
- Code: `crates/craftloop-recognition` (`fit/{line,circle,arc,rectangle}.rs`,
  `candidate.rs`, `beautify.rs`, `rejection_memory.rs`, `confidence.rs`,
  `tests/false_positive_benchmark.rs`).
- Tests: 243/243 passing workspace-wide (45 new). 4 real issues found and
  fixed: dead-code displacement() bug, arc-ambiguity heuristic redesign
  (Kåsa fit instability on short arcs), Arc/Circle ranking flakiness on a
  full circle, and confidence-scale miscalibration caught by the Task 047
  benchmark itself and corrected (0.25 -> 0.1).
- Lint/format: both clean (1 clippy finding fixed: `clone_on_copy`).

### Phase 07 — closed 2026-09-13
- Evidence: `09-phase-07-document.md`;
  `test-reports/phase-07-cargo-test.txt`;
  `test-reports/phase-07-cargo-clippy.txt`.
- Code: `crates/craftloop-document` (`document.rs`, `page.rs`, `entity.rs`,
  `note.rs`, `metadata.rs`, `units.rs`, `page_layout.rs`, `persistence.rs`,
  `migration.rs`, `autosave.rs`, `tests/save_reopen_golden.rs`); added
  `NoteId`/`PageId` to `craftloop-ids`, `DomainError::Document` to
  `craftloop-errors`.
- Tests: 286/286 passing workspace-wide (43 new). One significant bug
  (EntityId's derived enum serde broke `BTreeMap<EntityId, _>`
  serialization entirely) caught by the Task 055 golden test and fixed with
  a hand-written string-based Serialize/Deserialize.
- Lint/format: both clean (2 clippy findings fixed:
  `wrong_self_convention`, `derivable_impls`).

### Phase 08 — closed 2026-09-13
- Evidence: `10-phase-08-transactions.md`;
  `test-reports/phase-08-cargo-test.txt`;
  `test-reports/phase-08-cargo-clippy.txt`.
- Code: `crates/craftloop-document/src/{history,provenance,stale_result}.rs`
  (new), `document.rs` extended with revision/provenance,
  `tests/crash_safe_transactions.rs`.
- Tests: 342/342 passing workspace-wide (56 new). One design bug (fabricated
  UserCreated default on provenance-undo-to-None) caught and fixed before
  first compile.
- Lint/format: both clean, no clippy findings.

### Phase 09 — closed 2026-09-13
- Evidence: `11-phase-09-units.md`; `test-reports/phase-09-cargo-test.txt`;
  `test-reports/phase-09-cargo-clippy.txt`.
- Code: `crates/craftloop-units` (`length_unit.rs`, `numeric.rs`,
  `length.rs`, `angle.rs`, `radius_diameter.rs`,
  `tests/invalid_numeric_diagnostics.rs`); refactored
  `craftloop-document/src/units.rs` to re-export `LengthUnit`.
- Tests: 385/385 passing workspace-wide (43 new). 2 real bugs (auto-locale
  repeated-separator misclassification caught in design review;
  raw-input-truncation-after-suffix-stripping caught by the Task 069
  diagnostic test itself) fixed in-loop.
- Lint/format: both clean (1 clippy finding fixed: `derivable_impls`).

### Phase 10 — closed 2026-09-13
- Evidence: `12-phase-10-dimension.md`; `test-reports/phase-10-cargo-test.txt`;
  `test-reports/phase-10-cargo-clippy.txt`.
- Code: `crates/craftloop-dimension` (`kind.rs`, `role.rs`, `target.rs`,
  `dimension.rs`, `annotation.rs`, `store.rs`, `tests/serialization.rs`);
  added `DimensionAnnotationId` to `craftloop-ids`,
  `DomainError::Dimension` to `craftloop-errors`; added
  `SemanticEntity::Dimension` to `craftloop-document`.
- Tests: 378/378 passing workspace-wide (30 new). All passed on first run
  this phase.
- Lint/format: both clean, no clippy findings.

### Phase 11 — closed 2026-09-13
- Evidence: `13-phase-11-solver-spike.md`;
  `test-reports/phase-11-cargo-test.txt`;
  `test-reports/phase-11-cargo-clippy.txt`;
  `solver-evaluations/solver-decision-record.md`;
  `solver-evaluations/ezpz-spike-output.txt`;
  `solver-evaluations/naive-baseline-output.txt`.
- Code: `crates/craftloop-constraint` (`variable.rs`, `constraint.rs`,
  `residual.rs`, `result.rs`, `solver.rs`,
  `examples/naive_gradient_descent_baseline.rs`);
  `execution-evidence/solver-evaluations/ezpz-spike/` (standalone spike
  project, not a workspace member).
- Decision: adopt `ezpz` v0.2.29 (MIT, KittyCAD/ezpz), pinned exactly, for
  Phase 12 integration. Full rationale, license notes, and fallback plan in
  the decision record.
- Tests: 401/401 passing workspace-wide (23 new in craftloop-constraint).
  Real spike programs built and run (not simulated); one genuine spike
  finding (a sign-convention bug in the spike's own test data, which ezpz
  correctly flagged) investigated and corrected.
- Lint/format: both clean, no clippy findings.

### Phase 12 — closed 2026-09-13
- Evidence: `14-phase-12-constraint-engine.md`;
  `test-reports/phase-12-cargo-test.txt`;
  `test-reports/phase-12-cargo-clippy.txt`;
  `test-reports/phase-12-cargo-fmt.txt`;
  `solver-evaluations/solver-decision-record.md` (Tasks 089/090 addendum).
- Spike extension: `solver-evaluations/ezpz-spike/src/main.rs` gained real
  `LineTangentToCircle`/`CircleTangentToCircle`/`Symmetric` scenarios,
  all converging in 1 iteration -- Tasks 089/090 approved unconditionally
  before any production code was written.
- Code: `crates/craftloop-constraint` gained 4 `GeometricConstraint`
  variants (`EqualLength`, `LineTangentToCircle`, `CircleTangentToCircle`,
  `Symmetric`); `crates/craftloop-errors` gained `SketchErrorKind`; new
  crate `crates/craftloop-sketch` (`point_ref.rs`, `constraint_kind.rs`,
  `provenance.rs`, `ezpz_adapter.rs`, `sketch.rs`) -- the real `ezpz`
  backend binding plus the domain layer over
  `craftloop_recognition::BeautifiedPrimitive` geometry.
- Tests: 450/450 passing workspace-wide (49 new). One genuine RED-GREEN
  finding while writing Task 092's invalid/conflict tests: an assumed
  contradiction (`Horizontal` + `Vertical` on the same line) turned out to
  be satisfiable by collapsing the line to a point -- the failing test was
  corrected to assert the true property, and a real contradiction
  (two different `Fixed` values on one point) was used for the
  conflict-case test instead.
- Lint/format: both clean, no clippy findings.

### Phase 13 — closed 2026-09-13
- Evidence: `15-phase-13-constraint-state.md`;
  `test-reports/phase-13-cargo-test.txt`;
  `test-reports/phase-13-cargo-clippy.txt`;
  `test-reports/phase-13-cargo-fmt.txt`.
- Code: `crates/craftloop-constraint/src/solver.rs` gained
  `ConstraintSolver::underconstrained_variables` (default-empty); new
  `crates/craftloop-sketch/src/state.rs` (`DegreesOfFreedomState`,
  `Sketch::degrees_of_freedom`); `constraint_kind.rs` gained
  `SketchConstraintKind::is_equivalent`; `sketch.rs`'s `add_constraint` now
  returns `ConstraintOutcome` (`Added`/`Redundant`); new
  `crates/craftloop-dimension/src/feasible_range.rs`
  (`TriangleSideRange`); `role.rs` gained `DimensionRole::explain`.
- Tests: 474/474 passing workspace-wide (24 new). One genuine finding: an
  `underconstrained_variables` test assumed both coordinates of a
  distance-constrained point would be free, but only one was -- correct
  behavior (freedom analysis is a local/linearized judgment; the point sat
  at a degenerate axis-aligned initial guess). Fixed the test's initial
  guess, not the code.
- Lint/format: both clean (1 clippy finding fixed: `type_complexity`).

### Phase 14 — closed 2026-09-13
- Evidence: `16-phase-14-consistency-engine.md`;
  `test-reports/phase-14-cargo-test.txt`;
  `test-reports/phase-14-cargo-clippy.txt`;
  `test-reports/phase-14-cargo-fmt.txt`.
- Code: new crate `crates/craftloop-consistency` (`conflict.rs`,
  `geometry_validation.rs`, `dimension_validation.rs`,
  `unit_validation.rs`, `resolution.rs`); `craftloop-errors` gained two
  `ConsistencyErrorKind` variants (`AlreadyResolved`, `ChoiceNotOffered`)
  -- the first real use of `DomainError::Consistency` anywhere in the
  workspace (defined Phase 01, unused until now); `craftloop-document`
  gained `SemanticEntity::Conflict`/`EntityId::Conflict` plus
  `tests/conflict_persistence.rs`.
- Tests: 500/500 passing workspace-wide (26 new). No RED-GREEN surprises
  this phase -- every validator's first test run matched its design.
- Lint/format: both clean, no clippy findings.

### Phase 15 — closed 2026-09-13
- Evidence: `17-phase-15-design-intent.md`;
  `test-reports/phase-15-cargo-test.txt`;
  `test-reports/phase-15-cargo-clippy.txt`;
  `test-reports/phase-15-cargo-fmt.txt`.
- Design decision: no new "intent relation" type -- `Sketch`'s existing
  constraint store (`SketchConstraintKind` + `ConstraintProvenance`,
  Phase 12) already *is* the design-intent graph; Phase 15 adds a
  query/curation layer on top of it (`crates/craftloop-sketch/src/
  intent.rs`), not a parallel model.
- Code: `Sketch` gained `rejected_suggestions`; new methods
  `constraints_touching`, `affected_primitives`, `accept_suggestion`,
  `reject_suggestion`, `is_suggestion_rejected`,
  `observed_length_coincidences`.
- Tests: 514/514 passing workspace-wide (14 new). No RED-GREEN surprises.
- Lint/format: both clean, no clippy findings.
