use anyhow::Result;
use clap::Parser;

use agentic::adapters::adapter::VendorAdapter;
use agentic::adapters::{claude::ClaudeAdapter, codex::CodexAdapter};

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value = "./layers")]
    layers: String,
    #[arg(long)]
    profile: Option<String>,
    #[arg(long)]
    machine: Option<String>,
    #[arg(long)]
    project: Option<String>,
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let merged = agentic::pipeline::build_merged_config(
        &args.layers,
        args.profile.as_deref(),
        args.machine.as_deref(),
        args.project.as_deref(),
    )?;

    let adapters: Vec<Box<dyn VendorAdapter>> = vec![
        Box::new(ClaudeAdapter),
        Box::new(CodexAdapter),
    ];

    for adapter in adapters {
        // Only process if vendor exists in config
        if merged
            .get("vendors")
            .and_then(|v| v.get(adapter.name()))
            .is_none()
        {
            continue;
        }

        let compiled = adapter.compile(&merged)?;

        if args.dry_run {
            println!("=== {} ===", adapter.name());
            println!("{}", serde_json::to_string_pretty(&compiled)?);
            continue;
        }

        let output_path = adapter.default_output_path()?;
        agentic::output::write_json_to_path(&output_path, &compiled)?;

        println!("Wrote {} config to {}", adapter.name(), output_path.display());
    }

    Ok(())
}
