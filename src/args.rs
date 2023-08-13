use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};

use crate::chunk_type::ChunkTypeWrapper;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Path of the PNG file
    #[arg(short, long, value_name = "FILE")]
    file_path: PathBuf,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Encodes a message in a PNG file
    Encode(EncodeArgs),
    /// Show the message in a PNG file
    Decode(DecodeArgs),
    /// Removes a message in a PNG file
    Remove(RemoveArgs),
    /// Print all chunks in a PNG file
    Print
}

#[derive(Args)]
struct EncodeArgs {
    #[arg(long, value_parser = clap::value_parser!(ChunkTypeWrapper))]
    chunk_type: ChunkTypeWrapper,
    data: String,
    output_file: Option<PathBuf>,
}

#[derive(Args)]
struct DecodeArgs {
    #[arg(long, value_parser = clap::value_parser!(ChunkTypeWrapper))]
    chunk_type: ChunkTypeWrapper,
}

#[derive(Args)]
struct RemoveArgs {
    #[arg(long, value_parser = clap::value_parser!(ChunkTypeWrapper))]
    chunk_type: ChunkTypeWrapper,
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    pub fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
