use std::path::{Path, PathBuf};

use shared_infrastructure::fs_helper::{
    create_dir,
    create_file,
    create_unique_temp_dir,
    finalize_scaffold,
    map_fs_error,
    io_conflict,
    remove_dir,
};
use crate::templates::{
    bin::{BIN_CARGO_TOML, MAIN_RS_CONTENT},
    project::PROJECT_CARGO_TOML,
};

use workspace_domain::{
    project_entity::Project,
    errors::WorkspaceError,
};

fn workspace_error_from_fs(error: shared_infrastructure::fs_helper::FsHelperError) -> WorkspaceError {
    map_fs_error(
        error,
        |path, reason| WorkspaceError::ProjectStructureConflict { path, reason },
        |path, reason| WorkspaceError::InvalidProjectLayout { path, reason },
    )
}

fn check_valid_project_directory(path: &Path) -> Result<(), WorkspaceError> {
    if path.exists() {
        if path.is_dir() {
            let mut entries = path
                .read_dir()
                .map_err(|error| workspace_error_from_fs(io_conflict(path, "read project directory", error)))?;
            if entries.next().is_some() {
                return Err(WorkspaceError::ProjectStructureConflict {
                    path: path.to_path_buf(),
                    reason: "The target project directory is not empty".to_string(),
                });
            }
        } else {
            return Err(WorkspaceError::InvalidProjectLayout {
                path: path.to_path_buf(),
                reason: "The target project path is a file, expected a directory".to_string(),
            });
        }
    }

    Ok(())
}

fn create_project_setup(project: &Project, scaffold_root: &Path) -> Result<(), WorkspaceError> {
    create_dir(scaffold_root).map_err(workspace_error_from_fs)?;

    let cargo_toml_path = scaffold_root.join("Cargo.toml");
    create_file(&cargo_toml_path, PROJECT_CARGO_TOML).map_err(workspace_error_from_fs)?;

    let bin_path = scaffold_root.join("bin");
    create_dir(&bin_path).map_err(workspace_error_from_fs)?;

    let bin_cargo_toml = BIN_CARGO_TOML.replace("{bin_name}", project.name().as_str());
    create_file(&bin_path.join("Cargo.toml"), &bin_cargo_toml).map_err(workspace_error_from_fs)?;
    create_file(&bin_path.join("main.rs"), MAIN_RS_CONTENT).map_err(workspace_error_from_fs)?;

    let crates_path = scaffold_root.join("crates");
    create_dir(&crates_path).map_err(workspace_error_from_fs)?;

    Ok(())
}

pub fn scaffold_project(name: String, path: PathBuf) -> Result<Project, WorkspaceError> {
    check_valid_project_directory(&path)?;

    let parent = path.parent().unwrap_or(Path::new("."));
    let temp_root = create_unique_temp_dir(parent, "crust-project").map_err(workspace_error_from_fs)?;
    let project = Project::new(name, path.clone())?;

    if let Err(error) = create_project_setup(&project, &temp_root) {
        remove_dir(&temp_root);
        return Err(error);
    }

    if let Err(error) = finalize_scaffold(&temp_root, &path).map_err(workspace_error_from_fs) {
        remove_dir(&temp_root);
        return Err(error);
    }

    Ok(project)
}
