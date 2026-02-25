use std::{fs, path::PathBuf, process::ExitCode};

use clap::Parser;
use reclaw_migrate_openclaw::{migrate_config, OpenClawConfig};

#[derive(Debug, Parser)]
#[command(name = "reclaw-migrate-openclaw", version)]
struct Args {
    #[arg(long)]
    input: PathBuf,

    #[arg(long)]
    output: Option<PathBuf>,

    #[arg(long)]
    dry_run: bool,

    #[arg(long)]
    force: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("migration failed: {error}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse();

    if !args.dry_run && args.output.is_none() {
        return Err("--output is required unless --dry-run is set".to_owned());
    }

    let input_text = fs::read_to_string(&args.input)
        .map_err(|error| format!("failed to read {}: {error}", args.input.display()))?;

    let source: OpenClawConfig = serde_json::from_str(&input_text)
        .map_err(|error| format!("failed to parse input JSON: {error}"))?;

    let target = migrate_config(source).map_err(|error| error.to_string())?;
    let toml_text = toml::to_string_pretty(&target)
        .map_err(|error| format!("failed to encode TOML output: {error}"))?;

    if args.dry_run {
        println!("{toml_text}");
        return Ok(());
    }

    let output = args
        .output
        .ok_or_else(|| "missing output path".to_owned())?;

    if output.exists() && !args.force {
        return Err(format!(
            "{} already exists; use --force to overwrite",
            output.display()
        ));
    }

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }

    fs::write(&output, toml_text)
        .map_err(|error| format!("failed to write {}: {error}", output.display()))?;

    println!("wrote migrated config to {}", output.display());
    Ok(())
}
