mod helpers;

use clap::{Parser, Subcommand};
use helpers::{filter, read, run, save};

#[derive(Subcommand)]
enum Commands {
    Export {
        #[arg(short, long, value_name = "SAVE PATH")]
        path: String,

        #[arg(long, value_name = "VECTOR DB CANISTER ID")]
        canister: String,

        #[arg(long, value_name = "OWNER's IDENTITY")]
        identity: String,

        #[arg(long, value_name = "NETWORK", default_value = "local")]
        network: String,
    },
    Import {
        #[arg(short, long, value_name = "SAVE PATH")]
        path: String,

        #[arg(long, value_name = "VECTOR DB CANISTER ID")]
        canister: String,

        #[arg(long, value_name = "OWNER's IDENTITY")]
        identity: String,

        #[arg(long, value_name = "NETWORK", default_value = "local")]
        network: String,
    },
}

#[derive(clap::Parser)]
#[command(
    name = "my_cli_tool",
    version = "1.0",
    author = "Your Name <you@example.com>",
    about = "Does awesome things"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Export {
            path,
            canister,
            identity,
            network,
        } => {
            let cmd = format!(
                "canister call {} --network {} --identity {}  export ",
                canister, network, identity,
            );

            let args: Vec<&str> = cmd.split_whitespace().collect();
            // println!("{:?}", args);
            let stdout = run(args);
            let result = filter(&stdout).expect("Error");
            let _ = save(result, path.to_string());
        }
        Commands::Import {
            path,
            canister,
            identity,
            network,
        } => {
            // let network: Network = network.parse().expect("Invalid network type");
            let data = read(path.to_string());
            let cmd = format!(
                "canister call {} --network {} --identity {} import {}",
                canister, network, identity, data
            );

            let args: Vec<&str> = cmd.split_whitespace().collect();
            let result = run(args);
            println!("Result: {result}");
        }
    }
}
