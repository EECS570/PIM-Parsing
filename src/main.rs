mod base_type_pim;
mod parser;
use crate::parser::parse_str;
use anyhow::Result;
use base_type_pim::GeneralBlock;
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn semantic_analysis<'input>(general: Vec<GeneralBlock<'input>>) {
    let mut node_types = HashMap::new();
    let mut edge_types = HashMap::new();
    let mut walker_types = HashMap::new();
    for block in general {
        match block {
            GeneralBlock::NodeBlock(node) => {
                node_types.insert(node.0.name, Rc::new(node.0));
            }
            GeneralBlock::EdgeBlock(edge) => {
                edge_types.insert(edge.named_block.name, Rc::new(edge));
            }
            GeneralBlock::WalkerBlock(walker) => {
                walker_types.insert(walker.name, Rc::new(walker));
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Reading from: {}", args.file);
    let file_content = fs::read_to_string(args.file)?;
    println!("File content: {}", file_content);
    let _ = semantic_analysis(parse_str(&file_content)?);
    Ok(())
}
