use shared_domain::errors::SharedError;

#[derive(Debug, PartialEq)]
pub enum ScaffoldingError {
    InvalidArtifactName { value: String, reason: String },
    InvalidBoundedContextName { value: String, reason: String },
    InvalidArtifactKind { value: String, reason: String },
    ArtifactAlreadyExists { name: String, bounded_context: String },
    BoundedContextNotFound { name: String },
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