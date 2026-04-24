use std::{path::PathBuf, rc::Rc};

use scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
	scaffolder_ports::QueryUsecaseScaffolder,
	value_object::ArtifactKind,
};
use shared_domain::bounded_context_entity::BoundedContext;

pub struct CreateQueryUsecaseInput {
	pub project_root: PathBuf,
	pub bounded_context_name: String,
	pub usecase_name: String,
}

impl CreateQueryUsecaseInput {
	pub fn new(
		project_root: PathBuf,
		bounded_context_name: String,
		usecase_name: String,
	) -> Self {
		Self {
			project_root,
			bounded_context_name,
			usecase_name,
		}
	}
}

pub struct CreateQueryUsecase {
	scaffolder: Rc<dyn QueryUsecaseScaffolder>,
}

impl CreateQueryUsecase {
	pub fn new(scaffolder: Rc<dyn QueryUsecaseScaffolder>) -> Self {
		Self { scaffolder }
	}

	pub fn execute(&self, input: CreateQueryUsecaseInput) -> Result<(), ScaffoldingError> {
		let bounded_context = BoundedContext::new(input.bounded_context_name)?;
		let artifact = Artifact::new(bounded_context, ArtifactKind::QueryUsecase, input.usecase_name)?;
		self.scaffolder.scaffold(&input.project_root, artifact)
	}
}
