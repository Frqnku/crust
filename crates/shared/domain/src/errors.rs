#[derive(Debug, PartialEq)]
pub enum SharedError {
	InvalidBoundedContextName { value: String, reason: String },
}