
mod clear;
mod tailwind;
mod customize;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use dialoguer::Confirm;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use tailwind::setup_tailwind;
use tempfile::tempdir;

const COPENHAGEN_THEME_ZIP_URL: &str =
    "https://github.com/zendesk/copenhagen_theme/archive/refs/heads/main.zip";

#[derive(Parser, Debug)]
#[command(name = "hc-cli")]
#[command(author = "Matheus DP")]
#[command(version = "0.1.0")]
#[command(about = "CLI tool for bootstrapping Help Center projects", long_about = None)]

struct Args {
    project_name: String,
    #[arg(default_value = ".")]
    output: String,
    #[arg(long, default_value_t = false)]
    tailwind: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let project_path = Path::new(&args.output).join(&args.project_name);

    if args.output == "." {
        let current_dir = std::env::current_dir()
            .context("Failed to get current directory")?;

        println!(
            "{} Project '{}' will be created in the current directory, specify a custom directory if this is not desired : {}",
            "📁".bold(),
            args.project_name.cyan(),
            current_dir.display().to_string().yellow()
        );

        let confirmed = Confirm::new()
            .with_prompt("Do you want to continue?")
            .default(true)
            .interact()
            .context("Failed to read user input")?;

        if !confirmed {
            println!("{} Operation cancelled.", "❌".bold());
            return Ok(());
        }

        println!();
    }

    println!(
        "{} {} {}",
        "🚀".bold(),
        "Creating Help Center project:".green().bold(),
        args.project_name.cyan()
    );

    if project_path.exists() {
        anyhow::bail!(
            "Directory '{}' already exists. Please choose a different name or remove the existing directory.",
            project_path.display()
        );
    }

    download_and_extract_theme(&project_path)?;

    clear::clear_folders_and_files(&project_path)?;
    customize::customize_project(&project_path)?;

    if args.tailwind {
        setup_tailwind(&project_path)?;
    }

    println!();
    println!(
        "{} {}",
        "✅".bold(),
        "Project created successfully!".green().bold()
    );

    println!();
    println!("Next steps:");
    println!(
        "  {} {}",
        "cd".cyan(),
        args.project_name.cyan()
    );

    println!(
        "  {} {}",
        "# Start developing your Help Center theme with zcli themes:preview --logs".dimmed(),
        ""
    );
    Ok(())
}

fn download_and_extract_theme(project_path: &Path) -> Result<()> {
    println!(
        "{} {}",
        "📦".bold(),
        "Downloading latest Copenhagen theme...".yellow()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Fetching from GitHub...");

    let temp_dir = tempdir().context("Failed to create temporary directory")?;
    let zip_path = temp_dir.path().join("theme.zip");
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(COPENHAGEN_THEME_ZIP_URL)
        .header("User-Agent", "hc-cli")
        .send()
        .context("Failed to download Copenhagen theme")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to download theme: HTTP {}",
            response.status()
        );
    }

    let bytes = response.bytes().context("Failed to read response body")?;

    pb.set_message("Saving theme archive...");
    let mut file = File::create(&zip_path).context("Failed to create temporary file")?;
    file.write_all(&bytes)
        .context("Failed to write zip file")?;

    pb.set_message("Extracting theme files...");

    let file = File::open(&zip_path).context("Failed to open zip file")?;
    let mut archive = zip::ZipArchive::new(file).context("Failed to read zip archive")?;

    fs::create_dir_all(project_path).context("Failed to create project directory")?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => {
                let components: Vec<_> = path.components().skip(1).collect();
                if components.is_empty() {
                    continue;
                }
                let mut dest = project_path.to_path_buf();
                for comp in components {
                    dest.push(comp);
                }
                dest
            }
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    pb.finish_with_message("Theme downloaded and extracted!");
    Ok(())
}
