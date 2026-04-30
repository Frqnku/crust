use std::{path::PathBuf, rc::Rc};

use scaffolding_domain::{
	domain_port_entity::DomainPort,
	errors::ScaffoldingError,
	port_implementation_entity::PortImplementation,
	scaffolder_ports::{DomainPortScaffolder, PortImplementationScaffolder},
};
use shared_domain::bounded_context_entity::BoundedContext;

pub struct CreateDomainPortWithImplementationInput {
	pub project_root: PathBuf,
	pub bounded_context_name: String,
	pub feature_name: String,
	pub port_name: String,
	pub tech_name: Option<String>,
}

impl CreateDomainPortWithImplementationInput {
	pub fn new(
		project_root: PathBuf,
		bounded_context_name: String,
		feature_name: String,
		port_name: String,
		tech_name: Option<String>,
	) -> Self {
		Self {
			project_root,
			bounded_context_name,
			feature_name,
			port_name,
			tech_name,
		}
	}
}

pub struct CreateDomainPortWithImplementation {
	domain_port_scaffolder: Rc<dyn DomainPortScaffolder>,
	port_implementation_scaffolder: Rc<dyn PortImplementationScaffolder>,
}

impl CreateDomainPortWithImplementation {
	pub fn new(
		domain_port_scaffolder: Rc<dyn DomainPortScaffolder>,
		port_implementation_scaffolder: Rc<dyn PortImplementationScaffolder>,
	) -> Self {
		Self {
			domain_port_scaffolder,
			port_implementation_scaffolder,
		}
	}

	pub fn execute(&self, input: CreateDomainPortWithImplementationInput) -> Result<(), ScaffoldingError> {
		let bounded_context = BoundedContext::new(input.bounded_context_name)?;
		let domain_port = DomainPort::new(
			bounded_context.clone(),
			input.feature_name.clone(),
			input.port_name.clone(),
		)?;

		// If an implementation is requested, build the implementation object and validate BEFORE creating the port
		let implementation = if let Some(ref tech_name) = input.tech_name {
			let impl_obj = PortImplementation::new(
				bounded_context.clone(),
				input.feature_name.clone(),
				input.port_name.clone(),
				tech_name.clone(),
			)?;
			// Check that the tech infrastructure directory exists before proceeding
			// This prevents creating an orphaned port if the tech doesn't exist
			let impl_directory = impl_obj.implementation_directory(&input.project_root);
			if !impl_directory.is_dir() {
				return Err(ScaffoldingError::BoundedContextNotFound {
					name: format!(
						"{}/infrastructure/{}",
						impl_obj.bounded_context().name().as_str(),
						impl_obj.tech_name().as_str()
					),
				});
			}
			Some(impl_obj)
		} else {
			None
		};

		// Now safe to create the domain port (all validations passed)
		self.domain_port_scaffolder
			.scaffold(&input.project_root, domain_port)?;

		// Then create implementation if requested
		if let Some(impl_obj) = implementation {
			self.port_implementation_scaffolder
				.scaffold(&input.project_root, impl_obj)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::{cell::RefCell, path::Path, rc::Rc};

	struct MockDomain {
		called: RefCell<bool>,
	}

	impl MockDomain {
		fn new() -> Self {
			Self { called: RefCell::new(false) }
		}
	}

	impl DomainPortScaffolder for MockDomain {
		fn scaffold(
			&self,
			_project_root: &Path,
			_domain_port: scaffolding_domain::domain_port_entity::DomainPort,
		) -> Result<(), scaffolding_domain::errors::ScaffoldingError> {
			*self.called.borrow_mut() = true;
			Ok(())
		}
	}

	struct MockImpl {
		called: RefCell<bool>,
	}

	impl MockImpl {
		fn new() -> Self {
			Self { called: RefCell::new(false) }
		}
	}

	impl PortImplementationScaffolder for MockImpl {
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
	fn execute_creates_port_only_when_no_tech_is_provided() {
		let domain = Rc::new(MockDomain::new());
		let implementation = Rc::new(MockImpl::new());
		let usecase = CreateDomainPortWithImplementation::new(domain.clone(), implementation.clone());
		let input = CreateDomainPortWithImplementationInput::new(PathBuf::from("."), "test".into(), "auth".into(), "repo".into(), None);
		let res = usecase.execute(input);
		assert!(res.is_ok());
		assert!(*domain.called.borrow());
		assert!(!*implementation.called.borrow());
	}

	#[test]
	fn missing_tech_directory_returns_error_without_calling_scaffolders() {
		let domain = Rc::new(MockDomain::new());
		let implementation = Rc::new(MockImpl::new());
		let usecase = CreateDomainPortWithImplementation::new(domain.clone(), implementation.clone());
		let input = CreateDomainPortWithImplementationInput::new(PathBuf::from("."), "test".into(), "auth".into(), "repo".into(), Some("postgres".into()));
		let res = usecase.execute(input);
		assert!(matches!(res, Err(ScaffoldingError::BoundedContextNotFound { .. })));
		assert!(!*domain.called.borrow());
		assert!(!*implementation.called.borrow());
	}
}