mod detection;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// List names of variants that this provider provides
    Names,
    /// List state of environment in terms of variants that this provider provides
    State,
    /// List constraints that the current state imposes on other packages and variants
    Constraints,
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Commands::Names => {
            println!("{}", detection::get_variant_names().join("\n"))
        },
        Commands::State => {
            println!("{}", serde_json::to_string(&detection::State::new()).unwrap())
        },
        Commands::Constraints => {
            println!("Constraints")
        },
    }
}
