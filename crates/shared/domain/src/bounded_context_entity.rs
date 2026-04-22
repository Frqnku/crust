use crate::errors::SharedError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedContextName(String);

impl BoundedContextName {
    pub fn new(value: String) -> Result<Self, SharedError> {
        if value.is_empty() {
            return Err(SharedError::InvalidBoundedContextName {
                value,
                reason: "name cannot be empty".to_string(),
            });
        }

        if value.chars().any(|character| character == '/' || character == '\\') {
            return Err(SharedError::InvalidBoundedContextName {
                value,
                reason: "name cannot contain path separators".to_string(),
            });
        }

        if !value
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_' || character == '-')
        {
            return Err(SharedError::InvalidBoundedContextName {
                value,
                reason: "name must use lowercase ASCII letters, digits, underscore, or hyphen".to_string(),
            });
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for BoundedContextName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for BoundedContextName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct BoundedContext {
    name: BoundedContextName,
}

impl BoundedContext {
    pub fn new(name: String) -> Result<Self, SharedError> {
        Ok(Self {
            name: BoundedContextName::new(name)?,
        })
    }

    pub fn name(&self) -> &BoundedContextName {
        &self.name
    }
}