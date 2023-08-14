use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};

use crate::chunk_type::ChunkTypeWrapper;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    /// Path of the PNG file
    #[arg(short, long, value_name = "FILE")]
    pub file_path: PathBuf,
}

#[derive(Subcommand)]
pub enum Commands {
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
pub struct EncodeArgs {
    #[arg(value_parser = clap::value_parser!(ChunkTypeWrapper))]
    pub chunk_type: ChunkTypeWrapper,
    pub data: String,
    pub output_file: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodeArgs {
    #[arg(value_parser = clap::value_parser!(ChunkTypeWrapper))]
    pub chunk_type: ChunkTypeWrapper,
}

#[derive(Args)]
pub struct RemoveArgs {
    #[arg(value_parser = clap::value_parser!(ChunkTypeWrapper))]
    pub chunk_type: ChunkTypeWrapper,
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
