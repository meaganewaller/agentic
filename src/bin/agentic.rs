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

    /// Vendor(s) to build (e.g. --vendor claude --vendor codex)
    #[arg(long)]
    vendor: Vec<String>,
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

    // Determine which vendors are requested
    let requested_vendors = if args.vendor.is_empty() {
        None
    } else {
        Some(args.vendor.iter().map(|v| v.to_lowercase()).collect::<Vec<_>>())
    };

    for adapter in adapters {
        let name = adapter.name();

        // Skip if vendor filter provided and this one isn't included
        if let Some(ref list) = requested_vendors {
            if !list.contains(&name.to_string()) {
                continue;
            }
        }

        // Skip if vendor doesn't exist in config
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
