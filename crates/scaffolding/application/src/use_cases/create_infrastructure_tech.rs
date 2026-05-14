use std::{path::PathBuf, rc::Rc};

use crust_scaffolding_domain::{
	artifact_entity::Artifact,
	errors::ScaffoldingError,
	scaffolder_ports::InfrastructureTechScaffolder,
	value_object::ArtifactKind,
};
use crust_shared_domain::bounded_context_entity::BoundedContext;

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

	impl InfrastructureTechScaffolder for MockScaffolder {
		fn scaffold(
			&self,
			_project_root: &Path,
			_artifact: crust_scaffolding_domain::artifact_entity::Artifact,
		) -> Result<(), crust_scaffolding_domain::errors::ScaffoldingError> {
			*self.called.borrow_mut() = true;
			Ok(())
		}
	}

	#[test]
	fn execute_calls_scaffolder() {
		let mock = Rc::new(MockScaffolder::new());
		let usecase = CreateInfrastructureTech::new(mock.clone());
		let input = CreateInfrastructureTechInput::new(PathBuf::from("."), "test".into(), "psql".into());
		let res = usecase.execute(input);
		assert!(res.is_ok());
		assert!(*mock.called.borrow());
	}

	#[test]
	fn invalid_bounded_context_returns_error_without_calling_scaffolder() {
		let mock = Rc::new(MockScaffolder::new());
		let usecase = CreateInfrastructureTech::new(mock.clone());
		let input = CreateInfrastructureTechInput::new(PathBuf::from("."), "Bad Context".into(), "psql".into());
		let res = usecase.execute(input);
		assert!(matches!(res, Err(ScaffoldingError::InvalidBoundedContextName { .. })));
		assert!(!*mock.called.borrow());
	}
}
