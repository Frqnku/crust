use std::fmt;
use std::path::PathBuf;

use crate::errors::WorkspaceError;
use shared_domain::helper::validate_crust_identifier;
use shared_domain::bounded_context_entity::BoundedContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectName(String);

impl ProjectName {
    pub fn new(value: String) -> Result<Self, WorkspaceError> {
        if let Err(reason) = validate_crust_identifier(&value) {
            return Err(WorkspaceError::InvalidProjectName {
                value,
                reason: reason.to_string(),
            });
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ProjectName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ProjectName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    name: ProjectName,
    path: PathBuf,
    bounded_contexts: Vec<BoundedContext>,
}

impl Project {
    pub fn new(name: String, path: PathBuf) -> Result<Self, WorkspaceError> {
        Ok(Self {
            name: ProjectName::new(name)?,
            path,
            bounded_contexts: Vec::new(),
        })
    }

    pub fn expected_crates_path(&self) -> PathBuf {
        self.path.join("crates")
    }

    pub fn name(&self) -> &ProjectName {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn bounded_contexts(&self) -> &[BoundedContext] {
        &self.bounded_contexts
    }

    pub fn bounded_context_root(&self, bounded_context_name: &str) -> PathBuf {
        self.expected_crates_path().join(bounded_context_name)
    }

    pub fn ensure_can_create_bounded_context(&self, bounded_context: &BoundedContext) -> Result<PathBuf, WorkspaceError> {
        let path = self.bounded_context_root(bounded_context.name().as_str());

        if self.bounded_contexts.iter().any(|context| context.name() == bounded_context.name()) {
            return Err(WorkspaceError::BoundedContextAlreadyExists { path });
        }

        Ok(path)
    }

    pub fn add_bounded_context(&mut self, bounded_context: BoundedContext) -> Result<(), WorkspaceError> {
        self.ensure_can_create_bounded_context(&bounded_context)?;

        self.bounded_contexts.push(bounded_context);

        Ok(())
    }
}

pub trait ProjectScaffolder {
    fn create_project(&self, name: String, path: PathBuf) -> Result<Project, WorkspaceError>;
    fn create_bounded_context(&self, project: &mut Project, bounded_context: BoundedContext) -> Result<(), WorkspaceError>;
}