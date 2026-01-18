use anyhow::{Context, Result};
use colored::Colorize;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

pub fn setup_tailwind(project_path: &Path) -> Result<()> {
    println!("{} {}", "🎨".bold(), "Setting up Tailwind CSS...".yellow());

    let package_json_path = project_path.join("package.json");
    let package_json_content =
        fs::read_to_string(&package_json_path).context("Failed to read package.json")?;

    let mut package_json: Value =
        serde_json::from_str(&package_json_content).context("Failed to parse package.json")?;

    if let Some(scripts) = package_json
        .get_mut("scripts")
        .and_then(|s| s.as_object_mut())
    {
        scripts.insert(
            "build:css".to_string(),
            json!("postcss styles/tailwind.css -o assets/tailwind.css"),
        );
        scripts.insert(
            "watch:css".to_string(),
            json!("postcss styles/tailwind.css -o assets/tailwind.css --watch"),
        );
        scripts.insert(
            "start".to_string(),
            json!("concurrently -k -r \"rollup -c -w\" \"npm run watch:css\" \"wait-on script.js style.css && zcli themes:preview\""),
        );
    }

    if let Some(dev_deps) = package_json
        .get_mut("devDependencies")
        .and_then(|d| d.as_object_mut())
    {
        dev_deps.insert("autoprefixer".to_string(), json!("^10.4.21"));
        dev_deps.insert("postcss".to_string(), json!("^8.5.3"));
        dev_deps.insert("postcss-cli".to_string(), json!("^11.0.1"));
        dev_deps.insert("tailwindcss".to_string(), json!("^3.4.17"));
    }

    let updated_content =
        serde_json::to_string_pretty(&package_json).context("Failed to serialize package.json")?;
    fs::write(&package_json_path, updated_content).context("Failed to write package.json")?;

    let styles_dir = project_path.join("styles");
    let sources_css = r#"
        @tailwind base;
        @tailwind components;
        @tailwind utilities;

        @layer utilities {
          .debug-border {
            @apply border border-red-500;
          }
        }
      "#;

    let sources_css_path = styles_dir.join("tailwind.css");
    fs::write(&sources_css_path, sources_css).context("Failed to create tailwind source CSS")?;

    // Create tailwind.config.js
    let tailwind_config = r#"
      /* eslint-disable no-undef */
      /* eslint-disable @typescript-eslint/no-var-requires */
      /** @type {import('tailwindcss').Config} */
            const defaultTheme = require("tailwindcss/defaultTheme");
            module.exports = {
              content: [
                "./templates/**/*.hbs",
                "./src/**/*.{js,ts}",
                "./src/modules/**/*.tsx",
              ],
              theme: {
                extend: {},
              },
              plugins: [],
            }
      "#;

    let tailwind_config_path = project_path.join("tailwind.config.js");
    fs::write(&tailwind_config_path, tailwind_config)
        .context("Failed to create tailwind.config.js")?;

    let postcssconfig = 
    r#"
      module.exports = {
        plugins: {
          tailwindcss: {},
          autoprefixer: {},
          ...(process.env.NODE_ENV === "production"
            ? {
                "@fullhuman/postcss-purgecss": {
                  content: ["./templates/**/*.hbs"],
                  defaultExtractor: (content) =>
                    content.match(/[\w-/:]+(?<!:)/g) || [],
                },
              }
            : {}),
        },
        };
    "#;

    let postcssconfigpath = project_path.join("postcss.config.cjs"); 
     fs::write(&postcssconfigpath, postcssconfig)
        .context("Failed to create postcss.config.js")?;

    let document_head_path = project_path.join("templates").join("document_head.hbs");

    if document_head_path.exists() {
        let content = fs::read_to_string(&document_head_path)
            .context("Failed to read document_head.hbs")?;

        if !content.contains("tailwind.css") {
            let tailwind_link = r#"<link rel="stylesheet" href="{{asset 'tailwind.css'}}" />"#;
            let new_content = content.replace(
                r#"<meta content="width=device-width, initial-scale=1.0" name="viewport" />"#,
                &format!(
                    r#"<meta content="width=device-width, initial-scale=1.0" name="viewport" />{}"#,
                    tailwind_link
                ),
            );

            fs::write(&document_head_path, new_content)
                .context("Failed to update document_head.hbs")?;

            println!("  {} {}", "✓".green(), "Added Tailwind CSS to document_head.hbs");
        } else {
            println!("  {} {}", "✓".green(), "Tailwind CSS already in document_head.hbs");
        }
    } else {
        println!("  {} {}", "⚠".yellow(), "document_head.hbs not found");
    }
    // let safety_kleen_path = styles_dir.join("safetykleen");
    // fs::remove_dir_all(&safety_kleen_path).context("Failed to delete styles/safetyKleen")?;

    Ok(())
}
