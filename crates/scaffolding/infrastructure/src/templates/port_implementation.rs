pub const PORT_IMPLEMENTATION_CONTENT: &str = r#"use {bounded_context_name}_domain::{feature_name}::{port_name}::{trait_name};

pub struct {struct_name};

impl {trait_name} for {struct_name} {
    // Implement trait methods here.
}
"#;
