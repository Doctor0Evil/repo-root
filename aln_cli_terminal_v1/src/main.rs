use aln_cli_terminal_v1::commands::run_exec_acts_sys_maintenance_v1_5;
use clap::Parser;

mod commands;

#[derive(Parser, Debug)]
#[command(
    name = "aln-terminal",
    version = "1.0.1.5",
    about = "ALN terminal commands executor for syntax evolution and VM deployment"
)]
struct Cli {
    #[arg(long, default_value = "exec.acts.sys.maintenance_v1.5")]
    command: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command.as_str() {
        "exec.acts.sys.maintenance_v1.5" => run_exec_acts_sys_maintenance_v1_5().await?,
        other => {
            eprintln!("Unknown command: {}", other);
        }
    }

    Ok(())
}
