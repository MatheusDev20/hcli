use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn customize_project(path: &Path) -> Result<()> {
    let package_json_path = path.join("package.json");

    let package_json_content =
        fs::read_to_string(&package_json_path).context("Failed to read package.json")?;

    let mut package_json: Value =
        serde_json::from_str(&package_json_content).context("Failed to parse package.json")?;

    if let Some(obj) = package_json.as_object_mut() {
        obj.insert("name".to_string(), Value::String("HML v1.0.0".to_string()));
        obj.remove("repository");
    }

    if let Some(scripts) = package_json
        .get_mut("scripts")
        .and_then(|s| s.as_object_mut())
    {
        scripts.remove("prepare");
    }
    // Write updated package.json
    let updated_content =
        serde_json::to_string_pretty(&package_json).context("Failed to serialize package.json")?;
    fs::write(&package_json_path, updated_content).context("Failed to write package.json")?;

    Ok(())
}
