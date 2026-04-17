pub mod project_entity;
pub mod errors;

pub mod entities {
	pub use crate::project_entity::{Project, ProjectName, ProjectScaffolder};
	pub use shared_domain::bounded_context_entity::{BoundedContext, BoundedContextName};
}