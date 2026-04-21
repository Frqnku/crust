use std::fmt::{Display, Formatter};
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

impl Display for WorkspaceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProjectStructureConflict { path, reason } => {
                write!(formatter, "Project structure conflict at '{}': {reason}", path.display())
            }
            Self::InvalidProjectLayout { path, reason } => {
                write!(formatter, "Invalid project layout at '{}': {reason}", path.display())
            }
            Self::InvalidProjectName { value, reason } => {
                write!(formatter, "Invalid project name '{value}': {reason}")
            }
            Self::InvalidBoundedContextName { value, reason } => {
                write!(formatter, "Invalid bounded context name '{value}': {reason}")
            }
            Self::BoundedContextAlreadyExists { path } => {
                write!(formatter, "Bounded context already exists at '{}'", path.display())
            }
        }
    }
}

impl std::error::Error for WorkspaceError {}