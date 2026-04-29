use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use crate::output::ErrorMessage;

#[derive(Debug)]
pub enum CliError {
	CurrentDir(std::io::Error),
	ProjectRootNotFound(PathBuf),
	OperationFailed(String),
}

impl CliError {
	/// Convert CLI error to a formatted ErrorMessage for presentation
	pub fn to_message(&self) -> ErrorMessage {
		match self {
			Self::CurrentDir(error) => {
				ErrorMessage::new("Failed to resolve current directory")
					.with_detail(format!("{}", error))
					.with_hint("Make sure you have permissions to access the current directory")
			}
			Self::ProjectRootNotFound(start) => {
				ErrorMessage::new("Could not find a crust project")
					.with_detail(format!("Started searching from: {}", start.display()))
					.with_hint("Run 'crust init' to create a new project, or navigate to an existing crust project")
			}
			Self::OperationFailed(message) => {
				ErrorMessage::new("Operation failed")
					.with_detail(message.clone())
			}
		}
	}
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
