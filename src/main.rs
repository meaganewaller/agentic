mod cli;
mod config;
mod io;
mod convert;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use config::layer::LayerSpec;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    let layers_dir = PathBuf::from(&args.layers);
    let spec = LayerSpec {
        base: layers_dir.join("base.yaml"),
        profile: args.profile.as_ref().map(|p| layers_dir.join(format!("profile-{p}.yaml"))),
        machine: args.machine.as_ref().map(|m| layers_dir.join(format!("machine-{m}.yaml"))),
        project: args.project.as_ref().map(PathBuf::from),
    };

    for p in spec.ordered_paths() {
        if !p.exists() {
            eprintln!("warning: missing {}", p.display());
            continue;
        }
        let yaml = io::read_yaml(&p)?;
        let json = convert::yaml_to_json(yaml);
        println!("--- {} ---\n{}", p.display(), serde_json::to_string_pretty(&json)?);
    }

    Ok(())
}
