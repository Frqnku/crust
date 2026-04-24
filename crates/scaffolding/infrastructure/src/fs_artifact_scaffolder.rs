use scaffolding_domain::scaffolder_ports::{
    DomainFeatureScaffolder,
    InfrastructureTechScaffolder,
    CommandUsecaseScaffolder,
    QueryUsecaseScaffolder,
};
use scaffolding_domain::artifact_entity::Artifact;
use scaffolding_domain::errors::ScaffoldingError;
use std::path::Path;

use crate::{scaffold_domain::scaffold_domain_feature, scaffold_usecase::scaffold_usecase};
use crate::scaffold_infrastructure_tech::scaffold_infrastructure_tech;

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