use crate::error::CliError;
use crate::helper::resolve_project_root;
use crate::parser::{AddArgs, AddInSubcommand, AddSubcommand};
use scaffolding_application::use_cases::{
	create_domain_feature::{CreateDomainFeature, CreateDomainFeatureInput},
	create_domain_port_with_implementation::{CreateDomainPortWithImplementation, CreateDomainPortWithImplementationInput},
	create_infrastructure_tech::{CreateInfrastructureTech, CreateInfrastructureTechInput},
	create_command_usecase::{CreateCommandUsecase, CreateCommandUsecaseInput},
	create_query_usecase::{CreateQueryUsecase, CreateQueryUsecaseInput},
};

fn resolve_optional_tech(with_keyword: Option<String>, tech_name: Option<String>) -> Result<Option<String>, CliError> {
	match (with_keyword, tech_name) {
		(None, None) => Ok(None),
		(Some(_), Some(tech)) => Ok(Some(tech)),
		_ => Err(CliError::OperationFailed(
			"Use port syntax as '<feature> <port> in <context>' or '<feature> <port> in <context> with <tech>'"
				.to_string(),
		)),
	}
}

fn parse_port_shortcut(tokens: Vec<String>) -> Result<(String, String, String, Option<String>), CliError> {
	match tokens.as_slice() {
		[feature_name, port_name, in_keyword, bounded_context] if in_keyword == "in" => Ok((
			feature_name.clone(),
			port_name.clone(),
			bounded_context.clone(),
			None,
		)),
		[feature_name, port_name, in_keyword, bounded_context, with_keyword, tech_name]
			if in_keyword == "in" && with_keyword == "with" =>
		{
			Ok((
				feature_name.clone(),
				port_name.clone(),
				bounded_context.clone(),
				Some(tech_name.clone()),
			))
		}
		_ => Err(CliError::OperationFailed(
			"Use port shortcut as 'add <feature> <port> in <context>' or 'add <feature> <port> in <context> with <tech>'"
				.to_string(),
		)),
	}
}

pub fn handle(
	create_domain_feature: &CreateDomainFeature,
	create_domain_port_artifact: &CreateDomainPortWithImplementation,
	create_infrastructure_tech: &CreateInfrastructureTech,
	create_command_usecase: &CreateCommandUsecase,
	create_query_usecase: &CreateQueryUsecase,
	args: AddArgs,
) -> Result<(), CliError> {
	let project_root = resolve_project_root()?;

	match args.kind {
		AddSubcommand::Query(add_args) => {
			let input = CreateQueryUsecaseInput::new(project_root, add_args.bounded_context, add_args.name);
			create_query_usecase.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating query usecase: {error}"))
			})?;
		}
		AddSubcommand::Command(add_args) => {
			let input = CreateCommandUsecaseInput::new(project_root, add_args.bounded_context, add_args.name);
			create_command_usecase.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating command usecase: {error}"))
			})?;
		}
		AddSubcommand::Tech(add_args) => {
			let input = CreateInfrastructureTechInput::new(project_root, add_args.bounded_context, add_args.name);
			create_infrastructure_tech.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating infrastructure tech: {error}"))
			})?;
		}
		AddSubcommand::Feature(add_args) => {
			let input = CreateDomainFeatureInput::new(project_root, add_args.bounded_context, add_args.name);
			create_domain_feature.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating domain feature: {error}"))
			})?;
		}
		AddSubcommand::Port(add_args) => {
			let tech_name = resolve_optional_tech(add_args.with_keyword, add_args.tech_name)?;
			let input = CreateDomainPortWithImplementationInput::new(
				project_root,
				add_args.bounded_context,
				add_args.feature_name,
				add_args.port_name,
				tech_name,
			);
			create_domain_port_artifact.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating domain port artifact: {error}"))
			})?;
		}
		AddSubcommand::In(in_args) => match in_args.kind {
			AddInSubcommand::Query(name_args) => {
				let input = CreateQueryUsecaseInput::new(project_root, in_args.bounded_context, name_args.name);
				create_query_usecase.execute(input).map_err(|error| {
					CliError::OperationFailed(format!("Error creating query usecase: {error}"))
				})?;
			}
			AddInSubcommand::Command(name_args) => {
				let input = CreateCommandUsecaseInput::new(project_root, in_args.bounded_context, name_args.name);
				create_command_usecase.execute(input).map_err(|error| {
					CliError::OperationFailed(format!("Error creating command usecase: {error}"))
				})?;
			}
			AddInSubcommand::Tech(name_args) => {
				let input = CreateInfrastructureTechInput::new(project_root, in_args.bounded_context, name_args.name);
				create_infrastructure_tech.execute(input).map_err(|error| {
					CliError::OperationFailed(format!("Error creating infrastructure tech: {error}"))
				})?;
			}
			AddInSubcommand::Feature(name_args) => {
				let input = CreateDomainFeatureInput::new(project_root, in_args.bounded_context, name_args.name);
				create_domain_feature.execute(input).map_err(|error| {
					CliError::OperationFailed(format!("Error creating domain feature: {error}"))
				})?;
			}
			AddInSubcommand::Port(name_args) => {
				let tech_name = resolve_optional_tech(name_args.with_keyword, name_args.tech_name)?;
				let input = CreateDomainPortWithImplementationInput::new(
					project_root,
					in_args.bounded_context,
					name_args.feature_name,
					name_args.port_name,
					tech_name,
				);
				create_domain_port_artifact.execute(input).map_err(|error| {
					CliError::OperationFailed(format!("Error creating domain port artifact: {error}"))
				})?;
			}
		},
		AddSubcommand::PortShortcut(tokens) => {
			let (feature_name, port_name, bounded_context, tech_name) = parse_port_shortcut(tokens)?;
			let input = CreateDomainPortWithImplementationInput::new(
				project_root,
				bounded_context,
				feature_name,
				port_name,
				tech_name,
			);
			create_domain_port_artifact.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating domain port artifact: {error}"))
			})?;
		}
	};

	println!("Artifact created successfully");
	Ok(())
}