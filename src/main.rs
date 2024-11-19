//! # ML Project Structure Generator
//! 
//! This tool creates a standardized directory structure for machine learning projects in Rust.
//! It sets up a workspace with multiple crates, configuration files, and development tools.
//! 
//! ## Features
//! 
//! - Creates a Cargo workspace with specialized ML crates
//! - Sets up directory structure for data, models, and documentation
//! - Configures development tools and testing environment
//! - Supports DVC for data version control
//! - Includes Docker configuration
//! 
//! ## Usage
//! 
//! ```bash
//! ml-cookie-cutter --name my-project --path /path/to/project
//! ```

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use anyhow::{Context, Error, Result};

/// Command line arguments for the ML project generator
#[derive(Parser, Debug)]
#[command(
    name = "ML Cookie Cutter",
    about = "Creates Rust ML project structure",
    after_help = "Enjoy and happy coding with Rust ML!"
)]
struct Args {
    /// Name of the directory to be created
    #[arg(long = "n", long = "name", help = "Name of the directory to be created, default = rust-ml-project")]
    name: Option<String>,

    /// Base path for project creation
    #[arg(long = "p", long = "path", help = "Provide the path where, default is $HOME dir")]
    path: Option<String>,

    /// Custom configuration file path
    #[arg(long = "c", long = "config", help = "Specify your own config file")]
    config: Option<String>,
}

/// Represents a directory or file in the project structure
#[derive(Debug, Serialize, Deserialize)]
struct ProjectItem {
    path: PathBuf,
    item_type: ItemType,
    content: Option<String>,
}

/// Enum representing different types of items in the project structure
#[derive(Debug, Serialize, Deserialize)]
enum ItemType {
    Directory,
    File,
    Crate,
}

/// Creates initial content for various Rust files
fn create_initial_content(path: &Path) -> Option<String> {
    match path.file_name()?.to_str()? {
        "lib.rs" => Some(r#"//! Main library module
            
#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod error;
pub mod types;
"#.into()),
        "Cargo.toml" => Some(r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#.into()),
        "README.md" => Some(r#"# Rust ML Project

## Overview

This is a machine learning project implemented in Rust.

## Project Structure

- `ml-core`: Core functionality and shared types
- `ml-data`: Data processing and pipelines
- `ml-models`: Model implementations
- `ml-train`: Training and evaluation

## Getting Started

1. Install Rust
2. Build: `cargo build`
3. Test: `cargo test`
"#.into()),
        _ => None,
    }
}

/// Creates the basic directory structure for a Rust crate
fn create_crate_structure(base_path: &Path, crate_name: &str) -> Result<()> {
    let crate_path = base_path.join(crate_name);
    fs::create_dir_all(&crate_path)?;
    
    // Create standard crate directories
    for dir in &["src", "tests", "benches", "examples"] {
        fs::create_dir_all(crate_path.join(dir))?;
    }
    
    // Create basic files
    let files = vec![
        ("src/lib.rs", Some(format!("//! {} crate\n\n#![warn(missing_docs)]\n", crate_name))),
        ("Cargo.toml", Some(format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#, crate_name))),
        ("tests/integration_test.rs", Some("//! Integration tests\n".into())),
    ];

    for (file_path, content) in files {
        let full_path = crate_path.join(file_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(full_path)?;
        if let Some(content) = content {
            file.write_all(content.as_bytes())?;
        }
    }

    Ok(())
}

/// Processes the YAML configuration and creates the project structure
fn process_yaml_structure(base_path: &Path, value: &Value) -> Result<()> {
    match value {
        Value::Sequence(seq) => {
            for item in seq {
                process_yaml_structure(base_path, item)?;
            }
        }
        Value::Mapping(map) => {
            for (key, value) in map {
                if let Value::String(key) = key {
                    let new_path = base_path.join(key);
                    
                    // Special handling for crates
                    if key == "crates" {
                        if let Value::Mapping(crates) = value {
                            for (crate_name, _) in crates {
                                if let Value::String(crate_name) = crate_name {
                                    create_crate_structure(base_path, crate_name)?;
                                }
                            }
                        }
                    } else {
                        fs::create_dir_all(&new_path)?;
                        process_yaml_structure(&new_path, value)?;
                    }
                }
            }
        }
        Value::String(filename) => {
            let file_path = base_path.join(filename);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            File::create(file_path)?;
        }
        _ => {}
    }
    Ok(())
}

/// Creates the workspace Cargo.toml file
fn create_workspace_cargo_toml(project_path: &Path) -> Result<()> {
    let content = r#"[workspace]
members = [
    "crates/ml-core",
    "crates/ml-data",
    "crates/ml-models",
    "crates/ml-train",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = []

[workspace.dependencies]
# ML and numerical computing
ndarray = { version = "0.15", features = ["rayon"] }
polars = { version = "0.35", features = ["lazy"] }
rand = "0.8"
rayon = "1.8"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging and metrics
tracing = "0.1"
tracing-subscriber = "0.3"

[profile.dev]
opt-level = 1

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
"#;
    
    let cargo_path = project_path.join("Cargo.toml");
    let mut file = File::create(cargo_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Main function to set up the project structure
fn main() -> Result<()> {
    let args = Args::parse();

    // Set up project path
    let project_name = args.name.unwrap_or_else(|| String::from("rust-ml-project"));
    let project_base = args.path
        .map(PathBuf::from)
        .unwrap_or_else(|| env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(".")));
    let project_path = project_base.join(&project_name);

    // Create main project directory
    fs::create_dir_all(&project_path)
        .with_context(|| format!("Failed to create project directory at {}", project_path.display()))?;

    // Load and process YAML configuration
    let config_path = args.config
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config.yaml"));
    
    let config = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
    
    let yaml: Value = serde_yaml::from_str(&config)
        .with_context(|| "Failed to parse YAML config")?;

    // Process the YAML structure
    process_yaml_structure(&project_path, &yaml)?;

    // Create workspace Cargo.toml
    create_workspace_cargo_toml(&project_path)?;

    println!("✨ Rust ML project structure created at: {}", project_path.display());
    println!("\n📦 Next steps:");
    println!("1. cd {}", project_name);
    println!("2. cargo build    # To verify everything is set up correctly");
    println!("3. cargo test     # To run tests");
    println!("\n🛠 Recommended development tools:");
    println!("- cargo install cargo-watch   # For development with auto-reload");
    println!("- cargo install cargo-flamegraph  # For performance profiling");
    println!("- cargo install cargo-criterion   # For benchmarking");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_crate_structure() -> Result<()> {
        let temp_dir = tempdir()?;
        create_crate_structure(&temp_dir.path(), "test-crate")?;
        
        // Verify basic structure
        assert!(temp_dir.path().join("test-crate/src/lib.rs").exists());
        assert!(temp_dir.path().join("test-crate/Cargo.toml").exists());
        assert!(temp_dir.path().join("test-crate/tests").exists());
        
        Ok(())
    }

    // Add more tests as needed
}
