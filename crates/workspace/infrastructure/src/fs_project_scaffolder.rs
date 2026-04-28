use std::path::PathBuf;

use crate::{
    scaffold_bounded_context::scaffold_bounded_context,
    scaffold_project::scaffold_project,
};

use shared_domain::bounded_context_entity::BoundedContext;
use workspace_domain::{
    project_entity::{
        Project,
        ProjectScaffolder
    },
    errors::WorkspaceError,
};

pub struct FsProjectScaffolder;

impl ProjectScaffolder for FsProjectScaffolder {
    fn create_project(&self, name: String, path: PathBuf) -> Result<Project, WorkspaceError> {
        scaffold_project(name, path)
    }

    fn create_bounded_context(&self, project: &mut Project, bounded_context: BoundedContext) -> Result<(), WorkspaceError> {
        scaffold_bounded_context(project, bounded_context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestWorkspace {
        root: PathBuf,
    }

    impl TestWorkspace {
        fn new(test_name: &str) -> Self {
            let mut root = std::env::temp_dir();
            let unique_suffix = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or_default();
            root.push(format!("crust-{test_name}-{unique_suffix}"));
            fs::create_dir_all(&root).expect("failed to create test workspace");

            Self { root }
        }

        fn path(&self, name: &str) -> PathBuf {
            self.root.join(name)
        }
    }

    impl Drop for TestWorkspace {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn directory_entries(path: &Path) -> Vec<String> {
        let mut entries = fs::read_dir(path)
            .expect("failed to read directory")
            .map(|entry| entry.expect("failed to read entry").file_name().to_string_lossy().to_string())
            .collect::<Vec<_>>();
        entries.sort();
        entries
    }

    fn assert_contains_exactly(path: &Path, expected: &[&str]) {
        let mut expected = expected.iter().map(|value| value.to_string()).collect::<Vec<_>>();
        expected.sort();

        assert_eq!(directory_entries(path), expected);
    }

    fn assert_layer_workspace(path: &Path) {
        assert_contains_exactly(path, &["Cargo.toml", "src"]);
        assert!(path.join("Cargo.toml").is_file());
        assert!(path.join("src").is_dir());
    }

    #[test]
    fn create_project_creates_exact_top_level_structure() {
        let workspace = TestWorkspace::new("create-project");
        let project_path = workspace.path("workspace");
        let scaffolder = FsProjectScaffolder;

        let project = scaffolder
            .create_project("demo".to_string(), project_path.clone())
            .expect("project should be created");

        assert_eq!(project.path(), &project_path);
        assert_contains_exactly(project.path(), &["Cargo.toml", "bin", "crates"]);
        assert!(project.path().join("Cargo.toml").is_file());
        assert!(project.path().join("bin").is_dir());
        assert!(project.path().join("bin").join("Cargo.toml").is_file());
        assert!(project.path().join("bin").join("main.rs").is_file());
        assert!(project.path().join("crates").is_dir());
    }

    #[test]
    fn create_bounded_context_creates_exact_layer_structure() {
        let workspace = TestWorkspace::new("create-bounded-context");
        let project_path = workspace.path("workspace");
        let scaffolder = FsProjectScaffolder;

        let mut project = scaffolder
            .create_project("demo".to_string(), project_path)
            .expect("project should be created");

        let bounded_context = BoundedContext::new("sales".to_string()).expect("bounded context should be valid");
        scaffolder
            .create_bounded_context(&mut project, bounded_context)
            .expect("bounded context should be created");

        let bounded_context_path = project.bounded_context_root("sales");
        assert!(bounded_context_path.is_dir());
        assert_contains_exactly(&bounded_context_path, &["application", "domain", "infrastructure"]);
        assert_layer_workspace(&bounded_context_path.join("application"));
        assert_layer_workspace(&bounded_context_path.join("domain"));
        assert_layer_workspace(&bounded_context_path.join("infrastructure"));

        let application_src_path = bounded_context_path.join("application").join("src");
        assert_contains_exactly(&application_src_path, &["command", "lib.rs", "query"]);
        assert!(application_src_path.join("query").join("mod.rs").is_file());
        assert!(application_src_path.join("command").join("mod.rs").is_file());
        let application_lib_rs = fs::read_to_string(application_src_path.join("lib.rs"))
            .expect("application lib.rs should be readable");
        assert!(
            application_lib_rs.contains("pub mod query;"),
            "application lib.rs should export query module"
        );
        assert!(
            application_lib_rs.contains("pub mod command;"),
            "application lib.rs should export command module"
        );

        let root_cargo_toml = fs::read_to_string(project.path().join("Cargo.toml"))
            .expect("root Cargo.toml should be readable");
        assert!(
            root_cargo_toml.contains("# Sales"),
            "root Cargo.toml should include the bounded contexts comment"
        );
        assert!(
            root_cargo_toml.contains("\"crates/sales/domain\""),
            "root Cargo.toml should include domain workspace member"
        );
        assert!(
            root_cargo_toml.contains("\"crates/sales/application\""),
            "root Cargo.toml should include application workspace member"
        );
        assert!(
            root_cargo_toml.contains("\"crates/sales/infrastructure\""),
            "root Cargo.toml should include infrastructure workspace member"
        );
        assert!(
            root_cargo_toml.contains("\n    \"crates/sales/domain\",\n"),
            "domain workspace member should be on its own line with trailing comma"
        );
        assert!(
            root_cargo_toml.contains("\n    \"crates/sales/application\",\n"),
            "application workspace member should be on its own line with trailing comma"
        );
        assert!(
            root_cargo_toml.contains("\n    \"crates/sales/infrastructure\",\n"),
            "infrastructure workspace member should be on its own line with trailing comma"
        );
        assert!(
            !root_cargo_toml.contains("\n, \"crates/sales/domain\""),
            "workspace members must not be rendered with a leading comma"
        );
    }

    #[test]
    fn duplicate_bounded_context_returns_deterministic_error() {
        let workspace = TestWorkspace::new("duplicate-bounded-context");
        let project_path = workspace.path("workspace");
        let scaffolder = FsProjectScaffolder;

        let mut project = scaffolder
            .create_project("demo".to_string(), project_path)
            .expect("project should be created");

        let bounded_context = BoundedContext::new("sales".to_string()).expect("bounded context should be valid");
        scaffolder
            .create_bounded_context(&mut project, bounded_context.clone())
            .expect("first bounded context should be created");

        let error = scaffolder
            .create_bounded_context(&mut project, bounded_context)
            .expect_err("duplicate bounded context should fail");

        assert_eq!(
            error,
            WorkspaceError::BoundedContextAlreadyExists {
                path: project.bounded_context_root("sales"),
            }
        );
        assert_eq!(project.bounded_contexts().len(), 1);
    }
}
