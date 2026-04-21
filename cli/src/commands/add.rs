use crate::error::CliError;
use crate::helper::resolve_project_root;
use crate::parser::AddArgs;
use scaffolding_application::use_cases::create_artifact::{CreateArtifact, CreateArtifactInput};
use scaffolding_domain::value_object::ArtifactKind;

pub fn handle(create_artifact: &CreateArtifact, args: AddArgs) -> Result<(), CliError> {
	let (kind, artifact_name) = match (args.query, args.command) {
		(Some(name), None) => (ArtifactKind::QueryUsecase, name),
		(None, Some(name)) => (ArtifactKind::CommandUsecase, name),
		_ => {
			return Err(CliError::OperationFailed(
				"Exactly one artifact flag is required: -q/--query or -c/--command".to_string(),
			));
		}
	};

	let project_root = resolve_project_root()?;
	let input = CreateArtifactInput::new(project_root, args.bounded_context, kind, artifact_name);

	create_artifact.execute(input).map_err(|error| {
		CliError::OperationFailed(format!("Error creating artifact: {error}"))
	})?;

	println!("Artifact created successfully");
	Ok(())
}