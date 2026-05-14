use crate::error::CliError;
use crate::helper::{execute_batch, project_name_from_root, resolve_project_root};
use crate::parser::NewArgs;
use crate::output::{Presenter, Success};
use crust_workspace_application::use_cases::add_bounded_context::{AddBoundedContext, AddBoundedContextInput};

pub fn handle(add_bounded_context: &AddBoundedContext, args: NewArgs) -> Result<(), CliError> {
	let project_root = resolve_project_root()?;
	let project_name = project_name_from_root(&project_root);

	let result = execute_batch(args.bounded_contexts, |bounded_context_name| {
		let input = AddBoundedContextInput::new(project_name.clone(), project_root.clone(), bounded_context_name.clone());
			add_bounded_context.execute(input).map_err(|error| error.to_string())
	});

	if result.succeeded.len() == result.total {
		// All succeeded
		let mut message = Success::new(format!(
			"Bounded context{} created successfully",
			if result.total == 1 { "" } else { "s" }
		));
		
		if result.total == 1 {
			message = message
				.with_detail(format!("Bounded Context: {}", result.succeeded[0]))
				.with_detail(format!("Location: {}", project_root.join("crates").join(&result.succeeded[0]).display()));
		} else {
			message = message
				.with_detail(format!("Created: {}", result.succeeded.join(", ")))
				.with_detail(format!("Location:\n	{}", result.succeeded
					.iter()
					.map(|name| format!("{}", project_root.join("crates").join(name).display()))
					.collect::<Vec<_>>()
					.join("\n	")
				));
		}
		
		Presenter::success(&message);
	} else {
		Presenter::batch_result_bounded_contexts(
			format!("Bounded contexts: {}/{} succeeded", result.succeeded.len(), result.total),
			&result.succeeded,
			&result.failed,
			&project_root
		);
	}

	Ok(())
}