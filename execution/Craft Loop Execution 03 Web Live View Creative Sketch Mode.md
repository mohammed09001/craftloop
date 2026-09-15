# Craft Loop Execution 03 — Web Live View, Creative Sketch Mode & Orthographic Completion

## Document Metadata

**Project:** Craft Loop  
**Execution:** 03  
**Version:** 1.0  
**Created:** 2026-09-15  
**Repository:** `mohammed09001/craftloop`  
**Baseline inspected:** `main` at `93e5bdb2c30919e9a73832b6759c3ed06553ab74`  
**Baseline title:** `Execution 02, Phase 10: dimensions/constraints slice, fix a real selection crash`  
**Product authority:** `Craft Loop MCP V1.md`  
**Previous executions:** `Craft Loop Execution 01.md`, `Craft Loop Execution 02 Androi Tablet Engineering Alpha.md`  
**Primary host:** Windows  
**Primary test surface:** Web Live View in a desktop browser  
**Production platforms preserved:** Android tablets and iPad tablets  
**Physical-device policy:** no tablet hardware validation is required in this execution  
**UI target:** top-centered, icon-first, creative engineering interface  
**Primary UX addition:** Main Creative Mode ⇄ 2D Sketch Mode  
**Command contract:** `S` or `Sketch` enters Sketch Mode; `B` or `Pen` returns to Main Creative Pen mode  
**Final deliverable:** a browser-accessible Craft Loop live engineering harness, backed by the real shared Rust core, that completes linked Orthographic behavior and introduces a professional but creative 2D Sketch environment without turning Craft Loop into dense 3D CAD.

---

# Article 1 — Execution Mission

Execution 03 continues the real repository rather than restarting it.

Execution 02 already produced a working `CraftLoopSession`, Android tooling, Jetpack Ink integration, pan/zoom/selection, an icon-first toolbar, dimensions, constraints, and real solver integration through Phase 10.

The inspected Android source still states that Top/Right linked-view presentation belongs to unfinished Phase 11. The inspected `MainActivity` also places `PrimaryToolbar` below `InkCanvas` in a `Column`, which explains the bottom toolbar seen during Android testing.

Execution 03 changes the validation strategy.

The browser becomes a fast **live engineering and product-design harness**. It exists so the developer can iterate on backend behavior, Sketch Mode, toolbar structure, Orthographic presentation, dimensions, constraints, persistence, and interaction state directly from Windows without repeatedly installing an APK.

The web harness is not a new Craft Loop product.

It is not a replacement for Android.

It is not a replacement for iPad.

It is a universal interaction laboratory backed by the same shared engineering truth.

---

# Article 2 — Goal Mode

Goal Mode dominates every implementation decision.

The final result must allow the developer to open a real Craft Loop Live View in a browser and perform this journey:

1. start one live-development command on Windows;
2. open the browser;
3. see a large canvas and a floating toolbar at the **top center**;
4. edit frontend code and observe HMR without manual rebuilding;
5. edit shared Rust/web-bridge code and automatically rebuild WebAssembly;
6. use the mouse as a simulated pen;
7. draw freehand;
8. press the Sketch icon or use `S` / `Sketch`;
9. see the same top toolbar morph into the dedicated 2D Sketch toolbar;
10. keep Pen available inside Sketch Mode;
11. create lines, arcs, circles, rectangles, and other supported 2D entities;
12. create dimensions and constraints through the real shared core;
13. see contextual constraint availability instead of a permanent wall of tools;
14. use Draw-and-Hold style refinement for freehand geometry where reliable;
15. use `B` / `Pen` to return to the Main Creative toolbar;
16. assign `FRONT`;
17. enter Orthographic;
18. see real linked `TOP` and `RIGHT` View Blocks;
19. keep unknown Depth explicitly unresolved;
20. enter Depth in `TOP`;
21. observe shared propagation to `RIGHT`;
22. introduce a contradictory shared value;
23. see a real cross-view conflict;
24. resolve or cancel it;
25. Undo/Redo;
26. save the shared document through a browser persistence adapter;
27. reload and recover the same semantic state;
28. build a static preview;
29. optionally publish a branch/PR preview URL through Vercel or Cloudflare Pages.

A compiling React app is not success.

A pretty mock is not success.

The shared Rust engine must be exercised.

---

# Article 3 — Plan Mode

Before every phase, the coding agent creates a concise plan containing:

- current repository evidence;
- exact missing behavior;
- source-of-truth backend;
- web-adapter responsibility;
- UI/UX invariant;
- failing test or explicit gap reproduction;
- likely files;
- verification commands;
- rollback boundary.

Plan Mode exists to reduce waste.

It must not become an approval checkpoint.

After a green phase gate, continue automatically unless a true blocker requires human action.

---

# Article 4 — Continuation of Execution 02

Execution 03 absorbs the unfinished platform-neutral work from Execution 02.

**Old Phase 11 — View Identity / Orthographic:** required here and must be completed visually and semantically.

**Old Phase 12 — Persistence / lifecycle:** completed here as shared serialization plus browser persistence and reload.

**Old Phase 13 — physical S Pen diagnostics:** intentionally deferred because this execution does not use a tablet.

**Old Phase 14 — handwriting-to-dimension:** split into a platform-neutral recognition boundary plus browser command/recognition simulation. Native ML Kit validation remains deferred.

**Old Phase 15 — Ink Commands:** completed at Command Bus and interaction-state level in the browser; native handwriting recognition is not falsely claimed.

**Old Phase 16 — Android CI:** still valuable and can be completed without a tablet.

**Old Phase 18 — physical Engineering Alpha acceptance:** replaced by Web Live Acceptance plus shared-core regression acceptance.

**Old Phase 19 — final audit:** completed here with universal-architecture and no-hallucination audits.

---

# Article 5 — Research Angles

This execution is informed by three research angles.

## Scholarly literature

The review includes SketchGraphs, Vitruvion, Free2CAD, and OpenAlex-based research discovery patterns.

SketchGraphs reinforces a relational sketch model: geometric primitives connected by explicit constraints.

Vitruvion reinforces that design intent lives in primitives plus constraints and should survive edits.

Free2CAD shows the value of translating freehand input into structured CAD operations while reducing command complexity for the user.

These papers inspire architecture and future assistance. They do not replace deterministic validation.

## Professional CAD sketch environments

The review includes Onshape, Fusion, and Shapr3D.

These systems show the value of a dedicated contextual Sketch environment, grouped tools, dimensions, constraints, snapping, construction geometry, and selection-aware tool availability.

## Creative sketch applications

The review includes Procreate, Concepts, Linea Sketch, and Morpholio Trace.

These products show how a canvas can remain primary, how precision can appear without overwhelming the user, and how freehand sketching can coexist with editable geometry.

---

# Article 6 — Research Requirements Converted to Craft Loop Rules

From **SketchGraphs / Vitruvion**:

> Sketch geometry and constraints are relational engineering state, not painted pixels.

From **Free2CAD**:

> Freehand interpretation may suggest structured operations, but the user remains in control and deterministic geometry validates the result.

From **Onshape**:

> Sketch is a contextual tool environment. Tool groups should reduce toolbar density. `S` is a natural Sketch shortcut.

From **Fusion**:

> Grid, snap, construction, dimensions, constraints, and contextual options belong to Sketch context, not permanently to the main interface.

From **Shapr3D**:

> Only constraints meaningful for the current selection should be prominent.

From **Procreate**:

> Draw-and-Hold refinement can add precision without making drawing feel like CAD command entry.

From **Concepts**:

> Infinite-canvas thinking, editable vector strokes, snap, guides, scale, and stylus-first creativity can coexist.

From **Linea Sketch**:

> The creation should dominate, not the interface chrome.

From **Morpholio Trace**:

> The speed and beauty of sketching can coexist with CAD intelligence.

---

# Article 7 — Design Doctrine: Creative Precision

The design doctrine is:

**Creative Precision.**

Craft Loop should feel like a sketchbook that understands engineering.

It must not feel like a CAD cockpit hidden inside a notebook.

The user begins with Pen.

Precision appears only when needed.

Sketch Mode adds engineering tools to the creative surface instead of replacing the creative surface.

No 3D cube.

No feature tree.

No permanent property inspector.

No full-width ribbon.

No dense textual toolbar.

No persistent wall of constraints.

No 3D modeling tools in this execution.

---

# Article 8 — Web Live View Is a Harness

Create:

`apps/web-live/`

It is a browser-based engineering and UI/UX test harness.

