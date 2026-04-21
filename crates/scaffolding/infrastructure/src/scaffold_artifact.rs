use std::{fs, path::Path};

use scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
};
use shared_infrastructure::fs_helper::{
	create_file,
	io_conflict,
	FsHelperError,
};
use crate::templates::usecase::USECASE_CONTENT;

fn map_fs_error(error: FsHelperError, artifact: &Artifact) -> ScaffoldingError {
	match error {
		FsHelperError::Conflict { path, reason } => ScaffoldingError::ArtifactIoConflict {
			name: artifact.name.as_str().to_string(),
			bounded_context: artifact.bounded_context.name.as_str().to_string(),
			path,
			reason,
		},
		FsHelperError::InvalidLayout { path, reason } => ScaffoldingError::ArtifactInvalidLayout {
			name: artifact.name.as_str().to_string(),
			bounded_context: artifact.bounded_context.name.as_str().to_string(),
			path,
			reason,
		},
	}
}

fn upsert_mod_file(mod_file_path: &Path, module_name: &str, artifact: &Artifact) -> Result<(), ScaffoldingError> {
	let module_declaration = format!("pub mod {module_name};");

	let mut content = if mod_file_path.exists() {
		fs::read_to_string(mod_file_path).map_err(|error| {
			map_fs_error(
				io_conflict(mod_file_path, "read module file", error),
				artifact,
			)
		})?
	} else {
		String::new()
	};

	if content.lines().any(|line| line.trim() == module_declaration) {
		return Ok(());
	}

	if !content.is_empty() && !content.ends_with('\n') {
		content.push('\n');
	}
	content.push_str(&module_declaration);
	content.push('\n');

	fs::write(mod_file_path, content).map_err(|error| {
		map_fs_error(
			io_conflict(mod_file_path, "write module file", error),
			artifact,
		)
	})
}

fn rollback_artifact_file(artifact_path: &Path, artifact: &Artifact) -> Result<(), ScaffoldingError> {
	fs::remove_file(artifact_path).map_err(|error| ScaffoldingError::ArtifactRollbackFailed {
		name: artifact.name.as_str().to_string(),
		bounded_context: artifact.bounded_context.name.as_str().to_string(),
		reason: format!("failed to remove '{}': {error}", artifact_path.display()),
	})
}

pub fn scaffold_artifact(project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError> {
	let artifact_directory = project_root.join("crates").join(artifact.relative_directory());

	if !artifact_directory.is_dir() {
		return Err(ScaffoldingError::BoundedContextNotFound {
			name: artifact.bounded_context.name.as_str().to_string(),
		});
	}

	let artifact_path = artifact_directory.join(artifact.as_file_name());
	if artifact_path.exists() {
		return Err(ScaffoldingError::ArtifactAlreadyExists {
			name: artifact.name.as_str().to_string(),
			bounded_context: artifact.bounded_context.name.as_str().to_string(),
		});
	}

	let usecase_content = USECASE_CONTENT.replace("{artifact_name}", &artifact.name.to_pascal_case());
	create_file(&artifact_path, &usecase_content).map_err(|error| map_fs_error(error, &artifact))?;

	let mod_file_path = artifact_directory.join("mod.rs");
	if let Err(error) = upsert_mod_file(&mod_file_path, artifact.name.as_str(), &artifact) {
		rollback_artifact_file(&artifact_path, &artifact)?;
		return Err(error);
	}

	Ok(())
}
