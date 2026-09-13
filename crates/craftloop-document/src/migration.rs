//! Schema migration framework.
//!
//! Execution 01, Phase 07, Task 052. Authority: Engine Contract 15;
//! Article 387 "Architecture Principle: Schema Evolution".
//!
//! There is exactly one schema version so far (`SchemaVersion::CURRENT`,
//! defined in Phase 01), so there is no *real* migration to register yet --
//! claiming one would violate the No-Hallucination Contract just as much as
//! claiming a document field that doesn't exist. What this task asks for
//! is the framework: a `Migration` trait operating on raw JSON (so it does
//! not depend on any particular historical `Document` struct still
//! compiling) and a runner that chains applicable migrations in order. The
//! test suite proves the mechanism with a synthetic migration rather than
//! a real historical one, and the crate's own doc comment says so plainly.

use craftloop_errors::{DomainError, PersistenceErrorKind};
use craftloop_serialization::SchemaVersion;

/// One step that upgrades a document's JSON representation from exactly
/// `source_version` to exactly `target_version`. Operates on `serde_json::Value`
/// rather than a typed struct so a migration written years from now can
/// still describe "how to read a version-1 document" without the
/// version-1 Rust struct needing to exist in the current codebase.
pub trait Migration {
    fn source_version(&self) -> SchemaVersion;
    fn target_version(&self) -> SchemaVersion;
    fn migrate(&self, value: serde_json::Value) -> Result<serde_json::Value, DomainError>;
}

/// Read a document JSON value's `schema_version` field without assuming
/// anything else about its shape (an old/malformed document might not
/// parse as the current `Document` struct at all).
fn read_schema_version(value: &serde_json::Value) -> Result<SchemaVersion, DomainError> {
    value
        .get("schema_version")
        .and_then(|v| v.as_u64())
        .map(|n| SchemaVersion(n as u32))
        .ok_or_else(|| DomainError::Persistence {
            kind: PersistenceErrorKind::CorruptedDocument,
            detail: "document JSON has no readable schema_version field".to_string(),
        })
}

/// Apply `migrations` in sequence until `value` reaches `target`
/// (typically `SchemaVersion::CURRENT`), or fail if no registered
/// migration covers the version the document is actually at.
pub fn migrate_to_current(
    mut value: serde_json::Value,
    migrations: &[Box<dyn Migration>],
    target: SchemaVersion,
) -> Result<serde_json::Value, DomainError> {
    loop {
        let current = read_schema_version(&value)?;
        if current == target {
            return Ok(value);
        }
        if current > target {
            return Err(DomainError::Persistence {
                kind: PersistenceErrorKind::SchemaVersionMismatch,
                detail: format!("document schema {current} is newer than target {target}"),
            });
        }
        let Some(step) = migrations.iter().find(|m| m.source_version() == current) else {
            return Err(DomainError::Persistence {
                kind: PersistenceErrorKind::SchemaVersionMismatch,
                detail: format!("no migration registered from schema {current} toward {target}"),
            });
        };
        value = step.migrate(value)?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// A synthetic "version 0 to version 1" migration, standing in for a
    /// real historical migration this codebase does not have yet (there is
    /// only ever been schema version 1). Proves the *mechanism*: renaming a
    /// field and bumping the version number.
    struct RenameTitleFieldV0ToV1;

    impl Migration for RenameTitleFieldV0ToV1 {
        fn source_version(&self) -> SchemaVersion {
            SchemaVersion(0)
        }
        fn target_version(&self) -> SchemaVersion {
            SchemaVersion(1)
        }
        fn migrate(&self, mut value: serde_json::Value) -> Result<serde_json::Value, DomainError> {
            if let Some(obj) = value.as_object_mut() {
                if let Some(old_title) = obj.remove("name") {
                    obj.insert("title".to_string(), old_title);
                }
                obj.insert("schema_version".to_string(), json!(1));
            }
            Ok(value)
        }
    }

    #[test]
    fn a_document_already_at_the_target_version_passes_through_unchanged() {
        let value = json!({ "schema_version": 1, "title": "Hello" });
        let migrations: Vec<Box<dyn Migration>> = vec![];
        let result = migrate_to_current(value.clone(), &migrations, SchemaVersion(1)).unwrap();
        assert_eq!(result, value);
    }

    #[test]
    fn a_registered_migration_upgrades_an_old_document() {
        let value = json!({ "schema_version": 0, "name": "Old Title" });
        let migrations: Vec<Box<dyn Migration>> = vec![Box::new(RenameTitleFieldV0ToV1)];
        let result = migrate_to_current(value, &migrations, SchemaVersion(1)).unwrap();
        assert_eq!(result["schema_version"], json!(1));
        assert_eq!(result["title"], json!("Old Title"));
        assert!(result.get("name").is_none());
    }

    #[test]
    fn an_unregistered_old_version_fails_with_a_structured_error_not_a_guess() {
        let value = json!({ "schema_version": 0, "name": "Old Title" });
        let migrations: Vec<Box<dyn Migration>> = vec![]; // no migration registered
        let result = migrate_to_current(value, &migrations, SchemaVersion(1));
        assert!(matches!(
            result,
            Err(DomainError::Persistence {
                kind: PersistenceErrorKind::SchemaVersionMismatch,
                ..
            })
        ));
    }

    #[test]
    fn a_document_newer_than_the_target_is_rejected_rather_than_downgraded() {
        let value = json!({ "schema_version": 2 });
        let migrations: Vec<Box<dyn Migration>> = vec![];
        let result = migrate_to_current(value, &migrations, SchemaVersion(1));
        assert!(result.is_err());
    }

    #[test]
    fn missing_schema_version_field_is_a_corrupted_document_error() {
        let value = json!({ "title": "no version field" });
        let migrations: Vec<Box<dyn Migration>> = vec![];
        let result = migrate_to_current(value, &migrations, SchemaVersion(1));
        assert!(matches!(
            result,
            Err(DomainError::Persistence {
                kind: PersistenceErrorKind::CorruptedDocument,
                ..
            })
        ));
    }
}
