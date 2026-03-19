mod math;

use clap::{Parser, Subcommand};
use serde_json::{Value, from_str, json, to_string_pretty};
use std::{fs, path::Path};

const SKILL_CONTENT: &str = include_str!("../SKILL.md");

#[derive(Parser)]
#[command(name = "alu")]
#[command(about = "Agent Logic Unit CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Maybe at some point we also add support for symbolic expressions, algebra, calculus, etc.
#[derive(Subcommand)]
enum Commands {
    /// Evaluate a mathematical expression
    Eval {
        /// The mathematical expression to evaluate
        expression: String,
    },
    /// Automatically install ALU as a skill for Claude and Codex
    Init {
        /// Install into a specific directory instead of the global home directory
        directory: Option<std::path::PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Eval { expression } => {
            let result = math::eval(&expression)?;
            println!("Result: {}", result);
        }
        Commands::Init { directory } => {
            install_skills(directory)?;
        }
    }

    Ok(())
}

fn install_skills(directory: Option<std::path::PathBuf>) -> anyhow::Result<()> {
    let base = match directory {
        Some(dir) => {
            if !dir.exists() {
                anyhow::bail!("Directory does not exist: {}", dir.display());
            }
            if !dir.is_dir() {
                anyhow::bail!("Path is not a directory: {}", dir.display());
            }
            dir
        }
        None => dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?,
    };

    let paths = vec![
        base.join(".claude/"),
        base.join(".codex/"),
        base.join(".agents/"),
    ];

    for path in paths {
        let skill_file = path.join("skills/alu/SKILL.md");
        if let Some(parent) = skill_file.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&skill_file, SKILL_CONTENT)?;
        println!("Installed ALU skill in {:?}", skill_file);
        authorize_command(&path)?;
        println!(
            "ALU has been added to your auto-approve allowlist for {:?}",
            path
        );
    }

    println!("\nALU is now discoverable! Restart your agent (Claude/Codex) to begin.");

    Ok(())
}

fn authorize_command(path: &Path) -> anyhow::Result<()> {
    let settings_path = path.join("settings.json");

    let raw = fs::read_to_string(&settings_path).unwrap_or_else(|_| "{}".to_string());
    let mut config: Value = from_str(&raw).unwrap_or(json!({}));

    if !config["permissions"].is_object() {
        config["permissions"] = json!({});
    }

    if !config["permissions"]["allow"].is_array() {
        config["permissions"]["allow"] = json!([]);
    }

    let allow_list = config["permissions"]["allow"].as_array_mut().unwrap();

    let pattern = "Bash(alu *)";

    if !allow_list.iter().any(|v| v.as_str() == Some(pattern)) {
        allow_list.push(Value::String(pattern.to_string()));

        fs::create_dir_all(settings_path.parent().unwrap())?;
        fs::write(&settings_path, to_string_pretty(&config)?)?;
    }

    Ok(())
}
