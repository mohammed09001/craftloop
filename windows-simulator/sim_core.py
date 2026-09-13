from __future__ import annotations

from dataclasses import dataclass, asdict
from enum import Enum
from math import hypot
from time import perf_counter
from typing import Iterable, List, Optional


class Tool(str, Enum):
    PEN = "pen"
    LINE = "line"
    CIRCLE = "circle"
    RECTANGLE = "rectangle"
    ERASER = "eraser"


class PointerSource(str, Enum):
    SIMULATED_MOUSE = "simulated_mouse"
    STYLUS = "stylus"
    TOUCH = "touch"


@dataclass(frozen=True)
class InputCapabilities:
    pressure: bool = False
    tilt: bool = False
    hover: bool = False
    palm_rejection: bool = False


@dataclass(frozen=True)
class PointerSample:
    x: float
    y: float
    timestamp_s: float
    pressure: float = 0.5
    tilt_x_deg: Optional[float] = None
    tilt_y_deg: Optional[float] = None
    source: PointerSource = PointerSource.SIMULATED_MOUSE


@dataclass
class Stroke:
    id: int
    samples: List[PointerSample]


@dataclass
class LineEntity:
    id: int
    x1: float
    y1: float
    x2: float
    y2: float
    provenance: str = "mouse-simulator"


@dataclass
class CircleEntity:
    id: int
    cx: float
    cy: float
    radius: float
    provenance: str = "mouse-simulator"


@dataclass
class RectangleEntity:
    id: int
    x1: float
    y1: float
    x2: float
    y2: float
    provenance: str = "mouse-simulator"


class SimulatedPen:
    """Maps a Windows mouse to a stylus-like event stream.

    This is intentionally honest: pressure is a constant default and tilt/hover/
    palm rejection are unsupported. The production Android/iPad adapters must
    provide real hardware capabilities.
    """

    capabilities = InputCapabilities()

    @staticmethod
    def sample(x: float, y: float) -> PointerSample:
        return PointerSample(
            x=x,
            y=y,
            timestamp_s=perf_counter(),
            pressure=0.5,
            source=PointerSource.SIMULATED_MOUSE,
        )


def _distance_point_to_segment(px, py, x1, y1, x2, y2) -> float:
    vx, vy = x2 - x1, y2 - y1
    wx, wy = px - x1, py - y1
    vv = vx * vx + vy * vy
    if vv == 0:
        return hypot(px - x1, py - y1)
    t = max(0.0, min(1.0, (wx * vx + wy * vy) / vv))
    qx, qy = x1 + t * vx, y1 + t * vy
    return hypot(px - qx, py - qy)


def line_likeness(samples: Iterable[PointerSample]) -> float:
    """Return normalized maximum deviation from endpoint segment.

    Lower values are more line-like. This is only a reference heuristic for the
    Windows harness, not the production recognizer.
    """
    pts = list(samples)
    if len(pts) < 2:
        return float("inf")
    a, b = pts[0], pts[-1]
    length = hypot(b.x - a.x, b.y - a.y)
    if length < 1e-9:
        return float("inf")
    max_dev = max(
        _distance_point_to_segment(p.x, p.y, a.x, a.y, b.x, b.y)
        for p in pts
    )
    return max_dev / length


def closed_stroke(samples: Iterable[PointerSample], ratio: float = 0.18) -> bool:
    pts = list(samples)
    if len(pts) < 8:
        return False
    xs = [p.x for p in pts]
    ys = [p.y for p in pts]
    diag = hypot(max(xs) - min(xs), max(ys) - min(ys))
    if diag < 1e-9:
        return False
    return hypot(pts[-1].x - pts[0].x, pts[-1].y - pts[0].y) / diag <= ratio


def circle_candidate(samples: Iterable[PointerSample]) -> Optional[CircleEntity]:
    """Very small reference recognizer used only by the test harness.

    It recognizes a roughly closed, near-square bounding box. It deliberately
    avoids pretending to be the production circle fitter.
    """
    pts = list(samples)
    if not closed_stroke(pts):
        return None
    xs = [p.x for p in pts]
    ys = [p.y for p in pts]
    width = max(xs) - min(xs)
    height = max(ys) - min(ys)
    if width <= 2 or height <= 2:
        return None
    aspect = width / height
    if not 0.70 <= aspect <= 1.30:
        return None
    cx = (min(xs) + max(xs)) / 2
    cy = (min(ys) + max(ys)) / 2
    radius = (width + height) / 4
    return CircleEntity(id=-1, cx=cx, cy=cy, radius=radius)


class SimulatorDocument:
    def __init__(self) -> None:
        self.strokes: list[Stroke] = []
        self.lines: list[LineEntity] = []
        self.circles: list[CircleEntity] = []
        self.rectangles: list[RectangleEntity] = []
        self._next_id = 1

    def next_id(self) -> int:
        value = self._next_id
        self._next_id += 1
        return value

    def add_raw_stroke(self, samples: list[PointerSample]) -> Stroke:
        stroke = Stroke(self.next_id(), samples)
        self.strokes.append(stroke)
        return stroke

    def add_line(self, a: PointerSample, b: PointerSample) -> LineEntity:
        entity = LineEntity(self.next_id(), a.x, a.y, b.x, b.y)
        self.lines.append(entity)
        return entity

    def add_circle(self, a: PointerSample, b: PointerSample) -> CircleEntity:
        radius = hypot(b.x - a.x, b.y - a.y)
        entity = CircleEntity(self.next_id(), a.x, a.y, radius)
        self.circles.append(entity)
        return entity

    def add_rectangle(self, a: PointerSample, b: PointerSample) -> RectangleEntity:
        entity = RectangleEntity(self.next_id(), a.x, a.y, b.x, b.y)
        self.rectangles.append(entity)
        return entity

    def export_dict(self) -> dict:
        def conv(items):
            return [asdict(x) for x in items]
        return {
            "schema": "craft-loop-windows-simulator/0.1",
            "strokes": conv(self.strokes),
            "lines": conv(self.lines),
            "circles": conv(self.circles),
            "rectangles": conv(self.rectangles),
        }
