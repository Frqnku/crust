use std::{path::PathBuf, rc::Rc};

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

pub struct InitializeProject {
    scaffolder: Rc<dyn ProjectScaffolder>,
}

impl InitializeProject {
    pub fn new(scaffolder: Rc<dyn ProjectScaffolder>) -> Self {
        Self { scaffolder }
    }

    pub fn execute(&self, input: InitializeProjectInput) -> Result<Project, WorkspaceError> {
        self.scaffolder.create_project(input.name, input.path)
    }
}