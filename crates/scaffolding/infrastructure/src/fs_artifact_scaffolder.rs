use scaffolding_domain::{
    artifact_entity::{Artifact, ArtifactScaffolder},
    errors::ScaffoldingError,
    value_object::ArtifactKind,
};
use std::path::Path;

use crate::{scaffold_domain::scaffold_domain_feature, scaffold_usecase::scaffold_usecase};
use crate::scaffold_infrastructure_tech::scaffold_infrastructure_tech;

pub struct FsArtifactScaffolder;

impl ArtifactScaffolder for FsArtifactScaffolder {
    fn create_artifact(&self, project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError> {
        match artifact.kind() {
            ArtifactKind::Domain => scaffold_domain_feature(project_root, artifact),
            ArtifactKind::InfrastructureTech => scaffold_infrastructure_tech(project_root, artifact),
            ArtifactKind::CommandUsecase | ArtifactKind::QueryUsecase => scaffold_usecase(project_root, artifact),
            _ => Err(ScaffoldingError::BoundedContextNotFound { name: "unsupported".to_string() }),
        }
    }
}