//! Bounded single-page PDF export prototype.
//!
//! Execution 01, Phase 26, Task 188. Authority: MCP Article 128 "Document
//! Model".
//!
//! Platform-independent PDF generation with a full third-party PDF
//! library is not a stable Execution 01 dependency choice (license/build
//! footprint unverified for this workspace's target platforms). Per the
//! task's own escape hatch ("if platform-independent PDF is not stable
//! within Execution 01, provide a clearly documented temporary path and
//! test it"), this module hand-writes a minimal, valid single-page PDF
//! directly -- no external crate, byte-exact `xref` offsets, real
//! `/Length` values -- covering exactly the same visible content as
//! `svg::export_svg`.
//!
//! **Explicit, bounded fidelity limits** (this is a prototype, not a
//! production PDF writer):
//! - Circles and arcs are approximated as `CIRCLE_SEGMENTS`-sided
//!   polygons, not true PDF Bezier curves.
//! - Coordinates are passed straight through in document millimeters as
//!   PDF user-space units (no mm-to-point scaling, no y-axis flip), the
//!   same convention `svg::export_svg` uses -- internally consistent and
//!   testable, but **not** guaranteed to print at true physical scale on
//!   a real PDF-consuming printer. A future phase that owns physical
//!   print-scale fidelity should add that conversion explicitly rather
//!   than this prototype guessing at it.
//! - Text is written as literal bytes with the base 14 `Helvetica` font;
//!   non-ASCII characters are not guaranteed to render correctly.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use craftloop_dimension::DimensionAnnotation;
use craftloop_document::{DocumentUnits, Page, SemanticEntity};
use craftloop_geometry::Point2;
use craftloop_ids::StrokeId;
use craftloop_recognition::BeautifiedPrimitive;
use craftloop_standards::DimensionTypography;

use crate::visible_entities::visible_entities;

const CIRCLE_SEGMENTS: usize = 48;
const DEFAULT_TEXT_HEIGHT_MM: f64 = 3.5;
const MARGIN_MM: f64 = 10.0;

fn escape_pdf_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
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

fn write_polyline(out: &mut String, points: &[Point2], close: bool) {
    if points.is_empty() {
        return;
    }
    let _ = writeln!(out, "{} {} m", points[0].x, points[0].y);
    for p in &points[1..] {
        let _ = writeln!(out, "{} {} l", p.x, p.y);
    }
    if close {
        let _ = writeln!(out, "h");
    }
    let _ = writeln!(out, "S");
}

fn write_primitive(out: &mut String, primitive: &BeautifiedPrimitive, bounds: &mut Bounds) {
    match primitive {
        BeautifiedPrimitive::Line(segment) => {
            bounds.include(segment.a);
            bounds.include(segment.b);
            write_polyline(out, &[segment.a, segment.b], false);
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
            let points: Vec<Point2> = (0..CIRCLE_SEGMENTS)
                .map(|i| {
                    let angle = 2.0 * std::f64::consts::PI * (i as f64) / (CIRCLE_SEGMENTS as f64);
                    Point2::new(
                        circle.center.x + circle.radius * angle.cos(),
                        circle.center.y + circle.radius * angle.sin(),
                    )
                })
                .collect();
            write_polyline(out, &points, true);
        }
        BeautifiedPrimitive::Arc(arc) => {
            bounds.include(Point2::new(
                arc.center.x - arc.radius,
                arc.center.y - arc.radius,
            ));
            bounds.include(Point2::new(
                arc.center.x + arc.radius,
                arc.center.y + arc.radius,
            ));
            let steps = CIRCLE_SEGMENTS.max(2);
            let points: Vec<Point2> = (0..=steps)
                .map(|i| {
                    let t = arc.start_angle + arc.sweep_angle * (i as f64) / (steps as f64);
                    Point2::new(
                        arc.center.x + arc.radius * t.cos(),
                        arc.center.y + arc.radius * t.sin(),
                    )
                })
                .collect();
            for &p in &points {
                bounds.include(p);
            }
            write_polyline(out, &points, false);
        }
        BeautifiedPrimitive::Rectangle(rect) => {
            for corner in rect.corners {
                bounds.include(corner);
            }
            write_polyline(out, &rect.corners, true);
        }
    }
}

