use crust_scaffolding_domain::{artifact_entity::Artifact, errors::ScaffoldingError};
use crust_shared_infrastructure::fs_helper::{FsHelperError, map_fs_error};

pub fn map_artifact_fs_error(error: FsHelperError, artifact: &Artifact) -> ScaffoldingError {
	map_fs_error(
		error,
		|path, reason| ScaffoldingError::ArtifactIoConflict {
			name: artifact.name().as_str().to_string(),
			bounded_context: artifact.bounded_context().name().as_str().to_string(),
			path,
			reason,
		},
		|path, reason| ScaffoldingError::ArtifactInvalidLayout {
			name: artifact.name().as_str().to_string(),
			bounded_context: artifact.bounded_context().name().as_str().to_string(),
			path,
			reason,
		},
	)
}
