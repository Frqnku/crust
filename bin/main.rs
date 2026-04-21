use std::rc::Rc;

use scaffolding_application::use_cases::create_artifact::CreateArtifact;
use scaffolding_infrastructure::fs_artifact_scaffolder::FsArtifactScaffolder;
use workspace_application::use_cases::add_bounded_context::AddBoundedContext;
use workspace_application::use_cases::initialize_project::InitializeProject;
use workspace_infrastructure::fs_project_scaffolder::FsProjectScaffolder;
use cli::app::App;

struct CompositionRoot {
    initialize_project: InitializeProject,
    add_bounded_context: AddBoundedContext,
    create_artifact: CreateArtifact,
}

impl CompositionRoot {
    fn new() -> Self {
        let project_scaffolder = Rc::new(FsProjectScaffolder);
        let artifact_scaffolder = Rc::new(FsArtifactScaffolder);

        Self {
            initialize_project: InitializeProject::new(project_scaffolder.clone()),
            add_bounded_context: AddBoundedContext::new(project_scaffolder),
            create_artifact: CreateArtifact::new(artifact_scaffolder),
        }
    }
}

fn main() {
    let composition_root = CompositionRoot::new();
    let app = App::new(
        composition_root.initialize_project,
        composition_root.add_bounded_context,
        composition_root.create_artifact,
    );

    if let Err(error) = app.run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
