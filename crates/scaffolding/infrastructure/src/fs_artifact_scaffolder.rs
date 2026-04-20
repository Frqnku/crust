use scaffolding_domain::{artifact_entity::{Artifact, ArtifactScaffolder}, errors::ScaffoldingError};
use std::path::Path;

use crate::scaffold_artifact::scaffold_artifact;

pub struct FsArtifactScaffolder;

impl ArtifactScaffolder for FsArtifactScaffolder {
    fn create_artifact(&mut self, project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError> {
        scaffold_artifact(project_root, artifact)
    }
}