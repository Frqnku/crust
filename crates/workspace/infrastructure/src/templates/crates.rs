const DOMAIN_CARGO_TOML: &str = r#"[package]
name = "{bounded_context_name}_domain"
version = "0.1.0"
edition = "2024"

[dependencies]
"#;

const APPLICATION_CARGO_TOML: &str = r#"[package]
name = "{bounded_context_name}_application"
version = "0.1.0"
edition = "2024"

[dependencies]
{bounded_context_name}_domain = { path = "../domain" }
"#;

const INFRASTRUCTURE_CARGO_TOML: &str = r#"[package]
name = "{bounded_context_name}_infrastructure"
version = "0.1.0"
edition = "2024"

[dependencies]
{bounded_context_name}_domain = { path = "../domain" }
"#;

pub struct WorkspaceTemplate {
    pub name: &'static str,
    pub template: &'static str,
}

pub const WORKSPACE_TEMPLATES: &[WorkspaceTemplate] = &[
    WorkspaceTemplate { name: "domain", template: DOMAIN_CARGO_TOML },
    WorkspaceTemplate { name: "application", template: APPLICATION_CARGO_TOML },
    WorkspaceTemplate { name: "infrastructure", template: INFRASTRUCTURE_CARGO_TOML },
];