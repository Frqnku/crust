use std::path::PathBuf;

use workspace_domain::{
    entities::{
        Project,
        ProjectScaffolder
    },
    errors::WorkspaceError
};

pub struct InitializeProjectInput {
    pub name: String,
    pub path: PathBuf,
}

impl InitializeProjectInput {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }
}

pub struct InitializeProject<'a> {
    scaffolder: &'a mut dyn ProjectScaffolder,
}

impl<'a> InitializeProject<'a> {
    pub fn new(scaffolder: &'a mut dyn ProjectScaffolder) -> Self {
        Self { scaffolder }
    }

    pub fn execute(&mut self, input: InitializeProjectInput) -> Result<Project, WorkspaceError> {
        self.scaffolder.create_project(input.name, input.path)
    }
}