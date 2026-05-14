use crust_shared_domain::bounded_context_entity::BoundedContext;
use std::path::Path;

use crate::{errors::ScaffoldingError, value_object::{ArtifactKind, ArtifactName}};

pub struct Artifact {
    bounded_context: BoundedContext,
    kind: ArtifactKind,
    name: ArtifactName,
}

impl Artifact {
    pub fn new(bounded_context: BoundedContext, kind: ArtifactKind, name: String) -> Result<Self, ScaffoldingError> {
        Ok(Self {
            bounded_context,
            kind,
            name: ArtifactName::new(name)?,
        })
    }

    pub fn as_file_name(&self) -> String {
        format!("{}.rs", self.name.as_str())
    }

    pub fn bounded_context(&self) -> &BoundedContext {
        &self.bounded_context
    }

    pub fn kind(&self) -> &ArtifactKind {
        &self.kind
    }

    pub fn name(&self) -> &ArtifactName {
        &self.name
    }

    pub fn relative_directory(&self) -> String {
        match self.kind() {
            ArtifactKind::InfrastructureTech | ArtifactKind::Domain => format!(
                "{}/{}/src",
                self.bounded_context().name().as_str(),
                self.kind().parent_directory()
            ),
            _ => format!(
                "{}/{}/src/{}",
                self.bounded_context().name().as_str(),
                self.kind().parent_directory(),
                self.kind().as_str()
            ),
        }
    }
}

pub trait ArtifactScaffolder {
    fn create_artifact(&self, project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError>;
}