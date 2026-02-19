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

    let detected_machine = if args.machine.is_none() {
        Some(agentic::context::detect_hostname()?)
    } else {
        None
    };

    let detected_project = if args.project.is_none() {
        agentic::context::detect_project_config()
    } else {
        None
    };

    let project_path_owned: Option<String> = match args.project {
        Some(ref p) => Some(p.clone()),
        None => detected_project
            .and_then(|p| p.to_str().map(|s| s.to_string())),
    };
    
    let machine_name = args.machine
        .as_deref()
        .or(detected_machine.as_deref());
    
    let merged = agentic::pipeline::build_merged_config(
        &args.layers,
        args.profile.as_deref(),
        machine_name,
        project_path_owned.as_deref(),
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
