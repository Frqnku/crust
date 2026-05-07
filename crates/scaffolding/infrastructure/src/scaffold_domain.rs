use std::{fs, path::Path};

use scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
};
use shared_infrastructure::{fs_helper::io_conflict, FileSystemTransaction};

use crate::fs_scaffold_helper::map_artifact_fs_error;
use crate::templates::domain::{DOMAIN_ENTITY_CONTENT, DOMAIN_FEATURE_MOD_CONTENT};

pub fn scaffold_domain_feature(
	project_root: &Path,
	domain_feature: Artifact,
) -> Result<(), ScaffoldingError> {
	let domain_src_directory = project_root
        .join("crates")
		.join(domain_feature.relative_directory());

	if !domain_src_directory.is_dir() {
		return Err(ScaffoldingError::BoundedContextNotFound {
			name: domain_feature
				.bounded_context()
				.name()
				.as_str()
				.to_string(),
		});
	}

	let domain_feature_directory_path = domain_src_directory.join(domain_feature.name().as_str());
	if domain_feature_directory_path.exists() {
		return Err(ScaffoldingError::ArtifactAlreadyExists {
			name: domain_feature.name().as_str().to_string(),
			bounded_context: domain_feature.bounded_context().name().as_str().to_string(),
			kind: domain_feature.kind().display().to_string(),
		});
	}

	let mod_file_path = domain_feature_directory_path.join("mod.rs");
	let entity_file_path = domain_feature_directory_path.join("entity.rs");
	let entity_content = DOMAIN_ENTITY_CONTENT.replace("{entity_name}", &domain_feature.name().to_pascal_case());

	let domain_lib_path = domain_src_directory.join("lib.rs");
	let mut domain_lib_content = if domain_lib_path.exists() {
		fs::read_to_string(&domain_lib_path)
			.map_err(|error| map_artifact_fs_error(io_conflict(&domain_lib_path, "read module file", error), &domain_feature))?
	} else {
		String::new()
	};
	let module_declaration = format!("pub mod {};", domain_feature.name().as_str());
	if !domain_lib_content.lines().any(|line| line.trim() == module_declaration) {
		if !domain_lib_content.is_empty() && !domain_lib_content.ends_with('\n') {
			domain_lib_content.push('\n');
		}
		domain_lib_content.push_str(&module_declaration);
		domain_lib_content.push('\n');
	}

	let mut tx = FileSystemTransaction::new();
	tx.create_dir(&domain_feature_directory_path);
	tx.create_file(&mod_file_path, DOMAIN_FEATURE_MOD_CONTENT);
	tx.create_file(&entity_file_path, entity_content);
	tx.modify_file(&domain_lib_path, domain_lib_content)
		.map_err(|error| map_artifact_fs_error(error, &domain_feature))?;
	tx.commit().map_err(|error| map_artifact_fs_error(error, &domain_feature))?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::TempDir;
	use std::fs;

	#[test]
	fn scaffold_creates_feature_and_updates_lib() {
		let tmp = TempDir::new().unwrap();
		let project_root = tmp.path();
		let bc = "test".to_string();
		let bc_path = project_root.join("crates").join(&bc).join("domain").join("src");
		fs::create_dir_all(&bc_path).unwrap();
		fs::write(bc_path.join("lib.rs"), "").unwrap();

		let artifact = scaffolding_domain::artifact_entity::Artifact::new(
			shared_domain::bounded_context_entity::BoundedContext::new(bc.clone()).unwrap(),
			scaffolding_domain::value_object::ArtifactKind::Domain,
			"auth".to_string(),
		).unwrap();

		let res = scaffold_domain_feature(project_root, artifact);
		assert!(res.is_ok());

		let domain_feature_dir = project_root.join("crates").join(&bc).join("domain").join("src").join("auth");
		assert!(domain_feature_dir.is_dir());
		assert!(domain_feature_dir.join("mod.rs").is_file());
		assert!(domain_feature_dir.join("entity.rs").is_file());
		let lib = project_root.join("crates").join(&bc).join("domain").join("src").join("lib.rs");
		let content = fs::read_to_string(lib).unwrap();
		assert!(content.contains("pub mod auth;"));
	}

	#[test]
	fn missing_bounded_context_directory_returns_error() {
		let tmp = TempDir::new().unwrap();
		let project_root = tmp.path();
		let artifact = scaffolding_domain::artifact_entity::Artifact::new(
			shared_domain::bounded_context_entity::BoundedContext::new("test".to_string()).unwrap(),
			scaffolding_domain::value_object::ArtifactKind::Domain,
			"auth".to_string(),
		).unwrap();

		let res = scaffold_domain_feature(project_root, artifact);
		assert!(matches!(res, Err(ScaffoldingError::BoundedContextNotFound { .. })));
	}
}
