# Craft Loop — Back End Skill

**Document type:** Vetted agent-skill installation and usage policy  
**Created:** 2026-09-13  
**Applies to:** Claude Code, Codex CLI/App, and repository-level agent workflows  
**Scope:** Craft Loop shared engineering core, geometry, constraints, document model, testing, debugging, persistence, orthographic intelligence, code review, and execution discipline

## Purpose

This file does not define the Craft Loop product. `Craft Loop MCP V1.md` and `Craft Loop Execution 01.md` define the product and execution contract.

This file defines the external agent skills that are worth installing for the engineering side of the repository, plus the rules that prevent those skills from overriding the Craft Loop execution contract.

A skill is an implementation aid, not a source of product truth.

The execution document wins when a skill disagrees with the Craft Loop architecture or scope.

## Trust Policy

Prefer official vendor repositories or strongly vetted sources.

Do not install a skill merely because its title sounds useful.

Before adopting a skill:
- verify source organization;
- inspect its `SKILL.md`;
- inspect license;
- inspect scripts it executes;
- verify whether it writes files or invokes shell commands;
- pin or record the revision when reproducibility matters;
- test the skill on a disposable branch before making it mandatory.

The current OpenAI `openai/skills` repository is marked deprecated for current Codex examples. Use `openai/plugins` for current official Codex plugin examples instead.

## Tier A — Install for Execution 01

### 1. OpenAI Superpowers

**Source:** `openai/plugins`, plugin `superpowers`  
**Why it is valuable:** It provides a full engineering loop: test-driven development, systematic debugging, verification before completion, writing plans, executing plans, code review, worktrees, and subagent-driven development.

For Craft Loop, the most valuable skills are:
- `test-driven-development`
- `systematic-debugging`
- `verification-before-completion`
- `requesting-code-review`
- `receiving-code-review`
- `using-git-worktrees`
- `executing-plans`
- `subagent-driven-development`
- `dispatching-parallel-agents`
- `finishing-a-development-branch`

**Craft Loop override:** Do not let a generic brainstorming skill restart product discovery. MCP V1 and Execution 01 are the approved product and execution specification. Only revisit architecture if repository evidence proves a blocking contradiction.

**Claude Code installation path:** use the Superpowers marketplace documented by its repository.  
**Codex:** install Superpowers from the official Codex plugin marketplace.

**Mandatory use in Execution 01:** TDD, systematic debugging, verification before completion, and code review gates.

### 2. Anthropic Skill Creator

**Source:** `anthropics/skills/skills/skill-creator`  
**Why it is valuable:** It gives a disciplined workflow for creating and evaluating project-specific skills rather than writing untested prompt fragments.

Use it after the repository baseline exists to create Craft Loop-specific skills such as:
- `craft-loop-core-engineering`
- `craft-loop-orthographic-verifier`
- `craft-loop-windows-harness`
- `craft-loop-android-ink`
- `craft-loop-ios-ink`

**Important:** Keep the project-specific skill small and route large details to reference files. The official skill recommends progressive disclosure and evals.

**Claude Code marketplace bootstrap:**
```text
/plugin marketplace add anthropics/skills
```

Then install only the relevant official skill/plugin payload according to the current Anthropic marketplace instructions.

### 3. OpenAI Code Review / Review Workflow

**Source:** `openai/codex` and `openai/plugins/superpowers`  
**Why it is valuable:** Code review must be independent of the implementation context and must report concrete file/line findings.

Use review after:
- each engine milestone;
- solver integration;
- serialization changes;
- cross-view propagation changes;
- FFI changes;
- final Execution 01 integration.

Do not treat “tests pass” as sufficient evidence that architecture is correct.

## Tier B — Use When the Relevant Phase Begins

### 4. Google Android Skills

**Source:** `android/skills`  
**Why it is valuable:** This is an official Android repository of AI-optimized skills grounded in Android developer guidance.

Install when the Android adapter phase begins.

Useful areas include:
- Android CLI;
- adaptive tablet UI;
- edge-to-edge behavior;
- current Android build/tooling guidance.

Example documented installer pattern:
```text
android skills add --all --project=.
```

For Craft Loop, prefer installing only the needed skills rather than loading everything into every execution context.

### 5. OpenAI Build iOS Apps Plugin

**Source:** `openai/plugins/plugins/build-ios-apps`  
**Why it is valuable:** It includes current SwiftUI implementation, refactor, performance, simulator, debugger, and memory-profiling workflows.

Relevant skills include:
- `ios-debugger-agent`
- `ios-simulator-browser`
- `ios-ettrace-performance`
- `ios-memgraph-leaks`
- `swiftui-performance-audit`
- `swiftui-ui-patterns`
- `swiftui-view-refactor`

Use only when a macOS/Xcode execution environment exists. Do not allow the absence of macOS to block shared-core development on Windows.

## Tier C — Evaluate, Do Not Mandate Yet

### Anthropic Web Application Testing

`anthropics/skills` includes a Playwright-based `webapp-testing` skill.

Craft Loop Execution 01 uses a native Windows engineering harness, not a web application. Therefore this skill is not mandatory for the core path.

Use it only if the team later builds a browser-based diagnostic viewer or documentation surface.

### Anthropic MCP Builder

`mcp-builder` is high quality for Model Context Protocol servers.

Craft Loop Execution 01 does not require an MCP server.

Do not install it merely because this document is named MCP V1; “MCP” in the project document name is not a requirement to implement Model Context Protocol.

## Project-Specific Skill Creation Policy

After Phase 0 repository reconnaissance, use Skill Creator to build project-specific skills only for recurring workflows with measurable value.

Every custom skill must have:
- one narrow responsibility;
- explicit trigger conditions;
- explicit non-goals;
- repository paths it may change;
- required tests;
- forbidden shortcuts;
- at least five positive trigger evals;
- at least five negative trigger evals;
- baseline comparison without the skill;
- version metadata.

Do not encode the entire 15,000-line execution document inside one skill.

Use references and progressive disclosure.

## Required Engineering Loop

For every nontrivial task:

1. Read the exact relevant section of Execution 01.
2. Inspect repository evidence.
3. State the invariant being changed.
4. Write or update a failing test.
5. Run the failing test and record the expected failure.
6. Implement the smallest coherent change.
7. Run focused tests.
8. Run neighboring regression tests.
9. Run static analysis.
10. Run code review.
11. Fix critical and important findings.
12. Run verification-before-completion.
13. Commit or checkpoint only after evidence is green.
14. Continue to the next task without waiting for human approval unless a true blocker is reached.

## True Blocker Definition

A true blocker is one of:
- missing credential or secret that cannot be replaced with a local stub;
- missing proprietary standard text required to make a compliance claim;
- unavailable platform toolchain required for that exact platform task;
- contradictory requirements that would corrupt engineering truth;
- repository corruption;
- dependency license incompatible with the project;
- repeated deterministic test failure after root-cause investigation shows the specification itself is inconsistent.

A difficult bug is not a blocker.

A large task is not a blocker.

An unfamiliar library is not a blocker.

## Source Notes

Vetted sources for this file were rechecked on 2026-09-13:
- Anthropic official skills repository;
- OpenAI official plugins repository;
- OpenAI Codex repository;
- Android official skills repository;
- Agent Skills open specification.

Always recheck upstream instructions before installation because agent-plugin ecosystems change quickly.
