use workspace_application::use_cases::add_bounded_context::{
	AddBoundedContext,
	AddBoundedContextInput,
};
use workspace_infrastructure::fs_project_scaffolder::FsProjectScaffolder;
use std::path::{Path, PathBuf};

fn find_project_root(start: &Path) -> Option<PathBuf> {
	for candidate in start.ancestors() {
		if candidate.join("Cargo.toml").is_file() && candidate.join("crates").is_dir() {
			return Some(candidate.to_path_buf());
		}
	}

	None
}

pub fn handle(name: Option<&str>) {
	let current_dir = match std::env::current_dir() {
		Ok(path) => path,
		Err(error) => {
			eprintln!("Error resolving current directory: {}", error);
			return;
		}
	};

	let project_root = match find_project_root(&current_dir) {
		Some(path) => path,
		None => {
			eprintln!("Could not find project root from '{}'.", current_dir.display());
			return;
		}
	};

	let project_name = project_root
		.file_name()
		.and_then(|n| n.to_str())
		.map_or_else(|| "project".to_string(), |n| n.to_string());

	let bounded_context_name = name.unwrap_or("new_context").to_string();
	let mut scaffolder = FsProjectScaffolder;
	let mut use_case = AddBoundedContext::new(&mut scaffolder);
	let input = AddBoundedContextInput::new(
		project_name,
		project_root.clone(),
		bounded_context_name.clone(),
	);

	match use_case.execute(input) {
		Ok(()) => println!(
			"Bounded context '{}' created at '{}'",
			bounded_context_name,
			project_root
				.join("crates")
				.join(&bounded_context_name)
				.display()
		),
		Err(error) => eprintln!("Error creating bounded context: {:?}", error),
	}
}
