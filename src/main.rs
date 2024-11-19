use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use clap::Parser;
use serde_yaml::Value;
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
#[command(
    name = "ML Cookie Cutter",
    about = "Creates ML project cookie cutter structure",
    after_help = "Enjoy and happy coding"
)]
struct Args {
    #[arg(long = "n", long = "name", help = "Name of the directory to be created, default = ml-cookie-project")]
    name: Option<String>,

    #[arg(long = "p", long = "path", help = "Provide the path where, default is $HOME dir")]
    path: Option<String>,

    #[arg(long = "c", long = "config", help = "Specify your own config file")]
    config: Option<String>,

    #[arg(long = "v", long = "venv", help = "Create a virtual env [ignore if you are already on a virtual env]")]
    venv: bool,
}

fn load_config(config_file: &Path) -> Result<Value> {
    let f = File::open(config_file)
        .with_context(|| format!("Failed to open config file: {}", config_file.display()))?;
    serde_yaml::from_reader(f)
        .with_context(|| "Failed to parse YAML config")
}

fn create_virtual_env(project_path: &Path) -> Result<()> {
    let venv_path = project_path.join("venv");
    println!("virtual env path is {}", venv_path.display());
    
    if !venv_path.exists() {
        // In Rust, we'll use the system's python to create the venv
        std::process::Command::new("python3")
            .args(["-m", "venv", venv_path.to_str().unwrap()])
            .output()
            .with_context(|| "Failed to create virtual environment")?;
        
        println!("virtual env created at {}", venv_path.display());
        println!("you can activate using the following command");
        println!("source {}/bin/activate", venv_path.display());
    }
    Ok(())
}

fn create_directories(project_path: &Path, config: &Value) -> Result<()> {
    match config {
        Value::String(s) => {
            let item_path = project_path.join(s);
            File::create(&item_path)
                .with_context(|| format!("Failed to create file: {}", item_path.display()))?;
        }
        Value::Sequence(seq) => {
            for item in seq {
                create_directories(project_path, item)?;
            }
        }
        Value::Mapping(map) => {
            for (key, value) in map {
                if let Value::String(key) = key {
                    if key == "files" {
                        if let Value::Sequence(files) = value {
                            for file in files {
                                if let Value::String(filename) = file {
                                    let file_path = project_path.join(filename);
                                    File::create(&file_path)
                                        .with_context(|| format!("Failed to create file: {}", file_path.display()))?;
                                }
                            }
                        }
                    } else {
                        let sub_path = project_path.join(key);
                        fs::create_dir_all(&sub_path)
                            .with_context(|| format!("Failed to create directory: {}", sub_path.display()))?;
                        create_directories(&sub_path, value)?;
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config_path = args.config
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config.yaml"));
    
    let config = load_config(&config_path)?;

    let project_name = args.name.unwrap_or_else(|| String::from("ml-cookie-cutter"));
    let project_base = args.path
        .map(PathBuf::from)
        .unwrap_or_else(|| env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(".")));

    let project_path = project_base.join(project_name);

    create_directories(&project_path, &config)?;
    println!("ML directory structure created at: {}", project_path.display());

    if args.venv {
        create_virtual_env(&project_path)?;
    }

    Ok(())
}
