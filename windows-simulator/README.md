# Craft Loop Windows Engineering Test Harness

This is a deliberately temporary Windows test surface for Craft Loop Execution 01.

It is **not** the final Craft Loop desktop product and it must not become the production tablet user interface.

## Why it exists

The production target is iPad and Android tablets. During the engine-first phase, the project needs a low-friction way to exercise interaction flows from Windows before Android and iPad device validation is mature.

This harness maps the Windows mouse to a simulated pen event stream:

- position: real mouse coordinates
- pressure: constant `0.5`
- tilt: unavailable
- stylus hover: unavailable
- palm rejection: unavailable

This means it can validate engine wiring and interaction state, but it **cannot certify stylus hardware behavior**.

## Run on Windows

From PowerShell:

```powershell
cd .\windows-simulator
py -m unittest -v
py .\app.py
```

Python's standard `tkinter` module is used, so the harness has no third-party runtime dependency on a normal Python installation that includes Tk.

## Current seed behavior

- mouse-as-pen pointer abstraction
- Pen, Line, Circle, Rectangle and Eraser test tools
- optional freehand-to-line reference refinement
- optional rough closed-stroke-to-circle reference refinement
- view identity selector for Front / Top / Right / Back testing
- event/state panel
- JSON export
- reference unit tests

The recognizers are intentionally simple reference heuristics. Execution 01 must replace them with the real engine interfaces rather than expanding this file into a production geometry system.

## Architecture rule

The simulator is an adapter.

The future structure is:

```text
Windows mouse harness ─┐
Android Jetpack Ink ───┼──> normalized pointer/ink events ──> shared Craft Loop core
iPad PencilKit ────────┘
```

Do not put permanent engineering truth in the simulator UI.
