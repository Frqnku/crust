use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use shared_domain::errors::SharedError;

#[derive(Debug, PartialEq)]
pub enum ScaffoldingError {
    InvalidArtifactName { value: String, reason: String },
    InvalidBoundedContextName { value: String, reason: String },
    ArtifactAlreadyExists { name: String, bounded_context: String, kind: String },
    ArtifactAlreadyExistsInDomain {
        name: String,
        domain: String,
        bounded_context: String,
        kind: String,
    },
    ArtifactNotFound { name: String, bounded_context: String, kind: String },
    BoundedContextNotFound { name: String },
    ArtifactIoConflict {
        name: String,
        bounded_context: String,
        path: PathBuf,
        reason: String,
    },
    ArtifactInvalidLayout {
        name: String,
        bounded_context: String,
        path: PathBuf,
        reason: String,
    },
    ArtifactRollbackFailed { name: String, bounded_context: String, reason: String },
}

impl From<SharedError> for ScaffoldingError {
    fn from(error: SharedError) -> Self {
        match error {
            SharedError::InvalidBoundedContextName { value, reason } => {
                ScaffoldingError::InvalidBoundedContextName { value, reason }
            }
        }
    }
}

impl Display for ScaffoldingError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArtifactName { value, reason } => {
                write!(formatter, "Invalid artifact name '{value}': {reason}")
            }
            Self::InvalidBoundedContextName { value, reason } => {
                write!(formatter, "Invalid bounded context name '{value}': {reason}")
            }
            Self::ArtifactAlreadyExists { name, bounded_context, kind } => {
                write!(formatter, "{kind} '{name}' already exists in bounded context '{bounded_context}'")
            }
            Self::ArtifactAlreadyExistsInDomain { name, domain, bounded_context, kind } => {
                write!(formatter, "{kind} '{name}' already exists in domain '{domain}' in bounded context '{bounded_context}'")
            }
            Self::ArtifactNotFound { name, bounded_context, kind } => {
                write!(formatter, "{kind} '{name}' was not found in bounded context '{bounded_context}'")
            }
            Self::BoundedContextNotFound { name } => {
                write!(formatter, "Bounded context '{name}' was not found")
            }
            Self::ArtifactIoConflict { name, bounded_context, path, reason } => {
                write!(
                    formatter,
                    "Artifact '{name}' in bounded context '{bounded_context}' failed at '{}': {reason}",
                    path.display()
                )
            }
            Self::ArtifactInvalidLayout { name, bounded_context, path, reason } => {
                write!(
                    formatter,
                    "Artifact '{name}' in bounded context '{bounded_context}' has invalid layout at '{}': {reason}",
                    path.display()
                )
            }
            Self::ArtifactRollbackFailed { name, bounded_context, reason } => {
                write!(formatter, "Failed to roll back artifact '{name}' in bounded context '{bounded_context}': {reason}")
            }
        }
    }
}

impl std::error::Error for ScaffoldingError {}