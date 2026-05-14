use crate::error::CliError;
use crate::helper::current_dir;
use crate::parser::InitArgs;
use crate::output::{Presenter, Success};
use crust_workspace_application::use_cases::initialize_project::{InitializeProject, InitializeProjectInput};

pub fn handle(initialize_project: &InitializeProject, args: InitArgs) -> Result<(), CliError> {
	let current_dir = current_dir()?;

	let (name, path) = match args.name {
		Some(name) => {
			let path = current_dir.join(&name);
			(name, path)
		}
		None => {
			let name = current_dir
				.file_name()
				.and_then(|n| n.to_str())
				.map_or_else(|| "project".to_string(), |n| n.to_string());
			(name, current_dir)
		}
	};

	let input = InitializeProjectInput::new(name.clone(), path.clone());
	let project = initialize_project.execute(input).map_err(|error| {
		CliError::OperationFailed(format!("Error initializing project: {error}"))
	})?;

	let success = Success::new("Project initialized successfully")
		.with_detail(format!("Name: {}", project.name()))
		.with_detail(format!("Location: {}", project.path().display()));
	
	Presenter::success(&success);
	Ok(())
}