use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsHelperError {
    Conflict { path: PathBuf, reason: String },
    InvalidLayout { path: PathBuf, reason: String },
}

pub fn io_conflict(path: &Path, action: &str, error: std::io::Error) -> FsHelperError {
    FsHelperError::Conflict {
        path: path.to_path_buf(),
        reason: format!("Failed to {action}: {error}"),
    }
}

pub fn map_fs_error<T, ConflictMapper, InvalidLayoutMapper>(
    error: FsHelperError,
    conflict_mapper: ConflictMapper,
    invalid_layout_mapper: InvalidLayoutMapper,
) -> T
where
    ConflictMapper: FnOnce(PathBuf, String) -> T,
    InvalidLayoutMapper: FnOnce(PathBuf, String) -> T,
{
    match error {
        FsHelperError::Conflict { path, reason } => conflict_mapper(path, reason),
        FsHelperError::InvalidLayout { path, reason } => invalid_layout_mapper(path, reason),
    }
}

pub fn create_dir(path: &Path) -> Result<(), FsHelperError> {
    fs::create_dir_all(path).map_err(|error| io_conflict(path, "create directory", error))
}

pub fn create_file(path: &Path, content: &str) -> Result<(), FsHelperError> {
    fs::write(path, content).map_err(|error| io_conflict(path, "write file", error))
}

pub fn remove_dir(path: &Path) {
    let _ = fs::remove_dir_all(path);
}

pub fn move_dir(from: &Path, to: &Path) -> Result<(), FsHelperError> {
    if to.exists() {
        return Err(FsHelperError::Conflict {
            path: to.to_path_buf(),
            reason: "Refusing to finalize scaffold: destination already exists".to_string(),
        });
    }

    fs::rename(from, to).map_err(|error| io_conflict(to, "finalize scaffold", error))
}

pub fn directory_is_empty(path: &Path) -> Result<bool, FsHelperError> {
    let mut entries = path
        .read_dir()
        .map_err(|error| io_conflict(path, "read directory", error))?;
    Ok(entries.next().is_none())
}

pub fn finalize_scaffold(from: &Path, to: &Path) -> Result<(), FsHelperError> {
    if to.exists() {
        if !to.is_dir() {
            return Err(FsHelperError::InvalidLayout {
                path: to.to_path_buf(),
                reason: "The target project path is a file, expected a directory".to_string(),
            });
        }

        if !directory_is_empty(to)? {
            return Err(FsHelperError::Conflict {
                path: to.to_path_buf(),
                reason: "Refusing to finalize scaffold: destination directory is not empty".to_string(),
            });
        }

        for entry in fs::read_dir(from).map_err(|error| io_conflict(from, "read temporary scaffold", error))? {
            let entry = entry.map_err(|error| io_conflict(from, "read temporary scaffold entry", error))?;
            let source_path = entry.path();
            let destination_path = to.join(entry.file_name());
            fs::rename(&source_path, &destination_path)
                .map_err(|error| io_conflict(&destination_path, "move scaffold entry", error))?;
        }

        fs::remove_dir(from).map_err(|error| io_conflict(from, "remove temporary scaffold directory", error))?;
        return Ok(());
    }

    move_dir(from, to)
}

pub fn create_unique_temp_dir(parent: &Path, prefix: &str) -> Result<PathBuf, FsHelperError> {
    for attempt in 0..10_u32 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();

        let temp_dir = parent.join(format!(".{prefix}-{}-{timestamp}-{attempt}", process::id()));

        match fs::create_dir(&temp_dir) {
            Ok(()) => return Ok(temp_dir),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(io_conflict(&temp_dir, "create temporary directory", error)),
        }
    }

    Err(FsHelperError::Conflict {
        path: parent.to_path_buf(),
        reason: "Unable to create a unique temporary directory for scaffolding".to_string(),
    })
}
