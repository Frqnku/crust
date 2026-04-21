use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum SharedError {
	InvalidBoundedContextName { value: String, reason: String },
}

impl Display for SharedError {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidBoundedContextName { value, reason } => {
				write!(formatter, "Invalid bounded context name '{value}': {reason}")
			}
		}
	}
}

impl std::error::Error for SharedError {}