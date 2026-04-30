use std::{fs, path::Path};

use scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
};
use shared_infrastructure::{fs_helper::io_conflict, FileSystemTransaction};

use crate::fs_scaffold_helper::map_artifact_fs_error;

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

	let mod_file_path = tech_directory_path.join("mod.rs");
	let infrastructure_lib_path = infrastructure_src_directory.join("lib.rs");
	let mut lib_content = if infrastructure_lib_path.exists() {
		fs::read_to_string(&infrastructure_lib_path)
			.map_err(|error| map_artifact_fs_error(io_conflict(&infrastructure_lib_path, "read module file", error), &infrastructure_tech))?
	} else {
		String::new()
	};
	let module_declaration = format!("pub mod {};", infrastructure_tech.name().as_str());
	if !lib_content.lines().any(|line| line.trim() == module_declaration) {
		if !lib_content.is_empty() && !lib_content.ends_with('\n') {
			lib_content.push('\n');
		}
		lib_content.push_str(&module_declaration);
		lib_content.push('\n');
	}

	let mut tx = FileSystemTransaction::new();
	tx.create_dir(&tech_directory_path);
	tx.create_file(&mod_file_path, "");
	tx.modify_file(&infrastructure_lib_path, lib_content)
		.map_err(|error| map_artifact_fs_error(error, &infrastructure_tech))?;
	tx.commit().map_err(|error| map_artifact_fs_error(error, &infrastructure_tech))?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::TempDir;
	use std::fs;

	#[test]
	fn scaffold_creates_tech_and_updates_lib() {
		let tmp = TempDir::new().unwrap();
		let project_root = tmp.path();
		let bc = "test".to_string();
		let bc_path = project_root.join("crates").join(&bc).join("infrastructure").join("src");
		fs::create_dir_all(&bc_path).unwrap();
		fs::write(bc_path.join("lib.rs"), "").unwrap();

		let artifact = scaffolding_domain::artifact_entity::Artifact::new(
			shared_domain::bounded_context_entity::BoundedContext::new(bc.clone()).unwrap(),
			scaffolding_domain::value_object::ArtifactKind::InfrastructureTech,
			"postgres".to_string(),
		).unwrap();

		let res = scaffold_infrastructure_tech(project_root, artifact);
		assert!(res.is_ok());

		let tech_dir = project_root.join("crates").join(&bc).join("infrastructure").join("src").join("postgres");
		assert!(tech_dir.is_dir());
		assert!(tech_dir.join("mod.rs").is_file());
		let lib = project_root.join("crates").join(&bc).join("infrastructure").join("src").join("lib.rs");
		let content = fs::read_to_string(lib).unwrap();
		assert!(content.contains("pub mod postgres;"));
	}

	#[test]
	fn missing_bounded_context_directory_returns_error() {
		let tmp = TempDir::new().unwrap();
		let project_root = tmp.path();
		let artifact = scaffolding_domain::artifact_entity::Artifact::new(
			shared_domain::bounded_context_entity::BoundedContext::new("test".to_string()).unwrap(),
			scaffolding_domain::value_object::ArtifactKind::InfrastructureTech,
			"postgres".to_string(),
		).unwrap();

		let res = scaffold_infrastructure_tech(project_root, artifact);
		assert!(matches!(res, Err(ScaffoldingError::BoundedContextNotFound { .. })));
	}
}
