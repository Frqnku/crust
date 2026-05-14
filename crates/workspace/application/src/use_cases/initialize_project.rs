use std::{path::PathBuf, rc::Rc};

use crust_workspace_domain::{
    project_entity::{
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, path::PathBuf, rc::Rc};
    use crust_workspace_domain::project_entity::ProjectScaffolder;
    use crust_shared_domain::bounded_context_entity::BoundedContext;

    struct MockProjectScaffolder {
        called: RefCell<bool>,
    }

    impl MockProjectScaffolder {
        fn new() -> Self {
            Self { called: RefCell::new(false) }
        }
    }

    impl ProjectScaffolder for MockProjectScaffolder {
        fn create_project(&self, name: String, path: PathBuf) -> Result<crust_workspace_domain::project_entity::Project, crust_workspace_domain::errors::WorkspaceError> {
            *self.called.borrow_mut() = true;
            Ok(crust_workspace_domain::project_entity::Project::new(name, path)?)
        }

        fn create_bounded_context(&self, _project: &mut crust_workspace_domain::project_entity::Project, _bounded_context: BoundedContext) -> Result<(), crust_workspace_domain::errors::WorkspaceError> {
            Ok(())
        }
    }

    #[test]
    fn execute_calls_scaffolder_and_returns_project() {
        let mock = Rc::new(MockProjectScaffolder::new());
        let usecase = InitializeProject::new(mock.clone());
        let input = InitializeProjectInput::new("myproj".into(), PathBuf::from("."));
        let res = usecase.execute(input);
        assert!(res.is_ok());
        assert!(*mock.called.borrow());
    }

    #[test]
    fn invalid_project_name_returns_error() {
        let mock = Rc::new(MockProjectScaffolder::new());
        let usecase = InitializeProject::new(mock.clone());
        let input = InitializeProjectInput::new("Bad Project".into(), PathBuf::from("."));
        let res = usecase.execute(input);
        assert!(matches!(res, Err(crust_workspace_domain::errors::WorkspaceError::InvalidProjectName { .. })));
    }
}