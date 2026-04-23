use std::{fs, path::Path};

use scaffolding_domain::{artifact_entity::Artifact, errors::ScaffoldingError};
use shared_infrastructure::fs_helper::{FsHelperError, io_conflict, map_fs_error};

pub fn upsert_mod_declaration(
	mod_file_path: &Path,
	module_name: &str,
	declaration_prefix: &str,
) -> Result<(), FsHelperError> {
	let module_declaration = format!("{declaration_prefix} {module_name};");

	let mut content = if mod_file_path.exists() {
		fs::read_to_string(mod_file_path)
			.map_err(|error| io_conflict(mod_file_path, "read module file", error))?
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

	fs::write(mod_file_path, content).map_err(|error| io_conflict(mod_file_path, "write module file", error))
}

pub fn rollback_created_file(path: &Path) -> Result<(), std::io::Error> {
	fs::remove_file(path)
}

pub fn rollback_created_directory(path: &Path) -> Result<(), std::io::Error> {
	fs::remove_dir_all(path)
}

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
