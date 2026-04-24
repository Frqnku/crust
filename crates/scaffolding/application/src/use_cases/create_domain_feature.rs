use std::{path::PathBuf, rc::Rc};

use scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
	scaffolder_ports::DomainFeatureScaffolder,
	value_object::ArtifactKind,
};
use shared_domain::bounded_context_entity::BoundedContext;

pub struct CreateDomainFeatureInput {
	pub project_root: PathBuf,
	pub bounded_context_name: String,
	pub feature_name: String,
}

impl CreateDomainFeatureInput {
	pub fn new(
		project_root: PathBuf,
		bounded_context_name: String,
		feature_name: String,
	) -> Self {
		Self {
			project_root,
			bounded_context_name,
			feature_name,
		}
	}
}

pub struct CreateDomainFeature {
	scaffolder: Rc<dyn DomainFeatureScaffolder>,
}

impl CreateDomainFeature {
	pub fn new(scaffolder: Rc<dyn DomainFeatureScaffolder>) -> Self {
		Self { scaffolder }
	}

	pub fn execute(&self, input: CreateDomainFeatureInput) -> Result<(), ScaffoldingError> {
		let bounded_context = BoundedContext::new(input.bounded_context_name)?;
		let artifact = Artifact::new(bounded_context, ArtifactKind::Domain, input.feature_name)?;
		self.scaffolder.scaffold(&input.project_root, artifact)
	}
}
