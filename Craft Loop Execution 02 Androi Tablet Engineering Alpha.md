# Craft Loop Execution 02 — Android Tablet Engineering Alpha

## Document Metadata

**Project:** Craft Loop  
**Execution:** 02  
**Execution name:** Android Tablet Engineering Alpha  
**Document type:** Execution specification for Claude Code, Codex CLI, and equivalent coding agents  
**Version:** 1.0  
**Created:** 2026-09-14  
**Primary development host:** Windows  
**Primary physical validation device:** Android tablet connected by USB, with Samsung tablet and S Pen as the first real-device target  
**Primary production target in this execution:** Android tablets  
**Deferred but protected production target:** iPad tablets  
**Shared-core policy:** Universal application architecture; Android is the first production adapter, not the source of engineering truth  
**UI/UX maturity target:** Functional engineering test interface only; icon-first toolbar; no final visual design  
**Success artifact:** A debug APK that can be built on Windows, installed over USB, launched on the Samsung tablet, used with the S Pen, and exercised through the complete Engineering Alpha workflow  
**Source product specification:** `Craft Loop MCP V1.md`  
**Source implementation baseline:** Craft Loop repository after Execution 01  
**Primary repository:** `mohammed09001/craftloop`  
**Execution authority:** This document governs Execution 02. Execution 01 remains historical evidence and shared-core foundation. MCP V1 remains product authority.

---

# Article 1 — Execution Purpose

Execution 02 exists to close one specific gap: Craft Loop already has a substantial shared engineering core, but the Android application is still only a placeholder shell.

The repository currently contains a Kotlin/Compose Android project, but that project does not yet expose a real drawing surface, does not use Jetpack Ink, does not package a validated Android-native Rust library as part of a reproducible build, and does not expose enough of the Rust core through the mobile boundary to operate the product.

Execution 02 must transform that placeholder into an **Android Tablet Engineering Alpha**.

The result is not the final Craft Loop product.

The result is the first physical version of Craft Loop that the developer can hold, draw on with an S Pen, and use to validate the existing engineering engines in a real tablet environment.

The application must remain intentionally simple in appearance.

The execution budget belongs to engineering integration, stylus input, semantic state, reliability, and device validation.

Final visual identity, production onboarding, pricing, account systems, collaboration, cloud synchronization, and iPad implementation are outside this execution.

---

# Article 2 — Goal Mode

## Goal Mode Definition

Goal Mode defines the end state that the agent must continuously optimize toward.

The goal is not to complete a collection of disconnected Android tasks.

The goal is to produce one coherent usable Android Engineering Alpha.

The final Alpha must allow the developer to perform this physical workflow on the Samsung tablet:

1. Connect the Android tablet to the Windows workstation by USB.
2. Verify the tablet through Android Debug Bridge.
3. Build Craft Loop from the repository using a reproducible command.
4. Install the debug APK to the tablet.
5. Launch Craft Loop.
6. Draw with the S Pen.
7. See the ink appear immediately with low-latency Android-native rendering.
8. Complete the stroke and pass normalized stroke data into the shared Rust core.
9. Convert eligible strokes into structured geometry or keep them as raw ink.
10. Select structured geometry.
11. Create and edit a semantic dimension.
12. Apply at least the core geometric constraints required for the Alpha.
13. See invalid geometry rejected through a structured conflict rather than silent mutation.
14. Assign `FRONT` as a semantic View Identity.
15. Enter Orthographic mode.
16. Create or reveal linked `TOP` and `RIGHT` View Blocks.
17. Preserve unresolved information instead of fabricating missing geometry.
18. Enter a missing depth value in an appropriate view.
19. Propagate that shared value across linked views.
20. Deliberately enter a contradictory shared value.
21. See a conflict.
22. Resolve or cancel the conflict.
23. Undo and redo meaningful actions.
24. Save the document.
25. Kill the application process.
26. Relaunch the application.
27. Reopen the document.
28. Verify that geometry, dimensions, constraints, View Identity, Orthographic relationships, unresolved state, and conflict state are restored correctly.

The application must also collect and expose real Android stylus capability evidence:

- stylus source;
- pressure when the device reports it;
- tilt when the device reports it;
- hover capability when the device reports it;
- palm/finger rejection behavior;
- S Pen barrel-button state where available;
- physical input latency observations.

The Alpha must not claim support for a stylus capability that was not actually observed on the connected device.

## Goal Mode Completion Statement

Execution 02 is complete only when the developer can say:

> “I connected my Samsung tablet to Windows by USB, built Craft Loop, installed it, drew with the S Pen, used the core engineering workflow through Orthographic resolution, saved it, reopened it, and the same shared Rust engineering model behaved correctly on the physical Android tablet.”

Compilation alone is not Goal Mode completion.

A launch screen alone is not Goal Mode completion.

A Jetpack Ink demo alone is not Goal Mode completion.

A Rust FFI smoke test alone is not Goal Mode completion.

The complete physical workflow is the Goal.

---

# Article 3 — Plan Mode

## Plan Mode Definition

Plan Mode is the execution discipline used before and during every phase.

Plan Mode must prevent the coding agent from reacting to the repository as though it were a blank Android project.

Before every substantial modification, the agent must answer:

**What already exists?**

**Which shared-core engine owns this behavior?**

**Which Android adapter is actually missing?**

**Which FFI surface is required?**

**Which user-visible workflow will prove the change?**

**Which test will fail before the change and pass after it?**

**Does this change preserve iPad portability?**

Plan Mode does not mean stopping after every task to ask the human for approval.

Plan Mode is internal reasoning converted into concise repository evidence.

At each phase boundary the agent must inspect current state, compare it to the phase outcome, identify the smallest missing vertical slice, update the plan if repository evidence invalidates an assumption, execute, verify, record evidence, and continue automatically.

Do not implement a separate approval harness that pauses after every phase.

Do not ask “shall I continue?” after an internal gate.

Pause only for a true blocker defined in this document.

---

# Article 4 — Universal Application Doctrine

Craft Loop is a universal application even though Execution 02 focuses exclusively on Android tablets.

“Universal” in this project does not mean one user-interface codebase.

It means one engineering truth.

The architecture must remain:

```text
                         Shared Craft Loop Core
                                  │
                  ┌───────────────┴───────────────┐
                  │                               │
            Android Adapter                  iPad Adapter
        Kotlin / Compose / Ink          Swift / UIKit / SwiftUI
             Jetpack Ink                    PencilKit
                  │                               │
          Android Tablet                     iPad Tablet
```

Android must not become the location where dimensions, constraints, Orthographic truth, document semantics, or conflict rules are reimplemented.

Kotlin owns Android interaction.

Rust owns shared engineering truth.

A future Swift/iPad adapter must be able to call the same semantic operations without reverse engineering Kotlin behavior.

Every Android-domain decision must therefore pass one question:

> “Could the future iPad adapter perform the same semantic operation without knowing anything about Jetpack Compose or Jetpack Ink?”

If the answer is no, move the behavior into the shared core or into a platform-neutral FFI contract.

---

# Article 5 — Repository Starting Point

Execution 02 starts from real repository evidence.

The current Android project is a placeholder shell.

Its existing README explicitly says that it is not a real drawing surface, not Jetpack Ink integration, not a design system, and was not build-validated in the original execution environment.

The current Compose activity performs one representative `resolveCommand("pen")` call and renders a text line.

The current `craftloop-mobile-ffi` boundary intentionally exposes only representative input and command functions rather than the full product session.

The current Android Gradle configuration already establishes useful baseline facts:

- package namespace exists;
- Compose is enabled;
- Java 17 / JVM 17 are configured;
- minimum Android API is tablet-oriented;
- JNA is present for the generated UniFFI bindings.

Execution 02 must preserve useful baseline work while replacing placeholder behavior with real functionality.

Do not delete shared-core code merely because the Android application does not expose it yet.

Do not rewrite Execution 01 engines inside Kotlin.

---

# Article 6 — Execution 02 Non-Goals

Execution 02 does not implement final Craft Loop UI/UX.

Execution 02 does not implement the iPad application.

