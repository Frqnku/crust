use std::path::Path;

use shared_infrastructure::fs_helper::{
    create_dir,
    create_file,
    create_unique_temp_dir,
    finalize_scaffold,
    FsHelperError,
    remove_dir,
};
use crate::templates::crates::WORKSPACE_TEMPLATES;

use workspace_domain::{
    entities::{BoundedContext, Project},
    errors::WorkspaceError,
};

fn map_fs_error(error: FsHelperError) -> WorkspaceError {
    match error {
        FsHelperError::Conflict { path, reason } => WorkspaceError::ProjectStructureConflict { path, reason },
        FsHelperError::InvalidLayout { path, reason } => WorkspaceError::InvalidProjectLayout { path, reason },
    }
}

fn create_layer_workspace(
    bounded_context_path: &Path,
    bounded_context: &BoundedContext,
    workspace_layer_name: &str,
    cargo_toml_template: &str,
) -> Result<(), WorkspaceError> {
    let workspace_path = bounded_context_path.join(workspace_layer_name);
    create_dir(&workspace_path).map_err(map_fs_error)?;
    create_dir(&workspace_path.join("src")).map_err(map_fs_error)?;
    let cargo_toml_content = cargo_toml_template.replace("{bounded_context_name}", bounded_context.name.as_str());
    create_file(&workspace_path.join("Cargo.toml"), &cargo_toml_content).map_err(map_fs_error)?;

    Ok(())
}

pub fn scaffold_bounded_context(
    project: &mut Project,
    bounded_context: BoundedContext,
) -> Result<(), WorkspaceError> {
    let bounded_context_path = project.ensure_can_create_bounded_context(&bounded_context)?;
    if bounded_context_path.exists() {
        return Err(WorkspaceError::BoundedContextAlreadyExists {
            path: bounded_context_path,
        });
    }

    let temp_root_parent = project.expected_crates_path();
    let temp_root = create_unique_temp_dir(&temp_root_parent, "crust-bounded-context").map_err(map_fs_error)?;

    let scaffold_result = (|| {
        for ws in WORKSPACE_TEMPLATES {
            create_layer_workspace(&temp_root, &bounded_context, ws.name, ws.template)?;
        }

        Ok(())
    })();

    if let Err(error) = scaffold_result {
        remove_dir(&temp_root);
        return Err(error);
    }

    if let Err(error) = finalize_scaffold(&temp_root, &bounded_context_path).map_err(map_fs_error) {
        remove_dir(&temp_root);
        return Err(error);
    }

    project.add_bounded_context(bounded_context)
}
