use crate::error::CliError;
use crate::helper::resolve_project_root;
use crate::parser::{AddArgs, AddSubcommand};
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
			"Use port syntax as 'add port <feature> <port> in <context>' or 'add port <feature> <port> in <context> with <tech>'"
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
	};

	println!("Artifact created successfully");
	Ok(())
}