Execution 02 does not implement cloud synchronization, user accounts, subscription billing, collaboration, web application, three-dimensional reconstruction, three-dimensional modeling, autonomous design, full geometric dimensioning and tolerancing, Play Store release signing, or final branding.

Execution 02 does not expand the shared engine merely to make the Alpha visually impressive.

---

# Article 7 — Android Engineering Alpha UI Contract

The Alpha user interface must be intentionally simple.

The canvas is the primary surface.

The toolbar must be icon-first.

The toolbar must not display persistent text labels for normal tools.

Every icon must have an accessibility `contentDescription`.

A long press or optional tooltip may expose the tool name.

The toolbar must remain understandable without becoming final product design.

## Required Primary Icons

- Pen.
- Eraser.
- Select.
- Line.
- Circle.
- Rectangle.
- Dimension.
- Constraint.
- View Identity.
- Orthographic.
- Undo.
- Redo.
- Save.

A secondary overflow or compact panel may contain New, Open, Delete, Fit View, Debug Inspector, Input Capability Inspector, and Alpha-only settings.

Prefer simple vector icons.

Use Material Symbols or local vector assets only when semantics are clear.

Do not use persistent labels such as `LINE`, `CIRCLE`, or `DIMENSION` in the primary toolbar.

The icon system must be replaceable later without touching the engineering core.

---

# Article 8 — Alpha Screen Structure

The main Android Alpha screen contains four conceptual regions.

## Canvas Region

Most of the screen.

It renders live raw ink immediately through Android-native ink infrastructure.

It also renders structured geometry returned by the shared core, dimensions, selection state, View Blocks, ghost/unresolved state, and conflicts.

## Tool Region

A compact icon toolbar.

Landscape is the primary test orientation unless repository/device evidence supports both orientations cleanly.

## Context Region

A small popover or bottom sheet used only when a tool requires options: constraints, View Identity, numeric dimension, conflict resolution, or diagnostics.

## Debug Region

A collapsible Alpha-only diagnostic overlay showing active tool, pointer source, pressure, tilt, hover capability, selected entity IDs, current View Identity, active Orthographic Set, last Rust transaction, last conflict, current document state, and input timing.

It is not production UI.

---

# Article 9 — Immediate Ink Architecture

The S Pen must not wait for a round trip through UniFFI and Rust before a visible stroke appears.

The architecture is:

```text
S Pen / MotionEvent
       │
       ├──────────────► Jetpack Ink live in-progress rendering
       │
       └──────────────► Android stroke collector
                               │
                               ▼
                     completed normalized stroke
                               │
                               ▼
                     UniFFI / CraftLoopSession
                               │
                               ▼
                    shared Rust interpretation
                               │
                               ▼
                     semantic scene snapshot
                               │
                               ▼
                  Compose structured rendering
```

Live visual ink belongs to Android.

Committed engineering meaning belongs to Rust.

Do not call a heavyweight FFI operation on every sample merely to draw the stroke.

Batch completed strokes.

Introduce incremental semantic updates later only if evidence requires them.

---

# Article 10 — Jetpack Ink Baseline

Use Jetpack Ink as the Android inking foundation.

Prefer the stable 1.0 release line for Execution 02 unless a specific physical-device defect requires a documented move to the 1.1 alpha line.

Do not use an alpha simply because it is newer.

Verify the exact modules before adding dependencies.

Jetpack Ink is an Android adapter and renderer.

It must not become the Craft Loop semantic document model.

---

# Article 11 — S Pen and Touch Interaction Policy

Default interaction:

```text
S Pen  → drawing, writing, engineering input
Finger → pan, zoom, interface controls
```

Finger drawing may be an optional Alpha setting.

Palm and finger touches during active stylus drawing must not create engineering strokes by default.

Pressure and tilt are captured only when reported.

Hover is optional.

S Pen barrel-button information may be diagnostic-only.

---

# Article 12 — CraftLoopSession Mobile API

Execution 02 must evolve the current small FFI surface into a session-oriented API.

Introduce `CraftLoopSession` or an equivalent repository-consistent abstraction.

The session owns or references active semantic document state.

Minimum capabilities:

- create/open/save document;
- get scene snapshot;
- submit completed stroke;
- keep/accept/reject recognition result;
- direct Line/Circle/Rectangle creation for explicit tools;
- select/clear/delete;
- create/edit semantic dimension;
- apply/remove supported constraint;
- assign View Identity;
- enter Orthographic;
- create/query Orthographic Set;
- resolve shared dimension;
- link/unlink where required;
- resolve/cancel conflict;
- undo/redo;
- query debug state.

Kotlin requests operations.

Rust validates and commits or rejects them.

Android renders resulting semantic state.

---

# Article 13 — Scene Snapshot Contract

The Android renderer needs an FFI-safe scene snapshot.

The first Alpha may use coarse snapshots.

Optimize to diffs later only if measurement justifies it.

Snapshot content should include raw committed ink, structured primitives, selection, dimension annotations, View Blocks, visible View labels, confirmed/suggested/unresolved geometry, conflicts, projection guides, units, undo/redo availability, stable IDs, and active conflict state.

The snapshot is presentation data.

Rust remains authoritative.

---

# Article 14 — Dimension Alpha Workflow

Handwriting recognition must not block the first physical Dimension test.

First flow:

1. Activate Dimension icon.
2. Select eligible geometry.
3. Show compact numeric input.
4. Enter value.
5. Commit through `CraftLoopSession`.
6. Rust validates.
7. Geometry updates.
8. Render semantic annotation.
9. Invalid value produces structured conflict.
10. Undo works.

Typed input is an intentional Alpha fallback.

---

# Article 15 — Handwriting Recognition Gate

After typed dimensions pass on the physical tablet, add Android handwriting recognition.

Google ML Kit Digital Ink Recognition is the preferred Android candidate for this execution.

Pipeline:

```text
S Pen strokes
    ↓
Digital Ink Recognition
    ↓
text candidates
    ↓
Engineering Handwriting Parser
    ↓
Dimension Association
    ↓
Constraint / Consistency validation
    ↓
user-confirmed semantic state
```

Recognition never mutates geometry directly.

Model download state must be explicit.

English engineering commands can be first.

Architecture must remain compatible with future Arabic and mixed-language notes.

---

# Article 16 — Ink Command Gate

After physical drawing, dimensions, constraints, Orthographic mode, persistence, and Undo are stable, connect handwriting recognition to the existing Command Engine.

Required Alpha path:

```text
write command
    ↓
confirmation circle
    ↓
recognition
    ↓
existing Rust grammar
    ↓
temporal confirmation
    ↓
shared Command Bus
    ↓
toolbar state
```

Plain notes containing a command word must not execute.

A geometry circle must not become command confirmation.

A lasso around older content must not become command confirmation.

---

# Article 17 — Constraint Alpha Workflow

Constraint icon opens a compact icon/grid popover.

Prioritize stable existing core constraints: Coincident, Horizontal, Vertical, Parallel, Perpendicular, Equal, and Concentric.

Add Tangent/Symmetry only if the existing engine already supports them reliably.

Do not expand solver scope merely to fill the toolbar.

Unsupported constraints are absent or disabled.

---

# Article 18 — View Identity and Orthographic Workflow

The View icon assigns Front, Top, Right, or Back.

Semantic identity is stored in Rust.

Orthographic icon enters the existing shared workflow.

Full dimensioning is not required.

Known information propagates.

Missing depth remains unresolved.

User can enter depth in Top or Right.

Shared values propagate.

Back-specific unknown features are never invented.

This is the central Alpha workflow.

---

# Article 19 — Android Build Reproducibility

The repository must support a clean Windows Android build.

Execution 02 must produce or validate:

- Gradle Wrapper;
- `gradlew` and `gradlew.bat`;
- pinned Android Gradle Plugin/Kotlin policy;
- SDK/NDK policy;
- Rust Android target;
- UniFFI Kotlin generation;
- `jniLibs` generation/packaging;
- ARM64 native library;
- debug APK;
- deterministic install instructions.

Do not depend on manually copying stale `.so` files or generated bindings from evidence folders.

