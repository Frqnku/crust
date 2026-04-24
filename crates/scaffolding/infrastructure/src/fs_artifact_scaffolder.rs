use scaffolding_domain::scaffolder_ports::{
    DomainFeatureScaffolder,
    DomainPortScaffolder,
    InfrastructureTechScaffolder,
    PortImplementationScaffolder,
    CommandUsecaseScaffolder,
    QueryUsecaseScaffolder,
};
use scaffolding_domain::artifact_entity::Artifact;
use scaffolding_domain::domain_port_entity::DomainPort;
use scaffolding_domain::errors::ScaffoldingError;
use scaffolding_domain::port_implementation_entity::PortImplementation;
use std::path::Path;

use crate::{scaffold_domain::scaffold_domain_feature, scaffold_usecase::scaffold_usecase};
use crate::scaffold_domain_port::scaffold_domain_port;
use crate::scaffold_infrastructure_tech::scaffold_infrastructure_tech;
use crate::scaffold_port_implementation::scaffold_port_implementation;

pub struct FsArtifactScaffolder;

impl DomainFeatureScaffolder for FsArtifactScaffolder {
    fn scaffold(&self, project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError> {
        scaffold_domain_feature(project_root, artifact)
    }
}

impl InfrastructureTechScaffolder for FsArtifactScaffolder {
    fn scaffold(&self, project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError> {
        scaffold_infrastructure_tech(project_root, artifact)
    }
}

impl CommandUsecaseScaffolder for FsArtifactScaffolder {
    fn scaffold(&self, project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError> {
        scaffold_usecase(project_root, artifact)
    }
}

impl QueryUsecaseScaffolder for FsArtifactScaffolder {
    fn scaffold(&self, project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError> {
        scaffold_usecase(project_root, artifact)
    }
}

impl DomainPortScaffolder for FsArtifactScaffolder {
    fn scaffold(&self, project_root: &Path, domain_port: DomainPort) -> Result<(), ScaffoldingError> {
        scaffold_domain_port(project_root, domain_port)
    }
}

impl PortImplementationScaffolder for FsArtifactScaffolder {
    fn scaffold(&self, project_root: &Path, implementation: PortImplementation) -> Result<(), ScaffoldingError> {
        scaffold_port_implementation(project_root, implementation)
    }
}