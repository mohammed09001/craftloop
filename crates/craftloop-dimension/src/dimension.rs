//! The semantic dimension itself.
//!
//! Execution 01, Phase 10, Tasks 070, 073. Authority: Engine Contract 09.

use craftloop_errors::{DimensionErrorKind, DomainError, DomainResult};
use craftloop_ids::DimensionId;
use serde::{Deserialize, Serialize};

use crate::kind::DimensionKind;
use crate::role::DimensionRole;
use crate::target::DimensionTarget;

/// A semantic dimension: what it measures ([`DimensionKind`]/[`DimensionTarget`]),
/// what role it plays ([`DimensionRole`]), and its current canonical value
/// -- millimeters for `Linear`/`Radius`/`Diameter`, radians for `Angular`
/// (`DimensionKind::is_angular`). This type has no visual/presentation
/// fields at all; see `annotation.rs` for that layer (Task 071).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticDimension {
    pub id: DimensionId,
    pub kind: DimensionKind,
    pub role: DimensionRole,
    pub target: DimensionTarget,
    value: f64,
    /// Only meaningful when `role == Bounded` (Task 073), but stored
    /// unconditionally rather than gated on role: a `Driving` dimension
    /// that a future constraint solver (Phase 12) additionally restricts
    /// to a feasible range is a legitimate combination this field already
    /// supports without a type change.
    feasible_range: Option<(f64, f64)>,
}

impl SemanticDimension {
    pub fn new(
        id: DimensionId,
        kind: DimensionKind,
        role: DimensionRole,
        target: DimensionTarget,
        value: f64,
    ) -> DomainResult<Self> {
        validate_value(kind, value)?;
        Ok(Self {
            id,
            kind,
            role,
            target,
            value,
            feasible_range: None,
        })
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn feasible_range(&self) -> Option<(f64, f64)> {
        self.feasible_range
    }

    /// Attach a feasible range, validating `min <= value <= max` and
    /// `min <= max`. Does not require `role == Bounded` at the type level
    /// (see the field doc), but a caller building a `Bounded` dimension
    /// should always set one.
    pub fn with_feasible_range(mut self, min: f64, max: f64) -> DomainResult<Self> {
        if !(min.is_finite() && max.is_finite()) || min > max {
            return Err(DomainError::Dimension {
                kind: DimensionErrorKind::InvalidValue,
                detail: format!("invalid feasible range [{min}, {max}]"),
            });
        }
        if self.value < min || self.value > max {
            return Err(DomainError::Dimension {
                kind: DimensionErrorKind::InvalidValue,
                detail: format!(
                    "current value {} is outside feasible range [{min}, {max}]",
                    self.value
                ),
            });
        }
        self.feasible_range = Some((min, max));
        Ok(self)
    }

    /// Internal setter used by `DimensionStore::edit_driving_value` (Task
    /// 075) after it has already confirmed `role.is_directly_editable()`.
    /// Not `pub`: every value change must go through the store so it can
    /// be recorded as one atomic, auditable edit rather than a bare field
    /// assignment any caller could reach around the role check with.
    pub(crate) fn set_value_unchecked(&mut self, value: f64) -> DomainResult<()> {
        validate_value(self.kind, value)?;
        if let Some((min, max)) = self.feasible_range {
            if value < min || value > max {
                return Err(DomainError::Dimension {
                    kind: DimensionErrorKind::InvalidValue,
                    detail: format!("value {value} is outside feasible range [{min}, {max}]"),
                });
            }
        }
        self.value = value;
        Ok(())
    }
}

fn validate_value(kind: DimensionKind, value: f64) -> DomainResult<()> {
    if !value.is_finite() {
        return Err(DomainError::Dimension {
            kind: DimensionErrorKind::InvalidValue,
            detail: format!("{kind:?} dimension value must be finite, got {value}"),
        });
    }
    let must_be_positive = matches!(
        kind,
        DimensionKind::Linear | DimensionKind::Radius | DimensionKind::Diameter
    );
    if must_be_positive && value <= 0.0 {
        return Err(DomainError::Dimension {
            kind: DimensionErrorKind::InvalidValue,
            detail: format!("{kind:?} dimension value must be positive, got {value}"),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use craftloop_ids::{CraftLoopId, PrimitiveId};

    fn target() -> DimensionTarget {
        DimensionTarget::Single(PrimitiveId::new())
    }

    #[test]
    fn a_positive_linear_value_is_accepted() {
        let dim = SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            target(),
            25.0,
        );
        assert!(dim.is_ok());
    }

    #[test]
    fn a_non_positive_linear_value_is_rejected() {
        assert!(SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            target(),
            0.0
        )
        .is_err());
        assert!(SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Driving,
            target(),
            -5.0
        )
        .is_err());
    }

    #[test]
    fn angular_dimensions_may_be_negative_or_zero() {
        assert!(SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Angular,
            DimensionRole::Driving,
            target(),
            0.0
        )
        .is_ok());
        assert!(SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Angular,
            DimensionRole::Driving,
            target(),
            -1.2
        )
        .is_ok());
    }

    #[test]
    fn non_finite_values_are_always_rejected() {
        assert!(SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Angular,
            DimensionRole::Driving,
            target(),
            f64::NAN
        )
        .is_err());
        assert!(SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Angular,
            DimensionRole::Driving,
            target(),
            f64::INFINITY
        )
        .is_err());
    }

    #[test]
    fn feasible_range_rejects_a_current_value_outside_it() {
        let dim = SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Bounded,
            target(),
            5.0,
        )
        .unwrap();
        assert!(dim.with_feasible_range(10.0, 20.0).is_err());
    }

    #[test]
    fn feasible_range_accepts_a_current_value_inside_it() {
        let dim = SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Bounded,
            target(),
            15.0,
        )
        .unwrap();
        let dim = dim.with_feasible_range(10.0, 20.0).unwrap();
        assert_eq!(dim.feasible_range(), Some((10.0, 20.0)));
    }

    #[test]
    fn set_value_unchecked_enforces_the_feasible_range_too() {
        let dim = SemanticDimension::new(
            DimensionId::new(),
            DimensionKind::Linear,
            DimensionRole::Bounded,
            target(),
            15.0,
        )
        .unwrap();
        let mut dim = dim.with_feasible_range(10.0, 20.0).unwrap();
        assert!(dim.set_value_unchecked(25.0).is_err());
        assert!(dim.set_value_unchecked(18.0).is_ok());
        assert_eq!(dim.value(), 18.0);
    }
}
