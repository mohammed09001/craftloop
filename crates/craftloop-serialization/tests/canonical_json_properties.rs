//! Property-based tests for `to_canonical_json`.
//!
//! Execution 01, Phase 27, Task 195. Authority: MCP Article 386
//! "Architecture Principle: Deterministic Serialization" -- "two
//! semantically identical documents must serialize to byte-identical
//! output."
//!
//! Two invariants, checked against hundreds of randomly generated maps
//! rather than one or two hand-picked fixtures: serializing the same
//! value twice produces byte-identical output (determinism), and the
//! serialized form round-trips back to an equal value (no silent data
//! loss).

use std::collections::BTreeMap;

use craftloop_serialization::to_canonical_json;
use proptest::prelude::*;

proptest! {
    #[test]
    fn serializing_the_same_value_twice_is_byte_identical(
        entries in prop::collection::btree_map("[a-z]{1,8}", any::<i64>(), 0..12)
    ) {
        let map: BTreeMap<String, i64> = entries;
        let first = to_canonical_json(&map).unwrap();
        let second = to_canonical_json(&map).unwrap();
        prop_assert_eq!(first, second);
    }

    #[test]
    fn canonical_json_round_trips_to_an_equal_value(
        entries in prop::collection::btree_map("[a-z]{1,8}", any::<i64>(), 0..12)
    ) {
        let map: BTreeMap<String, i64> = entries;
        let json = to_canonical_json(&map).unwrap();
        let back: BTreeMap<String, i64> = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(map, back);
    }
}
