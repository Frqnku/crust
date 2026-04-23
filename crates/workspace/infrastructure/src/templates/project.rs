pub const PROJECT_CARGO_TOML: &str = r#"[workspace]
resolver = "3"
members = [
    "bin",

    # Bounded context crates
]

[workspace.metadata.crust]
version = "1"

[workspace.dependencies]

[profile.dev]
opt-level = 1

[profile.release]
opt-level = 3
"#;