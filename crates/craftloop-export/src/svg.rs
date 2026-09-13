//! SVG export.
//!
//! Execution 01, Phase 26, Task 187. Authority: MCP Article 128 "Document
//! Model" (visible engineering content), Article 72 "Unit System".
//!
//! Exports visible vector geometry, freeform ink, notes, and dimension
//! value labels as a single-page SVG document. **Native semantics do not
//! survive**: a generic SVG viewer has no concept of `DimensionRole`,
//! constraint state, provenance, or `LineRole` beyond the raw dash
//! pattern/weight `craftloop-standards` (Phase 25) resolves it to --
//! anyone consuming this output sees shapes and text, not engineering
//! meaning. Only what `visible_entities::is_export_visible` allows through
//! (Task 190) is drawn; ephemeral command ink and `SemanticEntity::Conflict`
//! diagnostics never appear here.
//!
//! Dimension rendering is deliberately bounded: no phase through 26 has
//! built real dimension-line/extension-line-to-feature projection
//! geometry (`craftloop_dimension::DimensionAnnotation` stores only a
//! label position, not leader geometry), so a dimension renders as its
//! presented value as text at the annotation's position -- accurate, but
//! not a full leader-line drawing.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use craftloop_dimension::DimensionAnnotation;
use craftloop_document::{DocumentUnits, Page, SemanticEntity};
use craftloop_geometry::Point2;
use craftloop_ids::StrokeId;
use craftloop_recognition::BeautifiedPrimitive;
use craftloop_standards::{style_for_role, DimensionTypography, LineRole};

use crate::visible_entities::visible_entities;

const MARGIN_MM: f64 = 10.0;
const INK_STROKE_WIDTH_MM: f64 = 0.35;
const DEFAULT_TEXT_HEIGHT_MM: f64 = 3.5;

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

struct Bounds {
    min: Point2,
    max: Point2,
}

