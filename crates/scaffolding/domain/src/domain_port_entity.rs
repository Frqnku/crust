use std::path::Path;

use shared_domain::bounded_context_entity::BoundedContext;

use crate::{errors::ScaffoldingError, value_object::ArtifactName};

pub struct DomainPort {
    bounded_context: BoundedContext,
    feature_name: ArtifactName,
    port_name: ArtifactName,
}

impl DomainPort {
    pub fn new(
        bounded_context: BoundedContext,
        feature_name: String,
        port_name: String,
    ) -> Result<Self, ScaffoldingError> {
        Ok(Self {
            bounded_context,
            feature_name: ArtifactName::new(feature_name)?,
            port_name: ArtifactName::new(port_name)?,
        })
    }

    pub fn bounded_context(&self) -> &BoundedContext {
        &self.bounded_context
    }

    pub fn feature_name(&self) -> &ArtifactName {
        &self.feature_name
    }

    pub fn port_name(&self) -> &ArtifactName {
        &self.port_name
    }

    pub fn as_file_name(&self) -> String {
        format!("{}.rs", self.port_name.as_str())
    }

    pub fn relative_directory(&self) -> String {
        format!(
            "{}/domain/src/{}",
            self.bounded_context().name().as_str(),
            self.feature_name().as_str()
        )
    }

    pub fn feature_directory(&self, project_root: &Path) -> std::path::PathBuf {
        project_root.join("crates").join(self.relative_directory())
    }
}