use std::{fs, path::Path};

use shared_infrastructure::fs_helper::{io_conflict, FsHelperError};

pub fn upsert_mod_declaration(
	mod_file_path: &Path,
	module_name: &str,
	declaration_prefix: &str,
) -> Result<(), FsHelperError> {
	let module_declaration = format!("{declaration_prefix} {module_name};");

	let mut content = if mod_file_path.exists() {
		fs::read_to_string(mod_file_path)
			.map_err(|error| io_conflict(mod_file_path, "read module file", error))?
	} else {
		String::new()
	};

	if content.lines().any(|line| line.trim() == module_declaration) {
		return Ok(());
	}

	if !content.is_empty() && !content.ends_with('\n') {
		content.push('\n');
	}
	content.push_str(&module_declaration);
	content.push('\n');

	fs::write(mod_file_path, content).map_err(|error| io_conflict(mod_file_path, "write module file", error))
}

pub fn rollback_created_file(path: &Path) -> Result<(), std::io::Error> {
	fs::remove_file(path)
}

pub fn rollback_created_directory(path: &Path) -> Result<(), std::io::Error> {
	fs::remove_dir_all(path)
}
