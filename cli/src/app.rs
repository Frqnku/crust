use clap::Parser;
use scaffolding_application::use_cases::{
    create_domain_feature::CreateDomainFeature,
	create_domain_port::CreateDomainPort,
    create_infrastructure_tech::CreateInfrastructureTech,
    create_command_usecase::CreateCommandUsecase,
	create_port_implementation::CreatePortImplementation,
    create_query_usecase::CreateQueryUsecase,
};
use workspace_application::use_cases::add_bounded_context::AddBoundedContext;
use workspace_application::use_cases::initialize_project::InitializeProject;

use crate::commands;
use crate::error::CliError;
use crate::parser::{Cli, Commands};

pub struct App {
	initialize_project: InitializeProject,
	add_bounded_context: AddBoundedContext,
	create_domain_feature: CreateDomainFeature,
	create_domain_port: CreateDomainPort,
	create_infrastructure_tech: CreateInfrastructureTech,
	create_command_usecase: CreateCommandUsecase,
	create_query_usecase: CreateQueryUsecase,
	create_port_implementation: CreatePortImplementation,
}

impl App {
	pub fn new(
		initialize_project: InitializeProject,
		add_bounded_context: AddBoundedContext,
		create_domain_feature: CreateDomainFeature,
		create_domain_port: CreateDomainPort,
		create_infrastructure_tech: CreateInfrastructureTech,
		create_command_usecase: CreateCommandUsecase,
		create_query_usecase: CreateQueryUsecase,
		create_port_implementation: CreatePortImplementation,
	) -> Self {
		Self {
			initialize_project,
			add_bounded_context,
			create_domain_feature,
			create_domain_port,
			create_infrastructure_tech,
			create_command_usecase,
			create_query_usecase,
			create_port_implementation,
		}
	}

	pub fn run(&self) -> Result<(), CliError> {
		let cli = Cli::parse();

		match cli.command {
			Commands::Init(args) => commands::init::handle(&self.initialize_project, args),
			Commands::New(args) => commands::new::handle(&self.add_bounded_context, args),
			Commands::Add(args) => commands::add::handle(
				&self.create_domain_feature,
				&self.create_domain_port,
				&self.create_infrastructure_tech,
				&self.create_command_usecase,
				&self.create_query_usecase,
				&self.create_port_implementation,
				args,
			),
			Commands::Impl(args) => commands::implement::handle(
				&self.create_port_implementation,
				args,
			),
		}
	}
}