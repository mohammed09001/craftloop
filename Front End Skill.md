# Craft Loop — Front End Skill

**Document type:** Vetted agent-skill installation and usage policy  
**Created:** 2026-09-13  
**Applies to:** Windows engineering test harness, future Android tablet UI, future iPad UI, accessibility and visual QA  
**Scope:** Interaction surfaces only. Engineering truth remains in the shared core.

## Purpose

Craft Loop Execution 01 is engine-first.

The Windows interface built during this execution is a disposable engineering test harness.

It is intentionally not the final Craft Loop visual design.

The frontend skill policy must therefore prevent agents from spending the execution budget polishing a temporary surface while core geometry, dimensions, constraints, and orthographic intelligence remain incomplete.

## Tier A — Install or Prepare Now

### 1. Anthropic Frontend Design

**Source:** `anthropics/skills/skills/frontend-design`  
**Why it is valuable:** It is designed to prevent generic AI-generated visual output and encourages deliberate visual hierarchy, typography, layout, and subject-specific design.

**Execution 01 usage restriction:** Use lightly for the Windows harness so it is readable and coherent. Do not turn Execution 01 into the final Craft Loop UI redesign.

Use fully when the dedicated production UI execution begins.

Anthropic’s official skills repository is widely used and the frontend-design skill is one of its flagship design skills.

### 2. Google Android Skills

**Source:** `android/skills`  
**Why it is valuable:** Official Android agent instructions grounded in developer.android.com.

The Android adaptive skill is particularly relevant because Craft Loop is tablet-only and must account for:
- larger windows;
- pointing devices;
- text entry devices;
- tablet layouts;
- adaptive navigation;
- screenshot tests across form factors.

The Android CLI skill is also valuable for Windows development because it documents project creation, SDK management, emulators, connected devices, installation, and skills.

Install these when the Android application shell begins.

### 3. OpenAI Build iOS Apps Plugin

**Source:** `openai/plugins/plugins/build-ios-apps`  
**Why it is valuable:** It packages current iOS/SwiftUI workflows, simulator debugging, performance auditing, memory leak analysis, and refactoring.

Relevant skills:
- `swiftui-ui-patterns`
- `swiftui-performance-audit`
- `swiftui-view-refactor`
- `ios-debugger-agent`
- `ios-simulator-browser`
- `ios-ettrace-performance`
- `ios-memgraph-leaks`

**Execution restriction:** Do not attempt to fake Xcode execution from Windows. Shared core and iOS source scaffolding may be written on Windows, but real Xcode build/debug validation must run in macOS CI or a Mac environment.

## Tier B — Valuable for Later UI Review

### 4. Vercel Web Design Guidelines

**Source:** `vercel-labs/agent-skills`, skill `web-design-guidelines`  
**Why it is valuable:** It provides a broad accessibility, focus, forms, animation, typography, and interface-quality audit.

**Important limitation:** It is web-specific.

Do not mechanically apply HTML or browser rules to native Compose, SwiftUI, or the Rust Windows harness.

Use the general accessibility and interaction lessons only when relevant, or use it directly if Craft Loop later has a web surface.

Documented installation pattern:
```text
npx skills add vercel-labs/agent-skills --skill web-design-guidelines
```

### 5. Vercel React Best Practices

Do **not** install for Execution 01 unless the repository actually adopts React for a web diagnostic surface.

Craft Loop’s current production direction is native tablet shells plus a shared engineering core.

Installing React-specific guidance without React would add context noise rather than value.

## Windows Harness Frontend Rules

The Windows harness exists to make the engines observable.

It must provide:
- canvas;
- mouse-as-simulated-pen input;
- active tool state;
- raw stroke visibility;
- structured geometry visibility;
- engine-state inspector;
- dimension inspector;
- constraint inspector;
- orthographic view blocks;
- conflict panel;
- undo/redo;
- save/reopen;
- deterministic scenario loader;
- JSON diagnostic export.

It does not need:
- brand-final typography;
- production iconography;
- final animation system;
- App Store-quality onboarding;
- paid-plan surfaces;
- cloud account UI;
- final visual identity.

The harness should be ugly only if ugliness does not reduce testability. Readability is required.

## Production Android UI Rules

When Android implementation begins:
- use Kotlin and Jetpack Compose unless repository evidence supports another native approach;
- use Jetpack Ink for the low-latency ink boundary;
- keep the engineering core outside the Compose UI;
- use device-level Samsung testing for S Pen behavior;
- run tablet screenshot tests;
- test mouse/keyboard support only as secondary input;
- test palm rejection, pressure, tilt, hover and latency on real hardware where supported.

## Production iPad UI Rules

When iPad implementation begins:
- use Swift/SwiftUI/UIKit as appropriate;
- evaluate PencilKit for low-latency ink;
- evaluate PaperKit only where generic structured markup is useful;
- keep the engineering core outside PencilKit;
- use real Apple Pencil testing before release;
- use Xcode simulator for layout and state tests, not as proof of physical Pencil behavior;
- run performance and memory audits before TestFlight expansion.

## Cross-Platform UI Invariant

The visual implementation may be native on each platform.

The semantic behaviors must match:
- same dimension meaning;
- same command meaning;
- same shared-view relationship;
- same conflict semantics;
- same document state;
- same undo transaction intent.

Pixel identity is not required.

Behavioral identity is required.

## Source Notes

Vetted sources were rechecked on 2026-09-13:
- Anthropic official frontend-design skill;
- Google official Android skills;
- OpenAI official Build iOS Apps plugin;
- Vercel official agent skills.

Recheck upstream instructions before installation because versions and install commands evolve.
