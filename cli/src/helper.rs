use crate::error::CliError;
use std::path::{Path, PathBuf};

pub fn current_dir() -> Result<PathBuf, CliError> {
	std::env::current_dir().map_err(CliError::CurrentDir)
}

pub fn find_project_root(start: &Path) -> Option<PathBuf> {
	for candidate in start.ancestors() {
		if candidate.join("Cargo.toml").is_file() && candidate.join("crates").is_dir() {
			return Some(candidate.to_path_buf());
		}
	}

	None
}

pub fn resolve_project_root() -> Result<PathBuf, CliError> {
	let cwd = current_dir()?;
	find_project_root(&cwd).ok_or(CliError::ProjectRootNotFound(cwd))
}

pub fn project_name_from_root(root: &Path) -> String {
	root.file_name()
		.and_then(|name| name.to_str())
		.map_or_else(|| "project".to_string(), |name| name.to_string())
}
