use std::path::{Path, PathBuf};

use crust_shared_infrastructure::{
    fs_helper::{
        create_unique_temp_dir,
        finalize_scaffold,
        map_fs_error,
        io_conflict,
        remove_dir,
    },
    FileSystemTransaction,
};
use crate::templates::{
    bin::{BIN_CARGO_TOML, MAIN_RS_CONTENT},
    project::PROJECT_CARGO_TOML,
};

use crust_workspace_domain::{
    project_entity::Project,
    errors::WorkspaceError,
};

fn workspace_error_from_fs(error: crust_shared_infrastructure::fs_helper::FsHelperError) -> WorkspaceError {
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

/// Build a transaction for project setup
/// Returns a transaction that, when committed, creates the entire project structure atomically
fn build_project_setup_transaction(
    project: &Project,
    scaffold_root: &Path,
) -> Result<FileSystemTransaction, WorkspaceError> {
    let mut tx = FileSystemTransaction::new();

    // Create root directory
    tx.create_dir(scaffold_root);

    // Create workspace Cargo.toml
    tx.create_file(scaffold_root.join("Cargo.toml"), PROJECT_CARGO_TOML);

    // Create bin directory and files
    let bin_path = scaffold_root.join("bin");
    tx.create_dir(&bin_path);

    let bin_cargo_toml = BIN_CARGO_TOML.replace("{bin_name}", project.name().as_str());
    tx.create_file(bin_path.join("Cargo.toml"), bin_cargo_toml);
    tx.create_file(bin_path.join("main.rs"), MAIN_RS_CONTENT);

    // Create crates directory
    let crates_path = scaffold_root.join("crates");
    tx.create_dir(crates_path);

    Ok(tx)
}

pub fn scaffold_project(name: String, path: PathBuf) -> Result<Project, WorkspaceError> {
    check_valid_project_directory(&path)?;

    let parent = path.parent().unwrap_or(Path::new("."));
    let temp_root = create_unique_temp_dir(parent, "crust-project").map_err(workspace_error_from_fs)?;
    let project = Project::new(name, path.clone())?;

    // Build and commit the transaction atomically
    let tx = build_project_setup_transaction(&project, &temp_root)?;
    if let Err(error) = tx.commit().map_err(workspace_error_from_fs) {
        remove_dir(&temp_root);
        return Err(error);
    }

    // Finalize: move from temp to actual location
    if let Err(error) = finalize_scaffold(&temp_root, &path).map_err(workspace_error_from_fs) {
        remove_dir(&temp_root);
        return Err(error);
    }

    Ok(project)
}

