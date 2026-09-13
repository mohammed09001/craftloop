//! Autosave journal.
//!
//! Execution 01, Phase 07, Task 054. Authority: MCP Article 82 "Autosave".
//!
//! "Persist committed transactions without blocking the live input path."
//! The core of that requirement is architectural: append, don't rewrite.
//! Appending one small record to the end of a file is a cheap, bounded
//! operation regardless of document size, unlike re-serializing and
//! rewriting the whole document (`persistence::save_document_atomically`)
//! on every keystroke/stroke. This module provides that fast append path;
//! actually scheduling it off the input thread is a platform-adapter
//! concern (Phase 04's harness is single-threaded; real off-thread
//! scheduling is Android/iPad adapter work, Phases 28-29) that this
//! platform-independent crate does not fabricate a threading model for.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum AutosaveError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// An append-only log of serialized transaction records. Each call to
/// [`AutosaveJournal::append`] is one `write` + one line separator; no
/// prior journal content is read, rewritten, or re-serialized.
pub struct AutosaveJournal {
    path: PathBuf,
}

impl AutosaveJournal {
    pub fn open(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Append one already-serialized record (e.g. a transaction's
    /// canonical JSON) as a new line. Newline-delimited JSON so a reader
    /// can recover every record written so far even if the very last write
    /// was interrupted mid-line (everything before the last newline is
    /// still valid).
    pub fn append(&self, record_json: &str) -> Result<(), AutosaveError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        file.write_all(record_json.as_bytes())?;
        file.write_all(b"\n")?;
        Ok(())
    }

    /// Read back every complete (newline-terminated) record. A dangling
    /// final line with no trailing newline (evidence of an interrupted
    /// write) is skipped rather than returned as if it were a complete
    /// record.
    pub fn read_all(&self) -> Result<Vec<String>, AutosaveError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let contents = std::fs::read_to_string(&self.path)?;
        let complete = if contents.ends_with('\n') {
            &contents[..]
        } else {
            match contents.rfind('\n') {
                Some(idx) => &contents[..=idx],
                None => "",
            }
        };
        Ok(complete.lines().map(|s| s.to_string()).collect())
    }

    pub fn clear(&self) -> Result<(), AutosaveError> {
        if self.path.exists() {
            std::fs::remove_file(&self.path)?;
        }
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_journal_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "craftloop-autosave-test-{}-{name}.ndjson",
            std::process::id()
        ))
    }

    #[test]
    fn append_then_read_all_returns_records_in_order() {
        let path = temp_journal_path("order");
        let _ = std::fs::remove_file(&path);
        let journal = AutosaveJournal::open(&path);

        journal.append(r#"{"n":1}"#).unwrap();
        journal.append(r#"{"n":2}"#).unwrap();
        journal.append(r#"{"n":3}"#).unwrap();

        let records = journal.read_all().unwrap();
        assert_eq!(records, vec![r#"{"n":1}"#, r#"{"n":2}"#, r#"{"n":3}"#]);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn reading_a_nonexistent_journal_returns_an_empty_list_not_an_error() {
        let path = temp_journal_path("missing");
        let _ = std::fs::remove_file(&path);
        let journal = AutosaveJournal::open(&path);
        assert_eq!(journal.read_all().unwrap(), Vec::<String>::new());
    }

    #[test]
    fn a_dangling_incomplete_final_line_is_not_returned_as_a_record() {
        let path = temp_journal_path("dangling");
        let _ = std::fs::remove_file(&path);
        let journal = AutosaveJournal::open(&path);
        journal.append(r#"{"n":1}"#).unwrap();
        // Simulate an interrupted write: append bytes with no trailing
        // newline, as if the process died mid-write of the second record.
        {
            let mut file = OpenOptions::new().append(true).open(&path).unwrap();
            file.write_all(b"{\"n\":2, incomple").unwrap();
        }

        let records = journal.read_all().unwrap();
        assert_eq!(records, vec![r#"{"n":1}"#]);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn clear_removes_the_journal_file() {
        let path = temp_journal_path("clear");
        let journal = AutosaveJournal::open(&path);
        journal.append("x").unwrap();
        assert!(path.exists());
        journal.clear().unwrap();
        assert!(!path.exists());
        // Clearing an already-absent journal is not an error.
        assert!(journal.clear().is_ok());
    }
}
