mod cli;
mod config;

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

    println!("layer order:");
    for p in spec.ordered_paths() {
        println!("  {}", p.display());
    }

    Ok(())
}
