use clap::Parser;
use scaffolding_domain::value_object::ArtifactKind;

use crate::commands;
use crate::error::CliError;
use crate::parser::{Cli, Commands};

pub fn run() {
    if let Err(error) = run_inner() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run_inner() -> Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => commands::init::handle(args.name),
        Commands::New(args) => commands::new::handle(args.bounded_context),
        Commands::Add(args) => {
            let (kind, name) = match (args.query, args.command) {
                (Some(name), None) => (ArtifactKind::QueryUsecase, name),
                (None, Some(name)) => (ArtifactKind::CommandUsecase, name),
                _ => {
                    return Err(CliError::OperationFailed(
                        "Exactly one artifact flag is required: -q/--query or -c/--command"
                            .to_string(),
                    ));
                }
            };

            commands::add::handle(args.bounded_context, kind, name)
        }
    }
}