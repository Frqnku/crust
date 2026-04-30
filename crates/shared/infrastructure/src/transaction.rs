use std::path::PathBuf;
use std::fs;
use crate::fs_helper::FsHelperError;

/// Represents a single file system operation that can be rolled back
#[derive(Debug, Clone)]
pub enum FileOperation {
    /// Create a new file with content
    CreateFile { path: PathBuf, content: String },
    /// Create a new directory
    CreateDir { path: PathBuf },
    /// Modify existing file (stores original content for rollback)
    ModifyFile { path: PathBuf, original_content: String, new_content: String },
    /// Delete a file (stores content for rollback)
    DeleteFile { path: PathBuf, original_content: String },
}

impl FileOperation {
    /// Execute the operation
    pub fn execute(&self) -> Result<(), FsHelperError> {
        match self {
            FileOperation::CreateFile { path, content } => {
                fs::write(path, content)
                    .map_err(|e| FsHelperError::Conflict {
                        path: path.clone(),
                        reason: format!("Failed to create file: {}", e),
                    })
            }
            FileOperation::CreateDir { path } => {
                fs::create_dir_all(path)
                    .map_err(|e| FsHelperError::Conflict {
                        path: path.clone(),
                        reason: format!("Failed to create directory: {}", e),
                    })
            }
            FileOperation::ModifyFile { path, new_content, .. } => {
                fs::write(path, new_content)
                    .map_err(|e| FsHelperError::Conflict {
                        path: path.clone(),
                        reason: format!("Failed to modify file: {}", e),
                    })
            }
            FileOperation::DeleteFile { path, .. } => {
                fs::remove_file(path)
                    .map_err(|e| FsHelperError::Conflict {
                        path: path.clone(),
                        reason: format!("Failed to delete file: {}", e),
                    })
            }
        }
    }

    /// Rollback the operation (inverse of execute)
    pub fn rollback(&self) -> Result<(), FsHelperError> {
        match self {
            FileOperation::CreateFile { path, .. } => {
                let _ = fs::remove_file(path);
                Ok(())
            }
            FileOperation::CreateDir { path } => {
                let _ = fs::remove_dir_all(path);
                Ok(())
            }
            FileOperation::ModifyFile { path, original_content, .. } => {
                fs::write(path, original_content)
                    .map_err(|e| FsHelperError::Conflict {
                        path: path.clone(),
                        reason: format!("Failed to rollback file modification: {}", e),
                    })
            }
            FileOperation::DeleteFile { path, original_content } => {
                fs::write(path, original_content)
                    .map_err(|e| FsHelperError::Conflict {
                        path: path.clone(),
                        reason: format!("Failed to restore deleted file: {}", e),
                    })
            }
        }
    }
}

/// Journal of all operations in a transaction
#[derive(Debug, Default)]
pub struct OperationJournal {
    operations: Vec<FileOperation>,
}

impl OperationJournal {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a new operation
    pub fn record(&mut self, operation: FileOperation) {
        self.operations.push(operation);
    }

    /// Execute all operations in order and rollback on failure
    pub fn execute_all(&self) -> Result<(), FsHelperError> {
        for (index, operation) in self.operations.iter().enumerate() {
            if let Err(error) = operation.execute() {
                // Rollback all previously executed operations in reverse order
                self.rollback_up_to(index);
                return Err(error);
            }
        }
        Ok(())
    }

    /// Rollback operations up to (but not including) the given index
    fn rollback_up_to(&self, up_to_index: usize) {
        for operation in self.operations[..up_to_index].iter().rev() {
            let _ = operation.rollback();
        }
    }

    /// Get the number of recorded operations
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Check if journal is empty
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_operation_execute_and_rollback() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let op = FileOperation::CreateFile {
            path: file_path.clone(),
            content: "test".to_string(),
        };

        // Execute
        op.execute().unwrap();
        assert!(file_path.exists());

        // Rollback
        op.rollback().unwrap();
        assert!(!file_path.exists());
    }

    #[test]
    fn test_journal_execute_all_success() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("newdir");
        let file_path = dir_path.join("test.txt");

        let mut journal = OperationJournal::new();
        journal.record(FileOperation::CreateDir {
            path: dir_path.clone(),
        });
        journal.record(FileOperation::CreateFile {
            path: file_path.clone(),
            content: "content".to_string(),
        });

        journal.execute_all().unwrap();
        assert!(file_path.exists());
    }

    #[test]
    fn test_journal_rollback_on_failure() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let invalid_dir = PathBuf::from("/invalid/path/that/does/not/exist");
        let file3 = invalid_dir.join("file3.txt");

        let mut journal = OperationJournal::new();
        journal.record(FileOperation::CreateFile {
            path: file1.clone(),
            content: "1".to_string(),
        });
        journal.record(FileOperation::CreateFile {
            path: file2.clone(),
            content: "2".to_string(),
        });
        // This will fail because parent doesn't exist
        journal.record(FileOperation::CreateFile {
            path: file3,
            content: "3".to_string(),
        });

        let result = journal.execute_all();
        assert!(result.is_err());

        // Verify rollback: files 1 and 2 should be removed
        assert!(!file1.exists(), "file1 should be rolled back");
        assert!(!file2.exists(), "file2 should be rolled back");
    }
}
