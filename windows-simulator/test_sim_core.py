import unittest
from sim_core import (
    PointerSample,
    PointerSource,
    SimulatorDocument,
    closed_stroke,
    line_likeness,
    circle_candidate,
)


def p(x, y, t=0.0):
    return PointerSample(x=x, y=y, timestamp_s=t, source=PointerSource.SIMULATED_MOUSE)


class CoreReferenceTests(unittest.TestCase):
    def test_straight_stroke_is_line_like(self):
        pts = [p(0, 0), p(25, 0.4), p(50, -0.3), p(75, 0.1), p(100, 0)]
        self.assertLess(line_likeness(pts), 0.01)

    def test_bent_stroke_is_not_line_like(self):
        pts = [p(0, 0), p(25, 20), p(50, 30), p(75, 20), p(100, 0)]
        self.assertGreater(line_likeness(pts), 0.1)

    def test_closed_stroke(self):
        pts = [
            p(0, 0), p(10, -10), p(20, 0), p(20, 10),
            p(10, 20), p(0, 10), p(-2, 2), p(1, 1),
        ]
        self.assertTrue(closed_stroke(pts))

    def test_circle_candidate(self):
        pts = [
            p(10, 0), p(17, 3), p(20, 10), p(17, 17),
            p(10, 20), p(3, 17), p(0, 10), p(3, 3), p(10, 0),
        ]
        self.assertIsNotNone(circle_candidate(pts))

    def test_document_export_is_versioned(self):
        d = SimulatorDocument()
        d.add_line(p(0, 0), p(10, 0))
        out = d.export_dict()
        self.assertEqual(out["schema"], "craft-loop-windows-simulator/0.1")
        self.assertEqual(len(out["lines"]), 1)


if __name__ == "__main__":
    unittest.main()
