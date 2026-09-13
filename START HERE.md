# Craft Loop Execution 01 — Package

Created: 2026-09-13

## Files

- `Craft Loop Execution 01.md` — primary execution document for Claude Code / Codex.
- `Craft Loop MCP V1.md` — source product specification used by the execution.
- `Back End Skill.md` — vetted engineering/process agent-skill policy.
- `Front End Skill.md` — vetted native UI/platform agent-skill policy.
- `windows-simulator/` — immediately runnable disposable Windows bootstrap harness using the mouse as a simulated pen.

## Give this to Claude Code or Codex

Use the repository copy of these files.

Tell the coding agent:

> Read `Craft Loop Execution 01.md` as the implementation authority and `Craft Loop MCP V1.md` as the product authority. Read `Back End Skill.md` and `Front End Skill.md`. Inspect the repository before editing. Execute Execution 01 continuously from Phase 00 through the completion gate. Do not pause after internal phases unless a true blocker defined by the execution document is reached. Do not claim platform behavior that was not actually tested.

The execution document already contains the detailed context, loop, testing, architecture, and no-hallucination rules.

## Run the temporary Windows simulator now

From PowerShell:

```powershell
cd .\windows-simulator
py -m unittest -v
py .\app.py
```

The mouse is mapped to a simulated pen.

The harness intentionally cannot validate pressure, tilt, real stylus hover, palm rejection, or tablet latency.

## Important architecture note

The included Python simulator is an immediate bootstrap test surface.

Execution 01 instructs the coding agent to build the durable shared core in Rust and replace the bootstrap harness with a native Rust engineering harness using the shared core.

Do not expand the Python bootstrap into the production Android or iPad application.
