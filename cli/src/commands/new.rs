use workspace_application::use_cases::add_bounded_context::{
	AddBoundedContext,
	AddBoundedContextInput,
};
use workspace_infrastructure::fs_project_scaffolder::FsProjectScaffolder;

use crate::error::CliError;
use crate::helper::{project_name_from_root, resolve_project_root};

pub fn handle(bounded_context_name: String) -> Result<(), CliError> {
	let project_root = resolve_project_root()?;
	let project_name = project_name_from_root(&project_root);

	let mut scaffolder = FsProjectScaffolder;
	let mut use_case = AddBoundedContext::new(&mut scaffolder);
	let input = AddBoundedContextInput::new(
		project_name,
		project_root.clone(),
		bounded_context_name.clone(),
	);

	match use_case.execute(input) {
		Ok(()) => {
			println!(
				"Bounded context '{}' created at '{}'",
				bounded_context_name,
				project_root
					.join("crates")
					.join(&bounded_context_name)
					.display()
			);
			Ok(())
		}
		Err(error) => Err(CliError::OperationFailed(format!(
			"Error creating bounded context: {:?}",
			error
		))),
	}
}
