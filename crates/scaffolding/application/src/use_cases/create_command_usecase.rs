use std::{path::PathBuf, rc::Rc};

use scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
	scaffolder_ports::CommandUsecaseScaffolder,
	value_object::ArtifactKind,
};
use shared_domain::bounded_context_entity::BoundedContext;

pub struct CreateCommandUsecaseInput {
	pub project_root: PathBuf,
	pub bounded_context_name: String,
	pub usecase_name: String,
}

impl CreateCommandUsecaseInput {
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

pub struct CreateCommandUsecase {
	scaffolder: Rc<dyn CommandUsecaseScaffolder>,
}

impl CreateCommandUsecase {
	pub fn new(scaffolder: Rc<dyn CommandUsecaseScaffolder>) -> Self {
		Self { scaffolder }
	}

	pub fn execute(&self, input: CreateCommandUsecaseInput) -> Result<(), ScaffoldingError> {
		let bounded_context = BoundedContext::new(input.bounded_context_name)?;
		let artifact = Artifact::new(bounded_context, ArtifactKind::CommandUsecase, input.usecase_name)?;
		self.scaffolder.scaffold(&input.project_root, artifact)
	}
}
