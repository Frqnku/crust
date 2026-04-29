use std::fmt::Display;

/// Represents a successful operation result
pub struct Success {
	title: String,
	details: Vec<String>,
}

impl Success {
	pub fn new(title: impl Into<String>) -> Self {
		Self {
			title: title.into(),
			details: Vec::new(),
		}
	}

	pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
		self.details.push(detail.into());
		self
	}

	pub fn with_details(mut self, details: Vec<String>) -> Self {
		self.details = details;
		self
	}

	pub fn title(&self) -> &str {
		&self.title
	}

	pub fn details(&self) -> &[String] {
		&self.details
	}
}

/// Represents an error result
pub struct ErrorMessage {
	title: String,
	details: Vec<String>,
	hint: Option<String>,
}

impl ErrorMessage {
	pub fn new(title: impl Into<String>) -> Self {
		Self {
			title: title.into(),
			details: Vec::new(),
			hint: None,
		}
	}

	pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
		self.details.push(detail.into());
		self
	}

	pub fn with_details(mut self, details: Vec<String>) -> Self {
		self.details = details;
		self
	}

	pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
		self.hint = Some(hint.into());
		self
	}

	pub fn title(&self) -> &str {
		&self.title
	}

	pub fn details(&self) -> &[String] {
		&self.details
	}

	pub fn hint(&self) -> Option<&str> {
		self.hint.as_deref()
	}
}

impl Display for ErrorMessage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.title)
	}
}
