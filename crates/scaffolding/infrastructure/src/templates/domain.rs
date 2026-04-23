pub const DOMAIN_FEATURE_MOD_CONTENT: &str = r#"pub mod entity;"#;

pub const DOMAIN_ENTITY_CONTENT: &str = r#"pub struct {entity_name} {
    // example:
    // field: String,
}

impl {entity_name} {
    pub fn new(
        // field: String,
    ) -> Self {
        Self {
            // field,
        }
    }
}
"#;