from __future__ import annotations

import json
import tkinter as tk
from pathlib import Path
from tkinter import filedialog, messagebox, ttk

from sim_core import (
    PointerSample,
    SimulatedPen,
    SimulatorDocument,
    Tool,
    circle_candidate,
    line_likeness,
)


class CraftLoopSimulator(tk.Tk):
    def __init__(self) -> None:
        super().__init__()
        self.title("Craft Loop — Windows Engineering Test Harness")
        self.geometry("1280x820")
        self.minsize(900, 600)

        self.doc = SimulatorDocument()
        self.tool = tk.StringVar(value=Tool.PEN.value)
        self.auto_refine = tk.BooleanVar(value=True)
        self.view_identity = tk.StringVar(value="Unassigned")
        self.status = tk.StringVar(
            value="Mouse is acting as simulated pen. Pressure=0.5; tilt/hover/palm rejection unavailable."
        )

        self.active_samples: list[PointerSample] = []
        self.preview_item = None
        self.start_sample: PointerSample | None = None

        self._build_ui()
        self._bind_canvas()

    def _build_ui(self) -> None:
        root = ttk.Frame(self, padding=8)
        root.pack(fill="both", expand=True)

        toolbar = ttk.Frame(root)
        toolbar.pack(fill="x")

        ttk.Label(toolbar, text="TEST HARNESS", font=("Segoe UI", 10, "bold")).pack(side="left", padx=(0, 12))

        for tool in [Tool.PEN, Tool.LINE, Tool.CIRCLE, Tool.RECTANGLE, Tool.ERASER]:
            ttk.Radiobutton(
                toolbar,
                text=tool.value.title(),
                value=tool.value,
                variable=self.tool,
                command=self._tool_changed,
            ).pack(side="left", padx=3)

        ttk.Checkbutton(toolbar, text="Auto-refine pen strokes", variable=self.auto_refine).pack(side="left", padx=12)

        ttk.Label(toolbar, text="View:").pack(side="left", padx=(12, 4))
        ttk.Combobox(
            toolbar,
            state="readonly",
            width=12,
            textvariable=self.view_identity,
            values=["Unassigned", "Front", "Top", "Right", "Back"],
        ).pack(side="left")

        ttk.Button(toolbar, text="Clear", command=self.clear).pack(side="right", padx=3)
        ttk.Button(toolbar, text="Export JSON", command=self.export_json).pack(side="right", padx=3)

        body = ttk.Panedwindow(root, orient="horizontal")
        body.pack(fill="both", expand=True, pady=(8, 0))

        canvas_frame = ttk.Frame(body)
        side = ttk.Frame(body, width=280, padding=(8, 0, 0, 0))
        body.add(canvas_frame, weight=4)
        body.add(side, weight=1)

        self.canvas = tk.Canvas(
            canvas_frame,
            bg="#fbfbf8",
            highlightthickness=1,
            highlightbackground="#c9c9c9",
            cursor="crosshair",
        )
        self.canvas.pack(fill="both", expand=True)

        ttk.Label(side, text="Simulation Contract", font=("Segoe UI", 11, "bold")).pack(anchor="w")
        contract = (
            "This Windows surface is disposable test UI.\n\n"
            "Mouse → simulated pen pointer.\n"
            "Pressure → constant 0.5.\n"
            "Tilt → unavailable.\n"
            "Hover → unavailable.\n"
            "Palm rejection → unavailable.\n\n"
            "Use it to test geometry flow, recognition heuristics, engine state, "
            "commands, dimensions, constraints, orthographic logic, undo and persistence. "
            "Do NOT use it to certify real stylus behavior."
        )
        ttk.Label(side, text=contract, wraplength=250, justify="left").pack(anchor="w", pady=(4, 14))

        ttk.Label(side, text="Live State", font=("Segoe UI", 11, "bold")).pack(anchor="w")
        self.state_text = tk.Text(side, height=24, width=34, font=("Consolas", 9))
        self.state_text.pack(fill="both", expand=True, pady=(4, 0))
        self.state_text.configure(state="disabled")

        status = ttk.Label(root, textvariable=self.status, anchor="w")
        status.pack(fill="x", pady=(6, 0))
        self._refresh_state()

    def _bind_canvas(self) -> None:
        self.canvas.bind("<ButtonPress-1>", self.on_down)
        self.canvas.bind("<B1-Motion>", self.on_move)
        self.canvas.bind("<ButtonRelease-1>", self.on_up)
        self.canvas.bind("<Motion>", self.on_hover)

    def _tool_changed(self) -> None:
        self.status.set(f"Active tool: {self.tool.get()} — mouse mapped to simulated pen.")

    def sample(self, event) -> PointerSample:
        return SimulatedPen.sample(float(event.x), float(event.y))

    def on_hover(self, event) -> None:
        self.status.set(
            f"Tool={self.tool.get()}  x={event.x} y={event.y}  "
            "simPressure=0.5  source=mouse"
        )

    def on_down(self, event) -> None:
        s = self.sample(event)
        self.start_sample = s
        self.active_samples = [s]

        if self.tool.get() == Tool.ERASER.value:
            item = self.canvas.find_closest(event.x, event.y)
            if item:
                self.canvas.delete(item[0])
            return

        if self.tool.get() == Tool.PEN.value:
            self.preview_item = self.canvas.create_line(
                event.x, event.y, event.x + 1, event.y + 1,
                fill="#202020", width=2, smooth=True
            )

    def on_move(self, event) -> None:
        if self.start_sample is None:
            return
        s = self.sample(event)
        self.active_samples.append(s)
        tool = self.tool.get()

        if tool == Tool.PEN.value and self.preview_item:
            coords = []
            for p in self.active_samples:
                coords.extend([p.x, p.y])
            self.canvas.coords(self.preview_item, *coords)
        elif tool in (Tool.LINE.value, Tool.CIRCLE.value, Tool.RECTANGLE.value):
            if self.preview_item:
                self.canvas.delete(self.preview_item)
            a = self.start_sample
            if tool == Tool.LINE.value:
                self.preview_item = self.canvas.create_line(a.x, a.y, s.x, s.y, fill="#5a5a5a", dash=(4, 3), width=2)
            elif tool == Tool.CIRCLE.value:
                r = ((s.x-a.x)**2 + (s.y-a.y)**2) ** 0.5
                self.preview_item = self.canvas.create_oval(a.x-r, a.y-r, a.x+r, a.y+r, outline="#5a5a5a", dash=(4, 3), width=2)
            else:
                self.preview_item = self.canvas.create_rectangle(a.x, a.y, s.x, s.y, outline="#5a5a5a", dash=(4, 3), width=2)

    def on_up(self, event) -> None:
        if self.start_sample is None:
            return
        end = self.sample(event)
        self.active_samples.append(end)
        tool = self.tool.get()

        if self.preview_item:
            self.canvas.delete(self.preview_item)
            self.preview_item = None

        if tool == Tool.PEN.value:
            self._commit_pen_stroke(self.active_samples)
        elif tool == Tool.LINE.value:
            e = self.doc.add_line(self.start_sample, end)
            self.canvas.create_line(e.x1, e.y1, e.x2, e.y2, fill="#111111", width=2)
        elif tool == Tool.CIRCLE.value:
            e = self.doc.add_circle(self.start_sample, end)
            self.canvas.create_oval(e.cx-e.radius, e.cy-e.radius, e.cx+e.radius, e.cy+e.radius, outline="#111111", width=2)
        elif tool == Tool.RECTANGLE.value:
            e = self.doc.add_rectangle(self.start_sample, end)
            self.canvas.create_rectangle(e.x1, e.y1, e.x2, e.y2, outline="#111111", width=2)

        self.start_sample = None
        self.active_samples = []
        self._refresh_state()

    def _commit_pen_stroke(self, samples: list[PointerSample]) -> None:
        if len(samples) < 2:
            return

        if self.auto_refine.get():
            score = line_likeness(samples)
            if score < 0.055:
                e = self.doc.add_line(samples[0], samples[-1])
                self.canvas.create_line(e.x1, e.y1, e.x2, e.y2, fill="#111111", width=2)
                self.status.set(f"Refined freehand stroke → Line (normalized deviation {score:.3f})")
                return

            c = circle_candidate(samples)
            if c is not None:
                c.id = self.doc.next_id()
                self.doc.circles.append(c)
                self.canvas.create_oval(c.cx-c.radius, c.cy-c.radius, c.cx+c.radius, c.cy+c.radius, outline="#111111", width=2)
                self.status.set("Refined freehand closed stroke → Circle candidate")
                return

        stroke = self.doc.add_raw_stroke(samples)
        coords = []
        for p in samples:
            coords.extend([p.x, p.y])
        self.canvas.create_line(*coords, fill="#202020", width=2, smooth=True)
        self.status.set(f"Kept raw ink stroke #{stroke.id}")

    def clear(self) -> None:
        if not messagebox.askyesno("Clear test canvas", "Clear the disposable simulator canvas?"):
            return
        self.canvas.delete("all")
        self.doc = SimulatorDocument()
        self._refresh_state()
        self.status.set("Canvas cleared.")

    def export_json(self) -> None:
        target = filedialog.asksaveasfilename(
            title="Export simulator state",
            defaultextension=".json",
            filetypes=[("JSON", "*.json")],
            initialfile="craft-loop-simulator-state.json",
        )
        if not target:
            return
        Path(target).write_text(json.dumps(self.doc.export_dict(), indent=2), encoding="utf-8")
        self.status.set(f"Exported simulator state to {target}")

    def _refresh_state(self) -> None:
        state = self.doc.export_dict()
        summary = {
            "view_identity": self.view_identity.get(),
            "tool": self.tool.get(),
            "raw_strokes": len(state["strokes"]),
            "lines": len(state["lines"]),
            "circles": len(state["circles"]),
            "rectangles": len(state["rectangles"]),
            "input": {
                "source": "simulated_mouse",
                "pressure": "constant 0.5",
                "tilt": "unavailable",
                "hover": "UI mouse hover only; not stylus hover",
                "palm_rejection": "unavailable",
            },
        }
        self.state_text.configure(state="normal")
        self.state_text.delete("1.0", "end")
        self.state_text.insert("1.0", json.dumps(summary, indent=2))
        self.state_text.configure(state="disabled")


if __name__ == "__main__":
    CraftLoopSimulator().mainloop()
