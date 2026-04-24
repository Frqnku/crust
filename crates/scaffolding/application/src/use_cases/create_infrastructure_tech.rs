use std::{path::PathBuf, rc::Rc};

use scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
	scaffolder_ports::InfrastructureTechScaffolder,
	value_object::ArtifactKind,
};
use shared_domain::bounded_context_entity::BoundedContext;

pub struct CreateInfrastructureTechInput {
	pub project_root: PathBuf,
	pub bounded_context_name: String,
	pub tech_name: String,
}

impl CreateInfrastructureTechInput {
	pub fn new(
		project_root: PathBuf,
		bounded_context_name: String,
		tech_name: String,
	) -> Self {
		Self {
			project_root,
			bounded_context_name,
			tech_name,
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
		let artifact = Artifact::new(bounded_context, ArtifactKind::InfrastructureTech, input.tech_name)?;
		self.scaffolder.scaffold(&input.project_root, artifact)
	}
}
