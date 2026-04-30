use std::rc::Rc;

use scaffolding_application::use_cases::{
    create_domain_feature::CreateDomainFeature,
    create_domain_port::CreateDomainPort,
    create_infrastructure_tech::CreateInfrastructureTech,
    create_command_usecase::CreateCommandUsecase,
    create_port_implementation::CreatePortImplementation,
    create_query_usecase::CreateQueryUsecase,
};
use scaffolding_infrastructure::fs_artifact_scaffolder::FsArtifactScaffolder;
use workspace_application::use_cases::add_bounded_context::AddBoundedContext;
use workspace_application::use_cases::initialize_project::InitializeProject;
use workspace_infrastructure::fs_project_scaffolder::FsProjectScaffolder;
use cli::app::App;
use cli::output::Presenter;

struct CompositionRoot {
    initialize_project: InitializeProject,
    add_bounded_context: AddBoundedContext,
    create_domain_feature: CreateDomainFeature,
    create_domain_port: CreateDomainPort,
    create_infrastructure_tech: CreateInfrastructureTech,
    create_command_usecase: CreateCommandUsecase,
    create_query_usecase: CreateQueryUsecase,
    create_port_implementation: CreatePortImplementation,
}

impl CompositionRoot {
    fn new() -> Self {
        let project_scaffolder = Rc::new(FsProjectScaffolder);
        let artifact_scaffolder = Rc::new(FsArtifactScaffolder);

        Self {
            initialize_project: InitializeProject::new(project_scaffolder.clone()),
            add_bounded_context: AddBoundedContext::new(project_scaffolder),
            create_domain_feature: CreateDomainFeature::new(artifact_scaffolder.clone()),
            create_domain_port: CreateDomainPort::new(artifact_scaffolder.clone()),
            create_infrastructure_tech: CreateInfrastructureTech::new(artifact_scaffolder.clone()),
            create_command_usecase: CreateCommandUsecase::new(artifact_scaffolder.clone()),
            create_query_usecase: CreateQueryUsecase::new(artifact_scaffolder.clone()),
            create_port_implementation: CreatePortImplementation::new(artifact_scaffolder.clone()),
        }
    }
}

fn main() {
    let composition_root = CompositionRoot::new();
    let app = App::new(
        composition_root.initialize_project,
        composition_root.add_bounded_context,
        composition_root.create_domain_feature,
        composition_root.create_domain_port,
        composition_root.create_infrastructure_tech,
        composition_root.create_command_usecase,
        composition_root.create_query_usecase,
        composition_root.create_port_implementation,
    );

    if let Err(error) = app.run() {
        let error_message = error.to_message();
        Presenter::error(&error_message);
        std::process::exit(1);
    }
}
