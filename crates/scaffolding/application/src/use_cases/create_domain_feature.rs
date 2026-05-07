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
	pub domain_feature_name: String,
}

impl CreateDomainFeatureInput {
	pub fn new(
		project_root: PathBuf,
		bounded_context_name: String,
		domain_feature_name: String,
	) -> Self {
		Self {
			project_root,
			bounded_context_name,
			domain_feature_name,
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
		let artifact = Artifact::new(bounded_context, ArtifactKind::Domain, input.domain_feature_name)?;
		self.scaffolder.scaffold(&input.project_root, artifact)
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

	impl DomainFeatureScaffolder for MockScaffolder {
		fn scaffold(
			&self,
			_project_root: &Path,
			_artifact: scaffolding_domain::artifact_entity::Artifact,
		) -> Result<(), scaffolding_domain::errors::ScaffoldingError> {
			*self.called.borrow_mut() = true;
			Ok(())
		}
	}

	#[test]
	fn execute_calls_scaffolder() {
		let mock = Rc::new(MockScaffolder::new());
		let usecase = CreateDomainFeature::new(mock.clone());
		let input = CreateDomainFeatureInput::new(PathBuf::from("."), "test".into(), "auth".into());
		let res = usecase.execute(input);
		assert!(res.is_ok());
		assert!(*mock.called.borrow());
	}

	#[test]
	fn invalid_bounded_context_returns_error_without_calling_scaffolder() {
		let mock = Rc::new(MockScaffolder::new());
		let usecase = CreateDomainFeature::new(mock.clone());
		let input = CreateDomainFeatureInput::new(PathBuf::from("."), "Bad Context".into(), "auth".into());
		let res = usecase.execute(input);
		assert!(matches!(res, Err(ScaffoldingError::InvalidBoundedContextName { .. })));
		assert!(!*mock.called.borrow());
	}
}
