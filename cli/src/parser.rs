use clap::{Args, Parser, Subcommand};

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
pub struct AddArgs {
	#[command(subcommand)]
	pub kind: AddSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AddSubcommand {
	/// Create a query use-case artifact: add query <name> in <context>
	Query(AddTargetArgs),
	/// Create a command use-case artifact: add command <name> in <context>
	Command(AddTargetArgs),
	/// Create infrastructure technology folder: add tech <name> in <context>
	Tech(AddTargetArgs),
	/// Create domain feature folder: add feature <name> in <context>
	Feature(AddTargetArgs),
	/// Create domain port file: add port <feature> <port_name> in <context>
	Port(AddPortTargetArgs),
	/// Add an artifact with the canonical syntax: add in <context> <kind> <name>
	In(AddInArgs),
}

#[derive(Debug, Args)]
pub struct AddTargetArgs {
	/// Artifact/feature/tech name to create
	pub name: String,

	/// Natural-language separator keyword
	#[arg(value_parser = ["in"])]
	pub in_keyword: String,

	/// Target bounded context name
	pub bounded_context: String,
}

#[derive(Debug, Args)]
pub struct AddPortTargetArgs {
	/// Domain feature folder name
	pub feature_name: String,

	/// Domain port file name
	pub port_name: String,

	/// Natural-language separator keyword
	#[arg(value_parser = ["in"])]
	pub in_keyword: String,

	/// Target bounded context name
	pub bounded_context: String,
}

#[derive(Debug, Args)]
pub struct AddInArgs {
	/// Target bounded context name
	pub bounded_context: String,

	#[command(subcommand)]
	pub kind: AddInSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AddInSubcommand {
	/// Create a query use-case artifact
	Query(AddNameArgs),
	/// Create a command use-case artifact
	Command(AddNameArgs),
	/// Create infrastructure technology folder (e.g. kafka, postgre)
	Tech(AddNameArgs),
	/// Create domain feature folder inside the bounded context domain layer
	Feature(AddNameArgs),
	/// Create domain port file inside a domain feature folder
	Port(AddPortNameArgs),
}

#[derive(Debug, Args)]
pub struct AddNameArgs {
	/// Artifact/feature/tech name to create
	pub name: String,
}

#[derive(Debug, Args)]
pub struct AddPortNameArgs {
	/// Domain feature folder name
	pub feature_name: String,

	/// Domain port file name
	pub port_name: String,
}
