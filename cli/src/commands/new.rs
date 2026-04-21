use crate::error::CliError;
use crate::helper::{project_name_from_root, resolve_project_root};
use crate::parser::NewArgs;
use workspace_application::use_cases::add_bounded_context::{AddBoundedContext, AddBoundedContextInput};

pub fn handle(add_bounded_context: &AddBoundedContext, args: NewArgs) -> Result<(), CliError> {
	let project_root = resolve_project_root()?;
	let project_name = project_name_from_root(&project_root);
	let input = AddBoundedContextInput::new(
		project_name,
		project_root.clone(),
		args.bounded_context.clone(),
	);

	add_bounded_context.execute(input).map_err(|error| {
		CliError::OperationFailed(format!("Error creating bounded context: {error}"))
	})?;

	println!(
		"Bounded context '{}' created at '{}'",
		args.bounded_context,
		project_root.join("crates").join(&args.bounded_context).display()
	);
	Ok(())
}
