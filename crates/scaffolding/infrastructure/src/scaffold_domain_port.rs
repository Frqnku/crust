use std::{fs, path::Path};

use scaffolding_domain::{
	domain_port_entity::DomainPort,
	errors::ScaffoldingError,
};
use shared_infrastructure::{fs_helper::{io_conflict, FsHelperError}, FileSystemTransaction};
use crate::templates::domain_port::DOMAIN_PORT_CONTENT;

fn map_domain_port_fs_error(error: FsHelperError, domain_port: &DomainPort) -> ScaffoldingError {
	match error {
		FsHelperError::Conflict { path, reason } => ScaffoldingError::ArtifactIoConflict {
			name: domain_port.port_name().as_str().to_string(),
			bounded_context: domain_port.bounded_context().name().as_str().to_string(),
			path,
			reason,
		},
		FsHelperError::InvalidLayout { path, reason } => ScaffoldingError::ArtifactInvalidLayout {
			name: domain_port.port_name().as_str().to_string(),
			bounded_context: domain_port.bounded_context().name().as_str().to_string(),
			path,
			reason,
		},
	}
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
		return Err(ScaffoldingError::ArtifactAlreadyExistsInFeature {
			name: domain_port.port_name().as_str().to_string(),
			feature: domain_port.feature_name().as_str().to_string(),
			bounded_context: domain_port.bounded_context().name().as_str().to_string(),
			kind: "Domain port".to_string(),
		});
	}

	let mod_file_path = domain_port_directory.join("mod.rs");
	let mut mod_content = if mod_file_path.exists() {
		fs::read_to_string(&mod_file_path)
			.map_err(|error| map_domain_port_fs_error(io_conflict(&mod_file_path, "read module file", error), &domain_port))?
	} else {
		String::new()
	};
	let module_declaration = format!("pub mod {};", domain_port.port_name().as_str());
	if !mod_content.lines().any(|line| line.trim() == module_declaration) {
		if !mod_content.is_empty() && !mod_content.ends_with('\n') {
			mod_content.push('\n');
		}
		mod_content.push_str(&module_declaration);
		mod_content.push('\n');
	}

	let domain_port_content = DOMAIN_PORT_CONTENT.replace(
		"{domain_port_name}",
		&format!("{}{}",
			domain_port.feature_name().to_pascal_case(),
			domain_port.port_name().to_pascal_case()
		),
	);
	let mut tx = FileSystemTransaction::new();
	tx.create_file(&domain_port_path, &domain_port_content);
	tx.modify_file(&mod_file_path, mod_content)
		.map_err(|error| map_domain_port_fs_error(error, &domain_port))?;
	tx.commit().map_err(|error| map_domain_port_fs_error(error, &domain_port))?;

	Ok(())
}
