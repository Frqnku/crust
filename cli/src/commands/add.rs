use scaffolding_application::use_cases::create_artifact::{
    CreateArtifact,
    CreateArtifactInput
};
use scaffolding_domain::value_object::ArtifactKind;
use scaffolding_infrastructure::fs_artifact_scaffolder::FsArtifactScaffolder;
use std::path::{Path, PathBuf};

fn find_project_root(start: &Path) -> Option<PathBuf> {
    for candidate in start.ancestors() {
        if candidate.join("Cargo.toml").is_file() && candidate.join("crates").is_dir() {
            return Some(candidate.to_path_buf());
        }
    }

    None
}

pub fn handle(bounded_context: String, name: Option<&str>) {
    let current_dir = match std::env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error resolving current directory: {}", e);
            return;
        }
    };

    let project_root = match find_project_root(&current_dir) {
        Some(path) => path,
        None => {
            eprintln!("Could not find project root from '{}'.", current_dir.display());
            return;
        }
    };

    let artifact_name = name.unwrap_or("example").to_string();

    let mut scaffolder = FsArtifactScaffolder;
    let mut use_case = CreateArtifact::new(&mut scaffolder);
    let input = CreateArtifactInput::new(
        project_root,
        bounded_context,
        ArtifactKind::QueryUsecase,
        artifact_name,
    );

    match use_case.execute(input) {
        Ok(()) => println!("Artifact created successfully"),
        Err(e) => eprintln!("Error creating artifact: {:?}", e),
    }
}