It must use the shared Rust core.

It may use React and TypeScript for presentation.

It may use browser APIs for input and storage.

It must not create a parallel engineering model.

Future production Android and iPad adapters remain independent native frontends over the same shared semantics.

---

# Article 9 — Preferred Web Stack

Preferred implementation stack:

```text
Vite
React
TypeScript
Rust → WebAssembly
wasm-bindgen
HTML Canvas for transient/freehand ink
SVG overlay for structured engineering geometry
Vitest
Playwright
```

Vite is chosen for fast HMR and a simple static production build.

React is chosen for interaction-state composition.

TypeScript strict mode is required.

The executing agent may deviate only when repository evidence proves a concrete incompatibility.

---

# Article 10 — WebAssembly Bridge

Create a dedicated bridge such as:

`crates/craftloop-web-bridge`

Target:

`wasm32-unknown-unknown`

Use `wasm-bindgen` or another mature Rust/Wasm boundary.

Do not compile the Android UniFFI wrapper itself into the browser.

Reuse the same lower-level domain crates that `CraftLoopSession` already coordinates.

The web bridge should expose a session API conceptually aligned with native `CraftLoopSession`.

---

# Article 11 — Wasm Compatibility Audit

Before writing UI against the bridge, inspect every dependency used by the target session.

Classify:

- pure / Wasm-compatible;
- compatible behind feature flags;
- native filesystem dependent;
- randomness/time dependent;
- thread dependent;
- genuinely incompatible.

Do not fork the engineering engine because persistence currently uses `std::fs`.

Extract or feature-gate adapters.

Document exact changes.

---

# Article 12 — Fallback Bridge

If a specific dependency makes direct Wasm compilation impossible within reasonable scope, an explicit temporary fallback may be used:

```text
Browser
  ↓ HTTP/WebSocket
local Rust session server
  ↓
shared Craft Loop core
```

This fallback must be documented and must not silently become the long-term architecture.

Preferred outcome remains WebAssembly because static internet previews are simpler.

---

# Article 13 — Live Development

Target command:

```powershell
npm run dev:live
```

It should:

- build the current Wasm bridge if needed;
- watch relevant Rust crates;
- start Vite;
- print the local URL;
- rebuild Wasm on Rust changes;
- allow Vite HMR for frontend changes;
- reload the Wasm module when a Rust rebuild succeeds;
- surface Rust build failures clearly;
- never continue silently with stale Wasm after a failed backend rebuild.

---

# Article 14 — Internet Preview

The harness must support:

```powershell
npm run build
```

The output should be deployable as a static preview when direct Wasm is used.

Document at least one Git-connected preview provider path.

Recommended options:

- Vercel Preview Deployments;
- Cloudflare Pages Preview Deployments.

The repository must remain provider-neutral.

A provider login/authorization is a human setup step, not a reason to hard-code vendor logic into the app.

---

# Article 15 — Browser Input

Use Pointer Events.

The web harness must record or preserve:

- `pointerType`;
- pointer ID;
- position;
- timestamp;
- pressure where available;
- tilt where available;
- buttons.

Mouse is the default **simulated pen** on Windows.

Mouse input is not real S Pen validation.

Browser pen data is not proof of Android or Apple Pencil native behavior.

---

# Article 16 — Rendering Layers

Use a layered canvas architecture.

**Background layer:** paper/grid.

**Transient ink layer:** HTML Canvas for high-frequency drawing.

**Structured geometry layer:** SVG or equivalent vector overlay.

**Interaction overlay:** snapping cues, selection handles, dimension entry, conflict feedback, command confirmation.

**Toolbar layer:** top-center floating surface.

All geometry layers share one viewport transform.

---

# Article 17 — Toolbar Placement

The primary toolbar must be:

**top center**

not bottom.

The current Android baseline places `PrimaryToolbar` after `InkCanvas` inside a vertical `Column`. That bottom placement is explicitly superseded as the target interaction design.

The new toolbar:

- floats over the canvas;
- is horizontally centered;
- remains compact;
- avoids resizing the canvas;
- respects safe areas;
- does not become a full-width ribbon.

---

# Article 18 — Main Creative Toolbar

Required semantic actions:

```text
Pen
Sketch
Select
Eraser
View / Orthographic entry where appropriate
Undo
Redo
Save
More
```

Pen and Sketch receive strongest visual priority.

History actions may be visually quieter.

The toolbar is icon-first.

No persistent tool-name sentences.

---

# Article 19 — Sketch Mode

Sketch Mode is a dedicated **2D engineering sketch** context.

It is not 3D.

It is not a new document.

It is not a destructive conversion.

It changes:

- active toolbar toolset;
- command namespace;
- contextual precision tools;
- constraint availability;
- snap/inference behavior;
- sketch overlays.

It does not create bodies or expose 3D feature history.

---

# Article 20 — Entering Sketch Mode

All entry surfaces dispatch one semantic action.

Supported:

- Sketch icon;
- keyboard `S`;
- command `S`;
- command `Sketch`.

For on-canvas handwriting, the product rule remains:

**Write the intent. Circle to commit.**

Because this execution does not validate a native handwriting engine, the browser may include a developer command injector that simulates recognized text.

The simulator must use the real command resolver and Command Bus.

---

# Article 21 — Returning to Main Creative Mode

Supported:

- Pen icon;
- keyboard `B`;
- command `B`;
- command `Pen`.

`B` is a reserved explicit alias for the Pen/Brush return action.

It must not depend on shortest-prefix guessing.

Returning to Main Creative Mode keeps:

- document geometry;
- dimensions;
- constraints;
- Orthographic state;
- history.

It simply restores the Main Creative toolbar and activates Pen.

---

# Article 22 — Shared Interaction State

Do not implement mode semantics independently in every frontend.

Formalize a platform-neutral interaction contract.

Conceptual state:

```text
WorkspaceMode
  Creative
  Sketch2D
```

Session interaction state may also contain:

- active tool;
- last-used tool in grouped families;
- active selection;
- command namespace;
- snap/inference preferences.

Toolbar popover visibility remains frontend-local.

Engineering document state remains separate.

---

# Article 23 — Command Bus Unification

These inputs must produce the same action:

```text
Sketch icon
keyboard S
recognized S
recognized Sketch
```

Likewise:

```text
Pen icon
keyboard B
recognized B
recognized Pen
```

Source may be logged.

Meaning must be identical.

Do not duplicate mode side effects.

---

# Article 24 — Sketch Toolbar

The Sketch toolbar replaces or morphs from the Main toolbar in the same top-center anchor.

Primary concept:

```text
Pen | Line | Arc | Circle | Rectangle | Dimension | Constraint | Construction | Snap/Guide | More
```

Undo/Redo stay reachable.

The bar should feel like the same creative instrument revealing a precision layer.

---

# Article 25 — Grouped Tool Families

Avoid an icon wall.

Use grouped families inspired by professional CAD.

**Line:** Line; Polyline only if real backend support exists.

**Arc:** expose constructions that match real `Arc2` semantics.

**Circle:** center-radius first; variants only when implemented.

**Rectangle:** corner rectangle first; center rectangle only when real.

**Curves:** Ellipse can be exposed because the inspected geometry kernel contains `Ellipse2`.

**Spline:** do not display a Spline tool unless a tested spline representation is added. The inspected geometry kernel currently has Arc and Ellipse modules but no spline module.

Group icons may remember the last-used variant.

---

# Article 26 — Pen Stays in Sketch Mode

Pen is a first-class Sketch tool.

The user can:

- explore freehand;
- annotate;
- draw rough profiles;
- invoke refinement;
- switch to explicit geometry only when needed.

Sketch Mode should communicate:

> “my pen gained precision”

not:

> “I left the sketchbook.”

---

# Article 27 — Draw-and-Hold Refinement

Prototype a QuickShape-inspired Draw-and-Hold behavior.

1. draw;
2. keep pointer down briefly after finishing;
3. run real recognition/beautification;
4. show ghost structured candidate;
5. release/confirm to commit;
6. cancel to keep raw ink.

Do not copy Procreate branding or implementation.

Use the interaction principle only.

Low-confidence input should remain raw.

---

# Article 28 — Explicit Geometry Preview

Explicit geometry tools need live preview.

**Line:** rubber-band segment.

**Circle:** center/radius preview.

**Rectangle:** live bounds.

**Arc:** understandable multi-point preview based on actual `Arc2` model.

**Ellipse:** real kernel-driven preview.

