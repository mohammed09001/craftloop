//! Document metadata.
//!
//! Execution 01, Phase 07, Task 048.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: String,
    /// Seconds since an adapter-defined epoch (not necessarily wall-clock
    /// `SystemTime`), matching the same "caller supplies real time, this
    /// crate stays deterministic" convention `craftloop-input` established
    /// for `PointerSample::timestamp_seconds`.
    pub created_at_seconds: f64,
    pub modified_at_seconds: f64,
}

impl DocumentMetadata {
    pub fn new(title: impl Into<String>, now_seconds: f64) -> Self {
        Self {
            title: title.into(),
            created_at_seconds: now_seconds,
            modified_at_seconds: now_seconds,
        }
    }

    pub fn touch(&mut self, now_seconds: f64) {
        self.modified_at_seconds = now_seconds;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_created_and_modified_to_the_same_timestamp() {
        let meta = DocumentMetadata::new("Untitled", 100.0);
        assert_eq!(meta.created_at_seconds, 100.0);
        assert_eq!(meta.modified_at_seconds, 100.0);
    }

    #[test]
    fn touch_updates_modified_but_not_created() {
        let mut meta = DocumentMetadata::new("Untitled", 100.0);
        meta.touch(150.0);
        assert_eq!(meta.created_at_seconds, 100.0);
        assert_eq!(meta.modified_at_seconds, 150.0);
    }
}
