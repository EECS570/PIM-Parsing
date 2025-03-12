use crate::base_type_pim::{Edge, GeneralBlock, NamedBlock, Walker};
use anyhow::Result;
use std::collections::HashMap;
use std::rc::Rc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SemanticsError {
    #[error("Token `{0}` is not undefined.")]
    UndefinedToken(String),
    #[error("Unknown error.")]
    Unknown,
}

pub struct SemanticEdge {
    pub from: Rc<NamedBlock>,
    pub to: Rc<NamedBlock>,
    pub named_block: NamedBlock,
}

pub struct SemanticWalker {
    pub name: String,
    pub node_type: Rc<NamedBlock>,
}

fn transform_edge_hashmap_to_semantic<'input>(
    node_types: &HashMap<String, Rc<NamedBlock>>,
    edge_types: HashMap<String, Edge>,
) -> Result<HashMap<String, Rc<SemanticEdge>>> {
    let mut semantic_edge_types = HashMap::new();
    for (_, edge) in edge_types {
        semantic_edge_types.insert(
            edge.named_block.name.clone(),
            Rc::new(SemanticEdge {
                from: node_types
                    .get(&edge.from)
                    .ok_or(SemanticsError::UndefinedToken(String::from(edge.from)))?
                    .clone(),
                to: node_types
                    .get(&edge.to)
                    .ok_or(SemanticsError::UndefinedToken(String::from(edge.to)))?
                    .clone(),
                named_block: edge.named_block,
            }),
        );
    }
    Ok(semantic_edge_types)
}

fn transform_walker_hashmap_to_semantic(
    node_types: &HashMap<String, Rc<NamedBlock>>,
    walker_types: HashMap<String, Walker>,
) -> Result<HashMap<String, Rc<SemanticWalker>>> {
    let mut semantic_walker_types = HashMap::new();
    for (_, walker) in walker_types {
        semantic_walker_types.insert(
            walker.name.clone(),
            Rc::new(SemanticWalker {
                name: walker.name,
                node_type: node_types
                    .get(&walker.node_type)
                    .ok_or(SemanticsError::UndefinedToken(String::from(
                        walker.node_type,
                    )))?
                    .clone(),
            }),
        );
    }
    Ok(semantic_walker_types)
}

pub fn semantic_analysis(general: Vec<GeneralBlock>) -> Result<()> {
    let mut node_types = HashMap::new();
    let mut edge_types = HashMap::new();
    let mut walker_types = HashMap::new();
    for block in general {
        match block {
            GeneralBlock::NodeBlock(node) => {
                node_types.insert(node.0.name.clone(), Rc::new(node.0));
            }
            GeneralBlock::EdgeBlock(edge) => {
                edge_types.insert(edge.named_block.name.clone(), edge);
            }
            GeneralBlock::WalkerBlock(walker) => {
                walker_types.insert(walker.name.clone(), walker);
            }
            _ => {}
        }
    }

    let semantic_edge_types = transform_edge_hashmap_to_semantic(&node_types, edge_types)?;
    let semantic_walker_types = transform_walker_hashmap_to_semantic(&node_types, walker_types)?;

    Ok(())
}