---

# Article 20 — Rust Android Build Policy

First physical ABI: `arm64-v8a`.

Rust target: `aarch64-linux-android`.

Optional `x86_64` may be added for emulator/CI.

Automate cross-compilation.

Use a transparent maintained cargo-ndk style process or explicit Gradle task.

Avoid fragile dependency magic.

---

# Article 21 — Windows Host Toolchain Contract

Execution 02 assumes Windows.

Create a repository environment-doctor script that checks Git, Rust, rustup, Android Rust target, Android Studio/SDK, Platform Tools, NDK, Java, ADB, Gradle Wrapper, and connected device.

Do not require macOS.

---

# Article 22 — USB Device Workflow

The required developer loop is USB-first.

Expected verification:

```powershell
adb devices
```

Expected authorized state:

```text
<device-id>    device
```

Build/install should reduce to a documented Gradle/ADB path, preferably:

```powershell
cd android
.\gradlew.bat installDebug
```

Provide a launch command and filtered logcat command.

Include Developer Options, USB debugging, RSA authorization, Windows driver troubleshooting, `unauthorized`, and multiple-device handling.

---

# Article 23 — Android CI

Add or expand Android CI.

CI should set up Java, Android SDK, pinned NDK, Rust, Android target, cross-compile FFI, generate Kotlin bindings, build debug APK, run unit tests/lint, and upload APK artifact where useful.

CI does not certify S Pen hardware behavior.

---

# Article 24 — Physical Device Validation

The Samsung tablet is part of the execution.

Validate draw/write, erase, pressure, tilt if exposed, palm/finger rejection, hover if exposed, barrel button diagnostics, latency, jitter, rotation, app background/resume, and process restart.

Hardware evidence is required for hardware claims.

---

# Article 25 — Testing Layers

Required layers:

- Rust unit tests.
- Rust scenario tests.
- FFI contract tests.
- Kotlin unit tests.
- Compose tests.
- Instrumented Android tests.
- physical S Pen tests.

No single layer replaces the others.

---

# Article 26 — Performance Budget

Track pointer-to-visible-ink latency, stroke handoff, recognition, snapshot generation, structured rendering, dimension transaction, solve latency, Orthographic propagation, save/reopen, and memory growth.

Do not optimize without measurement.

Never block live ink on serialization.

---

# Article 27 — No-Hallucination Policy

Do not claim Jetpack Ink works before a real Android build runs.

Do not claim the APK works before physical installation.

Do not claim pressure/tilt/palm rejection without observation.

Do not claim handwriting from typed fixtures.

Do not claim Android Orthographic because Rust tests pass.

Do not claim persistence from in-memory state.

Do not claim iPad implementation exists.

Every claim requires executed evidence.

---

# Article 28 — Context Engineering Contract

Use progressive context.

For each task give only the relevant execution article/phase, related Execution 01 implementation, MCP V1 articles, exact repository files, current failure, and invariant.

Maintain `execution-evidence/execution-02/`.

Do not paste every project document into every subagent.

Repository code is authoritative.

---

# Article 29 — Prompt Engineering Contract

Every delegated task contains:

**Objective**  
**Current Evidence**  
**Product Invariant**  
**Allowed Scope**  
**Forbidden Shortcuts**  
**Test First**  
**Execution**  
**Verification**  
**Device Evidence** when applicable  
**Completion Condition**

Avoid “make Android work.”

---

# Article 30 — Loop Engineering Contract

Use:

**Inspect → Bound → Reproduce → Test → Implement → Build → Device Verify → Review → Record → Continue**

For shared-core tasks Device Verify may not apply.

For Android adapter tasks build is mandatory.

For S Pen behavior physical verification is mandatory.

Do not accumulate multiple uncertain changes before rebuilding.

---

# Article 31 — True Blockers

True blockers include unavailable Android SDK/NDK, device not recognized after real troubleshooting, binding strategy proven incompatible, incompatible license, device below required Android level, a shared-core defect that makes the workflow impossible, or contradictory source requirements.

A failed first build, unfamiliar Jetpack Ink API, missing icon, or incomplete UI is not a blocker.

---

# Article 32 — Agent Continuity Rule

Execute continuously.

Do not stop after setup, first APK launch, Jetpack Ink drawing, FFI linking, or first snapshot.

Continue through the physical acceptance gate.

Pause only for human-required actions such as enabling USB debugging or accepting the RSA prompt.

When human action is required, state one exact action and the next verification command.

---


# Article 33 — Phase Execution Protocol

Every task follows: Inspect → Invariant → Failing Evidence → Small Implementation → Focused Verification → Neighbor Verification → Android Build → Physical Verification when applicable → Review → Record → Continue.

Do not create a phase-approval loop. Phase gates are internal quality boundaries.

---

# Phase 00 — Baseline and Repository Lock

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 001 — Re-audit Android placeholder

**Objective.** Inspect `android/`, `crates/craftloop-mobile-ffi`, current Gradle files, generated Kotlin evidence, CI workflows, and Execution 01 final report. Produce an exact gap map before writing production code.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 002 — Run existing Rust baseline

**Objective.** Run formatting, clippy, workspace tests, and scenario tests. Record the baseline before Android changes.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 003 — Record Android dependency versions

**Objective.** Record current Android Gradle Plugin, Kotlin, Compose, compileSdk, minSdk, targetSdk, JNA, UniFFI, Rust toolchain, and existing Android CI assumptions.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 004 — Create Execution 02 evidence directory

**Objective.** Create a dedicated evidence directory and place baseline reports, toolchain checks, physical-device results, screenshots, and final report there.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 005 — Protect universal boundaries

**Objective.** Identify which crates are platform-neutral and explicitly forbid Android SDK types from entering those crates.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 01 — Windows Android Toolchain

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 006 — Verify Java runtime

**Objective.** Ensure the chosen JDK is compatible with the pinned Android Gradle Plugin; document the exact version used.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 007 — Install and verify Android SDK

**Objective.** Verify SDK root and required compile platform are available on Windows.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 008 — Install Platform Tools

**Objective.** Verify `adb version` and make ADB available from PowerShell or via an explicit repository helper script.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 009 — Install Android NDK

**Objective.** Pin an NDK version and verify the toolchain exists; do not rely on an unspecified latest NDK.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 010 — Install Rust Android target

**Objective.** Add `aarch64-linux-android` and verify Rust can locate the Android linker through the chosen NDK build path.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 011 — Create environment doctor script

**Objective.** Create a Windows-friendly script that prints PASS/FAIL for Java, SDK, NDK, adb, Rust, Rust Android target, Gradle Wrapper, and connected device state.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 02 — Gradle Reproducibility

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 012 — Add Gradle Wrapper

**Objective.** Commit wrapper configuration and scripts so the project builds from a clean Windows checkout without global Gradle.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 013 — Stabilize dependency resolution

**Objective.** Ensure Google and Maven Central are configured once and dependency resolution is deterministic.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 014 — Keep or deliberately update baseline AGP/Kotlin

**Objective.** Do not mix broad dependency modernization with FFI bring-up. Upgrade only if the baseline cannot build.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 015 — Define Alpha build type

**Objective.** Use a debuggable configuration with useful logging and no release-signing requirement.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 016 — Add clean build verification

**Objective.** From a clean state, prove `gradlew.bat assembleDebug` reaches the Android application build before native integration.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 03 — Rust-to-Android Native Build

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 017 — Choose and document cargo-ndk integration

**Objective.** Prefer a transparent maintained cargo-ndk workflow or explicit Gradle task. Record the decision.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 018 — Cross-compile mobile FFI crate

**Objective.** Produce the Android ARM64 shared library from `craftloop-mobile-ffi`.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 019 — Generate jniLibs layout

**Objective.** Place or generate `arm64-v8a/libcraftloop_mobile_ffi.so` in the Android packaging path without manual copying.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 020 — Automate native build dependency

**Objective.** Make Android debug packaging depend on the Rust native build so stale native code cannot silently ship.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 021 — Verify native library packaging

**Objective.** Inspect APK or intermediates and prove the ARM64 library is included.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 04 — UniFFI Product Session Boundary

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 022 — Introduce CraftLoopSession

