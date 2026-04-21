use workspace_application::use_cases::initialize_project::{
    InitializeProject,
    InitializeProjectInput
};
use workspace_infrastructure::fs_project_scaffolder::FsProjectScaffolder;

use crate::error::CliError;
use crate::helper::current_dir;

pub fn handle(name: Option<String>) -> Result<(), CliError> {
    let current_dir = current_dir()?;

    let (name, path) = match name {
        Some(name) => {
            let path = current_dir.join(&name);
            (name, path)
        }
        None => {
            let name = current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .map_or_else(|| "project".to_string(), |n| n.to_string());
            (name, current_dir)
        }
    };

    let mut scaffolder = FsProjectScaffolder;
    let mut use_case = InitializeProject::new(&mut scaffolder);
    let input = InitializeProjectInput::new(name, path);

    match use_case.execute(input) {
        Ok(project) => {
            println!(
                "Project '{}' initialized at '{}'",
                project.name,
                project.path.display()
            );
            Ok(())
        }
        Err(error) => Err(CliError::OperationFailed(format!(
            "Error initializing project: {:?}",
            error
        ))),
    }
}