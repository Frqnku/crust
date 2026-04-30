use std::path::Path;
use crate::transaction::{FileOperation, OperationJournal};
use crate::fs_helper::FsHelperError;

/// High-level transaction manager for safe file system operations
pub struct FileSystemTransaction {
    journal: OperationJournal,
}

impl FileSystemTransaction {
    pub fn new() -> Self {
        Self {
            journal: OperationJournal::new(),
        }
    }

    /// Record a file creation operation
    pub fn create_file(&mut self, path: impl AsRef<Path>, content: impl Into<String>) {
        self.journal.record(FileOperation::CreateFile {
            path: path.as_ref().to_path_buf(),
            content: content.into(),
        });
    }

    /// Record a directory creation operation
    pub fn create_dir(&mut self, path: impl AsRef<Path>) {
        self.journal.record(FileOperation::CreateDir {
            path: path.as_ref().to_path_buf(),
        });
    }

    /// Record a file modification operation (with automatic backup)
    pub fn modify_file(
        &mut self,
        path: impl AsRef<Path>,
        new_content: impl Into<String>,
    ) -> Result<(), FsHelperError> {
        let path = path.as_ref();
        let original_content = std::fs::read_to_string(path).map_err(|e| {
            FsHelperError::Conflict {
                path: path.to_path_buf(),
                reason: format!("Failed to read file for modification: {}", e),
            }
        })?;

        self.journal.record(FileOperation::ModifyFile {
            path: path.to_path_buf(),
            original_content,
            new_content: new_content.into(),
        });

        Ok(())
    }

    /// Execute all recorded operations atomically
    /// On failure, automatically rolls back all changes
    pub fn commit(self) -> Result<(), FsHelperError> {
        self.journal.execute_all()
    }

    /// Get the number of operations in this transaction
    pub fn operation_count(&self) -> usize {
        self.journal.len()
    }

    /// Check if transaction is empty
    pub fn is_empty(&self) -> bool {
        self.journal.is_empty()
    }
}

impl Default for FileSystemTransaction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_transaction_builder() {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();

        let mut tx = FileSystemTransaction::new();
        tx.create_dir(base.join("dir1"));
        tx.create_file(base.join("dir1/file.txt"), "content");

        assert_eq!(tx.operation_count(), 2);
        assert!(!tx.is_empty());
    }

    #[test]
    fn test_transaction_commit_success() {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();
        let file_path = base.join("test.txt");

        let mut tx = FileSystemTransaction::new();
        tx.create_file(&file_path, "hello");

        tx.commit().unwrap();
        assert!(file_path.exists());
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "hello");
    }

    #[test]
    fn test_transaction_rollback_on_invalid_path() {
        let temp_dir = TempDir::new().unwrap();
        let valid_file = temp_dir.path().join("valid.txt");
        let invalid_file = PathBuf::from("/invalid/path/file.txt");

        let mut tx = FileSystemTransaction::new();
        tx.create_file(&valid_file, "data");
        tx.create_file(&invalid_file, "data"); // This will fail

        let result = tx.commit();
        assert!(result.is_err());
        assert!(!valid_file.exists(), "valid file should be rolled back");
    }
}