**Objective.** Create a UniFFI-exported session object or equivalent platform-neutral handle around active document semantics.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 023 — Expose session lifecycle

**Objective.** Android must create a clean document session and release it safely.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 024 — Expose scene snapshot

**Objective.** Create FFI-safe scene data for structured rendering, selection, dimensions, View Blocks, unresolved state, and conflicts.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 025 — Expose stroke submission

**Objective.** Accept completed normalized stroke batches and route them into the existing ink/recognition/domain pipeline.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 026 — Expose direct primitive commands

**Objective.** Support explicit Line, Circle, and Rectangle test tools without reimplementing primitive logic in Kotlin.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 027 — Expose selection operations

**Objective.** Provide stable-ID selection and deletion required by the Alpha UI.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 028 — Expose dimension operations

**Objective.** Create/edit dimensions through the semantic dimension engine.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 029 — Expose constraint operations

**Objective.** Apply/remove the supported stable constraint set.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 030 — Expose View Identity and Orthographic operations

**Objective.** Assign principal views and enter/update the existing multiview engine.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 031 — Expose conflict resolution

**Objective.** Allow Android to query and resolve structured conflicts.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 032 — Expose undo/redo

**Objective.** Map Android toolbar actions to shared semantic transaction history.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 033 — Expose save/open

**Objective.** Use the existing document model rather than Android-only serialization.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 05 — Kotlin Binding Integration

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 034 — Generate Kotlin bindings deterministically

**Objective.** Make generation part of the build or a validated repository task.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 035 — Fail on stale bindings

**Objective.** Rust interface changes must not leave stale Kotlin APIs compiling accidentally.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 036 — Wrap UniFFI in an Android bridge

**Objective.** Create a small service/repository wrapper rather than leaking UniFFI details through UI code.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 037 — Implement structured error mapping

**Objective.** Translate Rust errors into Alpha diagnostics and user-visible states.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 038 — Add FFI smoke tests

**Objective.** Prove session creation, simple geometry, snapshot, undo, and save cross the boundary.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 06 — Android Application State Architecture

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 039 — Replace placeholder activity

**Objective.** Remove the one-line placeholder UI while preserving useful app/build foundations.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 040 — Create CraftLoopViewModel

**Objective.** Own Android-side ephemeral state: active tool, current snapshot, debug flags, session lifecycle.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 041 — Separate ephemeral UI from semantic state

**Objective.** Do not mirror dimensions or constraints as independent Kotlin truth.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 042 — Implement snapshot refresh strategy

**Objective.** Refresh after semantic transactions; avoid snapshot churn on every live sample.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 043 — Handle lifecycle

**Objective.** Retain or restore the session/document across configuration change and process recreation.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 07 — Jetpack Ink Live Surface

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 044 — Add stable Jetpack Ink dependencies

**Objective.** Pin required 1.0.x modules after confirming exact artifacts.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 045 — Create in-progress ink surface

**Objective.** Render S Pen strokes immediately through Jetpack Ink.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 046 — Capture completed strokes

**Objective.** Collect ordered samples with timestamps and stylus metadata.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 047 — Normalize samples

**Objective.** Translate Android coordinates and available pressure/tilt/button data into the existing FFI pointer structure.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 048 — Batch submit on completion

**Objective.** Send completed strokes to Rust without placing the FFI round trip in live rendering.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 049 — Render committed raw ink

**Objective.** If Rust keeps a stroke as raw ink, render it as persistent document content.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 050 — Render recognized structured geometry

**Objective.** When Rust returns a structured result, transition to semantic vector rendering.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 051 — Handle stroke cancellation

**Objective.** Canceled input must not create semantic geometry.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 08 — Pan, Zoom, Selection, and Editing

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 052 — Separate finger navigation from pen drawing

**Objective.** Default finger gestures to viewport navigation while S Pen draws.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 053 — Implement pan

**Objective.** Do not mutate engineering coordinates while moving viewport.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 054 — Implement zoom

**Objective.** Maintain stable semantic geometry and hit targets.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 055 — Implement selection tool

**Objective.** Use stable entity IDs and core-validated hit testing/selection.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 056 — Implement deletion

**Objective.** Delete semantic objects through session transactions.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 057 — Implement fit-to-content

**Objective.** Provide a debug convenience without altering document scale.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 09 — Icon-First Test Toolbar

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 058 — Create icon-only primary toolbar

**Objective.** Pen, Eraser, Select, Line, Circle, Rectangle, Dimension, Constraint, View, Ortho, Undo, Redo, Save.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 059 — Create accessible semantics

**Objective.** Every icon gets `contentDescription` and practical touch target.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 060 — Show active tool state

**Objective.** Use visual selection/pressed state rather than persistent text.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 061 — Create long-press tooltips

**Objective.** Optional names for discoverability.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 062 — Create constraint popover

**Objective.** Show only constraints valid for current selection.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 063 — Create View Identity popover

**Objective.** Front, Top, Right, Back.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 064 — Create Alpha overflow

**Objective.** New/Open/Fit/Debug/Settings only.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 10 — Dimensions and Constraints Vertical Slice

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 065 — Implement typed dimension entry

**Objective.** Use a compact numeric input as the first reliable physical path.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 066 — Create semantic dimension

**Objective.** Pass value and target to Rust; render returned annotation.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 067 — Edit dimension

**Objective.** Selecting a dimension allows a new value through the same semantic transaction.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 068 — Show invalid dimension conflict

**Objective.** Do not silently coerce invalid values.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 069 — Apply core constraints

**Objective.** Provide stable constraints through the popover.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 070 — Show solver conflict

**Objective.** Preserve last valid geometry and show structured explanation.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 071 — Verify undo/redo

**Objective.** Dimension and constraint operations undo coherently.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 11 — View Identity and Orthographic Vertical Slice

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 072 — Create a View Block from selected geometry

**Objective.** Use the shared View Block model.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 073 — Assign FRONT

**Objective.** Store semantic View Identity in Rust.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 074 — Enter Orthographic

**Objective.** Use existing readiness logic; full dimensioning is not required.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 075 — Render TOP and RIGHT blocks

**Objective.** Show linked view areas and known geometry.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 076 — Render unresolved depth

**Objective.** Explicitly show missing value as unknown.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 077 — Resolve depth in TOP

**Objective.** Enter a valid semantic depth value and propagate it to RIGHT.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 078 — Verify no master-view assumption

**Objective.** Change a shared value from another view and confirm propagation.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 079 — Create cross-view conflict

**Objective.** Enter a contradictory shared value.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 080 — Resolve conflict

**Objective.** Keep existing / replace / cancel according to current core capability.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 12 — Persistence and App Lifecycle

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 081 — Create New/Open flows

**Objective.** Simple Alpha document flow with explicit storage policy.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 082 — Save semantic document

**Objective.** Use shared document serialization or intended adapter.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 083 — Autosave committed transactions

**Objective.** Do not block live ink.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 084 — Kill-process test

**Objective.** Force-stop after save and reopen.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 085 — Restore Orthographic state

**Objective.** Verify view identity, shared dimensions, unresolved values, and conflicts survive.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 086 — Test lifecycle recreation

**Objective.** Do not lose semantic state on ordinary Activity recreation.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 13 — Physical S Pen Diagnostics

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 087 — Expose stylus source

**Objective.** Confirm tool type is stylus for S Pen strokes.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 088 — Expose real pressure values

**Objective.** Record min/max sample during manual pressure test.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 089 — Expose tilt values when available

**Objective.** Record observed range; do not synthesize.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 090 — Test palm rejection

**Objective.** Draw with hand resting on display and record whether non-stylus marks appear.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 091 — Test hover

**Objective.** Record capability and event behavior.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 092 — Record barrel button

**Objective.** Diagnostic only unless stable.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 093 — Measure practical latency

**Objective.** Capture qualitative and, where feasible, instrumented evidence.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 094 — Record jitter

**Objective.** Use straight/slow curves and note smoothing/recognition usability.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 14 — Handwriting Dimension Adapter

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 095 — Add ML Kit Digital Ink Recognition

