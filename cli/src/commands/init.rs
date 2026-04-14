use workspace_application::use_cases::initialize_project::{
    InitializeProject,
    InitializeProjectInput
};
use workspace_infrastructure::fs_project_scaffolder::FsProjectScaffolder;

pub fn handle(name: Option<&str>) {
    let current_dir = match std::env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error resolving current directory: {}", e);
            return;
        }
    };

    let (name, path) = match name {
        Some(name) => (name.to_string(), current_dir.join(name)),
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
        Ok(project) => println!("Project '{}' initialized at '{}'", project.name, project.path.display()),
        Err(e) => eprintln!("Error initializing project: {:?}", e),
    }
}