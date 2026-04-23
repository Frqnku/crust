pub const USECASE_CONTENT: &str = r#"pub struct {usecase_name} {
    // example:
    // dependency: Box<dyn TraitExample>,
}

impl {usecase_name} {
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