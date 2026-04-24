pub fn validate_crust_identifier(value: &str) -> Result<(), &'static str> {
    if value.is_empty() {
        return Err("name cannot be empty");
    }

    if value
        .chars()
        .any(|character| character == '/' || character == '\\')
    {
        return Err("name cannot contain path separators");
    }

    if !value.chars().all(|character| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || character == '_'
            || character == '-'
    }) {
        return Err("name must use lowercase ASCII letters, digits, underscore, or hyphen");
    }

    Ok(())
}