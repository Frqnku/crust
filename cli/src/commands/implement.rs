use crate::error::CliError;
use crate::helper::resolve_project_root;
use crate::parser::ImplArgs;
use scaffolding_application::use_cases::create_port_implementation::{
	CreatePortImplementation,
	CreatePortImplementationInput,
};

pub fn handle(
	create_port_implementation: &CreatePortImplementation,
	args: ImplArgs,
) -> Result<(), CliError> {
	let project_root = resolve_project_root()?;
	let input = CreatePortImplementationInput::new(
		project_root,
		args.bounded_context,
		args.feature_name,
		args.port_name,
		args.tech_name,
	);

	create_port_implementation.execute(input).map_err(|error| {
		CliError::OperationFailed(format!("Error creating port implementation: {error}"))
	})?;

	println!("Port implementation created successfully");
	Ok(())
}