Only final validated geometry enters history.

---

# Article 29 — Dimensions

Dimensions remain relationships, not labels.

In Sketch Mode:

- selection can make Dimension contextually prominent;
- numeric entry appears near relevant geometry;
- annotation stays visually quiet by default;
- invalid input preserves last valid geometry;
- conflict feedback is local and explicit.

Do not maintain a TypeScript-only dimension store.

---

# Article 30 — Adaptive Constraints

Constraint choices should depend on selected entity types and arity.

Do not repeat these rules in React, Kotlin, and future Swift.

Prefer backend-provided eligibility or one platform-neutral rule source.

One line may expose Horizontal/Vertical.

Two lines may expose Parallel/Perpendicular/Equal Length.

Circles/arcs may expose Concentric/Equal Radius/Tangent when real engine support exists.

Impossible choices are hidden or disabled.

---

# Article 31 — Constraint Visibility

Default:

- show selected/relevant constraints;
- show conflicts;
- keep unrelated badges quiet.

Optional:

- Show All Constraints;
- Show Dimensions.

Under-constrained is not an error.

Conflict is an error state.

---

# Article 32 — Construction Geometry

Construction/reference geometry must be semantic, not merely dashed CSS.

It may:

- participate in constraints;
- guide geometry;
- remain selectable;
- render differently;
- not count as ordinary profile boundary.

If the current domain lacks construction state, add the smallest platform-neutral representation.

---

# Article 33 — Snap and Inference

Provide ephemeral suggestions for:

- endpoint;
- midpoint;
- horizontal;
- vertical;
- coincident;
- center;
- alignment;
- tangent where reliable.

Inference should not silently create many permanent constraints.

Guides suggest.

Explicit policy commits.

---

# Article 34 — Grid and Precision

Provide a compact Sketch precision popover.

Possible controls:

- Grid visible;
- Grid snap;
- Geometry snap;
- Inference guides;
- Show all dimensions;
- Show all constraints.

Default grid should remain subtle or off.

No large settings panel.

---

# Article 35 — Selection

Hover may gently highlight geometry in the web harness.

Click selects.

Shift-click can multi-select.

Selection IDs must remain domain IDs.

Dragging geometry must use constraint-aware semantic operations, not direct SVG mutation.

---

# Article 36 — Toolbar Motion

When entering Sketch Mode:

- toolbar anchor does not move;
- icons crossfade/morph;
- width changes modestly;
- motion is short;
- reduced-motion preference is respected.

No decorative animation may delay tool access.

---

# Article 37 — Orthographic Completion

Execution 03 must complete the currently missing linked-view presentation.

Required:

- Front;
- Top;
- Right;
- Back when relevant;
- View Identity;
- shared axes;
- readiness;
- unresolved state;
- propagation;
- cross-view conflict;
- selection within a View Block.

Use the existing multiview/Orthographic engine.

Do not create decorative fake views.

---

# Article 38 — Orthographic Layout

Default live layout:

```text
          TOP

FRONT              RIGHT
```

These are 2D View Blocks.

No perspective.

No orbit.

No shaded solid.

No 3D cube.

Moving the blocks visually must not change engineering relationships.

---

# Article 39 — Unresolved State

Unknown Depth stays unknown.

Possible visual treatment:

- ghost extent;
- dashed placeholder;
- subtle question marker;
- unresolved dimension badge.

Never generate a plausible number merely to make the drawing look complete.

---

# Article 40 — Shared Propagation

Acceptance must prove:

- Width from Front can appear in Top;
- Depth from Top can appear in Right;
- Height from Right can appear in Front.

No permanent master view.

Updates should appear atomically after one semantic transaction.

---

# Article 41 — Cross-View Conflict

Example:

```text
Front Width = 100
Top attempts Width = 130
```

Required:

- do not create two independent truths;
- preserve valid state until resolution;
- show a real conflict;
- identify affected views/entities;
- offer only valid resolution choices;
- keep Undo coherent.

---

# Article 42 — Browser Persistence

The browser harness must support Save and reload.

Do not invent a second document format.

Serialize the shared Craft Loop document.

Use a browser adapter such as IndexedDB for test persistence.

UI preferences are stored separately from engineering semantics.

---

# Article 43 — Path-Independent Serialization

If persistence currently only accepts native paths, separate serialization from filesystem IO.

The shared document model should be serializable to/from bytes/string independent of platform.

Native file adapter remains native.

Web storage adapter remains web.

---

# Article 44 — Undo/Redo

Undo/Redo uses shared transaction history.

Do not add a React engineering-history stack.

Workspace-mode changes normally do not enter engineering document history.

Geometry, dimensions, constraints, propagation, and conflict resolution do.

---

# Article 45 — Web Command Simulator

Create a developer-only command simulator.

Inputs may include:

- recognized text;
- source;
- confirmation evidence;
- namespace.

It must call the real resolver / Command Bus.

Use it to validate:

```text
S
Sketch
B
Pen
```

and ambiguous inputs.

It is not proof of real handwriting recognition.

# Article 46 — Keyboard Shortcuts

Web Live View may use desktop shortcuts for fast iteration:

- `S` → Sketch Mode when text input is not focused;
- `B` → Creative Pen mode when text input is not focused;
- `Esc` → cancel active transient operation;
- `Ctrl/Cmd+Z` → Undo;
- `Ctrl/Cmd+Shift+Z` → Redo.

Keyboard shortcuts are a browser convenience.

They do not redefine the pen-first product.

---

# Article 47 — Interaction Source Independence

A semantic action must not change meaning based on source.

The system may record:

```text
toolbar
keyboard
simulated-ink
future-native-ink
```

for diagnostics.

The action result must remain the same.

---

# Article 48 — Web UI Test Strategy

Use component/unit tests and end-to-end tests.

Vitest or equivalent should cover:

- tool registry;
- mode reducer;
- command routing;
- snapshot mapping;
- persistence adapter;
- selection/eligibility helpers.

Playwright should cover:

- top-center toolbar;
- Main → Sketch;
- `S` enters Sketch;
- `B` returns to Pen;
- geometry creation;
- dimension;
- constraint;
- Orthographic entry;
- unresolved Depth;
- propagation;
- conflict;
- save/reload;
- Undo/Redo.

Screenshots support tests.

They do not replace semantic assertions.

---

# Article 49 — Visual Regression

Keep a small stable visual suite.

Recommended captures:

- Main Creative toolbar;
- Sketch toolbar;
- selected geometry;
- dimension input;
- constraint popover;
- unresolved Orthographic state;
- resolved Orthographic state;
- conflict state.

Avoid brittle pixel-perfect comparisons for antialiasing noise.

Prefer structure/layout assertions plus selected screenshots.

---

# Article 50 — Backend Regression

All existing Rust tests remain green.

Add tests for:

- WorkspaceMode transitions;
- command aliases;
- web bridge;
- path-independent serialization;
- construction geometry if added;
- constraint eligibility if centralized;
- Orthographic snapshot completeness;
- cross-view conflict data.

Never weaken existing tests merely to make Wasm compile.

---

# Article 51 — Android CI Without Tablet

Execution 03 should complete the missing Android CI work because it does not require physical hardware.

Required Android CI:

- Java setup;
- Android SDK;
- pinned NDK;
- Rust Android target;
- Rust ARM64 cross-compile;
- UniFFI Kotlin generation;
- `assembleDebug`;
- Android lint;
- JVM/unit tests.

Do not claim S Pen runtime correctness.

---

# Article 52 — Native Toolbar Parity Patch

After the web design passes acceptance, apply the interaction contract to Android Compose.

Required compile-only parity:

- primary toolbar moves from bottom to top-center floating placement;
- Main/Sketch mode structure is represented if shared interaction state is ready;
- semantic tool actions remain the same;
- Android builds and lints.

No physical tablet runtime test is required in this execution.

Do not copy CSS values literally into Compose.

Port the interaction design, not the browser implementation.

---

# Article 53 — iPad Preservation

No iPad UI is implemented here.

The execution must maintain this mapping:

```text
Web toolbar      → native toolbar interaction contract
Pointer Events   → native Pencil/Touch adapter
Wasm bridge      → native UniFFI session
IndexedDB        → native document persistence adapter
```

The web harness must never become a dependency of the iPad product.

---

# Article 54 — Accessibility

Even the harness establishes good semantics.

Required:

- accessible icon names;
- keyboard focus;
- visible focus state;
- active state beyond color;
- warning state beyond color;
- reduced motion;
- sufficient target sizes;
- understandable conflict text.

