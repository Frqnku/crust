use crate::error::CliError;
use crate::helper::resolve_project_root;
use crate::parser::{AddArgs, AddInSubcommand, AddSubcommand, AddTargetArgs};
use scaffolding_application::use_cases::create_artifact::{CreateArtifact, CreateArtifactInput};
use scaffolding_domain::value_object::ArtifactKind;

fn resolve_context(args: AddTargetArgs) -> Result<(String, String), CliError> {
	match (args.bounded_context, args.in_keyword, args.bounded_context_after_in) {
		(Some(context), None, None) => Ok((context, args.name)),
		(None, Some(in_keyword), Some(context)) if in_keyword == "in" => Ok((context, args.name)),
		_ => Err(CliError::OperationFailed(
			"Use either '<name> -c <context>' or '<name> in <context>'".to_string(),
		)),
	}
}

pub fn handle(create_artifact: &CreateArtifact, args: AddArgs) -> Result<(), CliError> {
	let project_root = resolve_project_root()?;
	let (kind, bounded_context, artifact_name) = match args.kind {
		AddSubcommand::Query(add_args) => {
			let (context, name) = resolve_context(add_args)?;
			(ArtifactKind::QueryUsecase, context, name)
		}
		AddSubcommand::Command(add_args) => {
			let (context, name) = resolve_context(add_args)?;
			(ArtifactKind::CommandUsecase, context, name)
		}
		AddSubcommand::Tech(add_args) => {
			let (context, name) = resolve_context(add_args)?;
			(ArtifactKind::InfrastructureTech, context, name)
		}
		AddSubcommand::Feature(add_args) => {
			let (context, name) = resolve_context(add_args)?;
			(ArtifactKind::Domain, context, name)
		}
		AddSubcommand::In(in_args) => match in_args.kind {
			AddInSubcommand::Query(name_args) => {
				(ArtifactKind::QueryUsecase, in_args.bounded_context, name_args.name)
			}
			AddInSubcommand::Command(name_args) => {
				(ArtifactKind::CommandUsecase, in_args.bounded_context, name_args.name)
			}
			AddInSubcommand::Tech(name_args) => {
				(ArtifactKind::InfrastructureTech, in_args.bounded_context, name_args.name)
			}
			AddInSubcommand::Feature(name_args) => {
				(ArtifactKind::Domain, in_args.bounded_context, name_args.name)
			}
		},
	};

	let input = CreateArtifactInput::new(project_root, bounded_context, kind, artifact_name);

	create_artifact.execute(input).map_err(|error| {
		CliError::OperationFailed(format!("Error creating artifact: {error}"))
	})?;

	println!("Artifact created successfully");
	Ok(())
}