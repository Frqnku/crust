use clap::Parser;
use scaffolding_application::use_cases::create_artifact::CreateArtifact;
use workspace_application::use_cases::add_bounded_context::AddBoundedContext;
use workspace_application::use_cases::initialize_project::InitializeProject;

use crate::commands;
use crate::error::CliError;
use crate::parser::{Cli, Commands};

pub struct App {
	initialize_project: InitializeProject,
	add_bounded_context: AddBoundedContext,
	create_artifact: CreateArtifact,
}

impl App {
	pub fn new(
		initialize_project: InitializeProject,
		add_bounded_context: AddBoundedContext,
		create_artifact: CreateArtifact,
	) -> Self {
		Self {
			initialize_project,
			add_bounded_context,
			create_artifact,
		}
	}

	pub fn run(&self) -> Result<(), CliError> {
		let cli = Cli::parse();

		match cli.command {
			Commands::Init(args) => commands::init::handle(&self.initialize_project, args),
			Commands::New(args) => commands::new::handle(&self.add_bounded_context, args),
			Commands::Add(args) => commands::add::handle(&self.create_artifact, args),
		}
	}
}