Final native accessibility remains platform-specific.

---

# Article 55 — Visual Language

Do not create a final brand system.

Use:

- restrained neutral surfaces;
- one primary accent;
- clear but calm warning treatment;
- subtle construction geometry;
- human-looking freehand ink;
- precise structured geometry.

Under-constrained geometry must not look like an error.

Conflict must be distinct from incompleteness.

---

# Article 56 — Creative Surface Rules

The canvas invites drawing.

Avoid:

- dominant coordinate axes;
- heavy grid by default;
- permanent side inspectors;
- crowded status bars;
- always-visible advanced settings.

If the user is simply using Pen, CAD chrome should recede.

---

# Article 57 — Constraint Micro-Feedback

When a likely relationship is detected, show a small temporary cue close to the pointer.

Examples:

- H/V;
- midpoint;
- coincident;
- center.

Temporary inference cues disappear.

Only committed constraints remain semantic state.

---

# Article 58 — Dimension Micro-Feedback

During dimensioning:

- highlight target geometry lightly;
- show candidate measurement near pointer;
- place numeric entry near context;
- commit through the real dimension engine;
- settle annotation into a readable position;
- keep conflict feedback local first.

---

# Article 59 — No 3D Contamination

Sketch Mode must not add:

- Extrude;
- Revolve;
- Loft;
- Shell;
- Fillet;
- 3D transform gizmos;
- 3D camera controls;
- solid/surface tabs.

Craft Loop remains 2D engineering sketch + Orthographic.

Future Craft 01 integration is separate.

---

# Article 60 — Literature-Informed ML Boundary

SketchGraphs, Vitruvion, and Free2CAD point toward future opportunities such as:

- constraint suggestion;
- sketch autocompletion;
- freehand command interpretation.

Execution 03 does not make learned models authoritative.

Any prototype must:

- suggest;
- rank;
- allow rejection;
- never silently commit;
- remain optional;
- pass deterministic validation.

---

# Article 61 — Performance

Measure:

- first load;
- Wasm initialization;
- pointer-to-transient-ink latency;
- commit-to-structured-result latency;
- snapshot conversion;
- constraint solve;
- Orthographic propagation;
- frame stability;
- save/reload.

The browser is a harness.

Do not assume native performance from web results.

---

# Article 62 — HMR and Rust Reload

Frontend HMR must be fast.

Rust changes require reliable Wasm rebuild/reload.

The development environment must:

- watch relevant Rust crates;
- rebuild development Wasm;
- tell Vite/browser the module changed;
- expose build errors;
- prevent stale Wasm from being mistaken for updated backend semantics.

---

# Article 63 — Public Preview Safety

The Web Live View is a development surface.

Do not put sensitive engineering documents into a public preview.

Do not embed secret keys.

Do not add analytics by default.

Use preview `noindex` behavior where provider supports it.

Private preview access can be added later if desired.

---

# Article 64 — Repository Structure

Recommended additions:

```text
apps/
  web-live/
    src/
    public/
    tests/
    package.json
    vite.config.ts
    playwright.config.ts

crates/
  craftloop-web-bridge/
    src/

scripts/
  web-live.ps1

execution-evidence/
  execution-03/
```

Repository evidence may justify a slightly different layout.

Do not reorganize unrelated crates.

---

# Article 65 — No-Hallucination Contract

Never claim:

- handwriting works because the command simulator works;
- S Pen works because Pointer Events work;
- Apple Pencil works because mouse input works;
- Android runtime works because CI builds;
- WebAssembly uses real core semantics while frontend geometry is mocked;
- Orthographic is complete if Top/Right are decorative frames;
- a constraint exists if only an icon is painted;
- persistence works if only React state survives;
- Sketch Mode is universal if logic exists only in React;
- internet preview exists before a real URL is produced.

Evidence outranks confidence.

---

# Article 66 — Context Engineering Contract

Use progressive context.

For each task, retrieve only:

1. current repository files;
2. relevant section of this execution;
3. relevant MCP V1 requirements;
4. relevant Execution 02 evidence;
5. failing test/log;
6. relevant research source if needed.

Do not flood subagents with the entire Execution 01 file.

Repository evidence is authoritative.

---

# Article 67 — Prompt Engineering Contract

Every delegated coding prompt should contain:

**Goal**  
**Why now**  
**Current evidence**  
**Source-of-truth modules**  
**Allowed scope**  
**Forbidden shortcuts**  
**Backend invariant**  
**UI/UX invariant**  
**Universal-software invariant**  
**Failing evidence**  
**Verification commands**  
**Visual evidence if relevant**  
**Completion condition**

Avoid prompts such as:

> Make the Sketch toolbar better.

Prefer:

> Implement the top-centered Sketch Mode toolbar in `apps/web-live`, driven by the shared interaction-mode contract. Keep Pen available; expose grouped Line/Arc/Circle/Rectangle/Dimension/Constraint/Construction tools; route actions through the real Wasm CraftLoop session; add Playwright tests for `S` entering Sketch and `B` returning to Creative mode.

---

# Article 68 — Loop Engineering Contract

Every task follows:

**Inspect → Define invariant → Reproduce gap → Test → Implement smallest coherent change → Run focused checks → Run neighboring checks → Inspect browser → Record evidence → Continue**

For design work:

**Research → hypothesis → live implementation → inspect → reduce clutter → semantic verification**

Do not let design work bypass backend correctness.

Do not let backend work create unusable interaction.

---

# Article 69 — True Blockers

True blockers include:

- a shared dependency proven impossible to compile to Wasm with no reasonable adapter;
- incompatible dependency license;
- contradiction in document format or interaction authority;
- an architecture change that would create two engineering truths;
- preview-provider authorization that only the human can grant.

Not blockers:

- first Wasm compile failure;
- CSS bugs;
- Vite configuration issues;
- browser cache;
- missing physical tablet;
- incomplete icon polish;
- Playwright failure.

This execution intentionally does not require physical hardware.

---

# Article 70 — Evidence Taxonomy

Record evidence as one of:

- Rust unit test;
- Rust scenario test;
- Wasm bridge test;
- frontend unit test;
- Playwright test;
- screenshot;
- local HMR observation;
- public preview URL;
- Android compile CI;
- deferred native hardware validation.

Do not mix evidence classes.

---

# Article 71 — Execution Branch

Recommended:

`execution/03-web-live-sketch`

Use a worktree if useful.

Do not run a large unreviewed execution directly on `main`.

Commit green milestones.

---

# Article 72 — Commit Strategy

Example milestone commits:

```text
build(web): scaffold Vite/Wasm live harness
feat(web): connect real CraftLoop session
feat(ui): move universal toolbar to top-center
feat(sketch): add creative Sketch workspace mode
feat(sketch): add grouped precision tools
feat(ortho): render linked orthographic views
feat(persist): save and reload web documents
test(web): add live acceptance coverage
ci: add web and Android compile gates
```

No commit message may claim hardware validation.

---

# Article 73 — Definition of a Real Web Live View

The Live View is real only if:

- actions call shared Rust semantics;
- constraints come from the real engine;
- dimensions come from the real engine;
- Orthographic state comes from the real engine;
- conflicts come from the real consistency engine;
- serialization uses the shared document;
- undo/redo uses shared transaction history.

A beautiful React mock is not acceptable.

---

# Article 74 — Definition of Sketch Mode Success

Sketch Mode succeeds when:

- icon/S/Sketch can enter it;
- B/Pen can leave it;
- toolbar stays top-center;
- Pen remains available;
- tool groups reduce clutter;
- live previews work;
- dimensions/constraints remain semantic;
- mode transition does not alter document meaning;
- the experience still feels creative.

---

# Article 75 — Definition of Orthographic Completion

Required flow:

```text
Front
↓
enter Orthographic
↓
Top + Right visible
↓
Depth unresolved
↓
set Depth in Top
↓
Right updates
↓
contradict shared value
↓
Conflict
↓
resolve/cancel
↓
Undo/Redo
↓
Save
↓
Reload
↓
State restored
```

No 3D reconstruction is required.

---

# Article 76 — Definition of Internet Preview Completion

Internet preview is complete when:

- `npm run build` succeeds;
- current Wasm is included;
- provider build succeeds;
- a preview URL exists;
- opening it loads the real Live View;
- a core smoke flow works there;
- no localhost-only backend dependency remains if direct Wasm is the chosen architecture.

If authorization is the only missing piece, report `BLOCKED_BY_HUMAN_AUTHORIZATION`.

