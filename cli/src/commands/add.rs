use crate::error::CliError;
use crate::helper::resolve_project_root;
use crate::parser::AddArgs;
use scaffolding_application::use_cases::create_artifact::{CreateArtifact, CreateArtifactInput};
use scaffolding_domain::value_object::ArtifactKind;

pub fn handle(create_artifact: &CreateArtifact, args: AddArgs) -> Result<(), CliError> {
	let project_root = resolve_project_root()?;
	let (kind, artifact_name) = match (args.query, args.command, args.infrastructure_tech, args.feature) {
		(Some(name), None, None, None) => (ArtifactKind::QueryUsecase, name),
		(None, Some(name), None, None) => (ArtifactKind::CommandUsecase, name),
		(None, None, Some(name), None) => (ArtifactKind::InfrastructureTech, name),
		(None, None, None, Some(name)) => (ArtifactKind::Domain, name),
		_ => {
			return Err(CliError::OperationFailed(
				"Exactly one flag is required: -q/--query, -c/--command, -t/--tech, or -f/--feature".to_string(),
			));
		}
	};

	let input = CreateArtifactInput::new(project_root, args.bounded_context, kind, artifact_name);

	create_artifact.execute(input).map_err(|error| {
		CliError::OperationFailed(format!("Error creating artifact: {error}"))
	})?;

	println!("Artifact created successfully");
	Ok(())
}