**Objective.** Use current supported Android dependency and model lifecycle.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 096 — Implement model availability/download state

**Objective.** Do not treat missing model as recognition failure.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 097 — Convert writing strokes into ML Kit Ink

**Objective.** Preserve ordering and timestamps.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 098 — Recognize numeric engineering input

**Objective.** Start with integer/decimal and required engineering symbols.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 099 — Route results into existing parser

**Objective.** Never let ML Kit directly mutate geometry.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 100 — Add confidence/choice behavior

**Objective.** Low-confidence candidates remain user-confirmed.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 101 — Validate handwritten 30

**Objective.** Handwrite value, recognize, associate, validate, commit.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 15 — Ink Command Adapter

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 102 — Recognize command words

**Objective.** Use handwriting adapter for a small English Alpha vocabulary.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 103 — Detect confirmation circle

**Objective.** Use temporal/spatial evidence and existing command rules.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 104 — Route through existing Command Bus

**Objective.** No Android-only command logic.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 105 — Reflect command in toolbar

**Objective.** If Line activates, Line icon becomes active.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 106 — Test note false positives

**Objective.** A sentence containing Line does not execute.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 107 — Test lasso/geometry conflict

**Objective.** Circle confirmation does not steal selection or explicit circle geometry.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 16 — Android CI and APK Artifact

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 108 — Create Android CI job

**Objective.** Build native Rust, bindings, and debug APK on relevant changes.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 109 — Run Android lint

**Objective.** Treat real lint failures as failures or documented Alpha exceptions.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 110 — Run Kotlin unit tests

**Objective.** Keep platform state logic covered.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 111 — Run Compose/instrumented tests

**Objective.** Use emulator for UI state, never S Pen certification.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 112 — Upload debug APK

**Objective.** Publish CI artifact where appropriate.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 17 — USB Developer Loop

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 113 — Document Samsung setup

**Objective.** Developer Options, USB debugging, authorization, Windows driver path if required.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 114 — Verify adb device

**Objective.** Record successful `adb devices` with serial redacted in committed evidence.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 115 — Install Debug APK

**Objective.** Use Gradle/ADB from Windows.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 116 — Launch through ADB

**Objective.** Record exact application/activity command.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 117 — Capture logcat

**Objective.** Provide a filtered command for Craft Loop logs.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 118 — Create one-command dev helper

**Objective.** Optional script to build, install, launch, and tail logs.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 18 — Engineering Alpha Acceptance

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 119 — Run physical draw workflow

**Objective.** S Pen live ink → committed stroke → semantic result.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 120 — Run dimension workflow

**Objective.** Create valid and invalid dimension.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 121 — Run constraint workflow

**Objective.** Apply at least Parallel or Perpendicular and verify conflict behavior.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 122 — Run FRONT→ORTHO workflow

**Objective.** Assign Front, enter Orthographic, reveal linked views.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 123 — Resolve missing depth

**Objective.** Enter depth in Top and verify Right propagation.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 124 — Run cross-view conflict

**Objective.** Introduce contradictory shared dimension and resolve.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 125 — Run undo/redo

**Objective.** Verify across geometry, dimension, and Orthographic changes.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 126 — Run save/kill/reopen

**Objective.** Verify semantic restoration.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 127 — Run S Pen capability checklist

**Objective.** Pressure/tilt/palm/hover evidence as available.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 128 — Run handwriting 30 workflow

**Objective.** Required if handwriting gate is enabled; otherwise document limitation.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 129 — Run Ink Command workflow

**Objective.** Required if command adapter is enabled; otherwise document limitation.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---

# Phase 19 — Final Audit and Handoff

This phase is a delivery boundary, not an approval pause. Close the gate and continue automatically unless a true blocker requires human action.

## Task 130 — Audit Android-only leakage

**Objective.** Search shared crates for Android/Compose/Jetpack types and remove violations.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 131 — Audit iPad portability

**Objective.** Prove a future Swift/PencilKit adapter can replace Android-specific responsibilities.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 132 — Audit UI scope

**Objective.** Remove accidental final-design complexity from the test surface.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 133 — Audit unsupported claims

**Objective.** No hardware or handwriting claim without evidence.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 134 — Run full Rust + Android suite

**Objective.** Record exact commands and results.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 135 — Write Execution 02 final report

**Objective.** Implemented, tested, hardware-validated, stubbed, deferred, limitations.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Task 136 — Define Execution 03 boundary

**Objective.** Do not begin final UI/UX or iPad work inside Execution 02.

**Required context.** Inspect the exact current repository files, corresponding Execution 01 implementation, and relevant MCP V1 product invariant. Do not assume the file layout if the repository has evolved.

**Universal-software check.** Android SDK, Compose, Jetpack Ink, and device-specific objects remain on the Android side. Shared engineering semantics stay platform-neutral.

**Test-first requirement.** Create or identify executable failing evidence. For hardware-only behavior, use a physical reproduction checklist and observable evidence.

**Implementation constraint.** Do not duplicate an existing Rust engine in Kotlin. Expand the session/FFI only when Android genuinely needs a shared-core operation.

**Verification.** Run the narrow relevant Rust/Kotlin/Gradle check, neighboring regressions, Android build where applicable, and physical-device verification when the task concerns stylus behavior.

**Evidence.** Record files changed, tests, build command, results, device result, and any explicit limitation.

**Completion condition.** The objective works through the intended layer, verification is green, no Android-only engineering truth is introduced, and the result advances the physical Android Alpha workflow.

---

## Phase Gate

- Every task has an evidence state.
- Relevant Rust tests are green.
- Relevant Android unit/build checks are green.
- Debug APK builds if the phase touches packaging.
- Physical behavior is verified where required.
- No shared-core platform leakage is introduced.
- No unsupported completion claim is made.
- Continue automatically.

---


# Article 34 — Android Engineering Alpha Acceptance Gate

Execution 02 must not use a vague percentage to describe readiness.

The final gate is binary at the workflow level.

## Gate A — Build

From a clean Windows checkout:

```powershell
cd android
.\gradlew.bat clean
.\gradlew.bat assembleDebug
```

must produce the debug APK.

The native Rust library and generated Kotlin bindings must be produced by the documented build chain.

## Gate B — USB

With the Samsung tablet connected:

```powershell
adb devices
```

must show an authorized physical device.

## Gate C — Install

A documented command must install the current debug APK without manual copying.

## Gate D — Launch

The app must launch from Android Studio or ADB.

## Gate E — Physical Ink

The S Pen must render an immediate in-progress stroke.

Completed stroke data must reach the shared core.

## Gate F — Structured Geometry

At least line/circle/rectangle test workflows must produce structured semantic geometry or an explicit keep-as-ink outcome.

## Gate G — Dimension

A valid dimension changes semantic geometry.

An invalid dimension creates a conflict.

## Gate H — Constraint

The required stable constraints can be applied.

Contradiction preserves last valid geometry.

## Gate I — View Identity

The user can mark a view as `FRONT`.

## Gate J — Orthographic

The user can enter Orthographic with an incompletely dimensioned source view.

## Gate K — Unresolved Information

Missing depth remains unresolved and is visibly represented as such.

## Gate L — Resolve Elsewhere

The user can enter depth in `TOP`.

`RIGHT` receives the shared value.

## Gate M — Cross-View Conflict

A contradictory shared dimension creates a conflict rather than a second independent truth.

## Gate N — Undo/Redo

Core engineering actions undo and redo coherently.

## Gate O — Persistence

Save → force-stop → relaunch → reopen preserves the semantic drawing.

## Gate P — Stylus Evidence

The physical capability checklist records real pressure, tilt, hover, and palm behavior according to the actual device.

When Gates A through P pass, Craft Loop is **Android Tablet Engineering Alpha Ready**.

Handwriting and Ink Commands can be either `ALPHA_ENABLED_AND_VALIDATED` or `DEFERRED_WITH_EXPLICIT_LIMITATION` only if all other core gates pass.

They must not be falsely marked complete.

---

# Article 35 — Golden Alpha Journey

Create a new document.

