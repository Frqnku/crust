use std::path::Path;

use scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
};
use shared_infrastructure::fs_helper::{create_dir, create_file};

use crate::{fs_scaffold_helper::{map_artifact_fs_error, rollback_created_directory, upsert_mod_declaration}, templates::domain::{DOMAIN_ENTITY_CONTENT, DOMAIN_FEATURE_MOD_CONTENT}};

fn rollback_domain_feature(feature_directory_path: &Path, domain_feature: &Artifact) -> Result<(), ScaffoldingError> {
	rollback_created_directory(feature_directory_path)
		.map_err(|error| ScaffoldingError::ArtifactRollbackFailed {
			name: domain_feature.name().as_str().to_string(),
			bounded_context: domain_feature.bounded_context().name().as_str().to_string(),
			reason: format!("failed to remove '{}': {error}", feature_directory_path.display()),
		})
}

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

	let feature_directory_path = domain_src_directory.join(domain_feature.name().as_str());
	if feature_directory_path.exists() {
		return Err(ScaffoldingError::ArtifactAlreadyExists {
			name: domain_feature.name().as_str().to_string(),
			bounded_context: domain_feature.bounded_context().name().as_str().to_string(),
            kind: domain_feature.kind().display().to_string(),
		});
	}

	create_dir(&feature_directory_path).map_err(|error| map_artifact_fs_error(error, &domain_feature))?;

	let mod_file_path = feature_directory_path.join("mod.rs");
	create_file(&mod_file_path, DOMAIN_FEATURE_MOD_CONTENT)
		.map_err(|error| map_artifact_fs_error(error, &domain_feature))?;

	let entity_file_path = feature_directory_path.join("entity.rs");
    let entity_content = DOMAIN_ENTITY_CONTENT.replace("{entity_name}", &domain_feature.name().to_pascal_case());
	create_file(&entity_file_path, &entity_content)
		.map_err(|error| map_artifact_fs_error(error, &domain_feature))?;

	let domain_lib_path = domain_src_directory.join("lib.rs");
	if let Err(error) = upsert_mod_declaration(&domain_lib_path, domain_feature.name().as_str(), "pub mod")
		.map_err(|error| map_artifact_fs_error(error, &domain_feature))
	{
		rollback_domain_feature(&feature_directory_path, &domain_feature)?;
		return Err(error);
	}

	Ok(())
}
