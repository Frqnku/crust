use crate::errors::ScaffoldingError;

const RUST_RESERVED_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "async", "await", "dyn",
];

pub enum ArtifactKind {
    QueryUsecase,
    CommandUsecase,
}

impl ArtifactKind {
    pub fn as_str(&self) -> &str {
        match self {
            ArtifactKind::QueryUsecase => "query",
            ArtifactKind::CommandUsecase => "command",
        }
    }

    pub fn parent_directory(&self) -> &str {
        match self {
            ArtifactKind::QueryUsecase => "application",
            ArtifactKind::CommandUsecase => "application",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactName(String);

impl ArtifactName {
    pub fn new(value: String) -> Result<Self, ScaffoldingError> {
        if value.is_empty() {
            return Err(ScaffoldingError::InvalidArtifactName {
                value,
                reason: "name cannot be empty".to_string(),
            });
        }

        if RUST_RESERVED_KEYWORDS.contains(&value.as_str()) {
            return Err(ScaffoldingError::InvalidArtifactName {
                value,
                reason: "name is a reserved Rust keyword; choose a different identifier".to_string(),
            });
        }

        if value.chars().any(|character| character == '/' || character == '\\') {
            return Err(ScaffoldingError::InvalidArtifactName {
                value,
                reason: "name cannot contain path separators".to_string(),
            });
        }

        let first = value.chars().next().expect("value is guaranteed to be non-empty");
        if !first.is_ascii_lowercase() && first != '_' {
            return Err(ScaffoldingError::InvalidArtifactName {
                value,
                reason: "name must start with a lowercase ASCII letter or underscore".to_string(),
            });
        }

        if !value
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_')
        {
            return Err(ScaffoldingError::InvalidArtifactName {
                value,
                reason: "name must use only lowercase ASCII letters, digits, or underscore".to_string(),
            });
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_pascal_case(&self) -> String {
        self.0
            .split(|c: char| c == '_' || c == '-')
            .filter(|part| !part.is_empty())
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    Some(first) => {
                        let mut result = String::new();
                        result.push(first.to_ascii_uppercase());
                        result.push_str(chars.as_str());
                        result
                    }
                    None => String::new(),
                }
            })
            .collect::<String>()
    }
}