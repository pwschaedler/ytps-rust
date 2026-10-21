use std::path::PathBuf;

use clap::{Parser, Subcommand};
use ytps_rust::{login, refresh};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command[subcommand]]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Login using Google OAuth to enable actions on user data
    Login {
        /// Path to JSON client secret from Google Cloud Console
        client_secret: PathBuf,
    },

    /// Refresh cached playlist items
    Refresh,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let res = match args.command {
        Commands::Login { client_secret } => login::execute(client_secret).await,
        Commands::Refresh => refresh::execute().await,
    };
    if let Err(err) = res {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
