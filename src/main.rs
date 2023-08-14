mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use clap::Parser;

use args::Commands;
use commands::{encode, decode, remove, print};

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    let input_file = &cli.file_path;

    match &cli.command {
        Commands::Encode(args) => encode(args, input_file),
        Commands::Decode(args) => decode(args, input_file),
        Commands::Remove(args) => remove(args, input_file),
        Commands::Print => print(input_file)
    }
}
