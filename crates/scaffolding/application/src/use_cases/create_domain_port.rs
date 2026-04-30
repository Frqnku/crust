use std::{path::PathBuf, rc::Rc};

use scaffolding_domain::{
	domain_port_entity::DomainPort,
	errors::ScaffoldingError,
	scaffolder_ports::DomainPortScaffolder,
};
use shared_domain::bounded_context_entity::BoundedContext;

pub struct CreateDomainPortInput {
	pub project_root: PathBuf,
	pub bounded_context_name: String,
	pub feature_name: String,
	pub port_name: String,
}

impl CreateDomainPortInput {
	pub fn new(
		project_root: PathBuf,
		bounded_context_name: String,
		feature_name: String,
		port_name: String,
	) -> Self {
		Self {
			project_root,
			bounded_context_name,
			feature_name,
			port_name,
		}
	}
}

pub struct CreateDomainPort {
	scaffolder: Rc<dyn DomainPortScaffolder>,
}

impl CreateDomainPort {
	pub fn new(scaffolder: Rc<dyn DomainPortScaffolder>) -> Self {
		Self { scaffolder }
	}

	pub fn execute(&self, input: CreateDomainPortInput) -> Result<(), ScaffoldingError> {
		let bounded_context = BoundedContext::new(input.bounded_context_name)?;
		let domain_port = DomainPort::new(bounded_context, input.feature_name, input.port_name)?;
		self.scaffolder.scaffold(&input.project_root, domain_port)
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

	impl DomainPortScaffolder for MockScaffolder {
		fn scaffold(
			&self,
			_project_root: &Path,
			_domain_port: scaffolding_domain::domain_port_entity::DomainPort,
		) -> Result<(), scaffolding_domain::errors::ScaffoldingError> {
			*self.called.borrow_mut() = true;
			Ok(())
		}
	}

	#[test]
	fn execute_calls_scaffolder() {
		let mock = Rc::new(MockScaffolder::new());
		let usecase = CreateDomainPort::new(mock.clone());
		let input = CreateDomainPortInput::new(PathBuf::from("."), "test".into(), "auth".into(), "repo".into());
		let res = usecase.execute(input);
		assert!(res.is_ok());
		assert!(*mock.called.borrow());
	}

	#[test]
	fn invalid_bounded_context_returns_error_without_calling_scaffolder() {
		let mock = Rc::new(MockScaffolder::new());
		let usecase = CreateDomainPort::new(mock.clone());
		let input = CreateDomainPortInput::new(PathBuf::from("."), "Bad Context".into(), "auth".into(), "repo".into());
		let res = usecase.execute(input);
		assert!(matches!(res, Err(ScaffoldingError::InvalidBoundedContextName { .. })));
		assert!(!*mock.called.borrow());
	}
}
