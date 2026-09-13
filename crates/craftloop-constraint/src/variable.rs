//! Solver variables.
//!
//! Execution 01, Phase 11, Task 078. Authority: Engine Contract 10
//! (Constraint Solver: "Variables, constraints, incremental solve,
//! diagnostics").
//!
//! A [`VariableId`] is a solver-internal scalar unknown -- e.g. "the x
//! coordinate of point P" -- deliberately **not** a `craftloop-ids` domain
//! identifier. A `PrimitiveId` names a geometric entity; a constraint
//! solver reasons about numbers (a point contributes two variables, its x
//! and y), and several different domain entities can share the same
//! underlying variable (two coincident points' x-coordinates, for
//! instance). Conflating the two would force every future backend to
//! adopt Craft Loop's domain ID scheme even though the whole point of this
//! interface is to stay backend-neutral (Task 078's own title: "before
//! choosing implementation").

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VariableId(pub u64);

/// One scalar unknown plus the initial guess a numeric solver iterates
/// from. Every real constraint solver (Newton-style or otherwise) needs a
/// starting point; garbage-in/garbage-out for this value is a known
/// failure mode, so it is a required field, never defaulted to zero
/// silently.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub id: VariableId,
    pub initial_value: f64,
}

impl Variable {
    pub fn new(id: VariableId, initial_value: f64) -> Self {
        Self { id, initial_value }
    }
}

/// The two scalar variables backing one 2D point, grouped for readability
/// at constraint call sites. Not a solver primitive itself -- every
/// constraint below still ultimately refers to the two `VariableId`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointVariables {
    pub x: VariableId,
    pub y: VariableId,
}

impl PointVariables {
    pub fn new(x: VariableId, y: VariableId) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_id_supports_equality_and_ordering_for_deterministic_output() {
        let a = VariableId(1);
        let b = VariableId(2);
        assert!(a < b);
        assert_ne!(a, b);
    }

    #[test]
    fn point_variables_group_two_distinct_ids() {
        let p = PointVariables::new(VariableId(0), VariableId(1));
        assert_ne!(p.x, p.y);
    }

    #[test]
    fn serialization_round_trips() {
        let v = Variable::new(VariableId(3), 12.5);
        let json = serde_json::to_string(&v).unwrap();
        let back: Variable = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }
}
