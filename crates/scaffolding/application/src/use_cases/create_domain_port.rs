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
