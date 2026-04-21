use std::{path::PathBuf, rc::Rc};

use scaffolding_domain::{
	artifact_entity::{Artifact, ArtifactScaffolder},
	errors::ScaffoldingError,
	value_object::ArtifactKind,
};
use shared_domain::bounded_context_entity::BoundedContext;

pub struct CreateArtifactInput {
	pub project_root: PathBuf,
	pub bounded_context_name: String,
	pub kind: ArtifactKind,
	pub artifact_name: String,
}

impl CreateArtifactInput {
	pub fn new(
		project_root: PathBuf,
		bounded_context_name: String,
		kind: ArtifactKind,
		artifact_name: String,
	) -> Self {
		Self {
			project_root,
			bounded_context_name,
			kind,
			artifact_name,
		}
	}
}

pub struct CreateArtifact {
	scaffolder: Rc<dyn ArtifactScaffolder>,
}

impl CreateArtifact {
	pub fn new(scaffolder: Rc<dyn ArtifactScaffolder>) -> Self {
		Self { scaffolder }
	}

	pub fn execute(&self, input: CreateArtifactInput) -> Result<(), ScaffoldingError> {
		let bounded_context = BoundedContext::new(input.bounded_context_name)?;
		let artifact = Artifact::new(bounded_context, input.kind, input.artifact_name)?;
		self.scaffolder.create_artifact(&input.project_root, artifact)
	}
}
