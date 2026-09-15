//! Atomic local persistence and integrity/recovery.
//!
//! Execution 01, Phase 07, Tasks 051 and 053. Authority: Engine Contract 15
//! ("Confirmed semantics survive recognition-model changes"); MCP Article
//! 81 "Local-First Product Behavior", Article 415/416 "Reliability
//! Requirement: Crash Recovery/Corruption Detection".
//!
//! **Save (Task 051):** never write over the live document file in place.
//! Serialize to canonical JSON, write it to a uniquely-named temp file in
//! the same directory, `sync_all` it to disk, best-effort snapshot the
//! previous good file as a `.bak`, then atomically rename the temp file
//! onto the real path. A crash at any point before the final rename leaves
//! the original file untouched; a crash during/after the rename leaves
//! either the old or the new content, in full, never a half-written mix.
//!
//! **Load with recovery (Task 053):** parse and validate the primary file;
//! if that fails (missing, malformed JSON, or `Document::validate` fails),
//! fall back to the `.bak` snapshot before giving up.
//!
//! **`wasm32-unknown-unknown` note (Execution 03, Phase 03, Task 021):**
//! this module compiles cleanly there -- `std::fs` exists as a stub on
//! that target -- but every call returns an IO error at runtime, since
//! `wasm32-unknown-unknown` has no filesystem. This is deliberately not
//! `cfg`-gated out: doing so would fork the engine's public surface per
//! platform for no compile-time benefit. `craftloop-web-bridge` must
//! simply never call [`save_document_atomically`] or [`load_document`];
//! it persists through [`craftloop_serialization::to_canonical_json`]
//! directly plus a browser storage adapter instead (Article 42-43).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use craftloop_errors::DomainError;

use crate::document::Document;

#[derive(Debug, thiserror::Error)]
pub enum PersistError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("integrity error: {0}")]
    Integrity(#[from] DomainError),
}

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn backup_path_for(path: &Path) -> PathBuf {
    let mut s = path.as_os_str().to_owned();
    s.push(".bak");
    PathBuf::from(s)
}

fn unique_temp_path_for(path: &Path) -> PathBuf {
    let n = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut s = path.as_os_str().to_owned();
    s.push(format!(".tmp{}-{n}", std::process::id()));
    PathBuf::from(s)
}

/// Save `document` to `path`, atomically with respect to crashes: readers
/// of `path` (including a concurrent `load_document`) only ever see the
/// fully-old or fully-new content, never a partial write.
pub fn save_document_atomically(path: &Path, document: &Document) -> Result<(), PersistError> {
    let json = craftloop_serialization::to_canonical_json(document)?;

    let tmp_path = unique_temp_path_for(path);
    {
        let mut file = fs::File::create(&tmp_path)?;
        std::io::Write::write_all(&mut file, json.as_bytes())?;
        file.sync_all()?;
    }

    // Best-effort backup of the previous good file. A failure here must
    // not block saving the new (valid) content -- an outdated or missing
    // backup is a smaller problem than refusing to persist real work.
    if path.exists() {
        let _ = fs::copy(path, backup_path_for(path));
    }

    fs::rename(&tmp_path, path)?;
    Ok(())
}

fn load_and_validate(path: &Path) -> Result<Document, PersistError> {
    let contents = fs::read_to_string(path)?;
    let document: Document = serde_json::from_str(&contents)?;
    document.validate()?;
    Ok(document)
}

/// Load a document from `path`, falling back to its `.bak` snapshot if the
/// primary file is missing, malformed, or fails integrity validation.
/// Returns the primary file's error if *both* fail, since that is the more
/// actionable diagnostic (the backup is a fallback, not the source of
/// truth).
pub fn load_document(path: &Path) -> Result<Document, PersistError> {
    match load_and_validate(path) {
        Ok(document) => Ok(document),
        Err(primary_err) => load_and_validate(&backup_path_for(path)).or(Err(primary_err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_doc_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "craftloop-doc-test-{}-{name}.json",
            std::process::id()
        ))
    }

    fn cleanup(path: &Path) {
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(backup_path_for(path));
    }

    #[test]
    fn save_then_load_round_trips_an_equal_document() {
        let path = temp_doc_path("roundtrip");
        cleanup(&path);
        let document = Document::new("Test Notebook", 42.0);

        save_document_atomically(&path, &document).unwrap();
        let loaded = load_document(&path).unwrap();
        assert_eq!(document, loaded);

        cleanup(&path);
    }

    #[test]
    fn a_second_save_creates_a_backup_of_the_first() {
        let path = temp_doc_path("backup");
        cleanup(&path);

        let first = Document::new("First", 1.0);
        save_document_atomically(&path, &first).unwrap();
        assert!(
            !backup_path_for(&path).exists(),
            "no prior file existed yet, so no backup should be made"
        );

        let second = Document::new("Second", 2.0);
        save_document_atomically(&path, &second).unwrap();
        assert!(backup_path_for(&path).exists());

        let backup_contents = load_and_validate(&backup_path_for(&path)).unwrap();
        assert_eq!(backup_contents, first);

        cleanup(&path);
    }

    #[test]
    fn a_corrupted_primary_file_recovers_from_backup() {
        let path = temp_doc_path("recover");
        cleanup(&path);

        let good = Document::new("Good", 1.0);
        save_document_atomically(&path, &good).unwrap();
        // A second save makes `good` the backup.
        save_document_atomically(&path, &Document::new("Second", 2.0)).unwrap();

        // Corrupt the primary file directly (simulating disk corruption or
        // a torn write this crate's own atomic save is designed to avoid,
        // but that some other actor -- a bug, a bad edit -- could still
        // cause).
        fs::write(&path, b"{ not valid json at all").unwrap();

        let recovered = load_document(&path).unwrap();
        assert_eq!(recovered, good);

        cleanup(&path);
    }

    #[test]
    fn both_primary_and_backup_corrupted_is_a_real_error_not_a_silent_default() {
        let path = temp_doc_path("both-corrupt");
        cleanup(&path);

        fs::write(&path, b"not json").unwrap();
        fs::write(backup_path_for(&path), b"also not json").unwrap();

        let result = load_document(&path);
        assert!(result.is_err());

        cleanup(&path);
    }

    #[test]
    fn missing_file_is_a_clean_io_error() {
        let path = temp_doc_path("missing");
        cleanup(&path);
        let result = load_document(&path);
        assert!(matches!(result, Err(PersistError::Io(_))));
    }
}
