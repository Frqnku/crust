use std::path::{Path, PathBuf};

use crust_shared_domain::bounded_context_entity::BoundedContext;

use crate::{errors::ScaffoldingError, value_object::ArtifactName};

pub struct PortImplementation {
    bounded_context: BoundedContext,
    domain_feature_name: ArtifactName,
    port_name: ArtifactName,
    tech_name: ArtifactName,
}

impl PortImplementation {
    pub fn new(
        bounded_context: BoundedContext,
        domain_feature_name: String,
        port_name: String,
        tech_name: String,
    ) -> Result<Self, ScaffoldingError> {
        Ok(Self {
            bounded_context,
            domain_feature_name: ArtifactName::new(domain_feature_name)?,
            port_name: ArtifactName::new(port_name)?,
            tech_name: ArtifactName::new(tech_name)?,
        })
    }

    pub fn bounded_context(&self) -> &BoundedContext {
        &self.bounded_context
    }

    pub fn domain_feature_name(&self) -> &ArtifactName {
        &self.domain_feature_name
    }

    pub fn port_name(&self) -> &ArtifactName {
        &self.port_name
    }

    pub fn tech_name(&self) -> &ArtifactName {
        &self.tech_name
    }

    pub fn port_file_path(&self, project_root: &Path) -> PathBuf {
        project_root
            .join("crates")
            .join(self.bounded_context().name().as_str())
            .join("domain")
            .join("src")
            .join(self.domain_feature_name().as_str())
            .join(format!("{}.rs", self.port_name().as_str()))
    }

    pub fn implementation_directory(&self, project_root: &Path) -> PathBuf {
        project_root
            .join("crates")
            .join(self.bounded_context().name().as_str())
            .join("infrastructure")
            .join("src")
            .join(self.tech_name().as_str())
    }

    pub fn implementation_file_name(&self) -> String {
        format!("{}_{}.rs", self.domain_feature_name().as_str(), self.port_name().as_str())
    }

    pub fn implementation_file_path(&self, project_root: &Path) -> PathBuf {
        self.implementation_directory(project_root)
            .join(self.implementation_file_name())
    }

    pub fn trait_name(&self) -> String {
        format!(
            "{}{}",
            self.domain_feature_name().to_pascal_case(),
            self.port_name().to_pascal_case()
        )
    }

    pub fn struct_name(&self) -> String {
        format!(
            "{}{}{}",
            self.domain_feature_name().to_pascal_case(),
            self.port_name().to_pascal_case(),
            self.tech_name().to_pascal_case()
        )
    }

    pub fn implementation_module_name(&self) -> String {
        format!("{}_{}", self.domain_feature_name().as_str(), self.port_name().as_str())
    }
}