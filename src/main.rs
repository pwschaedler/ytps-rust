use std::path::PathBuf;

use clap::{Parser, Subcommand};
use ytps_rust::login;

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
}

fn main() {
    let args = Args::parse();
    let res = match args.command {
        Commands::Login { client_secret } => login::execute(client_secret),
    };
    if let Err(err) = res {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
