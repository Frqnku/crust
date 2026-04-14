use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum WorkspaceError {
    ProjectStructureConflict { path: PathBuf, reason: String },
    InvalidProjectLayout { path: PathBuf, reason: String },
    InvalidProjectName { value: String, reason: String },
    InvalidBoundedContextName { value: String, reason: String },
    BoundedContextAlreadyExists { path: PathBuf },
}