Select Pen.

Draw one rough line with the S Pen.

Observe immediate Android-native in-progress ink.

Lift the pen.

Observe the stroke commit.

Create a simple front profile.

Select an edge.

Activate Dimension.

Enter `100`.

Confirm the geometry updates.

Apply a stable geometric constraint.

Attempt an incompatible value.

Confirm the system shows a conflict.

Undo the incompatible action.

Assign the structured sketch as `FRONT`.

Activate Orthographic.

Confirm `TOP` and `RIGHT` appear.

Confirm unknown depth is not invented.

Select the appropriate Top extent.

Enter `40`.

Confirm the Right view now uses the same semantic depth.

Attempt a contradictory shared value.

Confirm a shared-value conflict appears.

Cancel or resolve the conflict.

Undo.

Redo.

Save.

Force-stop.

Relaunch.

Open the document.

Confirm geometry and engineering relationships are restored.

This is the primary manual success scenario for Execution 02.

---

# Article 36 — USB Test Guide Required From the Execution

The repository must end Execution 02 with a developer guide containing steps equivalent to:

## Samsung Tablet Setup

Enable Developer Options.

Enable USB debugging.

Connect the tablet with a data-capable USB cable.

Accept the RSA debugging authorization prompt.

On Windows:

```powershell
adb devices
```

Expected:

```text
<device-id>    device
```

If `unauthorized`, re-authorize on the tablet.

If absent, check cable/USB mode, Platform Tools, OEM driver if required, `adb kill-server`, `adb start-server`, and Android Studio Connection Assistant.

## Build and Install

Target flow:

```powershell
cd android
.\gradlew.bat installDebug
```

or:

```powershell
.\gradlew.bat assembleDebug
adb -d install -r <path-to-debug-apk>
```

## Launch

Use Android Studio Run or:

```powershell
adb -d shell am start -n <application-id>/<main-activity>
```

## Logs

Provide a project-specific `adb logcat` filter.

---

# Article 37 — Dependency Policy

Do not perform a broad “latest everything” upgrade.

Sequence:

1. establish a green build from the existing project;
2. add only required Alpha dependencies;
3. pin versions;
4. update one dependency family at a time;
5. build and smoke-test after meaningful changes.

Jetpack Ink should prefer stable 1.0.x.

If a physical-device defect requires 1.1 alpha, document the exact reason and rollback path.

ML Kit Digital Ink Recognition is added only after the core typed-dimension Alpha is green.

---

# Article 38 — Current Research Basis

This execution was refreshed against current Android guidance on 2026-09-14.

## Physical Device Deployment

Android's official device guidance requires real-device testing and documents USB debugging and `adb devices`.

## Jetpack Ink

Jetpack Ink has a stable 1.0 release line and a newer 1.1 alpha line. This execution prefers stable for the Engineering Alpha unless device evidence requires otherwise.

## Stylus Quality

Current Android stylus quality guidance explicitly calls out pressure, tilt, palm/finger rejection, and physical latency tests.

## Android NDK

Android Studio can install and pin NDK versions. A pinned NDK is required because the Rust core must cross-compile to Android.

## ML Kit Digital Ink Recognition

ML Kit recognizes handwriting from ordered stroke data. Models are downloaded dynamically. Writing area and context can improve recognition. The engineering parser and consistency engine remain authoritative.

---

# Article 39 — iPad Deferred, Not Forgotten

Execution 02 does not build iPad.

It must nevertheless produce an Android implementation that makes the iPad adapter straightforward.

The final report must include an iPad Portability Audit.

Map Android-specific responsibilities to future iPad equivalents:

```text
Jetpack Ink          → PencilKit
Compose presentation → SwiftUI/UIKit presentation
Android ViewModel    → iPad presentation/session controller
Android handwriting  → iPad handwriting adapter
Android storage      → iPad storage/document adapter
ADB deployment       → Xcode/TestFlight deployment
```

Shared session, dimensions, constraints, document model, Orthographic graph, conflict model, and transactions must not require reimplementation.

If Android implementation forces future iPad to rewrite engineering semantics, the universal architecture has failed.

---

# Article 40 — Final Report Contract

Create:

`execution-evidence/execution-02/FINAL-REPORT.md`

It must include:

## Result

One of:

`ANDROID TABLET ENGINEERING ALPHA READY`

`ANDROID TABLET ENGINEERING ALPHA READY WITH DISCLOSED LIMITATIONS`

`PARTIALLY COMPLETE`

## Build Evidence

Exact Windows commands.

APK output path.

Native-library ABI.

## USB Evidence

ADB recognition result with serial redacted.

## Physical Device

Android version and observed stylus capabilities.

## Engineering Workflow

PASS / FAIL / DEFERRED for every acceptance gate.

## Handwriting

One of:
- not implemented;
- implemented but not physical validated;
- physical validated.

## Ink Commands

Same explicit status.

## Core Regressions

Rust tests, Android unit tests, lint, CI.

## Performance

Measured observations.

## Known Limitations

No optimistic language.

## Universal Architecture

Confirm shared core remains iPad-ready.

## Next Execution

Define Execution 03 boundary.

Do not silently start it.

---

# Article 41 — Coding Agent Start Prompt

From the repository root, the human should be able to use:

```text
Read Craft Loop Execution 02 Android Tablet Engineering Alpha.md as the current execution authority.

Read Craft Loop MCP V1.md as product authority.

Inspect the current repository before modifying anything.

Execution 01 already built the shared engine foundation. Do not rebuild working engines in Kotlin.

Execute Execution 02 continuously from Phase 00 through the final Android Tablet Engineering Alpha acceptance gate.

Primary development host: Windows.
Primary physical target: connected Samsung Android tablet over USB.
Production architecture: shared universal engineering core with native Android adapter now and iPad adapter later.

The Android UI is test-only and must be icon-first. Do not spend execution budget on final visual design.

Use Plan Mode at phase/task boundaries and Goal Mode to preserve the final physical Android Alpha outcome.

Do not stop after each phase. Continue automatically unless a true blocker requires human action.

Do not claim S Pen, pressure, tilt, palm rejection, handwriting, Orthographic, persistence, or APK readiness without executed evidence.

Goal: I must be able to connect my tablet by USB, build, install, launch, draw with S Pen, dimension, constrain, enter Orthographic, resolve missing depth, create a cross-view conflict, undo/redo, save, kill the app, reopen it, and recover the same engineering state.
```

---

# Article 42 — Closing Principle

Execution 01 proved that Craft Loop can have an engineering brain.

Execution 02 must put that brain under the developer's hand.

Do not optimize for screenshots.

Do not optimize for feature count.

Do not optimize for Android-specific convenience that damages universal architecture.

Optimize for one outcome:

> A real Samsung tablet, connected to a Windows computer, running a real Craft Loop Engineering Alpha, using the real S Pen to exercise the real shared engineering core.

**Hand First. Engineering Underneath. Universal Core. Android First. iPad Next.**

# Article 33 — Phase Execution Protocol

Every task below uses: **Inspect → Invariant → Failing Evidence → Small Implementation → Focused Verification → Neighbor Verification → Android Build → Physical Verification When Required → Review → Record → Continue.**

A phase is a delivery boundary, not a permission checkpoint.

---

# Article 34 — Execution Phases

# Phase 00 — Baseline and Repository Lock

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 001 — Re-audit Android placeholder

**Objective:** Inspect `android/`, `crates/craftloop-mobile-ffi`, current Gradle files, generated Kotlin evidence, CI workflows, and Execution 01 final report. Produce an exact gap map before writing production code.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 002 — Run existing Rust baseline

**Objective:** Run formatting, Clippy, workspace tests, and scenario tests. Record the baseline before Android changes.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 003 — Record Android dependency versions

**Objective:** Record current Android Gradle Plugin, Kotlin, Compose, compileSdk, minSdk, targetSdk, JNA, UniFFI, Rust toolchain, and existing Android CI assumptions.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 004 — Create Execution 02 evidence directory

**Objective:** Create `execution-evidence/execution-02/` and place baseline reports, toolchain checks, device results, screenshots, benchmarks, and the final report there.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 005 — Protect universal boundaries