---

# Article 77 — Native Validation Deferral

This execution intentionally does not validate:

- real S Pen pressure;
- Android palm rejection;
- Android hover;
- Android ML Kit handwriting;
- real Apple Pencil pressure;
- Apple Pencil tilt/hover;
- iPad runtime layout.

These remain future native-validation gates.

Do not mark them complete.

---

# Article 78 — Design Review Questions

Every UI review should ask:

- Is the toolbar still top-center?
- Does the canvas dominate?
- Does Sketch Mode feel like added precision rather than a different application?
- Is Pen still easy to reach?
- Are advanced constraints hidden until context makes them relevant?
- Are dimensions readable without overwhelming the canvas?
- Are under-constrained states calm?
- Are conflicts explicit?
- Is any 3D visual language leaking into the Sketch experience?
- Can a future Android/iPad adapter reproduce the semantic behavior?

---

# Article 79 — Backend Review Questions

Every backend review should ask:

- Is there one engineering truth?
- Did TypeScript duplicate geometry or solver logic?
- Did browser persistence invent a second format?
- Are interaction modes separate from document semantics?
- Are command aliases deterministic?
- Are constraint eligibility rules centralized?
- Can the same action be triggered from native adapters later?
- Did Wasm changes weaken native behavior?
- Are Orthographic relationships still graph-based rather than layout-based?

---

# Article 80 — Execution Continuity

Do not stop after:

- first browser page;
- first Wasm call;
- first top toolbar;
- first Sketch transition;
- first Orthographic frame.

Continue until the full Web Live Acceptance Gate is executed or a true blocker is documented.


# Article 81 — Detailed Phase Task Cards

# Phase 00 — Re-Audit and Execution Lock

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 001 — Pin current repository state

**Objective:** Record branch, commit, dirty state, current Android toolbar placement, current CraftLoopSession API, current tests, CI, and unfinished Execution 02 work.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 002 — Run Rust baseline

**Objective:** Run fmt, Clippy, workspace tests, and scenario tests before Web/Wasm changes.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 003 — Map unfinished Execution 02 work

**Objective:** Create a ledger mapping old Phases 11/12/14/15/16/18/19 to this execution and explicitly defer only hardware-specific gates.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 004 — Create evidence area

**Objective:** Create `execution-evidence/execution-03/` for baseline, research, architecture, browser, preview, CI, and final-report evidence.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 005 — Create execution branch

**Objective:** Use `execution/03-web-live-sketch` or an equivalent isolated worktree branch.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 01 — Research-to-Design Contract

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 006 — Summarize scholarly findings

**Objective:** Turn SketchGraphs, Vitruvion, Free2CAD, and OpenAlex research-discovery findings into backend/interaction constraints.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 007 — Study Onshape Sketch UX

**Objective:** Record contextual toolbar, grouped tools, top placement, shortcut behavior, dimensions and constraints.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 008 — Study Fusion Sketch UX

**Objective:** Record contextual Sketch environment, Sketch Palette, grid, snap, construction, dimension/constraint visibility.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 009 — Study Shapr3D Sketch UX

**Objective:** Record adaptive constraints, direct manipulation, icon-first access, under-defined/fully-defined feedback.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 010 — Study creative sketch apps

**Objective:** Record Procreate QuickShape, Concepts precision tools, Linea minimal UI, Morpholio Trace creative-CAD balance.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 011 — Write Creative Precision contract

**Objective:** Create a concise design-contract artifact used by every frontend phase.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 02 — Web Workspace Scaffold

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 012 — Create apps/web-live

**Objective:** Create React/TypeScript/Vite app under the existing `apps` structure.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 013 — Configure strict TypeScript

**Objective:** Enable strict typing and appropriate linting.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 014 — Create scripts

**Objective:** Add `dev`, `dev:live`, `build`, `test`, and `preview` commands.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 015 — Create full-canvas shell

**Objective:** Render a blank canvas with no fake engineering objects.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 016 — Create top-center toolbar host

**Objective:** Reserve the final toolbar anchor before tool implementation.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 017 — Set up tests

**Objective:** Configure Vitest and Playwright with a smoke test.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 03 — Rust/Wasm Compatibility Audit

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 018 — Audit shared dependencies

**Objective:** Classify filesystem, randomness, time, threading, and platform assumptions.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 019 — Create craftloop-web-bridge

**Objective:** Add a dedicated Rust web bridge rather than compiling the Android UniFFI wrapper.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 020 — Compile first real shared-core call

**Objective:** Prove `wasm32-unknown-unknown` works for a genuine domain operation.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 021 — Feature-gate native adapters

**Objective:** Isolate path/filesystem or native-only behavior without forking semantics.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 022 — Document compatibility changes

**Objective:** Record every shared-crate change and why it is safe for native targets.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 04 — Web CraftLoopSession

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 023 — Create browser session

**Objective:** Expose a session object aligned conceptually with native CraftLoopSession.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 024 — Expose scene snapshot

**Objective:** Return real presentation data from Rust.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 025 — Expose stroke submission

**Objective:** Route simulated pen strokes through real recognition.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 026 — Expose primitive creation

**Objective:** Line/Circle/Rectangle plus Arc/Ellipse when supported.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 027 — Expose dimensions

**Objective:** Create/edit real semantic dimensions.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 028 — Expose constraints

**Objective:** Apply/remove/solve through the real sketch engine.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 029 — Expose View Identity

**Objective:** Assign principal views.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 030 — Expose Orthographic transition

**Objective:** Enter real multiview state.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 031 — Expose shared-axis operations

**Objective:** Read/write Width/Height/Depth through the multiview graph.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 032 — Expose conflicts

**Objective:** Query and resolve real conflicts.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 033 — Expose undo/redo

**Objective:** Use shared history.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 034 — Expose serialization

**Objective:** Serialize/deserialize without a native filesystem path.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 05 — Scene Snapshot Enrichment

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 035 — Audit snapshot completeness

**Objective:** List missing geometry, dimensions, constraints, View Blocks, shared axes, unresolved state, and conflicts.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 036 — Add geometry summaries

**Objective:** Expose coordinates for Segment/Circle/Arc/Ellipse/Rectangle rendering.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 037 — Add dimension summaries

**Objective:** Expose anchors, value, role, and state.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 038 — Add constraint summaries

**Objective:** Expose real relation metadata for badges and contextual UI.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 039 — Add View Block summaries

**Objective:** Expose identity, members, readiness, and layout-neutral data.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 040 — Add Orthographic summaries

**Objective:** Expose linked set, shared axes, unresolved/confirmed state.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 041 — Add conflict summaries

**Objective:** Expose involved IDs, kind, status, and allowed resolutions.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 042 — Keep read-model discipline

**Objective:** Prove frontend cannot mutate snapshot as engineering truth.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 06 — Browser Canvas Stack

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 043 — Implement viewport transform

**Objective:** Use one transform for Canvas and SVG layers.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 044 — Implement transient ink Canvas

**Objective:** Mouse/pen drawing appears immediately.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 045 — Implement structured SVG layer

**Objective:** Render real engineering geometry and annotations.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 046 — Implement Pointer Events adapter

**Objective:** Mouse simulates pen; preserve source/pressure/tilt when browser provides them.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 047 — Implement pan/zoom

**Objective:** Change viewport only.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 048 — Implement semantic hit testing

**Objective:** Selection returns stable domain IDs.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 049 — Implement selection handles

**Objective:** Keep them lightweight and platform-neutral.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 07 — Main Toolbar Top-Center Redesign

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 050 — Build floating toolbar surface

**Objective:** Top-center, compact, over the canvas.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 051 — Eliminate bottom-primary-toolbar assumption

**Objective:** Web target never places the primary toolbar below the canvas.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 052 — Create main tool registry

**Objective:** Pen, Sketch, Select, Eraser, View/Ortho as appropriate, Undo, Redo, Save, More.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 053 — Create icon-first buttons

**Objective:** No persistent text labels; accessible names/tooltips.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 054 — Add active/disabled states

**Objective:** Accessible and not color-only.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 055 — Run canvas-dominance review

**Objective:** Ensure toolbar does not become a ribbon.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 08 — Shared Interaction Mode and Commands

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 056 — Add WorkspaceMode

**Objective:** Model Creative and Sketch2D as session interaction state.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 057 — Add EnterSketchMode

**Objective:** One semantic action shared by all entry sources.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 058 — Add EnterCreativePenMode

