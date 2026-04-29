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
	/// Create an infrastructure implementation for an existing domain port
	Impl(ImplArgs),
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
	/// Create domain port file: add port <feature> <port_name> in <context> [with <tech>]
	Port(AddPortTargetArgs),
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

	/// Optional keyword to trigger implementation scaffolding
	#[arg(value_parser = ["with"])]
	pub with_keyword: Option<String>,

	/// Optional infrastructure tech (e.g. postgres)
	pub tech_name: Option<String>,
}


#[derive(Debug, Args)]
pub struct ImplArgs {
	/// Domain feature folder name
	pub feature_name: String,

	/// Domain port file name
	pub port_name: String,

	/// Natural-language separator keyword
	#[arg(value_parser = ["for"])]
	pub for_keyword: String,

	/// Infrastructure tech folder name (e.g. postgre)
	pub tech_name: String,

	/// Natural-language separator keyword
	#[arg(value_parser = ["in"])]
	pub in_keyword: String,

	/// Target bounded context name
	pub bounded_context: String,
}
