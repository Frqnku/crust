use std::path::PathBuf;
use shared_domain::errors::SharedError;

#[derive(Debug, PartialEq)]
pub enum WorkspaceError {
    ProjectStructureConflict { path: PathBuf, reason: String },
    InvalidProjectLayout { path: PathBuf, reason: String },
    InvalidProjectName { value: String, reason: String },
    InvalidBoundedContextName { value: String, reason: String },
    BoundedContextAlreadyExists { path: PathBuf },
}

impl From<SharedError> for WorkspaceError {
    fn from(error: SharedError) -> Self {
        match error {
            SharedError::InvalidBoundedContextName { value, reason } => {
                WorkspaceError::InvalidBoundedContextName { value, reason }
            }
        }
    }
}