fn write_text(out: &mut String, position: Point2, size: f64, text: &str) {
    let _ = writeln!(
        out,
        "BT /F1 {size} Tf {} {} Td ({}) Tj ET",
        position.x,
        position.y,
        escape_pdf_text(text)
    );
}

fn build_pdf(media_box: (f64, f64, f64, f64), content_stream: &str) -> Vec<u8> {
    let (x0, y0, x1, y1) = media_box;
    let objects: Vec<String> = vec![
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [{x0} {y0} {x1} {y1}] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>"
        ),
        format!(
            "<< /Length {} >>\nstream\n{content_stream}endstream",
            content_stream.len()
        ),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
    ];

    let mut out = String::new();
    out.push_str("%PDF-1.4\n");
    let mut offsets = Vec::with_capacity(objects.len());
    for (i, obj) in objects.iter().enumerate() {
        offsets.push(out.len());
        let _ = write!(out, "{} 0 obj\n{obj}\nendobj\n", i + 1);
    }
    let xref_offset = out.len();
    let _ = writeln!(out, "xref\n0 {}", objects.len() + 1);
    out.push_str("0000000000 65535 f \n");
    for off in &offsets {
        let _ = writeln!(out, "{off:010} 00000 n ");
    }
    let _ = write!(
        out,
        "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref_offset}\n%%EOF",
        objects.len() + 1
    );
    out.into_bytes()
}

