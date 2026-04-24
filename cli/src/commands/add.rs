use crate::error::CliError;
use crate::helper::resolve_project_root;
use crate::parser::{AddArgs, AddInSubcommand, AddSubcommand};
use scaffolding_application::use_cases::{
	create_domain_feature::{CreateDomainFeature, CreateDomainFeatureInput},
	create_domain_port::{CreateDomainPort, CreateDomainPortInput},
	create_infrastructure_tech::{CreateInfrastructureTech, CreateInfrastructureTechInput},
	create_command_usecase::{CreateCommandUsecase, CreateCommandUsecaseInput},
	create_query_usecase::{CreateQueryUsecase, CreateQueryUsecaseInput},
};

pub fn handle(
	create_domain_feature: &CreateDomainFeature,
	create_domain_port: &CreateDomainPort,
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
			let input = CreateDomainPortInput::new(
				project_root,
				add_args.bounded_context,
				add_args.feature_name,
				add_args.port_name,
			);
			create_domain_port.execute(input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating domain port: {error}"))
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
				let input = CreateDomainPortInput::new(
					project_root,
					in_args.bounded_context,
					name_args.feature_name,
					name_args.port_name,
				);
				create_domain_port.execute(input).map_err(|error| {
					CliError::OperationFailed(format!("Error creating domain port: {error}"))
				})?;
			}
		},
	};

	println!("Artifact created successfully");
	Ok(())
}