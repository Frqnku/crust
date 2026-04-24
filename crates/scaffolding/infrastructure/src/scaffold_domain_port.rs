use std::path::Path;

use scaffolding_domain::{
	domain_port_entity::DomainPort,
	errors::ScaffoldingError,
};
use shared_infrastructure::fs_helper::{
	create_file,
	map_fs_error,
	FsHelperError,
};
use crate::templates::domain_port::DOMAIN_PORT_CONTENT;
use crate::fs_scaffold_helper::{rollback_created_file, upsert_mod_declaration};

fn map_domain_port_fs_error(error: FsHelperError, domain_port: &DomainPort) -> ScaffoldingError {
	map_fs_error(
		error,
		|path, reason| ScaffoldingError::ArtifactIoConflict {
			name: domain_port.port_name().as_str().to_string(),
			bounded_context: domain_port.bounded_context().name().as_str().to_string(),
			path,
			reason,
		},
		|path, reason| ScaffoldingError::ArtifactInvalidLayout {
			name: domain_port.port_name().as_str().to_string(),
			bounded_context: domain_port.bounded_context().name().as_str().to_string(),
			path,
			reason,
		},
	)
}

fn rollback_domain_port_file(domain_port_path: &Path, domain_port: &DomainPort) -> Result<(), ScaffoldingError> {
	rollback_created_file(domain_port_path).map_err(|error| ScaffoldingError::ArtifactRollbackFailed {
		name: domain_port.port_name().as_str().to_string(),
		bounded_context: domain_port.bounded_context().name().as_str().to_string(),
		reason: format!("failed to remove '{}': {error}", domain_port_path.display()),
	})
}

pub fn scaffold_domain_port(project_root: &Path, domain_port: DomainPort) -> Result<(), ScaffoldingError> {
	let domain_port_directory = domain_port.feature_directory(project_root);

	if !domain_port_directory.is_dir() {
		return Err(ScaffoldingError::BoundedContextNotFound {
			name: format!(
				"{}/domain/{}",
				domain_port.bounded_context().name().as_str(),
				domain_port.feature_name().as_str()
			),
		});
	}

	let domain_port_path = domain_port_directory.join(domain_port.as_file_name());
	if domain_port_path.exists() {
		return Err(ScaffoldingError::ArtifactAlreadyExists {
			name: domain_port.port_name().as_str().to_string(),
			bounded_context: domain_port.bounded_context().name().as_str().to_string(),
			kind: "Domain port".to_string(),
		});
	}

	let domain_port_content = DOMAIN_PORT_CONTENT.replace(
		"{domain_port_name}",
		&format!("{}{}",
			domain_port.feature_name().to_pascal_case(),
			domain_port.port_name().to_pascal_case()
		),
	);
	create_file(&domain_port_path, &domain_port_content)
		.map_err(|error| map_domain_port_fs_error(error, &domain_port))?;

	let mod_file_path = domain_port_directory.join("mod.rs");
	if let Err(error) = upsert_mod_declaration(&mod_file_path, domain_port.port_name().as_str(), "pub mod")
		.map_err(|error| map_domain_port_fs_error(error, &domain_port))
	{
		rollback_domain_port_file(&domain_port_path, &domain_port)?;
		return Err(error);
	}

	Ok(())
}
