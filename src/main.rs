mod base_type;
mod parser;
mod semantics_analysis;
use crate::parser::parse_str;
use anyhow::Result;
use clap::Parser;
use semantics_analysis::semantic_analysis;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Reading from: {}", args.file);
    let file_content = fs::read_to_string(args.file)?;
    println!("File content: {}", file_content);
    let _ = semantic_analysis(parse_str(&file_content)?);
    Ok(())
}
