# Immutable Execution Inputs — Phase 00, Task 002

Recorded: 2026-09-13

Craft Loop MCP V1 (`Craft Loop MCP V1.md`) is **product truth**.
Craft Loop Execution 01 (`Craft Loop Execution 01.md`) is **implementation truth**.
Both are treated as read-only inputs for the duration of Execution 01. If a
contradiction between them is ever found, it is recorded as a true blocker
(contradictory requirements) rather than silently resolved by an agent.

SHA-256 hashes at execution start are recorded in `01-immutable-inputs.sha256`
(git-tracked, generated with `sha256sum`). Re-run

```
sha256sum "Craft Loop Execution 01.md" "Craft Loop MCP V1.md" "Back End Skill.md" "Front End Skill.md" "START HERE.md"
```

and diff against that file to detect drift in the authority documents during
execution.

| File | Role | Lines |
|---|---|---|
| `Craft Loop Execution 01.md` | Implementation authority | 22,772 |
| `Craft Loop MCP V1.md` | Product authority | 13,282 |
| `Back End Skill.md` | Vetted engineering skill policy | 218 |
| `Front End Skill.md` | Vetted interaction-surface skill policy | 167 |
| `START HERE.md` | Onboarding pointer | 44 |
