use std::path::Path;

use scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
};
use shared_infrastructure::fs_helper::{
	create_file,
	map_fs_error,
};
use crate::templates::usecase::USECASE_CONTENT;
use crate::fs_scaffold_helper::{rollback_created_file, upsert_mod_declaration};

fn rollback_usecase_file(usecase_path: &Path, usecase: &Artifact) -> Result<(), ScaffoldingError> {
	rollback_created_file(usecase_path).map_err(|error| ScaffoldingError::ArtifactRollbackFailed {
		name: usecase.name().as_str().to_string(),
		bounded_context: usecase.bounded_context().name().as_str().to_string(),
		reason: format!("failed to remove '{}': {error}", usecase_path.display()),
	})
}

pub fn scaffold_usecase(project_root: &Path, usecase: Artifact) -> Result<(), ScaffoldingError> {
	let usecase_directory = project_root.join("crates").join(usecase.relative_directory());

	if !usecase_directory.is_dir() {
		return Err(ScaffoldingError::BoundedContextNotFound {
			name: usecase.bounded_context().name().as_str().to_string(),
		});
	}

	let usecase_path = usecase_directory.join(usecase.as_file_name());
	if usecase_path.exists() {
		return Err(ScaffoldingError::ArtifactAlreadyExists {
			name: usecase.name().as_str().to_string(),
			bounded_context: usecase.bounded_context().name().as_str().to_string(),
			kind: usecase.kind().display().to_string(),
		});
	}

	let usecase_content = USECASE_CONTENT.replace("{usecase_name}", &usecase.name().to_pascal_case());
	create_file(&usecase_path, &usecase_content).map_err(|error| {
		map_fs_error(
			error,
			|path, reason| ScaffoldingError::ArtifactIoConflict {
				name: usecase.name().as_str().to_string(),
				bounded_context: usecase.bounded_context().name().as_str().to_string(),
				path,
				reason,
			},
			|path, reason| ScaffoldingError::ArtifactInvalidLayout {
				name: usecase.name().as_str().to_string(),
				bounded_context: usecase.bounded_context().name().as_str().to_string(),
				path,
				reason,
			},
		)
	})?;

	let mod_file_path = usecase_directory.join("mod.rs");
	if let Err(error) = upsert_mod_declaration(&mod_file_path, usecase.name().as_str(), "pub mod")
		.map_err(|error| {
			map_fs_error(
				error,
				|path, reason| ScaffoldingError::ArtifactIoConflict {
					name: usecase.name().as_str().to_string(),
					bounded_context: usecase.bounded_context().name().as_str().to_string(),
					path,
					reason,
				},
				|path, reason| ScaffoldingError::ArtifactInvalidLayout {
					name: usecase.name().as_str().to_string(),
					bounded_context: usecase.bounded_context().name().as_str().to_string(),
					path,
					reason,
				},
			)
		})
	{
		rollback_usecase_file(&usecase_path, &usecase)?;
		return Err(error);
	}

	Ok(())
}
