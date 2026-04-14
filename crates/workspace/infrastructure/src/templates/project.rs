pub const PROJECT_CARGO_TOML: &str = r#"[workspace]
resolver = "3"
members = [
    "bin"
]

[workspace.dependencies]

[profile.dev]
opt-level = 1

[profile.release]
opt-level = 3
"#;