/// Render `page`'s visible content (same filtering as `svg::export_svg`,
/// Task 190) as a minimal single-page PDF. See module docs for the
/// explicit, bounded fidelity limitations of this prototype.
pub fn export_pdf_bounded(
    page: &Page,
    units: DocumentUnits,
    annotations: &[DimensionAnnotation],
    ephemeral_strokes: &BTreeSet<StrokeId>,
) -> Vec<u8> {
    let mut content = String::new();
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
                    write_polyline(&mut content, &points, false);
                }
            }
            SemanticEntity::Primitive { beautified, .. } => {
                write_primitive(&mut content, &beautified.primitive, &mut bounds);
            }
            SemanticEntity::Note(note) => {
                bounds.include(note.position);
                write_text(
                    &mut content,
                    note.position,
                    DEFAULT_TEXT_HEIGHT_MM,
                    &note.text,
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
                let text = format!("{displayed_value:.3}{}", units.suffix());
                write_text(
                    &mut content,
                    *presented.position,
                    presented.text_height,
                    &text,
                );
            }
            SemanticEntity::Conflict(_) => unreachable!("filtered by visible_entities"),
        }
    }

    let media_box = if bounds.is_empty() {
        (0.0, 0.0, 100.0, 100.0)
    } else {
        (
            bounds.min.x - MARGIN_MM,
            bounds.min.y - MARGIN_MM,
            bounds.max.x + MARGIN_MM,
            bounds.max.y + MARGIN_MM,
        )
    };

    build_pdf(media_box, &content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_dimension::{DimensionKind, DimensionRole, DimensionTarget, SemanticDimension};
    use craftloop_ids::{
        CraftLoopId, DimensionAnnotationId, DimensionId, NoteId, PageId, PrimitiveId,
    };
    use craftloop_recognition::Beautified;

    fn bytes_to_ascii(bytes: &[u8]) -> &str {
        std::str::from_utf8(bytes).expect("prototype PDF output must be valid UTF-8/ASCII")
    }

    #[test]
    fn output_starts_with_the_pdf_header_and_ends_with_eof() {
        let page = Page::new(PageId::new(), "Page 1");
        let bytes = export_pdf_bounded(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        let text = bytes_to_ascii(&bytes);
        assert!(text.starts_with("%PDF-1.4"));
        assert!(text.trim_end().ends_with("%%EOF"));
    }

    #[test]
    fn every_xref_offset_points_at_the_real_start_of_its_object() {
        let mut page = Page::new(PageId::new(), "Page 1");
        page.insert(SemanticEntity::Note(craftloop_document::Note::new(
            NoteId::new(),
            Point2::new(1.0, 1.0),
            "hello",
        )))
        .unwrap();
        let bytes = export_pdf_bounded(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        let text = bytes_to_ascii(&bytes);

        let xref_start = text.find("xref\n").unwrap();
        let lines: Vec<&str> = text[xref_start..].lines().collect();
        // lines[0] = "xref", lines[1] = "0 6", lines[2] = free entry, lines[3..8] = objects 1..5
        for (i, line) in lines.iter().enumerate().skip(3).take(5) {
            let offset: usize = line[..10].parse().unwrap();
            let object_number = i - 2; // line 3 -> object 1
            let expected_marker = format!("{object_number} 0 obj");
            assert_eq!(
                &text[offset..offset + expected_marker.len()],
                expected_marker,
                "xref entry for object {object_number} does not point at its own header"
            );
        }
    }

    #[test]
    fn the_stream_length_matches_the_actual_content_byte_count() {
        let mut page = Page::new(PageId::new(), "Page 1");
        page.insert(SemanticEntity::Note(craftloop_document::Note::new(
            NoteId::new(),
            Point2::new(1.0, 1.0),
            "hello",
        )))
        .unwrap();
        let bytes = export_pdf_bounded(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        let text = bytes_to_ascii(&bytes);

        let length_marker = "/Length ";
        let start = text.find(length_marker).unwrap() + length_marker.len();
        let end = start + text[start..].find(' ').unwrap();
        let declared_length: usize = text[start..end].parse().unwrap();

        let stream_start = text.find("stream\n").unwrap() + "stream\n".len();
        let stream_end = text[stream_start..].find("endstream").unwrap();
        assert_eq!(declared_length, stream_end);
    }

    #[test]
    fn a_beautified_line_produces_path_drawing_operators() {
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
        let bytes = export_pdf_bounded(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        let text = bytes_to_ascii(&bytes);
        assert!(text.contains(" m\n"));
        assert!(text.contains(" l\n"));
        assert!(text.contains("\nS\n"));
    }

    #[test]
    fn a_beautified_circle_produces_a_closed_polygon_path() {
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
        let bytes = export_pdf_bounded(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        let text = bytes_to_ascii(&bytes);
        assert!(text.contains(" m\n"));
        assert!(
            text.contains("\nh\n"),
            "a circle path must close (h operator)"
        );
        assert!(text.contains("\nS\n"));
    }

    #[test]
    fn a_beautified_arc_produces_an_open_polyline_path() {
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
        let bytes = export_pdf_bounded(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        let text = bytes_to_ascii(&bytes);
        assert!(text.contains(" m\n"));
        assert!(text.contains(" l\n"));
        assert!(text.contains("\nS\n"));
    }

    #[test]
    fn a_beautified_rectangle_produces_a_closed_four_sided_path() {
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
        let bytes = export_pdf_bounded(&page, DocumentUnits::Millimeters, &[], &BTreeSet::new());
        let text = bytes_to_ascii(&bytes);
        assert!(
            text.contains("\nh\n"),
            "a rectangle path must close (h operator)"
        );
    }

    #[test]
    fn ephemeral_command_ink_never_appears_in_pdf_content() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let ephemeral_id = StrokeId::new();
        let sample = craftloop_input::PointerSample {
            position: Point2::new(3.0, 4.0),
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

        let bytes = export_pdf_bounded(&page, DocumentUnits::Millimeters, &[], &ephemeral);
        let text = bytes_to_ascii(&bytes);
        assert!(!text.contains("3 4 m"));
    }

    #[test]
    fn a_dimension_value_is_converted_to_the_documents_display_unit_in_pdf_text() {
        let mut page = Page::new(PageId::new(), "Page 1");
        let dimension = SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            DimensionTarget::Single(PrimitiveId::new()),
            25.4,
        )
        .unwrap();
        let dimension_id = dimension.id;
        page.insert(SemanticEntity::Dimension(dimension)).unwrap();
        let annotation = DimensionAnnotation::new(
            DimensionAnnotationId::new(),
            dimension_id,
            Point2::new(1.0, 1.0),
        );

        let bytes = export_pdf_bounded(
            &page,
            DocumentUnits::Inches,
            &[annotation],
            &BTreeSet::new(),
        );
        let text = bytes_to_ascii(&bytes);
        assert!(text.contains("(1.000in)"));
        assert!(!text.contains("25.4"));
    }
}