impl Bounds {
    fn new() -> Self {
        Self {
            min: Point2::new(f64::INFINITY, f64::INFINITY),
            max: Point2::new(f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    fn include(&mut self, p: Point2) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
    }

    fn is_empty(&self) -> bool {
        !self.min.x.is_finite()
    }
}

fn dasharray(pattern: &[f64]) -> String {
    if pattern.is_empty() {
        "none".to_string()
    } else {
        pattern
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

/// Render one geometry primitive as an SVG element, using `role`'s
/// `craftloop-standards` style, tracking every point touched in `bounds`.
fn write_primitive(out: &mut String, primitive: &BeautifiedPrimitive, bounds: &mut Bounds) {
    let style = style_for_role(LineRole::Visible);
    let stroke_width = style.weight;
    let dash = dasharray(&style.dash_pattern);
    match primitive {
        BeautifiedPrimitive::Line(segment) => {
            bounds.include(segment.a);
            bounds.include(segment.b);
            let _ = writeln!(
                out,
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="{stroke_width}" stroke-dasharray="{dash}"/>"#,
                segment.a.x, segment.a.y, segment.b.x, segment.b.y
            );
        }
        BeautifiedPrimitive::Circle(circle) => {
            bounds.include(Point2::new(
                circle.center.x - circle.radius,
                circle.center.y - circle.radius,
            ));
            bounds.include(Point2::new(
                circle.center.x + circle.radius,
                circle.center.y + circle.radius,
            ));
            let _ = writeln!(
                out,
                r#"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="black" stroke-width="{stroke_width}" stroke-dasharray="{dash}"/>"#,
                circle.center.x, circle.center.y, circle.radius
            );
        }
        BeautifiedPrimitive::Arc(arc) => {
            let start = Point2::new(
                arc.center.x + arc.radius * arc.start_angle.cos(),
                arc.center.y + arc.radius * arc.start_angle.sin(),
            );
            let end_angle = arc.start_angle + arc.sweep_angle;
            let end = Point2::new(
                arc.center.x + arc.radius * end_angle.cos(),
                arc.center.y + arc.radius * end_angle.sin(),
            );
            bounds.include(start);
            bounds.include(end);
            bounds.include(Point2::new(
                arc.center.x - arc.radius,
                arc.center.y - arc.radius,
            ));
            bounds.include(Point2::new(
                arc.center.x + arc.radius,
                arc.center.y + arc.radius,
            ));
            let large_arc = if arc.sweep_angle.abs() > std::f64::consts::PI {
                1
            } else {
                0
            };
            let sweep_flag = if arc.sweep_angle > 0.0 { 1 } else { 0 };
            let _ = writeln!(
                out,
                r#"<path d="M {} {} A {} {} 0 {} {} {} {}" fill="none" stroke="black" stroke-width="{stroke_width}" stroke-dasharray="{dash}"/>"#,
                start.x, start.y, arc.radius, arc.radius, large_arc, sweep_flag, end.x, end.y
            );
        }
        BeautifiedPrimitive::Rectangle(rect) => {
            for corner in rect.corners {
                bounds.include(corner);
            }
            let points = rect
                .corners
                .iter()
                .map(|p| format!("{},{}", p.x, p.y))
                .collect::<Vec<_>>()
                .join(" ");
            let _ = writeln!(
                out,
                r#"<polygon points="{points}" fill="none" stroke="black" stroke-width="{stroke_width}" stroke-dasharray="{dash}"/>"#
            );
        }
    }
}

/// Render `page`'s visible content as a self-contained SVG document.
/// `units` is the document's display unit (Task 189: dimension text shows
/// this unit's suffix, converted from the canonical-millimeter stored
/// value -- never the raw millimeter number silently relabeled).
/// `annotations` supplies the position/visibility of each dimension's
/// label; `ephemeral_strokes` is passed straight through to
/// `visible_entities` (Task 190).
pub fn export_svg(
    page: &Page,
    units: DocumentUnits,
    annotations: &[DimensionAnnotation],
    ephemeral_strokes: &BTreeSet<StrokeId>,
) -> String {
    let mut body = String::new();
    let mut bounds = Bounds::new();
    let typography = DimensionTypography::new(DEFAULT_TEXT_HEIGHT_MM);

    for entity in visible_entities(page, ephemeral_strokes) {
        match entity {
            SemanticEntity::Stroke(stroke) => {
                let points: Vec<Point2> = stroke.samples().iter().map(|s| s.position).collect();
                for &p in &points {
                    bounds.include(p);
                }
                if points.len() >= 2 {
                    let path = points
                        .iter()
                        .map(|p| format!("{},{}", p.x, p.y))
                        .collect::<Vec<_>>()
                        .join(" ");
                    let _ = writeln!(
                        body,
                        r#"<polyline points="{path}" fill="none" stroke="black" stroke-width="{INK_STROKE_WIDTH_MM}"/>"#
                    );
                }
            }
            SemanticEntity::Primitive { beautified, .. } => {
                write_primitive(&mut body, &beautified.primitive, &mut bounds);
            }
            SemanticEntity::Note(note) => {
                bounds.include(note.position);
                let _ = writeln!(
                    body,
                    r#"<text x="{}" y="{}" font-size="{DEFAULT_TEXT_HEIGHT_MM}">{}</text>"#,
                    note.position.x,
                    note.position.y,
                    escape_xml(&note.text)
                );
            }
            SemanticEntity::Dimension(dimension) => {
                let Some(annotation) = annotations
                    .iter()
                    .find(|a| a.dimension_id == dimension.id && a.visible)
                else {
                    continue;
                };
                let presented = craftloop_standards::present(dimension, annotation, &typography);
                bounds.include(*presented.position);
                let displayed_value = units.from_millimeters(presented.value);
                let _ = writeln!(
                    body,
                    r#"<text x="{}" y="{}" font-size="{}">{:.3}{}</text>"#,
                    presented.position.x,
                    presented.position.y,
                    presented.text_height,
                    displayed_value,
                    units.suffix()
                );
            }
            SemanticEntity::Conflict(_) => unreachable!("filtered by visible_entities"),
        }
    }

    let (min_x, min_y, width, height) = if bounds.is_empty() {
        (0.0, 0.0, 100.0, 100.0)
    } else {
        (
            bounds.min.x - MARGIN_MM,
            bounds.min.y - MARGIN_MM,
            (bounds.max.x - bounds.min.x) + 2.0 * MARGIN_MM,
            (bounds.max.y - bounds.min.y) + 2.0 * MARGIN_MM,
        )
    };

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{min_x} {min_y} {width} {height}">
{body}</svg>
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_dimension::{DimensionKind, DimensionRole, DimensionTarget, SemanticDimension};
    use craftloop_ids::{
        CraftLoopId, DimensionAnnotationId, DimensionId, NoteId, PageId, PrimitiveId,
    };
    use craftloop_recognition::Beautified;

    #[test]
    fn a_note_becomes_visible_svg_text() {
        let mut page = Page::new(PageId::new(), "Page 1");
        page.insert(SemanticEntity::Note(craftloop_document::Note::new(
            NoteId::new(),
            Point2::new(5.0, 5.0),
            "M6 THRU",
        )))
        .unwrap();
        let svg = export_svg(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        assert!(svg.contains("M6 THRU"));
        assert!(svg.starts_with("<svg"));
    }

    #[test]
    fn a_beautified_line_becomes_an_svg_line_element() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let segment =
            craftloop_geometry::Segment2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 0.0));
        page.insert(SemanticEntity::Primitive {
            id: PrimitiveId::new(),
            beautified: Beautified {
                primitive: BeautifiedPrimitive::Line(segment),
                displacement: 0.0,
            },
        })
        .unwrap();
        let svg = export_svg(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        assert!(svg.contains("<line"));
    }

    #[test]
    fn a_beautified_circle_becomes_an_svg_circle_element() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let circle = craftloop_geometry::Circle2::new(Point2::new(5.0, 5.0), 3.0).unwrap();
        page.insert(SemanticEntity::Primitive {
            id: PrimitiveId::new(),
            beautified: Beautified {
                primitive: BeautifiedPrimitive::Circle(circle),
                displacement: 0.0,
            },
        })
        .unwrap();
        let svg = export_svg(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        assert!(svg.contains("<circle"));
    }

    #[test]
    fn a_beautified_arc_becomes_an_svg_path_element() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let arc = craftloop_geometry::Arc2::new(
            Point2::new(0.0, 0.0),
            4.0,
            0.0,
            std::f64::consts::FRAC_PI_2,
        )
        .unwrap();
        page.insert(SemanticEntity::Primitive {
            id: PrimitiveId::new(),
            beautified: Beautified {
                primitive: BeautifiedPrimitive::Arc(arc),
                displacement: 0.0,
            },
        })
        .unwrap();
        let svg = export_svg(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        assert!(svg.contains("<path"));
        assert!(svg.contains(" A "));
    }

    #[test]
    fn a_beautified_rectangle_becomes_an_svg_polygon_element() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let rectangle = craftloop_geometry::RelationalRectangle::from_axis_aligned(
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 5.0),
        )
        .unwrap();
        page.insert(SemanticEntity::Primitive {
            id: PrimitiveId::new(),
            beautified: Beautified {
                primitive: BeautifiedPrimitive::Rectangle(rectangle),
                displacement: 0.0,
            },
        })
        .unwrap();
        let svg = export_svg(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        assert!(svg.contains("<polygon"));
    }

    #[test]
    fn ephemeral_command_ink_never_appears_in_svg_output() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let ephemeral_id = StrokeId::new();
        let sample = craftloop_input::PointerSample {
            position: Point2::ORIGIN,
            timestamp_seconds: 0.0,
            pressure: None,
            tilt_x_deg: None,
            tilt_y_deg: None,
            source: craftloop_input::PointerSource::Stylus,
            buttons: Default::default(),
            capabilities: Default::default(),
        };
        page.insert(SemanticEntity::Stroke(
            craftloop_ink::Stroke::new(ephemeral_id, vec![sample, sample]).unwrap(),
        ))
        .unwrap();
        let mut ephemeral = BTreeSet::new();
        ephemeral.insert(ephemeral_id);

        let svg = export_svg(&page, DocumentUnits::Millimeters, &[], &ephemeral);
        assert!(!svg.contains("polyline"));
    }

    #[test]
    fn a_dimension_value_renders_converted_to_the_documents_display_unit() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let dimension = SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            25.4, // exactly 1 inch, stored canonically in mm
        )
        .unwrap();
        let dimension_id = dimension.id;
        page.insert(SemanticEntity::Dimension(dimension)).unwrap();
        let annotation = DimensionAnnotation::new(
            DimensionAnnotationId::new(),
            dimension_id,
            Point2::new(1.0, 1.0),
        );

