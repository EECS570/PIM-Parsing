mod base_type_pim;
mod parser;
use crate::parser::parse_str;
use anyhow::Result;
use base_type_pim::{Edge, GeneralBlock, NamedBlock, Walker};
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SemanticsError {
    #[error("Token `{0}` is not undefined.")]
    UndefinedToken(String),
    #[error("Unknown error.")]
    Unknown,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

pub struct SemanticEdge<'a> {
    pub from: Rc<NamedBlock<'a>>,
    pub to: Rc<NamedBlock<'a>>,
    pub named_block: NamedBlock<'a>,
}

pub struct SemanticWalker<'a> {
    pub name: &'a str,
    pub node_type: Rc<NamedBlock<'a>>,
}

fn transform_edge_hashmap_to_semantic<'input>(
    node_types: &HashMap<&'input str, Rc<NamedBlock<'input>>>,
    edge_types: HashMap<&'input str, Edge<'input>>,
) -> Result<HashMap<&'input str, Rc<SemanticEdge<'input>>>> {
    let mut semantic_edge_types = HashMap::new();
    for (_, edge) in edge_types {
        semantic_edge_types.insert(
            edge.named_block.name,
            Rc::new(SemanticEdge {
                from: node_types
                    .get(edge.from)
                    .ok_or(SemanticsError::UndefinedToken(String::from(edge.from)))?
                    .clone(),
                to: node_types
                    .get(edge.to)
                    .ok_or(SemanticsError::UndefinedToken(String::from(edge.to)))?
                    .clone(),
                named_block: edge.named_block,
            }),
        );
    }
    Ok(semantic_edge_types)
}

fn transform_walker_hashmap_to_semantic<'input>(
    node_types: &HashMap<&'input str, Rc<NamedBlock<'input>>>,
    walker_types: HashMap<&'input str, Walker<'input>>,
) -> Result<HashMap<&'input str, Rc<SemanticWalker<'input>>>> {
    let mut semantic_walker_types = HashMap::new();
    for (_, walker) in walker_types {
        semantic_walker_types.insert(
            walker.name,
            Rc::new(SemanticWalker {
                name: walker.name,
                node_type: node_types
                    .get(walker.node_type)
                    .ok_or(SemanticsError::UndefinedToken(String::from(
                        walker.node_type,
                    )))?
                    .clone(),
            }),
        );
    }
    Ok(semantic_walker_types)
}

fn semantic_analysis<'input>(general: Vec<GeneralBlock<'input>>) -> Result<()> {
    let mut node_types = HashMap::new();
    let mut edge_types = HashMap::new();
    let mut walker_types = HashMap::new();
    for block in general {
        match block {
            GeneralBlock::NodeBlock(node) => {
                node_types.insert(node.0.name, Rc::new(node.0));
            }
            GeneralBlock::EdgeBlock(edge) => {
                edge_types.insert(edge.named_block.name, edge);
            }
            GeneralBlock::WalkerBlock(walker) => {
                walker_types.insert(walker.name, walker);
            }
            _ => {}
        }
    }

    let semantic_edge_types = transform_edge_hashmap_to_semantic(&node_types, edge_types)?;
    let semantic_walker_types = transform_walker_hashmap_to_semantic(&node_types, walker_types)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Reading from: {}", args.file);
    let file_content = fs::read_to_string(args.file)?;
    println!("File content: {}", file_content);
    let _ = semantic_analysis(parse_str(&file_content)?);
    Ok(())
}
