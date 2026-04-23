use std::path::Path;

use scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
};
use shared_infrastructure::fs_helper::{create_dir, create_file, FsHelperError};

use crate::fs_scaffold_helper::{rollback_created_directory, upsert_mod_declaration};

fn map_fs_error(error: FsHelperError, infrastructure_tech: &Artifact) -> ScaffoldingError {
	match error {
		FsHelperError::Conflict { path, reason } => ScaffoldingError::ArtifactIoConflict {
			name: infrastructure_tech.name().as_str().to_string(),
			bounded_context: infrastructure_tech.bounded_context().name().as_str().to_string(),
			path,
			reason,
		},
		FsHelperError::InvalidLayout { path, reason } => {
			ScaffoldingError::ArtifactInvalidLayout {
				name: infrastructure_tech.name().as_str().to_string(),
				bounded_context: infrastructure_tech.bounded_context().name().as_str().to_string(),
				path,
				reason,
			}
		}
	}
}

fn rollback_infrastructure_tech(tech_directory_path: &Path, infrastructure_tech: &Artifact) -> Result<(), ScaffoldingError> {
	rollback_created_directory(tech_directory_path)
		.map_err(|error| ScaffoldingError::ArtifactRollbackFailed {
			name: infrastructure_tech.name().as_str().to_string(),
			bounded_context: infrastructure_tech.bounded_context().name().as_str().to_string(),
			reason: format!("failed to remove '{}': {error}", tech_directory_path.display()),
		})
}

pub fn scaffold_infrastructure_tech(
	project_root: &Path,
	infrastructure_tech: Artifact,
) -> Result<(), ScaffoldingError> {
	let infrastructure_src_directory = project_root
        .join("crates")
		.join(infrastructure_tech.relative_directory());

	if !infrastructure_src_directory.is_dir() {
		return Err(ScaffoldingError::BoundedContextNotFound {
			name: infrastructure_tech
				.bounded_context()
				.name()
				.as_str()
				.to_string(),
		});
	}

	let tech_directory_path = infrastructure_src_directory.join(infrastructure_tech.name().as_str());
	if tech_directory_path.exists() {
		return Err(ScaffoldingError::ArtifactAlreadyExists {
			name: infrastructure_tech.name().as_str().to_string(),
			bounded_context: infrastructure_tech.bounded_context().name().as_str().to_string(),
            kind: infrastructure_tech.kind().display().to_string(),
		});
	}

	create_dir(&tech_directory_path).map_err(|error| map_fs_error(error, &infrastructure_tech))?;

	let mod_file_path = tech_directory_path.join("mod.rs");
	create_file(&mod_file_path, "")
		.map_err(|error| map_fs_error(error, &infrastructure_tech))?;

	let infrastructure_lib_path = infrastructure_src_directory.join("lib.rs");
	if let Err(error) = upsert_mod_declaration(&infrastructure_lib_path, infrastructure_tech.name().as_str(), "pub mod")
		.map_err(|error| map_fs_error(error, &infrastructure_tech))
	{
		rollback_infrastructure_tech(&tech_directory_path, &infrastructure_tech)?;
		return Err(error);
	}

	Ok(())
}
