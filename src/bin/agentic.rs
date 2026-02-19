use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value = "./layers")]
    layers: String,
    #[arg(long)] profile: Option<String>,
    #[arg(long)] machine: Option<String>,
    #[arg(long)] project: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let merged = agentic::pipeline::build_merged_config(
        &args.layers,
        args.profile.as_deref(),
        args.machine.as_deref(),
        args.project.as_deref(),
    )?;

    println!("{}", serde_json::to_string_pretty(&merged)?);
    Ok(())
}
