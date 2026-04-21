use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub enum CliError {
	CurrentDir(std::io::Error),
	ProjectRootNotFound(PathBuf),
	OperationFailed(String),
}

impl Display for CliError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::CurrentDir(error) => write!(f, "Error resolving current directory: {error}"),
			Self::ProjectRootNotFound(start) => {
				write!(f, "Could not find project root from '{}'.", start.display())
			}
			Self::OperationFailed(message) => write!(f, "{message}"),
		}
	}
}

impl std::error::Error for CliError {}
