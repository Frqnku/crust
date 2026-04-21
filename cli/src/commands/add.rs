use scaffolding_application::use_cases::create_artifact::{
    CreateArtifact,
    CreateArtifactInput
};
use scaffolding_domain::value_object::ArtifactKind;
use scaffolding_infrastructure::fs_artifact_scaffolder::FsArtifactScaffolder;

use crate::error::CliError;
use crate::helper::resolve_project_root;

pub fn handle(
    bounded_context: String,
    artifact_kind: ArtifactKind,
    artifact_name: String,
) -> Result<(), CliError> {
    let project_root = resolve_project_root()?;

    let mut scaffolder = FsArtifactScaffolder;
    let mut use_case = CreateArtifact::new(&mut scaffolder);
    let input = CreateArtifactInput::new(
        project_root,
        bounded_context,
        artifact_kind,
        artifact_name,
    );

    match use_case.execute(input) {
        Ok(()) => {
            println!("Artifact created successfully");
            Ok(())
        }
        Err(error) => Err(CliError::OperationFailed(format!(
            "Error creating artifact: {:?}",
            error
        ))),
    }
}