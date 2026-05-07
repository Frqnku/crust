use crate::error::CliError;
use crate::helper::resolve_project_root;
use crate::parser::{AddArgs, AddSubcommand};
use crate::output::{Presenter, Success};
use colored::Colorize;
use scaffolding_application::use_cases::{
	create_domain_feature::{CreateDomainFeature, CreateDomainFeatureInput},
	create_domain_port::{CreateDomainPort, CreateDomainPortInput},
	create_infrastructure_tech::{CreateInfrastructureTech, CreateInfrastructureTechInput},
	create_command_usecase::{CreateCommandUsecase, CreateCommandUsecaseInput},
	create_port_implementation::{CreatePortImplementation, CreatePortImplementationInput},
	create_query_usecase::{CreateQueryUsecase, CreateQueryUsecaseInput},
};

/// Parse batch target args: extracts names and bounded context from args like ["user", "token", "in", "auth"]
fn parse_multiple_target_args(args: Vec<String>) -> Result<(Vec<String>, String), CliError> {
	// Find the "in" keyword
	let in_index = args
		.iter()
		.position(|arg| arg == "in")
		.ok_or_else(|| CliError::OperationFailed("Missing 'in' keyword in command".to_string()))?;

	// Names are everything before "in"
	if in_index == 0 {
		return Err(CliError::OperationFailed(
			"Expected at least one name before 'in'".to_string(),
		));
	}

	let names = args[..in_index].to_vec();

	// Context is everything after "in" (should be exactly one element)
	let context_args = &args[in_index + 1..];
	if context_args.len() != 1 {
		return Err(CliError::OperationFailed(
			"Expected exactly one context name after 'in'".to_string(),
		));
	}

	Ok((names, context_args[0].clone()))
}

fn resolve_optional_tech(with_keyword: Option<String>, tech_name: Option<String>) -> Result<Option<String>, CliError> {
	match (with_keyword, tech_name) {
		(None, None) => Ok(None),
		(Some(_), Some(tech)) => Ok(Some(tech)),
		_ => Err(CliError::OperationFailed(
			"Use port syntax as 'add port <domain> <port> in <context>' or 'add port <domain> <port> in <context> with <tech>'"
				.to_string(),
		)),
	}
}

/// Track success/failure for batch operations
struct BatchResult {
	total: usize,
	succeeded: Vec<String>,
	failed: Vec<(String, String)>,
}

impl BatchResult {
	fn new() -> Self {
		Self {
			total: 0,
			succeeded: Vec::new(),
			failed: Vec::new(),
		}
	}

	fn success(&mut self, name: String) {
		self.succeeded.push(name);
		self.total += 1;
	}

	fn failure(&mut self, name: String, error: String) {
		self.failed.push((name, error));
		self.total += 1;
	}
}

fn execute_batch<F>(names: Vec<String>, mut execute_one: F) -> BatchResult
where
	F: FnMut(String) -> Result<(), String>,
{
	let mut result = BatchResult::new();

	for name in names {
		match execute_one(name.clone()) {
			Ok(()) => result.success(name),
			Err(error) => result.failure(name, error),
		}
	}

	result
}

fn render_port_impl_warning(port_name: &str, tech_name: &str, bounded_context: &str, error: &str) {
	Presenter::warning(format!("Port implementation for '{port_name}' was not created"));
	println!("  • {}", format!("Context: {}", bounded_context).bright_white());
	println!("  • {}", format!("Technology: {}", tech_name).bright_white());
	println!("  • {}", format!("Reason: {}", error).red());
}

