use std::{fs, path::Path};

use scaffolding_domain::{
	errors::ScaffoldingError,
	port_implementation_entity::PortImplementation,
};
use shared_infrastructure::{fs_helper::{io_conflict, map_fs_error, FsHelperError}, FileSystemTransaction};

use crate::templates::port_implementation::PORT_IMPLEMENTATION_CONTENT;

fn map_port_implementation_fs_error(error: FsHelperError, implementation: &PortImplementation) -> ScaffoldingError {
	map_fs_error(
		error,
		|path, reason| ScaffoldingError::ArtifactIoConflict {
			name: implementation.implementation_module_name(),
			bounded_context: implementation.bounded_context().name().as_str().to_string(),
			path,
			reason,
		},
		|path, reason| ScaffoldingError::ArtifactInvalidLayout {
			name: implementation.implementation_module_name(),
			bounded_context: implementation.bounded_context().name().as_str().to_string(),
			path,
			reason,
		},
	)
}

/// Validate that all preconditions are met for creating a port implementation
/// This is a side-effect-free check to prevent orphaned artifacts
pub fn validate_port_implementation_preconditions(
	project_root: &Path,
	implementation: &PortImplementation,
) -> Result<(), ScaffoldingError> {
	let port_file_path = implementation.port_file_path(project_root);
	if !port_file_path.is_file() {
		return Err(ScaffoldingError::BoundedContextNotFound {
			name: format!(
				"{}/domain/{}/{}.rs",
				implementation.bounded_context().name().as_str(),
				implementation.domain_feature_name().as_str(),
				implementation.port_name().as_str()
			),
		});
	}

	let implementation_directory = implementation.implementation_directory(project_root);
	if !implementation_directory.is_dir() {
		return Err(ScaffoldingError::ArtifactNotFound {
			name: implementation.tech_name().as_str().to_string(),
			bounded_context: implementation.bounded_context().name().as_str().to_string(),
			kind: "Tech".to_string(),
		});
	}

	let implementation_path = implementation.implementation_file_path(project_root);
	if implementation_path.exists() {
		return Err(ScaffoldingError::ArtifactAlreadyExists {
			name: implementation.implementation_module_name(),
			bounded_context: implementation.bounded_context().name().as_str().to_string(),
			kind: "Port implementation".to_string(),
		});
	}

	Ok(())
}

pub fn scaffold_port_implementation(
	project_root: &Path,
	implementation: PortImplementation,
) -> Result<(), ScaffoldingError> {
	validate_port_implementation_preconditions(project_root, &implementation)?;

	let implementation_directory = implementation.implementation_directory(project_root);
	let implementation_path = implementation.implementation_file_path(project_root);
	let mod_file_path = implementation_directory.join("mod.rs");
	let mut mod_content = if mod_file_path.exists() {
		fs::read_to_string(&mod_file_path)
			.map_err(|error| map_port_implementation_fs_error(io_conflict(&mod_file_path, "read module file", error), &implementation))?
	} else {
		String::new()
	};
	let module_declaration = format!("pub mod {};", implementation.implementation_module_name());
	if !mod_content.lines().any(|line| line.trim() == module_declaration) {
		if !mod_content.is_empty() && !mod_content.ends_with('\n') {
			mod_content.push('\n');
		}
		mod_content.push_str(&module_declaration);
		mod_content.push('\n');
	}

	let implementation_content = PORT_IMPLEMENTATION_CONTENT
        .replace("{bounded_context_name}", implementation.bounded_context().name().as_str())
		.replace("{domain_feature_name}", implementation.domain_feature_name().as_str())
		.replace("{port_name}", implementation.port_name().as_str())
		.replace("{trait_name}", &implementation.trait_name())
		.replace("{struct_name}", &implementation.struct_name());
	let mut tx = FileSystemTransaction::new();
	tx.create_file(&implementation_path, &implementation_content);
	tx.modify_file(&mod_file_path, mod_content)
		.map_err(|error| map_port_implementation_fs_error(error, &implementation))?;
	tx.commit().map_err(|error| map_port_implementation_fs_error(error, &implementation))?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::TempDir;
	use std::fs;

	#[test]
	fn scaffold_creates_implementation_and_mod_update() {
		let tmp = TempDir::new().unwrap();
		let project_root = tmp.path();
		let bc = "test".to_string();
		// create port file path precondition
		let port_dir = project_root.join("crates").join(&bc).join("domain").join("src").join("auth");
		fs::create_dir_all(&port_dir).unwrap();
		fs::write(port_dir.join("repo.rs"), "// port").unwrap();

		// create implementation directory and module file that the transaction updates
		let impl_dir = project_root.join("crates").join(&bc).join("infrastructure").join("src").join("postgres");
		fs::create_dir_all(&impl_dir).unwrap();
		fs::write(impl_dir.join("mod.rs"), "").unwrap();

		let implementation = scaffolding_domain::port_implementation_entity::PortImplementation::new(
			shared_domain::bounded_context_entity::BoundedContext::new(bc.clone()).unwrap(),
			"auth".to_string(),
			"repo".to_string(),
			"postgres".to_string(),
		).unwrap();

		let res = scaffold_port_implementation(project_root, implementation);
		assert!(res.is_ok());

		let file = project_root.join("crates").join(&bc).join("infrastructure").join("src").join("postgres").join("auth_repo.rs");
		assert!(file.is_file());
		let mod_file = project_root.join("crates").join(&bc).join("infrastructure").join("src").join("postgres").join("mod.rs");
		let content = fs::read_to_string(mod_file).unwrap();
		assert!(content.contains("pub mod auth_repo;"));
	}

	#[test]
	fn missing_port_file_returns_error() {
		let tmp = TempDir::new().unwrap();
		let project_root = tmp.path();
		let bc = "test".to_string();
		let impl_dir = project_root.join("crates").join(&bc).join("infrastructure").join("src").join("postgres");
		fs::create_dir_all(&impl_dir).unwrap();

		let implementation = scaffolding_domain::port_implementation_entity::PortImplementation::new(
			shared_domain::bounded_context_entity::BoundedContext::new(bc.clone()).unwrap(),
			"auth".to_string(),
			"repo".to_string(),
			"postgres".to_string(),
		).unwrap();

		let res = scaffold_port_implementation(project_root, implementation);
		assert!(matches!(res, Err(ScaffoldingError::BoundedContextNotFound { .. })));
	}
}
