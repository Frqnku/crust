use std::{path::PathBuf, rc::Rc};

use scaffolding_domain::{
	errors::ScaffoldingError,
	infrastructure_tech_entity::{InfrastructureTech, InfrastructureTechScaffolder},
};
use shared_domain::bounded_context_entity::BoundedContext;

pub struct CreateInfrastructureTechInput {
	pub project_root: PathBuf,
	pub bounded_context_name: String,
	pub infrastructure_tech_name: String,
}

impl CreateInfrastructureTechInput {
	pub fn new(
		project_root: PathBuf,
		bounded_context_name: String,
		infrastructure_tech_name: String,
	) -> Self {
		Self {
			project_root,
			bounded_context_name,
			infrastructure_tech_name,
		}
	}
}

pub struct CreateInfrastructureTech {
	scaffolder: Rc<dyn InfrastructureTechScaffolder>,
}

impl CreateInfrastructureTech {
	pub fn new(scaffolder: Rc<dyn InfrastructureTechScaffolder>) -> Self {
		Self { scaffolder }
	}

	pub fn execute(&self, input: CreateInfrastructureTechInput) -> Result<(), ScaffoldingError> {
		let bounded_context = BoundedContext::new(input.bounded_context_name)?;
		let infrastructure_tech = InfrastructureTech::new(bounded_context, input.infrastructure_tech_name)?;
		self.scaffolder
			.create_infrastructure_tech(&input.project_root, infrastructure_tech)
	}
}
