mod settings;
mod cli;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::settings::RunEnvironment;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CliArguments {
    /// Run environment. Can be local, dev, test, prod (default: dev).
    #[clap(short, long, default_value("dev"))]
    run_env: RunEnvironment,

    /// Base directory where to find configuration files (default: ./config).
    #[clap(short, long, value_name="CONFIG_DIR", default_value("./config"))]
    config_dir: PathBuf,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the server in background mode
    Start,
    /// Start the server in foreground mode
    StartForeground,
    /// Stop the server
    Stop,
    /// Restart the server
    Restart,
    /// Display the running status
    Status,
    /// Starts the interactive command line
    Cli,
}

fn main() {
    let cli = CliArguments::parse();
    println!("Env: {:?} and config dir: {:?}", cli.run_env, cli.config_dir);
    let settings =
        settings::Settings::new(&cli.run_env, &cli.config_dir).expect("config can be loaded");

    match &cli.command {
        Commands::Start => {
            println!("Starting Zookeeper in background mode...");
        },
        Commands::StartForeground => {
            println!("Starting Zookeeper in foreground mode...");
        },
        Commands::Stop => {
            println!("Stopping Zookeeper...");
        },
        Commands::Restart => {
            println!("Restarting Zookeeper...");
        },
        Commands::Status => {
            println!("Zookeeper is started... or not started.");
        },
        Commands::Cli => {
            println!("Starting Zookeeper REPL...");
            cli::run_repl(&cli, &settings)
        }
    }

}