**Objective:** One semantic action shared by all return sources.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 059 — Reserve aliases

**Objective:** `S`/`Sketch` and `B`/`Pen` behave deterministically.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 060 — Wire keyboard

**Objective:** S/B work when text input is not focused.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 061 — Wire toolbar actions

**Objective:** Buttons dispatch the same actions.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 062 — Wire command simulator

**Objective:** Injected recognized text uses the real resolver/Command Bus.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 063 — Test history separation

**Objective:** Mode switching does not mutate engineering document history.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 09 — Creative Sketch Toolbar

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 064 — Implement toolbar morph

**Objective:** Replace Main tools in the same top-center anchor.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 065 — Keep Pen in Sketch mode

**Objective:** Pen remains first-class.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 066 — Add Line family

**Objective:** Only real supported variants.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 067 — Add Arc family

**Objective:** Match existing Arc2 semantics.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 068 — Add Circle family

**Objective:** Center-radius default; variants only when real.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 069 — Add Rectangle family

**Objective:** Corner rectangle default; variants only when real.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 070 — Add Ellipse

**Objective:** Expose because the current kernel includes Ellipse2.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 071 — Handle Spline honestly

**Objective:** Do not show it unless a tested spline kernel is added.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 072 — Add Dimension

**Objective:** Real semantic action.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 073 — Add Constraint

**Objective:** Selection-adaptive control.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 074 — Add Construction

**Objective:** Only after semantic backend support.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 075 — Add Snap/Guide

**Objective:** Compact precision options.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 076 — Add More

**Objective:** Only infrequent real tools.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 077 — Keep history actions reachable

**Objective:** Undo/Redo remain accessible.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 10 — Direct Geometry Preview and Creative Refinement

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 078 — Add Line preview

**Objective:** Rubber-band preview, validated Segment2 commit.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 079 — Add Circle preview

**Objective:** Center/radius preview.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 080 — Add Rectangle preview

**Objective:** Live bounds preview.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 081 — Add Arc interaction

**Objective:** Simple real construction matching Arc2.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 082 — Add Ellipse interaction

**Objective:** Real kernel-driven preview.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 083 — Add Draw-and-Hold timer

**Objective:** Detect hold at end of stroke.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 084 — Show ghost refined candidate

**Objective:** Use real recognizer/beautifier.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 085 — Confirm/cancel candidate

**Objective:** No silent conversion.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 086 — Preserve raw ink fallback

**Objective:** Low-confidence or canceled refinement remains ink.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 11 — Precision Inference and Construction

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 087 — Add inference candidate API

**Objective:** Centralize endpoint, midpoint, horizontal, vertical, coincident, center, alignment, and reliable tangent suggestions.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 088 — Render ephemeral guides

**Objective:** Show temporary cues near the pointer and remove them when no longer relevant.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 089 — Implement conservative auto-constraint policy

**Objective:** Do not silently create dense permanent relations.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 090 — Add construction geometry semantic state

**Objective:** Reference geometry is real backend state, not only dashed CSS.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 091 — Render construction geometry

**Objective:** Use a quieter but clear visual treatment.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 092 — Add grid control

**Objective:** Optional subtle grid.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 093 — Add snap control

**Objective:** Separate grid snap, geometry snap, and inference guides where practical.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- `npm run build` stays green after the web build pipeline exists.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 12 — Dimensions and Adaptive Constraints UX

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 094 — Implement geometry-adjacent dimension input

**Objective:** Place numeric entry near selection instead of a distant global panel.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 095 — Render real dimension annotations

**Objective:** Use scene-snapshot semantic data.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 096 — Implement dimension edit

**Objective:** Use the real shared dimension engine.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 097 — Render invalid dimension conflict

**Objective:** Keep last valid geometry and show explanation.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 098 — Centralize constraint eligibility

**Objective:** Avoid frontend-specific arity/type rule duplication.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 099 — Implement adaptive constraint popover

**Objective:** Emphasize only relations applicable to current selection.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 100 — Implement solver feedback

**Objective:** No visual-only success state.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 101 — Add visibility toggles

**Objective:** Show All Dimensions/Constraints only on demand.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- `npm run build` stays green after the web build pipeline exists.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 13 — Orthographic Linked-View Completion

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 102 — Render Front View Block

**Objective:** Use real View Identity and member geometry.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 103 — Render Top View Block

**Objective:** Use real linked-view state.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 104 — Render Right View Block

**Objective:** Use real linked-view state.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 105 — Render Back conditionally

**Objective:** Only when supported and never fabricate unseen details.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 106 — Lay out as 2D engineering regions

**Objective:** No 3D viewport language.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 107 — Render readiness/unresolved state

**Objective:** Depth remains explicitly unknown when unknown.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 108 — Implement shared-axis entry

**Objective:** Edit Width/Height/Depth through the real multiview graph.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 109 — Propagate shared values atomically

**Objective:** Update linked views from one semantic transaction.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 110 — Create cross-view conflict

**Objective:** Attempt a contradictory shared value and surface the real conflict.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 111 — Implement conflict resolution UI

**Objective:** Offer only resolution choices supported by the core.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 112 — Prove no permanent master view

**Objective:** Enter a shared value from a non-Front view and observe correct propagation.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- `npm run build` stays green after the web build pipeline exists.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 14 — Persistence and Reload

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 113 — Extract path-independent serialization

**Objective:** Separate document serialization from native filesystem paths if required.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 114 — Create IndexedDB adapter

**Objective:** Store the shared document payload in browser storage.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 115 — Create New/Open/Save harness flow

**Objective:** Simple document management only; no full production library.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 116 — Implement autosave

**Objective:** Debounce after committed semantic transactions, never pointer moves.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 117 — Run reload acceptance

**Objective:** Refresh/close/reopen and recover semantic state.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 118 — Keep UI preferences separate

**Objective:** Do not serialize temporary popovers into engineering document state.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- `npm run build` stays green after the web build pipeline exists.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 15 — Ink Command and Command-Simulator Completion

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 119 — Create command simulator

**Objective:** Developer-only text/source/confirmation input routed through real Command Bus.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 120 — Validate S

**Objective:** Reserved alias enters Sketch2D.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 121 — Validate Sketch

**Objective:** Full name enters Sketch2D.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 122 — Validate B

**Objective:** Reserved alias enters Creative Pen.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 123 — Validate Pen

**Objective:** Full name enters Creative Pen.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 124 — Validate confirmation policy

**Objective:** Simulated ink source respects confirmation requirements.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 125 — Validate ambiguity

**Objective:** Ambiguous prefixes never guess.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 126 — Validate toolbar parity

**Objective:** Toolbar and command actions produce the same semantic state.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 127 — Document handwriting deferral

**Objective:** Do not claim native handwriting from web simulation.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- `npm run build` stays green after the web build pipeline exists.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 16 — Creative UX Hardening

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 128 — Run clutter audit

**Objective:** Remove controls that do not require permanent visibility.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 129 — Run mode-transition audit

**Objective:** Sketch must feel like precision added, not a new application.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 130 — Run creative-surface audit

**Objective:** Pen-first behavior remains dominant.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 131 — Run CAD-professionalism audit

**Objective:** Precision tools remain credible and discoverable.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 132 — Run mouse/keyboard ergonomics audit

**Objective:** Web testing is efficient without redefining native gestures.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 133 — Run accessibility audit

**Objective:** Names, focus, reduced motion, non-color states.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 134 — Run responsive layout audit

**Objective:** Toolbar remains top-center at common browser widths.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- `npm run build` stays green after the web build pipeline exists.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 17 — Web Live Development and Internet Preview

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 135 — Create dev:live script

**Objective:** Start Wasm watcher plus Vite from one command.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 136 — Prevent stale Wasm

**Objective:** A failed Rust build must not leave the frontend silently using old semantics.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 137 — Create production build

**Objective:** `npm run build` produces deployable static assets with current Wasm.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 138 — Document preview providers

**Objective:** Provide Vercel and/or Cloudflare Pages instructions while keeping code provider-neutral.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 139 — Configure one preview path if authorization exists

**Objective:** Produce an actual branch/PR preview URL when possible.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 140 — Add preview smoke test

**Objective:** Load the deployed URL and exercise a basic real-core flow.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- `npm run build` stays green after the web build pipeline exists.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 18 — CI and Cross-Platform Build Hardening

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 141 — Add web CI

**Objective:** Typecheck, lint, unit tests, Wasm build, production build, Playwright smoke.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 142 — Add Android compile CI

