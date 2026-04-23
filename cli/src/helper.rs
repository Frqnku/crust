use crate::error::CliError;
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::Document;

pub fn current_dir() -> Result<PathBuf, CliError> {
	std::env::current_dir().map_err(CliError::CurrentDir)
}

pub fn find_project_root(start: &Path) -> Option<PathBuf> {
	for candidate in start.ancestors() {
		let cargo_toml_path = candidate.join("Cargo.toml");
		if !cargo_toml_path.is_file() {
			continue;
		}

		// Try to parse the Cargo.toml
		if let Ok(content) = fs::read_to_string(&cargo_toml_path) {
			if let Ok(doc) = content.parse::<Document<String>>() {
				// Look for [workspace] section with [workspace.metadata.crust] version = "1"
				if let Some(workspace) = doc.get("workspace").and_then(|w| w.as_table()) {
					if let Some(metadata) = workspace.get("metadata").and_then(|m| m.as_table()) {
						if let Some(crust) = metadata.get("crust").and_then(|c| c.as_table()) {
							if let Some(version) = crust.get("version").and_then(|v| v.as_str()) {
								if version == "1" {
									return Some(candidate.to_path_buf());
								}
							}
						}
					}
				}
			}
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
