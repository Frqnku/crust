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

#[cfg(test)]
mod tests {
	use super::*;
	use std::{rc::Rc, cell::RefCell, path::PathBuf};

	struct MockProjectScaffolder { called: RefCell<bool> }
	impl MockProjectScaffolder { fn new() -> Self { Self { called: RefCell::new(false) } } }
	impl workspace_domain::project_entity::ProjectScaffolder for MockProjectScaffolder {
		fn create_project(&self, _name: String, _path: PathBuf) -> Result<workspace_domain::project_entity::Project, workspace_domain::errors::WorkspaceError> { unimplemented!() }
		fn create_bounded_context(&self, _project: &mut workspace_domain::project_entity::Project, _bounded_context: shared_domain::bounded_context_entity::BoundedContext) -> Result<(), workspace_domain::errors::WorkspaceError> {
			*self.called.borrow_mut() = true; Ok(())
		}
	}

	#[test]
	fn execute_calls_create_bounded_context() {
		let mock = Rc::new(MockProjectScaffolder::new());
		let usecase = AddBoundedContext::new(mock.clone());
		let input = AddBoundedContextInput::new("proj".into(), PathBuf::from("."), "test".into());
		let res = usecase.execute(input);
		assert!(res.is_ok());
		assert!(*mock.called.borrow());
	}

	#[test]
	fn invalid_project_name_returns_error_without_calling_scaffolder() {
		let mock = Rc::new(MockProjectScaffolder::new());
		let usecase = AddBoundedContext::new(mock.clone());
		let input = AddBoundedContextInput::new("Bad Project".into(), PathBuf::from("."), "test".into());
		let res = usecase.execute(input);
		assert!(matches!(res, Err(workspace_domain::errors::WorkspaceError::InvalidProjectName { .. })));
		assert!(!*mock.called.borrow());
	}
}
