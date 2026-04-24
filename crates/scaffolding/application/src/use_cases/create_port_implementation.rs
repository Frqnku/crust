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
	pub feature_name: String,
	pub port_name: String,
	pub tech_name: String,
}

impl CreatePortImplementationInput {
	pub fn new(
		project_root: PathBuf,
		bounded_context_name: String,
		feature_name: String,
		port_name: String,
		tech_name: String,
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
			input.feature_name,
			input.port_name,
			input.tech_name,
		)?;
		self.scaffolder.scaffold(&input.project_root, implementation)
	}
}
