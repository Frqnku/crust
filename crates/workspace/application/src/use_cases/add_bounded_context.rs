use std::{path::PathBuf, rc::Rc};

use shared_domain::bounded_context_entity::BoundedContext;
use workspace_domain::{
	project_entity::{
		ProjectScaffolder,
		Project,
	},
	errors::WorkspaceError,
};

pub struct AddBoundedContextInput {
	pub project_name: String,
	pub project_path: PathBuf,
	pub bounded_context_name: String,
}

impl AddBoundedContextInput {
	pub fn new(project_name: String, project_path: PathBuf, bounded_context_name: String) -> Self {
		Self {
			project_name,
			project_path,
			bounded_context_name,
		}
	}
}

pub struct AddBoundedContext {
	scaffolder: Rc<dyn ProjectScaffolder>,
}

impl AddBoundedContext {
	pub fn new(scaffolder: Rc<dyn ProjectScaffolder>) -> Self {
		Self { scaffolder }
	}

	pub fn execute(&self, input: AddBoundedContextInput) -> Result<(), WorkspaceError> {
		let mut project = Project::new(input.project_name, input.project_path)?;
		let bounded_context = BoundedContext::new(input.bounded_context_name)?;
		self.scaffolder.create_bounded_context(&mut project, bounded_context)
	}
}
