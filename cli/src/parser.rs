use clap::{ArgGroup, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "crust")]
#[command(about = "CLI for crust workspace and scaffolding operations")]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	/// Initialize a project in the current directory or inside a new folder
	Init(InitArgs),
	/// Add a new bounded context to the current project
	New(NewArgs),
	/// Add an artifact to a bounded context
	Add(AddArgs),
}

#[derive(Debug, Args)]
pub struct InitArgs {
	/// Optional project name. If omitted, uses the current directory name.
	pub name: Option<String>,
}

#[derive(Debug, Args)]
pub struct NewArgs {
	/// Bounded context name to create
	pub bounded_context: String,
}

#[derive(Debug, Args)]
#[command(group(
	ArgGroup::new("artifact_type")
		.required(true)
		.args(["query", "command", "infrastructure_tech"])
))]
pub struct AddArgs {
	/// Target bounded context name
	pub bounded_context: String,

	/// Create a query use-case artifact
	#[arg(short = 'q', long = "query")]
	pub query: Option<String>,

	/// Create a command use-case artifact
	#[arg(short = 'c', long = "command")]
	pub command: Option<String>,

	/// Create infrastructure technology folder (e.g. kafka, postgre)
	#[arg(short = 't', long = "tech")]
	pub infrastructure_tech: Option<String>,
}