pub fn handle(
	create_domain_feature: &CreateDomainFeature,
	create_domain_port: &CreateDomainPort,
	create_infrastructure_tech: &CreateInfrastructureTech,
	create_command_usecase: &CreateCommandUsecase,
	create_query_usecase: &CreateQueryUsecase,
	create_port_implementation: &CreatePortImplementation,
	args: AddArgs,
) -> Result<(), CliError> {
	let project_root = resolve_project_root()?;

	match args.kind {
		AddSubcommand::Query(add_args) => {
			let (names, bounded_context) = parse_multiple_target_args(add_args.args)?;
			let result = execute_batch(names, |query_name| {
				let input = CreateQueryUsecaseInput::new(project_root.clone(), bounded_context.clone(), query_name.clone());
				create_query_usecase.execute(input).map_err(|error| error.to_string())
			});

			// Display results
			if result.succeeded.len() == result.total {
				// All succeeded
				let mut message = Success::new(format!(
					"Query use-case{} created successfully",
					if result.total == 1 { "" } else { "s" }
				))
				.with_detail(format!("Context: {}", bounded_context));
				
				if result.total == 1 {
					message = message.with_detail(format!("Query: {}", result.succeeded[0]));
				} else {
					message = message.with_detail(format!("Created: {}", result.succeeded.join(", ")));
				}
				
				Presenter::success(&message);
			} else {
				Presenter::batch_result(
					format!("Query use-cases: {}/{} succeeded", result.succeeded.len(), result.total),
					&bounded_context,
					&result.succeeded,
					&result.failed,
				);
			}
		}
		AddSubcommand::Command(add_args) => {
			let (names, bounded_context) = parse_multiple_target_args(add_args.args)?;
			let result = execute_batch(names, |command_name| {
				let input = CreateCommandUsecaseInput::new(project_root.clone(), bounded_context.clone(), command_name.clone());
				create_command_usecase.execute(input).map_err(|error| error.to_string())
			});

			// Display results
			if result.succeeded.len() == result.total {
				// All succeeded
				let mut message = Success::new(format!(
					"Command use-case{} created successfully",
					if result.total == 1 { "" } else { "s" }
				))
				.with_detail(format!("Context: {}", bounded_context));
				
				if result.total == 1 {
					message = message.with_detail(format!("Command: {}", result.succeeded[0]));
				} else {
					message = message.with_detail(format!("Created: {}", result.succeeded.join(", ")));
				}
				
				Presenter::success(&message);
			} else {
				Presenter::batch_result(
					format!("Command use-cases: {}/{} succeeded", result.succeeded.len(), result.total),
					&bounded_context,
					&result.succeeded,
					&result.failed,
				);
			}
		}
		AddSubcommand::Tech(add_args) => {
			let (names, bounded_context) = parse_multiple_target_args(add_args.args)?;
			let result = execute_batch(names, |tech_name| {
				let input = CreateInfrastructureTechInput::new(
					project_root.clone(),
					bounded_context.clone(),
					tech_name.clone(),
				);
				create_infrastructure_tech.execute(input).map_err(|error| error.to_string())
			});

			// Display results
			if result.succeeded.len() == result.total {
				// All succeeded
				let mut message = Success::new(format!(
					"Infrastructure technolog{} created successfully",
					if result.total == 1 { "y" } else { "ies" }
				))
				.with_detail(format!("Context: {}", bounded_context));
				
				if result.total == 1 {
					message = message.with_detail(format!("Tech: {}", result.succeeded[0]));
				} else {
					message = message.with_detail(format!("Created: {}", result.succeeded.join(", ")));
				}
				
				Presenter::success(&message);
			} else {
				Presenter::batch_result(
					format!("Infrastructure technologies: {}/{} succeeded", result.succeeded.len(), result.total),
					&bounded_context,
					&result.succeeded,
					&result.failed,
				);
			}
		}
		AddSubcommand::Domain(add_args) => {
			let (names, bounded_context) = parse_multiple_target_args(add_args.args)?;
			let result = execute_batch(names, |domain_feature_name| {
				let input = CreateDomainFeatureInput::new(
					project_root.clone(),
					bounded_context.clone(),
					domain_feature_name.clone(),
				);
				create_domain_feature.execute(input).map_err(|error| error.to_string())
			});

			// Display results
			if result.succeeded.len() == result.total {
				// All succeeded
				let mut message = Success::new(format!(
					"Domain{} created successfully",
					if result.total == 1 { "" } else { "s" }
				))
				.with_detail(format!("Context: {}", bounded_context));
				
				if result.total == 1 {
					message = message.with_detail(format!("Domain: {}", result.succeeded[0]));
				} else {
					message = message.with_detail(format!("Created: {}", result.succeeded.join(", ")));
				}
				
				Presenter::success(&message);
			} else {
				Presenter::batch_result(
					format!("Domains: {}/{} succeeded", result.succeeded.len(), result.total),
					&bounded_context,
					&result.succeeded,
					&result.failed,
				);
			}
		}
		AddSubcommand::Port(add_args) => {
			let tech_name = resolve_optional_tech(add_args.with_keyword, add_args.tech_name.clone())?;
			let port_input = CreateDomainPortInput::new(
				project_root.clone(),
				add_args.bounded_context.clone(),
				add_args.domain_feature_name.clone(),
				add_args.port_name.clone(),
			);
			create_domain_port.execute(port_input).map_err(|error| {
				CliError::OperationFailed(format!("Error creating domain port: {error}"))
			})?;

			let success = Success::new("Domain port created successfully")
				.with_detail(format!("Domain: {}", add_args.domain_feature_name))
				.with_detail(format!("Port: {}", add_args.port_name))
				.with_detail(format!("Context: {}", add_args.bounded_context));
			Presenter::success(&success);

			if let Some(tech) = tech_name {
				let impl_input = CreatePortImplementationInput::new(
					project_root,
					add_args.bounded_context.clone(),
					add_args.domain_feature_name.clone(),
					add_args.port_name.clone(),
					tech.clone(),
				);

				match create_port_implementation.execute(impl_input) {
					Ok(()) => {
						let impl_success = Success::new("Port implementation created successfully")
							.with_detail(format!("Domain: {}", add_args.domain_feature_name))
							.with_detail(format!("Port: {}", add_args.port_name))
							.with_detail(format!("Technology: {}", tech))
							.with_detail(format!("Context: {}", add_args.bounded_context));
						Presenter::success(&impl_success);
					}
					Err(error) => {
						render_port_impl_warning(
							&add_args.port_name,
							&tech,
							&add_args.bounded_context,
							&error.to_string(),
						);
					}
				}
			}
		}
	};

	Ok(())
}