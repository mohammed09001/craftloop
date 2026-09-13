//! Stroke grouping primitives.
//!
//! Execution 01, Phase 05, Task 035. Authority: Engine Contract 02; MCP
//! Article 242 "Detailed Specification of Stroke Grouping".
//!
//! Grouping is deliberately **reversible**: a group is nothing more than an
//! ordered list of `StrokeId`s. It never merges or mutates the underlying
//! strokes, so recomputing groups (with different tolerances, or after
//! deleting a stroke) never loses data -- there is nothing to lose. This is
//! the same "suggested information remains suggested" spirit as Article 5,
//! applied to temporal/spatial grouping instead of recognition.

use craftloop_ids::StrokeId;

use crate::stroke::Stroke;

/// How close in time and space two strokes must be to belong to the same
/// group.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroupingTolerance {
    /// Maximum gap, in seconds, between one stroke's last sample and the
    /// next stroke's first sample.
    pub max_time_gap_seconds: f64,
    /// Maximum gap, in document units, between the two strokes' bounding
    /// boxes (0 if they overlap).
    pub max_spatial_gap: f64,
}

impl GroupingTolerance {
    pub const fn new(max_time_gap_seconds: f64, max_spatial_gap: f64) -> Self {
        Self {
            max_time_gap_seconds,
            max_spatial_gap,
        }
    }
}

fn bounds_gap(a: craftloop_geometry::Bounds2, b: craftloop_geometry::Bounds2) -> f64 {
    let dx = (a.min.x.max(b.min.x)) - (a.max.x.min(b.max.x));
    let dy = (a.min.y.max(b.min.y)) - (a.max.y.min(b.max.y));
    // If the boxes overlap on an axis, that axis contributes 0 (not a
    // negative number) to the gap.
    dx.max(0.0).hypot(dy.max(0.0))
}

/// Group strokes into temporally/spatially contiguous clusters.
///
/// Strokes are first ordered by start time (input order is not assumed to
/// already be chronological -- e.g. a replayed scenario could interleave
/// strokes). A new group starts whenever consecutive strokes (in that
/// chronological order) exceed either tolerance. This is intentionally the
/// simplest grouping rule that satisfies Task 035: something more elaborate
/// (e.g. a graph-based clustering that can also merge across a temporal
/// gap if spatially coincident) can replace it later behind this same
/// function signature, with a test driving the richer behavior.
pub fn group_by_proximity(strokes: &[Stroke], tolerance: GroupingTolerance) -> Vec<Vec<StrokeId>> {
    let mut ordered: Vec<&Stroke> = strokes.iter().collect();
    ordered.sort_by(|a, b| {
        a.start_timestamp_seconds()
            .partial_cmp(&b.start_timestamp_seconds())
            .expect("timestamps are always finite (enforced by PointerSample::new)")
    });

    let mut groups: Vec<Vec<StrokeId>> = Vec::new();
    let mut current_group: Vec<StrokeId> = Vec::new();
    let mut previous: Option<&Stroke> = None;

    for stroke in ordered {
        let starts_new_group = match previous {
            None => true,
            Some(prev) => {
                let time_gap = stroke.start_timestamp_seconds() - prev.end_timestamp_seconds();
                let spatial_gap = bounds_gap(prev.bounds(), stroke.bounds());
                time_gap > tolerance.max_time_gap_seconds || spatial_gap > tolerance.max_spatial_gap
            }
        };
        if starts_new_group && !current_group.is_empty() {
            groups.push(std::mem::take(&mut current_group));
        }
        current_group.push(stroke.id);
        previous = Some(stroke);
    }
    if !current_group.is_empty() {
        groups.push(current_group);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_geometry::Point2;
    use craftloop_ids::CraftLoopId;
    use craftloop_input::{InputCapabilities, PointerButtons, PointerSample, PointerSource};

    fn stroke_at(x: f64, t: f64) -> Stroke {
        let sample = PointerSample::new(
            Point2::new(x, 0.0),
            t,
            None,
            None,
            None,
            PointerSource::SimulatedMouse,
            PointerButtons::default(),
            InputCapabilities::NONE,
        )
        .unwrap();
        Stroke::new(StrokeId::new(), vec![sample]).unwrap()
    }

    #[test]
    fn strokes_close_in_time_and_space_form_one_group() {
        let strokes = vec![stroke_at(0.0, 0.0), stroke_at(0.1, 0.05)];
        let groups = group_by_proximity(&strokes, GroupingTolerance::new(1.0, 5.0));
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 2);
    }

    #[test]
    fn strokes_far_apart_in_time_form_separate_groups() {
        let strokes = vec![stroke_at(0.0, 0.0), stroke_at(0.1, 10.0)];
        let groups = group_by_proximity(&strokes, GroupingTolerance::new(1.0, 5.0));
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn strokes_far_apart_in_space_form_separate_groups_even_if_close_in_time() {
        let strokes = vec![stroke_at(0.0, 0.0), stroke_at(1000.0, 0.05)];
        let groups = group_by_proximity(&strokes, GroupingTolerance::new(1.0, 5.0));
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn grouping_does_not_depend_on_input_order() {
        // Build the strokes once so both orderings group the *same*
        // strokes (by identity, not just by matching timestamps) -- two
        // separately constructed strokes would have different random IDs
        // even with identical positions/timestamps.
        let a = stroke_at(0.0, 0.0);
        let b = stroke_at(0.1, 0.1);
        let c = stroke_at(0.2, 0.2);

        let strokes_forward = vec![a.clone(), b.clone(), c.clone()];
        let strokes_reversed = vec![c, b, a];

        let tolerance = GroupingTolerance::new(1.0, 5.0);
        let groups_forward = group_by_proximity(&strokes_forward, tolerance);
        let groups_reversed = group_by_proximity(&strokes_reversed, tolerance);

        assert_eq!(groups_forward.len(), 1);
        assert_eq!(groups_reversed.len(), 1);
        // Same chronological order, so same membership order too.
        assert_eq!(groups_forward[0], groups_reversed[0]);
    }

    #[test]
    fn grouping_is_purely_an_id_partition_and_never_touches_stroke_content() {
        let strokes = vec![stroke_at(0.0, 0.0), stroke_at(0.1, 0.05)];
        let before: Vec<Stroke> = strokes.clone();
        let _ = group_by_proximity(&strokes, GroupingTolerance::new(1.0, 5.0));
        assert_eq!(
            strokes, before,
            "grouping must not mutate the strokes it groups"
        );
    }

    #[test]
    fn empty_input_produces_no_groups() {
        let groups = group_by_proximity(&[], GroupingTolerance::new(1.0, 1.0));
        assert!(groups.is_empty());
    }
}
