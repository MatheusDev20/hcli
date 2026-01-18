use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn clear_folders_and_files(project_path: &Path) -> Result<()> {
    println!("{}", "Cleaning template and Removing unnecessary files...".green());
    let folders_to_remove = [".github", ".husky"];

    for folder in folders_to_remove {
        let path = project_path.join(folder);
        if path.exists() {
            fs::remove_dir_all(&path)
                .context(format!("Failed to remove {}", folder))?;
        }
    }

    let files_to_remove = [".npmrc", ".releaserc", "CHANGELOG.md", "README.md"];
    for file in files_to_remove {
        let path = project_path.join(file);
        if path.exists() {
            fs::remove_file(&path)
                .context(format!("Failed to remove {}", file))?;
        }
    }

    Ok(())
}
