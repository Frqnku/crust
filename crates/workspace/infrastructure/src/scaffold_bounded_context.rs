use std::fs;
use std::path::Path;
use toml_edit::{Array, DocumentMut, Item, Value};

use shared_infrastructure::fs_helper::{
    create_dir,
    create_file,
    create_unique_temp_dir,
    finalize_scaffold,
    FsHelperError,
    remove_dir,
};
use crate::templates::crates::WORKSPACE_TEMPLATES;

use workspace_domain::{
    entities::{BoundedContext, Project},
    errors::WorkspaceError,
};

const APPLICATION_LAYER: &str = "application";

fn context_member_paths(bounded_context_name: &str) -> [String; 3] {
    [
        format!("crates/{bounded_context_name}/domain"),
        format!("crates/{bounded_context_name}/application"),
        format!("crates/{bounded_context_name}/infrastructure"),
    ]
}

fn add_member_if_missing(members: &mut Array, member: &str, prefix: &str) -> bool {
    if members.iter().any(|value| value.as_str() == Some(member)) {
        return false;
    }

    let mut formatted = Value::from(member);
    formatted.decor_mut().set_prefix(prefix);
    formatted.decor_mut().set_suffix("");
    members.push_formatted(formatted);
    true
}

fn bounded_context_member_prefix(is_first_member: bool, bounded_context_name: &str) -> String {
    if is_first_member {
        format!("\n\n    # {}{}\n    ",
            bounded_context_name.chars().next().unwrap_or('?').to_uppercase(),
            &bounded_context_name[1..]
        )
    } else {
        "\n    ".to_string()
    }
}

fn scaffold_application_sources(src_path: &Path) -> Result<(), WorkspaceError> {
    let query_path = src_path.join("query");
    let command_path = src_path.join("command");

    create_dir(&query_path).map_err(map_fs_error)?;
    create_dir(&command_path).map_err(map_fs_error)?;
    create_file(&query_path.join("mod.rs"), "").map_err(map_fs_error)?;
    create_file(&command_path.join("mod.rs"), "").map_err(map_fs_error)?;
    create_file(&src_path.join("lib.rs"), "pub mod query;\npub mod command;\n").map_err(map_fs_error)?;

    Ok(())
}

fn map_fs_error(error: FsHelperError) -> WorkspaceError {
    match error {
        FsHelperError::Conflict { path, reason } => WorkspaceError::ProjectStructureConflict { path, reason },
        FsHelperError::InvalidLayout { path, reason } => WorkspaceError::InvalidProjectLayout { path, reason },
    }
}

fn create_layer_workspace(
    bounded_context_path: &Path,
    bounded_context: &BoundedContext,
    workspace_layer_name: &str,
    cargo_toml_template: &str,
) -> Result<(), WorkspaceError> {
    let workspace_path = bounded_context_path.join(workspace_layer_name);
    create_dir(&workspace_path).map_err(map_fs_error)?;
    let src_path = workspace_path.join("src");
    create_dir(&src_path).map_err(map_fs_error)?;
    let cargo_toml_content = cargo_toml_template.replace("{bounded_context_name}", bounded_context.name.as_str());
    create_file(&workspace_path.join("Cargo.toml"), &cargo_toml_content).map_err(map_fs_error)?;

    if workspace_layer_name == APPLICATION_LAYER {
        scaffold_application_sources(&src_path)?;
    } else {
        create_file(&src_path.join("lib.rs"), "").map_err(map_fs_error)?;
    }

    Ok(())
}

fn upsert_root_workspace_members(project: &Project, bounded_context: &BoundedContext) -> Result<(), WorkspaceError> {
    let cargo_toml_path = project.path.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path)
        .map_err(|error| map_fs_error(FsHelperError::Conflict {
            path: cargo_toml_path.clone(),
            reason: format!("Failed to read file: {error}"),
        }))?;

    let mut document = cargo_toml_content
        .parse::<DocumentMut>()
        .map_err(|error| WorkspaceError::InvalidProjectLayout {
            path: cargo_toml_path.clone(),
            reason: format!("Invalid root Cargo.toml: {error}"),
        })?;

    if !document["workspace"].is_table() {
        document["workspace"] = Item::Table(toml_edit::Table::new());
    }

    if !document["workspace"]["members"].is_array() {
        document["workspace"]["members"] = Item::Value(Value::Array(Array::new()));
    }

    let members = document["workspace"]["members"]
        .as_array_mut()
        .ok_or_else(|| WorkspaceError::InvalidProjectLayout {
            path: cargo_toml_path.clone(),
            reason: "[workspace].members must be an array".to_string(),
        })?;

    let new_members = context_member_paths(bounded_context.name.as_str());

    let mut inserted_any = false;
    let mut is_first_inserted = true;
    for member in new_members {
        let prefix = bounded_context_member_prefix(is_first_inserted, bounded_context.name.as_str());
        if add_member_if_missing(members, &member, &prefix) {
            inserted_any = true;
            is_first_inserted = false;
        }
    }

    if inserted_any {
        members.set_trailing_comma(true);
        members.set_trailing("\n");
    }

    fs::write(&cargo_toml_path, document.to_string())
        .map_err(|error| map_fs_error(FsHelperError::Conflict {
            path: cargo_toml_path,
            reason: format!("Failed to write file: {error}"),
        }))?;

    Ok(())
}

pub fn scaffold_bounded_context(
    project: &mut Project,
    bounded_context: BoundedContext,
) -> Result<(), WorkspaceError> {
    let bounded_context_path = project.ensure_can_create_bounded_context(&bounded_context)?;
    if bounded_context_path.exists() {
        return Err(WorkspaceError::BoundedContextAlreadyExists {
            path: bounded_context_path,
        });
    }

    let temp_root_parent = project.expected_crates_path();
    let temp_root = create_unique_temp_dir(&temp_root_parent, "crust-bounded-context").map_err(map_fs_error)?;

    let scaffold_result = (|| {
        for ws in WORKSPACE_TEMPLATES {
            create_layer_workspace(&temp_root, &bounded_context, ws.name, ws.template)?;
        }

        Ok(())
    })();

    if let Err(error) = scaffold_result {
        remove_dir(&temp_root);
        return Err(error);
    }

    if let Err(error) = finalize_scaffold(&temp_root, &bounded_context_path).map_err(map_fs_error) {
        remove_dir(&temp_root);
        return Err(error);
    }

    upsert_root_workspace_members(project, &bounded_context)?;

    project.add_bounded_context(bounded_context)
}
