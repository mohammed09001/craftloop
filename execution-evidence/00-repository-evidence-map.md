# Repository Evidence Map — Phase 00, Task 001

Recorded: 2026-09-13
Branch: `execution-01/phase-00`

## State at execution start

The repository was **not** greenfield. It contained a disposable Windows bootstrap
simulator and four authority documents. No Rust workspace, no CI workflow, no
agent-instruction files (`AGENTS.md`, `CLAUDE.md`, `.agents/`, `.claude/`) existed
in the repository at execution start.

## Top-level inventory (pre-Execution-01 state)

| Path | Kind | Notes |
|---|---|---|
| `Craft Loop Execution 01.md` | Authority doc | Implementation authority. 22,772 lines. 33 phases (00–32), 30 Engine Contracts. |
| `Craft Loop MCP V1.md` | Authority doc | Product authority. 13,282 lines, 189+ numbered Articles. |
| `Back End Skill.md` | Policy doc | Vetted skill-installation policy for engineering work. 218 lines. |
| `Front End Skill.md` | Policy doc | Vetted skill-installation policy for interaction surfaces. 167 lines. |
| `START HERE.md` | Onboarding doc | Points a coding agent at the above files and the Windows simulator. |
| `windows-simulator/` | Disposable bootstrap | Python/Tk mouse-as-pen simulator. Explicitly temporary per its own README. |
| `windows-simulator/app.py` | Code | 269 lines. Tk UI wiring the reference recognizers to a canvas. |
| `windows-simulator/sim_core.py` | Code | 209 lines. Reference-only pointer/geometry/recognition types. |
| `windows-simulator/test_sim_core.py` | Test | 48 lines, 5 unit tests, all passing at baseline. |
| `windows-simulator/README.md` | Doc | States the simulator is a temporary adapter and must not become production. |
| `.git/` | VCS | Single commit at start: `c50ca0d Initialize Craft Loop Execution 01`. |

No `CI workflow` files existed (no `.github/workflows`, no other CI config).
No generated artifacts existed except `windows-simulator/__pycache__/` (untracked,
produced by the `unittest` run below; not part of source).

## Existing implementation assessment

`windows-simulator/sim_core.py` and `app.py` are an explicitly-labeled reference
bootstrap, not the shared engineering core described in Execution 01. Their own
README states: "Do not put permanent engineering truth in the simulator UI" and
"Execution 01 must replace them with the real engine interfaces rather than
expanding this file into a production geometry system."

Decision: per Execution 01 Architecture Decision ("Windows Harness"), this Python
bootstrap is retained as-is (not deleted, not expanded) until the Rust native
harness (Phase 04) reaches equivalent basic functionality. No new engineering
logic will be added to `sim_core.py`/`app.py` during Execution 01. This matches
the Agent Operating Directive: "Do not rewrite working code merely because
another implementation is aesthetically preferred."

## Conclusion

Repository is not greenfield for documentation/bootstrap purposes, but *is*
greenfield for the Rust shared engineering core, the requirement traceability
index, the evidence directory, and CI. Phase 00 proceeds on that basis.
