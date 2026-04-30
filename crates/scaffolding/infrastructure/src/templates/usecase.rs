pub const USECASE_CONTENT: &str = r#"pub struct {usecase_name};

impl {usecase_name} {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self) -> Result<(), String> {
        Ok(())
    }
}
"#;