**Objective:** Identify platform-neutral crates and explicitly forbid Android SDK, Compose, Jetpack Ink, or ML Kit types from entering them.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 01 — Windows Android Toolchain

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 006 — Verify Java runtime

**Objective:** Ensure the chosen JDK is compatible with the pinned Android Gradle Plugin and record the exact version.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 007 — Install and verify Android SDK

**Objective:** Verify SDK root and required compile platform on Windows.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 008 — Install Platform Tools

**Objective:** Verify `adb version` and make ADB usable from PowerShell or a repository helper script.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 009 — Install Android NDK

**Objective:** Pin an NDK version; do not rely on an unspecified latest NDK.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 010 — Install Rust Android target

**Objective:** Add `aarch64-linux-android` and verify the build tool can locate NDK linkers.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 011 — Create environment doctor script

**Objective:** Create a Windows-friendly script reporting PASS/FAIL for Java, SDK, NDK, adb, Rust, Android target, Gradle Wrapper, and connected-device state.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 02 — Gradle Reproducibility

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 012 — Add Gradle Wrapper

**Objective:** Commit wrapper configuration and scripts so a clean Windows checkout does not require globally installed Gradle.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 013 — Stabilize dependency resolution

**Objective:** Ensure Google/Maven repositories and plugin management are deterministic.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 014 — Keep or deliberately update baseline versions

**Objective:** Do not perform broad dependency modernization. Upgrade only when the existing baseline cannot build against required Android APIs.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 015 — Define Alpha debug build

**Objective:** Use a debuggable build with useful logging and no release-signing requirements.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 016 — Prove clean Android compilation

**Objective:** From a clean state, make `gradlew.bat assembleDebug` reach a green Android build before real native integration.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 03 — Rust-to-Android Native Build

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 017 — Choose cargo-ndk integration

**Objective:** Prefer a transparent maintained `cargo-ndk` workflow or explicit Gradle task. Record the decision.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 018 — Cross-compile mobile FFI crate

**Objective:** Produce the Android ARM64 shared library from `craftloop-mobile-ffi`.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 019 — Generate packaging layout

**Objective:** Produce `arm64-v8a/libcraftloop_mobile_ffi.so` in the Android packaging path without manual copying.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 020 — Automate native dependency

**Objective:** Make Android debug packaging depend on the Rust native build.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 021 — Verify APK native contents

**Objective:** Inspect the APK/build intermediates and prove the ARM64 library is included.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 04 — UniFFI Product Session Boundary

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 022 — Introduce CraftLoopSession

**Objective:** Create a UniFFI-exported session object or equivalent platform-neutral handle around active document semantics.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 023 — Expose session lifecycle

**Objective:** Android can create, open, save, close/release a session safely.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 024 — Expose scene snapshot

**Objective:** Create FFI-safe scene data for structured rendering, selection, dimensions, view blocks, unresolved state, and conflicts.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 025 — Expose completed stroke submission

**Objective:** Accept normalized stroke batches and route them into existing ink/recognition/domain logic.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 026 — Expose direct primitive actions

**Objective:** Support explicit Line/Circle/Rectangle test tools without reimplementing geometry in Kotlin.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 027 — Expose selection/delete

**Objective:** Use stable entity IDs.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 028 — Expose dimensions

**Objective:** Create/edit semantic dimensions through the existing engine.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 029 — Expose constraints

**Objective:** Apply/remove supported constraints.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 030 — Expose View Identity and Orthographic

**Objective:** Assign principal view identities and enter/update multiview state.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 031 — Expose conflicts

**Objective:** Query and resolve/cancel structured conflicts.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 032 — Expose undo/redo

**Objective:** Use shared transaction history.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 033 — Expose save/open

**Objective:** Use the existing document model, not Android-only serialization.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 05 — Kotlin Binding Integration

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 034 — Generate bindings deterministically

**Objective:** Make Kotlin UniFFI generation part of the build or a validated repository task.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 035 — Prevent stale bindings

**Objective:** Rust interface changes must regenerate bindings or fail clearly.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 036 — Create Android bridge layer

**Objective:** Wrap raw UniFFI calls behind a small Android repository/service adapter.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 037 — Map structured errors

**Objective:** Translate Rust errors into Alpha diagnostics and user-safe messages.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 038 — Add FFI smoke tests

**Objective:** Prove session creation, primitive transaction, snapshot, undo, and save cross the boundary.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 06 — Android Application State Architecture

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 039 — Replace placeholder activity

**Objective:** Remove the one-line placeholder screen while preserving valid package/build foundation.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 040 — Create CraftLoopViewModel/state holder

**Objective:** Own only ephemeral Android state: active tool, selected UI context, latest snapshot, debug flags, and session lifecycle.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 041 — Separate semantic and UI state

**Objective:** Do not mirror dimensions, constraints, or Orthographic truth as independent Kotlin state.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 042 — Implement snapshot refresh

**Objective:** Refresh after semantic transactions; do not regenerate full scene for every raw pointer sample.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 043 — Handle Activity lifecycle

**Objective:** Restore the session/document safely across configuration and process lifecycle according to the persistence design.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 07 — Jetpack Ink Live Surface

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 044 — Add stable Jetpack Ink modules

**Objective:** Pin the exact 1.0.x modules actually required after checking current official artifacts.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 045 — Create in-progress ink surface

**Objective:** Render S Pen strokes immediately through Jetpack Ink.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 046 — Capture finished strokes

**Objective:** Collect ordered samples with timestamps and stylus metadata.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 047 — Normalize Android input

**Objective:** Translate position, pressure, tilt, source, buttons, and capability flags into existing FFI pointer structures.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 048 — Batch submit completed strokes

**Objective:** Do not put the FFI round trip in the live-pixel path.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 049 — Render persistent raw ink

**Objective:** If the core keeps a stroke as raw ink, show it as document content.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 050 — Render structured result

**Objective:** When Rust produces geometry, render the semantic vector result.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 051 — Handle cancellation

**Objective:** Canceled input must not create semantic geometry.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 08 — Pan, Zoom, Selection, and Editing

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 052 — Finger navigation policy

**Objective:** Finger pans/zooms while S Pen draws by default.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 053 — Implement pan

**Objective:** Viewport motion never changes engineering coordinates.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 054 — Implement zoom

**Objective:** Maintain stable semantic geometry and hit targets.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 055 — Implement selection

**Objective:** Use stable entity IDs and core-validated hit testing where practical.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 056 — Implement deletion

**Objective:** Delete semantic objects through session transactions.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 057 — Implement fit-to-content

**Objective:** Diagnostic convenience only; do not alter document scale.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 09 — Icon-First Test Toolbar

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 058 — Create icon-only primary toolbar

**Objective:** Pen, Eraser, Select, Line, Circle, Rectangle, Dimension, Constraint, View, Ortho, Undo, Redo, Save.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 059 — Add accessibility semantics

**Objective:** Every icon has a `contentDescription` and practical tablet touch target.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 060 — Show active tool visually

**Objective:** Use selected/pressed state rather than persistent text.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 061 — Add long-press tooltips

**Objective:** Optional tool names for discovery only.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 062 — Create constraint popover

**Objective:** Show only valid/supported constraints for current selection.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 063 — Create View Identity popover

**Objective:** Front, Top, Right, Back.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 064 — Create Alpha overflow

**Objective:** New/Open/Fit/Diagnostics and test-only settings.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 10 — Dimensions and Constraints Vertical Slice

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 065 — Implement typed dimension entry

**Objective:** Use compact numeric entry as the first reliable physical dimension path.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 066 — Create semantic dimension

**Objective:** Pass value and target to Rust; render returned annotation.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 067 — Edit semantic dimension

**Objective:** Existing dimension edits use the same core transaction.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 068 — Show invalid value conflict

**Objective:** Do not coerce or silently repair invalid engineering input.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 069 — Apply core constraints

**Objective:** Expose the stable constraint set through the icon palette.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 070 — Show solver conflict

**Objective:** Preserve last valid geometry and show structured conflict.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 071 — Verify undo/redo