        let svg = export_svg(
            &page,
            DocumentUnits::Inches,
            &[annotation],
            &BTreeSet::new(),
        );
        assert!(
            svg.contains("1.000in"),
            "expected the inch-converted value, got: {svg}"
        );
        assert!(
            !svg.contains("25.4"),
            "must not leak the raw millimeter value when the document unit is inches"
        );
    }

    #[test]
    fn a_hidden_annotation_is_not_rendered() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let dimension = SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            10.0,
        )
        .unwrap();
        let dimension_id = dimension.id;
        page.insert(SemanticEntity::Dimension(dimension)).unwrap();
        let mut annotation = DimensionAnnotation::new(
            DimensionAnnotationId::new(),
            dimension_id,
            Point2::new(1.0, 1.0),
        );
        annotation.visible = false;

        let svg = export_svg(
            &page,
            DocumentUnits::Millimeters,
            &[annotation],
            &BTreeSet::new(),
        );
        assert!(!svg.contains("10.000mm"));
    }

    #[test]
    fn an_empty_page_still_produces_a_well_formed_svg() {
        let page = Page::new(PageId::new(), "Page 1");
        let svg = export_svg(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        assert!(svg.starts_with("<svg"));
        assert!(svg.trim_end().ends_with("</svg>"));
    }
}