**Objective:** Complete missing Execution 02 Android CI without hardware claims.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 143 — Preserve existing Rust CI

**Objective:** Keep Windows/Linux/macOS shared-core gates.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 144 — Cache responsibly

**Objective:** Do not hide stale Wasm/native artifacts.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 145 — Upload useful artifacts

**Objective:** Web dist and Android debug APK when appropriate.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 146 — Audit licenses

**Objective:** Review new JS/Wasm dependencies and licenses.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- `npm run build` stays green after the web build pipeline exists.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 19 — Native Parity Compile Patch

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 147 — Move Android toolbar to top-center

**Objective:** Translate the accepted interaction placement into Compose.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 148 — Add shared mode semantics to Android adapter

**Objective:** Consume platform-neutral Creative/Sketch state where architecture permits.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 149 — Align native tool semantics

**Objective:** Do not require pixel identity with web.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 150 — Compile Android

**Objective:** Build and lint only; no physical tablet runtime test.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 151 — Document remaining native gates

**Objective:** S Pen, pressure, palm, ML Kit, and native ergonomics remain deferred.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- `npm run build` stays green after the web build pipeline exists.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---

# Phase 20 — Full Web Acceptance and Final Audit

**Operating rule:** Goal Mode dominates. Plan Mode may change the route only when current repository evidence proves the planned route wrong.

## Task 152 — Run Main→Sketch journey

**Objective:** Icon/S/Sketch enter; Pen/B exits.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 153 — Run geometry journey

**Objective:** Pen and explicit supported sketch tools create real domain entities.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 154 — Run dimension/constraint journey

**Objective:** Valid and conflicting cases.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 155 — Run Orthographic journey

**Objective:** Front→Top/Right→Depth→propagation→conflict.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 156 — Run persistence journey

**Objective:** Save→reload→same semantic state.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 157 — Run Undo/Redo journey

**Objective:** Geometry, dimensions, constraints, Orthographic.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 158 — Run preview journey

**Objective:** Repeat smoke flow on deployed preview URL when configured.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 159 — Run no-hallucination audit

**Objective:** Separate web evidence from native-device evidence.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 160 — Run architecture audit

**Objective:** No React/DOM/browser types in shared engineering crates.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 161 — Run design audit

**Objective:** Top-center toolbar, Creative Precision, low clutter.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Task 162 — Write final report

**Objective:** Record PASS/FAIL/DEFERRED with commands, tests, screenshots, URLs, and limitations.

**Repository-first context:** Inspect the current source before changing it. Reuse existing engines and tests. Example paths and type names in this execution are guidance, not authority over the repository.

**Backend invariant:** Engineering truth remains in shared Rust/domain code. The browser may adapt and render but must not create a duplicate geometry, constraint, dimension, conflict, or Orthographic model.

**UI/UX invariant:** Preserve Creative Precision. The canvas dominates, the primary toolbar stays top-center, and Sketch Mode adds precision without becoming dense 3D CAD.

**Universal invariant:** React, DOM, PointerEvent, Canvas, SVG, Vite, and browser storage types stay outside platform-neutral engineering crates.

**Test-first rule:** Add or identify failing executable evidence before changing deterministic behavior. Visual tasks also require semantic assertions; screenshots alone are insufficient.

**Verification:** Run the narrow Rust/Wasm/frontend check, then neighboring regressions. Open Web Live View and exercise the behavior. Run production web build once the build pipeline exists.

**Evidence:** Record files, tests, commands, browser result, screenshots/recording when useful, and deferred native validation.

**Completion:** Real shared-core semantics and the browser harness agree; tests are green; no platform-semantic fork was introduced.

---

## Phase Gate

- Mandatory tasks have evidence.
- Relevant Rust/Wasm/frontend tests are green.
- Web Live View opens and the phase behavior is interactively testable.
- `npm run build` stays green after the web build pipeline exists.
- Shared semantics remain platform-neutral.
- No unsupported native-device claim is made.
- Continue automatically unless a true blocker requires human action.

---


# Article 82 — Web Live Acceptance Gate

Execution 03 does not use a vague percentage to describe readiness.

The gate is workflow-based.

## Gate A — Local Live View

A documented command starts the full frontend + Wasm live environment.

The browser opens a real Craft Loop surface.

Frontend edits update rapidly through HMR.

Rust changes rebuild and reload without stale semantics.

## Gate B — Real Shared Core

Browser geometry operations call the real Rust core.

No fake TypeScript geometry model acts as engineering truth.

## Gate C — Top-Center Main Toolbar

The toolbar is visibly top-center.

It is icon-first.

The canvas dominates.

## Gate D — Sketch Entry

Click Sketch.

Sketch Mode appears in the same toolbar anchor.

Press `S`.

Same semantic result.

Inject `Sketch` through the command simulator.

Same semantic result.

## Gate E — Creative Return

Click Pen or trigger `B` / `Pen`.

Return to Main Creative Mode.

The document remains unchanged.

Pen becomes active.

## Gate F — Sketch Geometry

Create all exposed core-supported Sketch entities.

Every committed entity exists in real domain state.

## Gate G — Pen + Refinement

Freehand input works.

Draw-and-Hold can show a real recognition candidate.

Cancel preserves raw ink.

Confirm commits real structured geometry.

## Gate H — Dimensions

Create and edit a semantic dimension.

Invalid input returns real domain conflict/rejection.

## Gate I — Constraints

Apply a valid constraint and solve through the real backend.

Contradictory state cannot silently corrupt geometry.

## Gate J — Orthographic

Assign `FRONT`.

Enter Orthographic.

Render real linked `TOP` and `RIGHT`.

## Gate K — Unresolved Depth

Depth remains unresolved until the user supplies it.

## Gate L — Shared Propagation

Enter Depth in `TOP`.

`RIGHT` updates from the shared graph.

Enter another shared value from a non-Front view and prove no permanent master view.

## Gate M — Cross-View Conflict

Attempt a contradictory shared value.

A real conflict appears.

Valid resolution choices are available.

## Gate N — Undo / Redo

Engineering actions use the shared transaction history.

## Gate O — Web Persistence

Save.

Reload.

Recover the same semantic document.

## Gate P — Internet Preview

When provider authorization is available:

- static build succeeds;
- preview URL exists;
- the URL loads current Wasm;
- a basic real-core smoke journey works.

If provider authorization is the only missing step, record `BLOCKED_BY_HUMAN_AUTHORIZATION`.

## Gate Q — Android Compile Regression

Android debug build remains compile-verified after shared API and interaction changes.

No physical-device claim is made.

---

# Article 83 — Golden Web Journey

Run this manually before declaring completion.

1. Start the Web Live View.
2. Confirm the primary toolbar is top-center.
3. Use Pen to draw freely.
4. Press `S`.
5. Confirm the same toolbar anchor becomes the Sketch toolbar.
6. Use Line.
7. Use Circle.
8. Use Rectangle.
9. Use Arc.
10. Use Ellipse if exposed.
11. Create a dimension.
12. Apply a constraint.
13. Press `B`.
14. Confirm Main Creative toolbar returns and Pen is active.
15. Trigger `Sketch` through the command simulator.
16. Confirm the same Sketch state.
17. Assign `FRONT`.
18. Enter Orthographic.
19. Confirm `TOP` and `RIGHT` appear.
20. Confirm Depth remains unresolved.
21. Enter Depth = 40 in Top.
22. Confirm Right receives the shared value.
23. Enter a contradictory shared value.
24. Confirm a conflict appears.
25. Resolve or cancel.
26. Undo.
27. Redo.
28. Save.
29. Reload the browser.
30. Recover the same semantic state.
31. Open the internet preview URL if configured.
32. Repeat a short real-core smoke flow there.

---

# Article 84 — Research Reference Ledger

The executing agent must re-check live documentation before changing library versions.

## Scholarly and OpenAlex-Oriented Research

### SketchGraphs: A Large-Scale Dataset for Modeling Relational Geometry in Computer-Aided Design

Ari Seff, Yaniv Ovadia, Wenda Zhou, Ryan P. Adams.  
https://arxiv.org/abs/2007.08506

**Execution relevance:** CAD sketches as primitives linked by explicit geometric constraints; constraint-graph reasoning; supports Craft Loop's relational backend direction.

### Vitruvion: A Generative Model of Parametric CAD Sketches

Ari Seff, Wenda Zhou, Nick Richardson, Ryan P. Adams.  
https://arxiv.org/abs/2109.14124

