use std::path::Path;

use crate::{
	artifact_entity::Artifact,
	domain_port_entity::DomainPort,
	errors::ScaffoldingError,
	port_implementation_entity::PortImplementation,
};

/// Trait for scaffolding domain features
pub trait DomainFeatureScaffolder {
	fn scaffold(&self, project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError>;
}

/// Trait for scaffolding infrastructure technical components
pub trait InfrastructureTechScaffolder {
	fn scaffold(&self, project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError>;
}

/// Trait for scaffolding command usecases
pub trait CommandUsecaseScaffolder {
	fn scaffold(&self, project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError>;
}

/// Trait for scaffolding query usecases
pub trait QueryUsecaseScaffolder {
	fn scaffold(&self, project_root: &Path, artifact: Artifact) -> Result<(), ScaffoldingError>;
}

/// Trait for scaffolding domain ports in a domain feature folder
pub trait DomainPortScaffolder {
	fn scaffold(&self, project_root: &Path, domain_port: DomainPort) -> Result<(), ScaffoldingError>;
}

/// Trait for scaffolding concrete infrastructure implementations for domain ports
pub trait PortImplementationScaffolder {
	fn scaffold(&self, project_root: &Path, implementation: PortImplementation) -> Result<(), ScaffoldingError>;
}
