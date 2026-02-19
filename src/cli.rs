use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "agentic", version, about = "Machine-aware agentic config compiler")]
pub struct Args {
    #[arg(long, default_value = "./layers")]
    pub layers: String,

    #[arg(long)]
    pub profile: Option<String>,

    #[arg(long)]
    pub machine: Option<String>,

    #[arg(long)]
    pub project: Option<String>,

    #[arg(long)]
    pub out: Option<String>,

    if args.explain {
        agentic::explain::explain(&output);
    }
}