**Execution relevance:** design intent encoded in primitives + constraints; downstream editing should propagate coherently.

### Free2CAD: Parsing Freehand Drawings into CAD Commands

Changjian Li, Hao Pan, Adrien Bousseau, Niloy J. Mitra.  
DOI: `10.1145/3528223.3530133`  
https://discovery.ucl.ac.uk/id/eprint/10159067/

**Execution relevance:** freehand input can be interpreted as structured command sequences while reducing user command burden; reinforces suggestion/interpretation architecture.

### OpenAlex API

https://help.openalex.org/api/  
https://help.openalex.org/api/searching/

**Execution relevance:** future systematic literature sweeps by topic, DOI, semantic search, and citation graph.

---

## Professional CAD Sketch References

### Onshape — Sketch Tools

https://cad.onshape.com/help/Content/Sketch/sketch_tools.htm

**Takeaway:** contextual Sketch toolbar, grouped tool families, dedicated sketch shortcut toolbar with `S`, professional dimensions and constraints.

### Onshape — Sketch Basics

https://cad.onshape.com/help/Content/Sketch/sketch_basics.htm

**Takeaway:** parametric 2D curves, dimensions, and constraints as sketch foundation.

### Onshape — Working with Constraints

https://cad.onshape.com/help/Content/Sketch/working_with_constraints.htm

**Takeaway:** constraint relations, constraint status, selection-aware use, under/fully constrained states.

### Fusion — Sketch Context

https://help.autodesk.com/view/fusion360/ENU/?contextId=SKT-3D-SKETCH

**Takeaway:** contextual Sketch tab, Sketch Palette, grid, snap, construction geometry, and dimensions/constraints visibility.

### Fusion — Constraints in Sketches

https://help.autodesk.com/cloudhelp/ENU/Fusion-Sketch/files/SKT-CONSTRAINTS.htm

**Takeaway:** professional constraint vocabulary and contextual sketch environment.

### Shapr3D — Sketching

https://support.shapr3d.com/hc/en-us/articles/18816009328284-Sketching-in-Shapr3D

**Takeaway:** streamlined precision, direct manipulation, dimensions, constraints, under-defined/fully-defined states.

### Shapr3D — Adding and Removing Constraints

https://support.shapr3d.com/hc/en-us/articles/7407627891100-Adding-and-removing-constraints

**Takeaway:** constraint menu adapts to current selection.

### Shapr3D — Accessing Tools

https://support.shapr3d.com/hc/en-us/articles/7378907587484-Accessing-tools

**Takeaway:** icon-first floating menus and context-sensitive access.

---

## Creative Sketch References

### Procreate — QuickShape

https://help.procreate.com/procreate/handbook/guides/quickshape

**Takeaway:** Draw-and-Hold precision that feels like drawing, not command entry.

### Procreate — Gestures

https://help.procreate.com/procreate/handbook/interface-gestures/gestures

**Takeaway:** canvas-first interaction and low interface obstruction.

### Concepts

https://concepts.app/en/  
https://concepts.app/en/manual/infinite-canvas  
https://concepts.app/en/manual/precision-tools

**Takeaway:** infinite flexible canvas, editable vector strokes, stylus-first interaction, live snap, grids, guides, measurement, real-world scale.

### Linea Sketch

https://apps.apple.com/us/app/linea-sketch/id1094770251

**Takeaway:** deliberately minimal UI that keeps focus on the creation.

### Morpholio Trace

https://apps.apple.com/us/app/morpholio-trace-sketch-cad/id547274918

**Takeaway:** creative sketching speed and visual freedom combined with CAD intelligence.

---

## Web Live View Technical References

### Vite

https://vite.dev/guide/

**Takeaway:** development server with fast HMR and static production build.

### MDN Pointer Events

https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events

**Takeaway:** unified mouse/pen/touch input with pressure and tilt metadata when available.

### wasm-bindgen

https://wasm-bindgen.github.io/wasm-bindgen/

**Takeaway:** Rust ↔ JavaScript/WebAssembly boundary suitable for a real shared-core browser harness.

### Vercel Git Preview Deployments

https://vercel.com/docs/git

**Takeaway:** branch/PR deployments can receive preview URLs.

### Cloudflare Pages Preview Deployments

https://developers.cloudflare.com/pages/configuration/preview-deployments/

**Takeaway:** Git-connected branch/PR preview URLs for static sites.

---

# Article 85 — Final Report Contract

Create:

`execution-evidence/execution-03/FINAL-REPORT.md`

Allowed results:

`WEB LIVE VIEW + CREATIVE SKETCH MODE READY`

`WEB LIVE VIEW READY WITH DISCLOSED LIMITATIONS`

`PARTIALLY COMPLETE`

The report must include:

## Repository
- branch;
- commit;
- clean/dirty state.

## Shared Core
- test results;
- Wasm compatibility changes;
- new domain additions;
- platform-neutral audit.

## Web Bridge
- target;
- session surface;
- serialization approach.

## Live View
- local command;
- HMR behavior;
- production build.

## Preview
- provider;
- URL when available;
- deployment result.

## Toolbar
- top-center evidence;
- Main Creative bar;
- Sketch bar;
- grouped tools;
- accessibility.

## Commands
- `S`;
- `Sketch`;
- `B`;
- `Pen`;
- source parity.

## Sketch
- freehand;
- explicit geometry;
- Draw-and-Hold;
- dimensions;
- constraints;
- construction;
- snap/inference.

## Orthographic
- Front / Top / Right;
- unresolved depth;
- propagation;
- conflict;
- no-master-view evidence.

## Persistence
- save/reload result.

## CI
- Rust;
- web;
- Android compile.

## Deferred Native Validation

Explicitly list:

- real S Pen;
- pressure;
- tilt;
- palm rejection;
- Android ML Kit handwriting;
- Apple Pencil;
- iPad runtime.

No web evidence may close those gates.

---

# Article 86 — Starter Prompt for Claude Code or Codex

```text
Read "Craft Loop Execution 03 Web Live View Creative Sketch Mode.md" as the current execution authority.

Read "Craft Loop MCP V1.md" as product authority.

Inspect the current repository before changing code. This document was authored against main at commit 93e5bdb2c30919e9a73832b6759c3ed06553ab74, but repository evidence at execution time is authoritative.

Execution 01 built the shared engineering engines.
Execution 02 built CraftLoopSession, Android tooling, Jetpack Ink integration, pan/zoom/selection, icon toolbar, dimensions, and constraints through Phase 10. Linked Orthographic UI and later phases remained unfinished.

Execution 03 changes the validation strategy:
- do not require a physical tablet;
- build a browser-based Web Live View for rapid live UI/UX and engineering testing;
- preserve Android and iPad as the production platforms;
- never replace shared Rust engineering truth with a React/TypeScript duplicate.

Primary goals:
1. create a real Vite/React/TypeScript Web Live View backed by the shared Rust core through WebAssembly;
2. support fast HMR/live development on Windows;
3. support a deployable internet preview build;
4. move the primary toolbar to a floating top-center position;
5. introduce Main Creative Mode and a dedicated 2D Sketch Mode;
6. Sketch icon, S, or Sketch enters Sketch Mode;
7. B or Pen returns to Main Creative Pen mode;
8. keep Pen available inside Sketch Mode;
9. make the Sketch toolbar professional but creative, inspired by CAD sketch environments without making Craft Loop feel like dense 3D CAD;
10. finish linked Orthographic Front/Top/Right presentation, unresolved depth, propagation, and cross-view conflict;
11. finish browser persistence and shared Undo/Redo;
12. complete web CI and Android compile CI without claiming physical-device validation.

Use Goal Mode, Plan Mode, Context Engineering, Prompt Engineering, Loop Engineering, and the no-hallucination contract.

Do not stop after each phase. Continue automatically unless a true blocker requires human action.

Do not claim real handwriting, S Pen, Apple Pencil, palm rejection, pressure, or native runtime validation from browser tests.

Start with Phase 00.
```

---

# Article 87 — Closing Principle

Craft Loop should not force the user to choose between creativity and precision.

The Pen is the beginning.

Sketch Mode is precision added to the Pen.

Constraints are design intent made explicit.

Orthographic is engineering relationships made visible.

The Web Live View is the laboratory that lets us refine those ideas quickly.

Android and iPad remain the products.

The shared Rust core remains the truth.

The primary toolbar moves to the top center.

Sketch Mode must feel like the same creative instrument becoming more capable.

**Creative first. Precise when needed. Universal underneath.**
