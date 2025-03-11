mod parser;
mod base_type_pim;
use std::fs;
use clap::Parser;
use crate::parser::parse_str;
use anyhow::Result;

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
    let _ = parse_str(&file_content)?;
    Ok(())
}
