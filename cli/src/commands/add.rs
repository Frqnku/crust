use crate::error::CliError;
use crate::helper::resolve_project_root;
use crate::parser::{AddArgs, AddSubcommand};
use crate::output::{Presenter, Success};
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
			let input = CreateQueryUsecaseInput::new(project_root, add_args.bounded_context.clone(), add_args.name.clone());
			create_query_usecase.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating query usecase: {error}"))
			})?;
			
			let success = Success::new("Query usecase created successfully")
				.with_detail(format!("Name: {}", add_args.name))
				.with_detail(format!("Context: {}", add_args.bounded_context));
			Presenter::success(&success);
		}
		AddSubcommand::Command(add_args) => {
			let input = CreateCommandUsecaseInput::new(project_root, add_args.bounded_context.clone(), add_args.name.clone());
			create_command_usecase.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating command usecase: {error}"))
			})?;
			
			let success = Success::new("Command usecase created successfully")
				.with_detail(format!("Name: {}", add_args.name))
				.with_detail(format!("Context: {}", add_args.bounded_context));
			Presenter::success(&success);
		}
		AddSubcommand::Tech(add_args) => {
			let input = CreateInfrastructureTechInput::new(project_root, add_args.bounded_context.clone(), add_args.name.clone());
			create_infrastructure_tech.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating infrastructure tech: {error}"))
			})?;
			
			let success = Success::new("Infrastructure technology created successfully")
				.with_detail(format!("Tech: {}", add_args.name))
				.with_detail(format!("Context: {}", add_args.bounded_context));
			Presenter::success(&success);
		}
		AddSubcommand::Feature(add_args) => {
			let input = CreateDomainFeatureInput::new(project_root, add_args.bounded_context.clone(), add_args.name.clone());
			create_domain_feature.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating domain feature: {error}"))
			})?;
			
			let success = Success::new("Domain feature created successfully")
				.with_detail(format!("Feature: {}", add_args.name))
				.with_detail(format!("Context: {}", add_args.bounded_context));
			Presenter::success(&success);
		}
		AddSubcommand::Port(add_args) => {
			let tech_name = resolve_optional_tech(add_args.with_keyword, add_args.tech_name.clone())?;
			let input = CreateDomainPortWithImplementationInput::new(
				project_root,
				add_args.bounded_context.clone(),
				add_args.feature_name.clone(),
				add_args.port_name.clone(),
				tech_name.clone(),
			);
			create_domain_port_artifact.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating domain port artifact: {error}"))
			})?;
			
			let mut success = Success::new("Domain port created successfully")
				.with_detail(format!("Feature: {}", add_args.feature_name))
				.with_detail(format!("Port: {}", add_args.port_name))
				.with_detail(format!("Context: {}", add_args.bounded_context));
			
			if let Some(tech) = tech_name {
				success = success.with_detail(format!("With implementation: {}", tech));
			}
			
			Presenter::success(&success);
		}
	};

	Ok(())
}