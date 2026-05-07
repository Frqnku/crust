use std::{path::PathBuf, rc::Rc};

use scaffolding_domain::{
	errors::ScaffoldingError,
	port_implementation_entity::PortImplementation,
	scaffolder_ports::PortImplementationScaffolder,
};
use shared_domain::bounded_context_entity::BoundedContext;

pub struct CreatePortImplementationInput {
	pub project_root: PathBuf,
	pub bounded_context_name: String,
	pub domain_feature_name: String,
	pub port_name: String,
	pub tech_name: String,
}

impl CreatePortImplementationInput {
	pub fn new(
		project_root: PathBuf,
		bounded_context_name: String,
		domain_feature_name: String,
		port_name: String,
		tech_name: String,
	) -> Self {
		Self {
			project_root,
			bounded_context_name,
			domain_feature_name,
			port_name,
			tech_name,
		}
	}
}

pub struct CreatePortImplementation {
	scaffolder: Rc<dyn PortImplementationScaffolder>,
}

impl CreatePortImplementation {
	pub fn new(scaffolder: Rc<dyn PortImplementationScaffolder>) -> Self {
		Self { scaffolder }
	}

	pub fn execute(&self, input: CreatePortImplementationInput) -> Result<(), ScaffoldingError> {
		let bounded_context = BoundedContext::new(input.bounded_context_name)?;
		let implementation = PortImplementation::new(
			bounded_context,
			input.domain_feature_name,
			input.port_name,
			input.tech_name,
		)?;
		self.scaffolder.scaffold(&input.project_root, implementation)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::{cell::RefCell, path::Path, rc::Rc};

	struct MockScaffolder {
		called: RefCell<bool>,
	}

	impl MockScaffolder {
		fn new() -> Self {
			Self { called: RefCell::new(false) }
		}
	}

	impl PortImplementationScaffolder for MockScaffolder {
		fn scaffold(
			&self,
			_project_root: &Path,
			_implementation: scaffolding_domain::port_implementation_entity::PortImplementation,
		) -> Result<(), scaffolding_domain::errors::ScaffoldingError> {
			*self.called.borrow_mut() = true;
			Ok(())
		}
	}

	#[test]
	fn execute_calls_scaffolder() {
		let mock = Rc::new(MockScaffolder::new());
		let usecase = CreatePortImplementation::new(mock.clone());
		let input = CreatePortImplementationInput::new(PathBuf::from("."), "test".into(), "auth".into(), "repo".into(), "postgres".into());
		let res = usecase.execute(input);
		assert!(res.is_ok());
		assert!(*mock.called.borrow());
	}

	#[test]
	fn invalid_bounded_context_returns_error_without_calling_scaffolder() {
		let mock = Rc::new(MockScaffolder::new());
		let usecase = CreatePortImplementation::new(mock.clone());
		let input = CreatePortImplementationInput::new(PathBuf::from("."), "Bad Context".into(), "auth".into(), "repo".into(), "postgres".into());
		let res = usecase.execute(input);
		assert!(matches!(res, Err(ScaffoldingError::InvalidBoundedContextName { .. })));
		assert!(!*mock.called.borrow());
	}
}
