//! Reusable test fixture builders for the Craft Loop shared engineering
//! core.
//!
//! Execution 01, Phase 01, Task 012. Authority: Back End Skill "Required
//! Engineering Loop" (tests-first discipline needs repeatable fixtures);
//! Loop Engineering Contract ("Test: Write or update the smallest meaningful
//! test").
//!
//! Task 012's objective lists builders for "geometry, dimensions, views,
//! pointer samples, and deterministic IDs." At Phase 01 only the ID crate
//! (`craftloop-ids`) exists — geometry lands in Phase 02, pointer samples in
//! Phase 03, dimensions in Phase 10, views in Phase 20. Fabricating builder
//! functions for types that do not exist yet would violate the
//! No-Hallucination Contract ("an engine is complete because its API
//! compiles" is exactly the kind of false signal that forbids). So this
//! crate currently provides only [`DeterministicIdSequence`], and each later
//! phase is expected to add its own builder module here (`geometry`,
//! `pointer`, `dimensions`, `views`) alongside the domain type it fixtures,
//! rather than duplicating ad hoc fixture code inside every crate's own
//! `#[cfg(test)]` module.
//!
//! The stated non-goal is equally important: "without hiding domain
//! behavior behind excessive test magic." These builders construct plain
//! values through the same public constructors production code uses; they
//! do not reach into private state or bypass validation.

use craftloop_ids::CraftLoopId;

/// Produces a reproducible sequence of IDs of type `T` for tests, instead of
/// the random `CraftLoopId::new()` every production call site uses.
///
/// Deterministic IDs make test assertions and fixture diffs readable (e.g.
/// "StrokeId #1" behaves predictably run to run) without requiring the
/// production ID type to weaken its own uniqueness guarantee.
pub struct DeterministicIdSequence<T> {
    next: u128,
    _marker: std::marker::PhantomData<T>,
}

impl<T: CraftLoopId> DeterministicIdSequence<T> {
    /// Start a sequence at 1. Zero is reserved for [`CraftLoopId::nil`], so
    /// a sequence never accidentally produces the sentinel value.
    pub fn new() -> Self {
        Self {
            next: 1,
            _marker: std::marker::PhantomData,
        }
    }

    /// Start a sequence at an explicit offset, useful when a test wants
    /// several independent sequences that are guaranteed not to collide.
    pub fn starting_at(first: u128) -> Self {
        assert!(first != 0, "0 is reserved for CraftLoopId::nil()");
        Self {
            next: first,
            _marker: std::marker::PhantomData,
        }
    }

    /// Produce the next ID in the sequence.
    pub fn next_id(&mut self) -> T {
        let id = T::from_u128(self.next);
        self.next += 1;
        id
    }
}

impl<T: CraftLoopId> Default for DeterministicIdSequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: CraftLoopId> Iterator for DeterministicIdSequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        Some(self.next_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::StrokeId;

    #[test]
    fn sequence_produces_distinct_reproducible_ids() {
        let mut seq: DeterministicIdSequence<StrokeId> = DeterministicIdSequence::new();
        let a = seq.next_id();
        let b = seq.next_id();
        assert_ne!(a, b);

        let mut seq2: DeterministicIdSequence<StrokeId> = DeterministicIdSequence::new();
        let a2 = seq2.next_id();
        assert_eq!(a, a2, "same starting sequence must reproduce the same IDs");
    }

    #[test]
    fn sequence_never_collides_with_the_nil_sentinel() {
        let mut seq: DeterministicIdSequence<StrokeId> = DeterministicIdSequence::new();
        for _ in 0..1000 {
            assert_ne!(seq.next_id(), StrokeId::nil());
        }
    }

    #[test]
    fn starting_at_allows_independent_non_colliding_sequences() {
        let mut low: DeterministicIdSequence<StrokeId> = DeterministicIdSequence::starting_at(1);
        let mut high: DeterministicIdSequence<StrokeId> =
            DeterministicIdSequence::starting_at(1_000_000);
        let low_ids: Vec<_> = (0..5).map(|_| low.next_id()).collect();
        let high_ids: Vec<_> = (0..5).map(|_| high.next_id()).collect();
        for id in &low_ids {
            assert!(!high_ids.contains(id));
        }
    }

    #[test]
    #[should_panic(expected = "reserved for CraftLoopId::nil")]
    fn starting_at_zero_panics_because_zero_is_the_nil_sentinel() {
        let _: DeterministicIdSequence<StrokeId> = DeterministicIdSequence::starting_at(0);
    }

    #[test]
    fn sequence_implements_iterator_for_ergonomic_bulk_fixture_creation() {
        let seq: DeterministicIdSequence<StrokeId> = DeterministicIdSequence::new();
        let ids: Vec<StrokeId> = seq.take(3).collect();
        assert_eq!(ids.len(), 3);
        assert_eq!(ids[0], StrokeId::from_u128(1));
        assert_eq!(ids[2], StrokeId::from_u128(3));
    }
}
