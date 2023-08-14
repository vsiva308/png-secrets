use crate::Result;
use crate::args::*;
use crate::chunk::Chunk;
use crate::png::Png;
use std::fs;
use std::path::PathBuf;

pub fn encode(args: &EncodeArgs, file_path: &PathBuf) -> Result<()> {
    let output_opt = args.output_file.as_ref();
    let output_file = output_opt.unwrap_or(file_path);

    let input_file = fs::read(file_path)?;
    let bytes = input_file.as_slice();
    let mut png = Png::try_from(bytes)?;

    let chunk = Chunk::new(args.chunk_type.0.clone(), args.data.as_bytes().to_vec());
    png.append_chunk(chunk);

    fs::write(output_file, png.as_bytes())?;
    Ok(())
}

pub fn decode(args: &DecodeArgs, file_path: &PathBuf) -> Result<()> {
    let input_file = fs::read(file_path)?;
    let bytes = input_file.as_slice();
    let png = Png::try_from(bytes)?;

    let chunk_type = String::from_utf8(Vec::from(args.chunk_type.0.bytes()))?;
    let chunk = png.chunk_by_type(&chunk_type);

    match chunk {
        Some(x) => println!("{}", x.data_as_string()?),
        None => println!("Data not found for this decoding!"),
    }

    Ok(())
}

pub fn remove(args: &RemoveArgs, file_path: &PathBuf) -> Result<()> {
    let input_file = fs::read(file_path)?;
    let bytes = input_file.as_slice();
    let mut png = Png::try_from(bytes)?;
    
    let chunk_type = String::from_utf8(Vec::from(args.chunk_type.0.bytes()))?;
    match png.remove_chunk(&chunk_type) {
        Ok(x) => println!("Chunk Removed: {}", x),
        Err(e) => println!("Error removing chunk: {}", e),
    }

    Ok(())
}

pub fn print(file_path: &PathBuf) -> Result<()> {
    let input_file = fs::read(file_path)?;
    let bytes = input_file.as_slice();
    let png = Png::try_from(bytes)?;

    println!("Chunks:");
    println!("{png}");

    Ok(())
}