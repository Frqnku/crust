use std::path::Path;

use scaffolding_domain::{
	errors::ScaffoldingError,
	port_implementation_entity::PortImplementation,
};
use shared_infrastructure::fs_helper::{create_file, map_fs_error, FsHelperError};

use crate::fs_scaffold_helper::{rollback_created_file, upsert_mod_declaration};
use crate::templates::port_implementation::PORT_IMPLEMENTATION_CONTENT;

fn map_port_implementation_fs_error(
	error: FsHelperError,
	implementation: &PortImplementation,
) -> ScaffoldingError {
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

fn rollback_implementation_file(
	implementation_path: &Path,
	implementation: &PortImplementation,
) -> Result<(), ScaffoldingError> {
	rollback_created_file(implementation_path).map_err(|error| ScaffoldingError::ArtifactRollbackFailed {
		name: implementation.implementation_module_name(),
		bounded_context: implementation.bounded_context().name().as_str().to_string(),
		reason: format!("failed to remove '{}': {error}", implementation_path.display()),
	})
}

pub fn scaffold_port_implementation(
	project_root: &Path,
	implementation: PortImplementation,
) -> Result<(), ScaffoldingError> {
	let port_file_path = implementation.port_file_path(project_root);
	if !port_file_path.is_file() {
		return Err(ScaffoldingError::BoundedContextNotFound {
			name: format!(
				"{}/domain/{}/{}.rs",
				implementation.bounded_context().name().as_str(),
				implementation.feature_name().as_str(),
				implementation.port_name().as_str()
			),
		});
	}

	let implementation_directory = implementation.implementation_directory(project_root);
	if !implementation_directory.is_dir() {
		return Err(ScaffoldingError::BoundedContextNotFound {
			name: format!(
				"{}/infrastructure/{}",
				implementation.bounded_context().name().as_str(),
				implementation.tech_name().as_str()
			),
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

	let implementation_content = PORT_IMPLEMENTATION_CONTENT
        .replace("{bounded_context_name}", implementation.bounded_context().name().as_str())
		.replace("{feature_name}", implementation.feature_name().as_str())
		.replace("{port_name}", implementation.port_name().as_str())
		.replace("{trait_name}", &implementation.trait_name())
		.replace("{struct_name}", &implementation.struct_name());
	create_file(&implementation_path, &implementation_content)
		.map_err(|error| map_port_implementation_fs_error(error, &implementation))?;

	let mod_file_path = implementation_directory.join("mod.rs");
	if let Err(error) = upsert_mod_declaration(
		&mod_file_path,
		&implementation.implementation_module_name(),
		"pub mod",
	)
	.map_err(|error| map_port_implementation_fs_error(error, &implementation))
	{
		rollback_implementation_file(&implementation_path, &implementation)?;
		return Err(error);
	}

	Ok(())
}
