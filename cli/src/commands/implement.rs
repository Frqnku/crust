use crate::error::CliError;
use crate::helper::resolve_project_root;
use crate::parser::ImplArgs;
use crate::output::{Presenter, Success};
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
		args.bounded_context.clone(),
		args.feature_name.clone(),
		args.port_name.clone(),
		args.tech_name.clone(),
	);

	create_port_implementation.execute(input).map_err(|error| {
		CliError::OperationFailed(format!("Error creating port implementation: {error}"))
	})?;

	let success = Success::new("Port implementation created successfully")
		.with_detail(format!("Feature: {}", args.feature_name))
		.with_detail(format!("Port: {}", args.port_name))
		.with_detail(format!("Technology: {}", args.tech_name))
		.with_detail(format!("Context: {}", args.bounded_context));
	
	Presenter::success(&success);
	Ok(())
}
