use clap::{Parser, Subcommand};
use agentic::adapters::adapter::CompileInput;

#[derive(Parser, Debug)]
#[command(
    name = "agentic",
    version,
    about = "Layered agent + vendor config compiler",
    long_about = "
agentic compiles layered YAML configuration and reusable
agent/skill repositories into vendor-specific AI tool configs.
"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build and write vendor configs
    Build(BuildArgs),

    /// Print merged config or resolved agent prompt
    Print(PrintArgs),

    /// Validate configuration without compiling adapters
    Validate(CommonArgs),
}

#[derive(Parser, Debug)]
struct CommonArgs {
    /// Path to layers directory
    #[arg(long, default_value = "./layers")]
    layers: String,

    /// Profile name (e.g. work, personal)
    #[arg(long)]
    profile: Option<String>,

    /// Override machine name (auto-detected if omitted)
    #[arg(long)]
    machine: Option<String>,

    /// Override project config path
    #[arg(long)]
    project: Option<String>,
}

#[derive(Parser, Debug)]
struct BuildArgs {
    #[command(flatten)]
    common: CommonArgs,

    /// Only compile specific vendor(s)
    #[arg(long)]
    vendor: Vec<String>,

    /// Print output instead of writing files
    #[arg(long)]
    dry_run: bool,

    /// Show how layered config values were resolved
    #[arg(long)]
    explain: bool,
}

#[derive(Parser, Debug)]
struct PrintArgs {
    #[command(flatten)]
    common: CommonArgs,

    /// Print merged JSON
    #[arg(long)]
    merged: bool,

    /// Print resolved agent prompt
    #[arg(long)]
    resolved: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build(args) => run_build(args),
        Commands::Print(args) => run_print(args),
        Commands::Validate(args) => run_validate(args),
    }
}

fn run_build(args: BuildArgs) -> anyhow::Result<()> {
    let output = build_pipeline(&args.common)?;

    if args.explain {
        agentic::explain::explain(&output);
    }

    let adapters = agentic::adapters::registry::all_adapters();

    for adapter in adapters {
        let compiled = adapter.compile(CompileInput {
            merged: &output.merged,
            resolved_agent_prompt: output.resolved_agent_prompt.as_deref(),
        })?;

        if args.dry_run {
            println!("=== {} ===", adapter.name());
            println!("{}", serde_json::to_string_pretty(&compiled)?);
            continue;
        }


        let path = adapter.default_output_path()?;
        agentic::output::write_json_to_path(&path, &compiled)?;
        println!("Wrote {} config to {}", adapter.name(), path.display());
    }

    Ok(())
}

fn run_validate(args: CommonArgs) -> anyhow::Result<()> {
    build_pipeline(&args)?;
    println!("Configuration is valid.");
    Ok(())
}

fn build_pipeline(common: &CommonArgs) -> anyhow::Result<agentic::pipeline::PipelineOutput> {
    let detected_machine = if common.machine.is_none() {
        Some(agentic::context::detect_hostname()?)
    } else {
        None
    };

    let machine_name = common.machine.as_deref()
        .or(detected_machine.as_deref());

    let project_path_owned = common.project.clone();

    agentic::pipeline::build_output(
        &common.layers,
        common.profile.as_deref(),
        machine_name,
        project_path_owned.as_deref(),
    )
}

fn run_print(args: PrintArgs) -> anyhow::Result<()> {
    let output = build_pipeline(&args.common)?;

    if args.merged {
        println!("{}", serde_json::to_string_pretty(&output.merged)?);
    }

    if args.resolved {
        if let Some(prompt) = output.resolved_agent_prompt {
            println!("{}", prompt);
        }
    }

    Ok(())
}



