pub const BIN_CARGO_TOML: &str = r#"[package]
name = "bin"
version = "0.1.0"
edition = "2024"

[dependencies]

[[bin]]
name = "{bin_name}"
path = "main.rs"
"#;

pub const MAIN_RS_CONTENT: &str = r#"fn main() {
    println!("Hello crust!");
}
"#;