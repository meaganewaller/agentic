use anyhow::Result;
use clap::Parser;

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
    out: Option<String>,
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

    let compiled = agentic::adapters::claude::compile(&merged)?;

    if args.dry_run {
        println!("{}", serde_json::to_string_pretty(&compiled)?);
        println!("(dry-run: not writing to disk)");
        return Ok(());
    }

    let output_path = if let Some(out) = args.out {
        std::path::PathBuf::from(out)
    } else {
        agentic::output::default_claude_path()?
    };

    agentic::output::write_json_to_path(&output_path, &compiled)?;

    println!("Wrote Claude config to {}", output_path.display());

    Ok(())
}

