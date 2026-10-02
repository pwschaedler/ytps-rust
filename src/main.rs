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
    Login,
}

fn main() {
    let args = Args::parse();
    let res = match args.command {
        Commands::Login => login::execute(),
    };
    if let Err(err) = res {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
