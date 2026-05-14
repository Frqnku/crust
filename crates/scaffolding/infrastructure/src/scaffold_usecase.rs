use std::{fs, path::Path};

use crust_scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
};
use crust_shared_infrastructure::{fs_helper::io_conflict, FileSystemTransaction};
use crate::templates::usecase::USECASE_CONTENT;
use crate::fs_scaffold_helper::map_artifact_fs_error;

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

	let mod_file_path = usecase_directory.join("mod.rs");
	let mut mod_content = if mod_file_path.exists() {
		fs::read_to_string(&mod_file_path)
			.map_err(|error| map_artifact_fs_error(io_conflict(&mod_file_path, "read module file", error), &usecase))?
	} else {
		String::new()
	};

	let module_declaration = format!("pub mod {};", usecase.name().as_str());
	if !mod_content.lines().any(|line| line.trim() == module_declaration) {
		if !mod_content.is_empty() && !mod_content.ends_with('\n') {
			mod_content.push('\n');
		}
		mod_content.push_str(&module_declaration);
		mod_content.push('\n');
	}

	let usecase_content = USECASE_CONTENT.replace("{usecase_name}", &usecase.name().to_pascal_case());
	let mut tx = FileSystemTransaction::new();
	tx.create_file(&usecase_path, usecase_content);
	tx.modify_file(&mod_file_path, mod_content)
		.map_err(|error| map_artifact_fs_error(error, &usecase))?;
	tx.commit().map_err(|error| map_artifact_fs_error(error, &usecase))?;

	Ok(())
}