**Objective:** Dimensions and constraints undo as coherent transactions.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 11 — View Identity and Orthographic Vertical Slice

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 072 — Create View Block

**Objective:** Use the shared View Block model.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 073 — Assign FRONT

**Objective:** Store semantic identity in Rust.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 074 — Enter Orthographic

**Objective:** Use existing readiness logic; complete dimensioning is not required.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 075 — Render TOP and RIGHT

**Objective:** Show linked view regions and known geometry.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 076 — Render unresolved depth

**Objective:** Explicitly preserve unknown state.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 077 — Resolve depth in TOP

**Objective:** Enter a valid shared depth and propagate to RIGHT.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 078 — Prove no permanent master view

**Objective:** Change a shared value from another linked view and confirm propagation.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 079 — Create cross-view conflict

**Objective:** Enter a contradictory shared value.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 080 — Resolve conflict

**Objective:** Use existing core semantics and verify undo.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 12 — Persistence and App Lifecycle

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 081 — Create New/Open Alpha flows

**Objective:** Simple single/recent-document UI is enough.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 082 — Save semantic document

**Objective:** Use the shared document model.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 083 — Autosave committed transactions

**Objective:** Never block live ink.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 084 — Force-stop test

**Objective:** Save, force-stop, relaunch, reopen.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 085 — Restore Orthographic state

**Objective:** View identities, shared dimensions, unresolved/conflict state survive.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 086 — Test Activity recreation

**Objective:** Ordinary Android lifecycle must not corrupt semantic state.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 13 — Physical S Pen Diagnostics

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 087 — Expose stylus source

**Objective:** Confirm actual stylus input path.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 088 — Expose pressure

**Objective:** Record min/max observed values during a physical pressure test.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 089 — Expose tilt when available

**Objective:** Record observed values; never synthesize.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 090 — Test palm rejection

**Objective:** Draw while resting hand on display and record stray-touch behavior.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 091 — Test hover

**Objective:** Record device capability and events.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 092 — Record barrel button

**Objective:** Diagnostic only.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 093 — Measure practical latency

**Objective:** Capture useful timing/observation without fake precision.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 094 — Record jitter

**Objective:** Slow lines/circles reveal input noise.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 14 — Handwriting Dimension Adapter

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 095 — Add ML Kit Digital Ink Recognition

**Objective:** Use current supported Android dependency and explicit model lifecycle.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 096 — Implement model state

**Objective:** Unavailable/downloading/ready/failed.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 097 — Convert writing strokes to ML Kit Ink

**Objective:** Preserve ordering/timestamps.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 098 — Recognize engineering numbers

**Objective:** Prioritize integers, decimals, angle/radius/diameter notation needed by Alpha.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 099 — Route candidates into Rust parser

**Objective:** ML Kit never directly changes geometry.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 100 — Handle ambiguity

**Objective:** Low confidence remains user-confirmed.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 101 — Physically validate handwritten `30`

**Objective:** S Pen → recognition → association → semantic dimension.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 15 — Ink Command Adapter

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 102 — Recognize small command vocabulary

**Objective:** Use handwriting adapter for Alpha command words.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 103 — Detect confirmation circle

**Objective:** Use temporal/spatial context.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 104 — Route through shared Command Bus

**Objective:** No Android-only command semantics.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 105 — Reflect active tool in toolbar

**Objective:** Command and icon state remain unified.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 106 — Test note false positives

**Objective:** A note containing a command word does not execute.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 107 — Test circle ambiguity

**Objective:** Confirmation circle does not steal explicit circle geometry or old-content selection.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 16 — Android CI and APK Artifact

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 108 — Create Android CI job

**Objective:** Build Rust Android library, bindings, and debug APK.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 109 — Run Android lint

**Objective:** Document only justified Alpha exceptions.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 110 — Run Kotlin tests

**Objective:** Keep Android presentation logic covered.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 111 — Run emulator/instrumented tests where suitable

**Objective:** Never treat emulator as stylus certification.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 112 — Upload debug APK artifact

**Objective:** Make the build reproducible outside the local machine.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 17 — USB Developer Loop

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 113 — Document Samsung setup

**Objective:** Developer Options, USB debugging, authorization, Windows driver troubleshooting.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 114 — Verify adb device

**Objective:** Record authorized physical device; redact serial in committed evidence.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 115 — Install debug APK

**Objective:** Use Gradle/ADB from Windows.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 116 — Launch through ADB

**Objective:** Record exact application/activity command.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 117 — Capture logcat

**Objective:** Provide a project-specific filter.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 118 — Create helper script

**Objective:** Optional PowerShell build→install→launch→logs loop.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 18 — Engineering Alpha Acceptance

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 119 — Run physical drawing workflow

**Objective:** S Pen live ink → completed stroke → shared-core semantic result.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 120 — Run dimension workflow

**Objective:** Valid and invalid dimension.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 121 — Run constraint workflow

**Objective:** Apply at least Parallel/Perpendicular and test conflict.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 122 — Run FRONT→ORTHO

**Objective:** Assign Front and enter linked view mode.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 123 — Resolve missing depth

**Objective:** Enter depth in Top and verify Right.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 124 — Create cross-view conflict

**Objective:** Contradict a shared value and resolve/cancel.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 125 — Run undo/redo

**Objective:** Across geometry, dimension, and Orthographic operations.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 126 — Run save/kill/reopen

**Objective:** Verify semantic restoration.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 127 — Run S Pen capability checklist

**Objective:** Pressure/tilt/palm/hover as device supports.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 128 — Run handwriting dimension

**Objective:** Required if handwriting gate is enabled; otherwise explicitly disclose deferral.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 129 — Run Ink Command

**Objective:** Required if command adapter is enabled; otherwise explicitly disclose deferral.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

# Phase 19 — Final Audit and Handoff

**Operating mode:** Goal Mode dominates. Plan Mode is used only to choose the shortest evidence-backed route to the phase result.

## Task 130 — Audit Android leakage

**Objective:** Search shared crates for Android/Compose/Jetpack/ML Kit types and remove violations.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 131 — Audit iPad portability

**Objective:** Map each Android-specific adapter to a future Swift/PencilKit equivalent.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 132 — Audit UI scope

**Objective:** Remove unnecessary final-design work that complicates testability.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 133 — Audit unsupported claims

**Objective:** No hardware/handwriting/Orthographic claim without evidence.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 134 — Run full test suites

**Objective:** Record exact Rust/Android/CI results.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 135 — Write final report

**Objective:** Implemented, tested, hardware-validated, stubbed, deferred, limitations.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Task 136 — Define Execution 03 boundary

**Objective:** Do not begin production UI/UX or iPad implementation inside Execution 02.

**Required context:** Inspect the exact current repository files owning this behavior, the relevant Execution 01 implementation, and the specific MCP V1 invariant. Do not trust an old file path merely because this execution mentions it.

**Universal-software check:** Android SDK, Compose, Jetpack Ink, ADB, ML Kit, and device-specific objects remain in adapter/presentation code. Shared engineering semantics stay platform-neutral.

**Test-first rule:** Add or identify failing executable evidence before modifying deterministic production behavior. Hardware-only tasks use a written physical reproduction and device evidence.

**Implementation rule:** Do not duplicate existing Rust engineering logic in Kotlin. Expand the session/FFI boundary only when the Android workflow genuinely needs a shared-core capability.

**Verification:** Run focused Rust/Kotlin/Gradle checks, then neighboring regression tests. If the task changes the installed application, build and exercise the result on the tablet when practical.

**Evidence:** Record files changed, tests added/changed, commands, results, physical device outcome when applicable, and any limitation or deferral.

**Completion:** The objective works through the intended layer, tests/build are green, universal architecture is preserved, and the physical Android Alpha is closer to the Goal.

---

## Phase Gate

- All mandatory tasks have evidence.
- Relevant Rust tests are green.
- Relevant Android tests/build are green.
- Debug APK builds if packaging is touched.
- Physical tablet behavior is verified where required.
- No platform leakage into the shared core exists.
- No unsupported completion claim exists.
- Continue automatically.

---

