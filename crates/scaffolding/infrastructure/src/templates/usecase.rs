pub const USECASE_CONTENT: &str = r#"pub struct {artifact_name} {
    // example:
    // dependency: Box<dyn TraitExample>,
}

impl {artifact_name} {
    pub fn new(
        // dependency: Box<dyn TraitExample>,
    ) -> Self {
        Self {
            // dependency,
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        todo!()
    }
}
"#;