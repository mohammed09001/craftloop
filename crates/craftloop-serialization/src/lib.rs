//! Deterministic serialization conventions for the Craft Loop shared
//! engineering core.
//!
//! Execution 01, Phase 01, Task 010. Authority: MCP Article 92
//! "Determinism", Article 386 "Architecture Principle: Deterministic
//! Serialization", Article 387 "Architecture Principle: Schema Evolution".
//!
//! Three conventions live here so every later crate follows them instead of
//! reinventing them per-engine:
//!
//! 1. **Schema versioning** — [`SchemaVersion`]. Every persisted document or
//!    wire payload carries one so Phase 07's migration framework has
//!    something concrete to act on.
//! 2. **Numeric policy** — [`OrderedF64`]. Plain `f64` has no total order
//!    (NaN) and JSON has no NaN/Infinity representation at all. Anything
//!    that must be hashed, sorted, or persisted uses this wrapper instead of
//!    a bare `f64`, so "not a number" is rejected at the boundary rather
//!    than silently serialized as `null` or crashing a sort.
//! 3. **Canonical ordering** — [`to_canonical_json`]. Two semantically
//!    identical documents must serialize to byte-identical output so
//!    golden/save-reopen tests (Phase 07, Task 055) and diagnostic export
//!    (Engine Contract 26) are reproducible. This crate deliberately does
//!    **not** enable `serde_json`'s `preserve_order` feature anywhere in the
//!    workspace: `serde_json::Value`'s default map type is a `BTreeMap`, so
//!    object keys are always emitted in sorted order. Array element order is
//!    never reordered, because array order is frequently semantically
//!    meaningful (e.g. an ordered stroke's pointer samples).

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

/// A monotonically increasing schema generation number for a persisted
/// document or wire payload.
///
/// Convention: bump this only when the on-disk/wire shape changes in a way
/// that requires a migration step (Phase 07, Task 052). Never silently
/// reinterpret an old schema version as the current one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaVersion(pub u32);

impl SchemaVersion {
    /// The schema version this build of the shared core writes.
    pub const CURRENT: SchemaVersion = SchemaVersion(1);

    /// True if a document at this version can be read (directly or via
    /// migration) by a build whose current version is `current`. A future
    /// schema version can never be read by an older build: that is a
    /// blocker (True Blocker Policy: "contradictory requirements"), not
    /// something to guess through.
    pub fn is_readable_by(&self, current: SchemaVersion) -> bool {
        self.0 <= current.0
    }
}

impl std::fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "schema-v{}", self.0)
    }
}

/// A finite `f64` with a total order, for contexts that need to sort or
/// canonically compare floating-point domain values (e.g. canonical export,
/// deterministic test fixtures).
///
/// Deliberately rejects NaN and infinite values at construction: those have
/// no defensible canonical JSON representation and MCP Article 4 forbids
/// fabricating engineering certainty out of an undefined numeric state.
/// Geometry code that can legitimately produce non-finite intermediate
/// values (e.g. a divide-by-zero during a solve attempt) must catch that at
/// the geometry/solver boundary and raise a structured `DomainError`
/// (`craftloop-errors`) rather than let it reach this type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OrderedF64(f64);

impl OrderedF64 {
    pub fn new(value: f64) -> Option<Self> {
        if value.is_finite() {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

impl PartialEq for OrderedF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}
impl Eq for OrderedF64 {}

impl PartialOrd for OrderedF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

/// Serialize `value` to JSON with canonical (sorted) object-key ordering by
/// round-tripping through `serde_json::Value` first. See the module
/// documentation for why this is safe and necessary.
pub fn to_canonical_json<T: Serialize>(value: &T) -> serde_json::Result<String> {
    let as_value = serde_json::to_value(value)?;
    serde_json::to_string(&as_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn schema_version_current_is_readable_by_itself() {
        assert!(SchemaVersion::CURRENT.is_readable_by(SchemaVersion::CURRENT));
    }

    #[test]
    fn future_schema_version_is_not_readable_by_an_older_build() {
        let future = SchemaVersion(SchemaVersion::CURRENT.0 + 1);
        assert!(!future.is_readable_by(SchemaVersion::CURRENT));
    }

    #[test]
    fn older_schema_version_is_readable_by_current() {
        let old = SchemaVersion(0);
        assert!(old.is_readable_by(SchemaVersion::CURRENT));
    }

    #[test]
    fn ordered_f64_rejects_nan_and_infinity() {
        assert!(OrderedF64::new(f64::NAN).is_none());
        assert!(OrderedF64::new(f64::INFINITY).is_none());
        assert!(OrderedF64::new(f64::NEG_INFINITY).is_none());
        assert!(OrderedF64::new(1.5).is_some());
    }

    #[test]
    fn ordered_f64_sorts_deterministically() {
        let mut values: Vec<OrderedF64> = [3.0, -1.0, 0.0, -0.0, 2.5]
            .into_iter()
            .map(|v| OrderedF64::new(v).unwrap())
            .collect();
        values.sort();
        let as_f64: Vec<f64> = values.iter().map(|v| v.get()).collect();
        // total_cmp orders -0.0 strictly before 0.0; this is intentional and
        // documented, not an accident: it gives a genuine total order.
        assert_eq!(as_f64, vec![-1.0, -0.0, 0.0, 2.5, 3.0]);
    }

    #[derive(Serialize)]
    struct OutOfOrderFields {
        zeta: u32,
        alpha: u32,
        middle: Vec<u32>,
    }

    #[test]
    fn canonical_json_sorts_object_keys_but_preserves_array_order() {
        let value = OutOfOrderFields {
            zeta: 1,
            alpha: 2,
            middle: vec![9, 1, 5],
        };
        let canonical = to_canonical_json(&value).expect("serialize");
        assert_eq!(canonical, r#"{"alpha":2,"middle":[9,1,5],"zeta":1}"#);
    }

    #[test]
    fn canonical_json_is_identical_for_structurally_equal_values_regardless_of_field_order() {
        #[derive(Serialize)]
        struct A {
            a: u32,
            b: u32,
        }
        #[derive(Serialize)]
        struct B {
            b: u32,
            a: u32,
        }
        let a = to_canonical_json(&A { a: 1, b: 2 }).unwrap();
        let b = to_canonical_json(&B { b: 2, a: 1 }).unwrap();
        assert_eq!(a, b);
    }
}
