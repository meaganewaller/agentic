mod cli;
mod config;
mod io;
mod convert;
mod merge;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use config::layer::LayerSpec;
use convert::yaml_to_json;
use io::read_yaml;
use merge::deep_merge;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    let layers_dir = PathBuf::from(&args.layers);

    let spec = LayerSpec {
        base: layers_dir.join("base.yaml"),
        profile: args.profile.as_ref()
            .map(|p| layers_dir.join(format!("profile-{p}.yaml"))),
        machine: args.machine.as_ref()
            .map(|m| layers_dir.join(format!("machine-{m}.yaml"))),
        project: args.project.as_ref().map(PathBuf::from),
    };

    let mut merged = serde_json::Value::Object(Default::default());

    for path in spec.ordered_paths() {
        if !path.exists() {
            eprintln!("warning: missing {}", path.display());
            continue;
        }

        let yaml = read_yaml(&path)?;
        let json = yaml_to_json(yaml);
        merged = deep_merge(merged, json);
    }

    println!("{}", serde_json::to_string_pretty(&merged)?);

    Ok(())
}
