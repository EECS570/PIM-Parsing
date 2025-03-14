use crate::base_type_pim::{Edge, GeneralBlock, Graph, NamedBlock, Walker};
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

pub struct SemanticNodeInst {
    pub varname: String,
    pub node_type: Rc<NamedBlock>,
}

pub struct SemanticEdgeInst {
    pub edge_type: Rc<SemanticEdge>,
    pub from_var: Rc<SemanticNodeInst>,
    pub to_var: Rc<SemanticNodeInst>,
    pub weight: i32,
}

pub struct SemanticGraph {
    pub node_insts: Vec<Rc<SemanticNodeInst>>,
    pub edge_insts: Vec<Rc<SemanticEdgeInst>>,
}

pub struct SemanticGlobal {
    pub edges: HashMap<String, Rc<SemanticEdge>>,
    pub walkers: HashMap<String, Rc<SemanticWalker>>,
    pub graphs: Vec<SemanticGraph>,
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

fn transform_graph_to_semantic(
    node_types: &HashMap<String, Rc<NamedBlock>>,
    walker_types: &HashMap<String, Rc<SemanticWalker>>,
    graphs: &Vec<Graph>,
) -> Result<()> {
    let sem_graphs: Result<Vec<SemanticGraph>> = graphs
        .into_iter()
        .map(|graph| -> Result<SemanticGraph> {
            let sem_node_insts: Result<Vec<Rc<SemanticNodeInst>>> = 
                graph
                .node_insts
                .iter()
                .map(|inst| -> Result<Rc<SemanticNodeInst>> {
                    Ok(Rc::new(SemanticNodeInst {
                        varname: inst.varname.clone(),
                        node_type: node_types
                            .get(&inst.node_type)
                            .ok_or(SemanticsError::UndefinedToken(String::from(
                                inst.varname.clone(),
                            )))?
                            .clone(),
                    }))
                })
                .into_iter()
                .collect();
            Ok(SemanticGraph {
                node_insts: sem_node_insts?,
                edge_insts: Vec::from([]),
            })
        }).collect();


    Ok(())
}

pub fn semantic_analysis(general: Vec<GeneralBlock>) -> Result<SemanticGlobal> {
    let mut node_types = HashMap::new();
    let mut edge_types = HashMap::new();
    let mut walker_types = HashMap::new();
    let mut graphs = Vec::new();
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
            GeneralBlock::GraphBlock(g) => {
                graphs.push(g);
            }
        }
    }

    let semantic_edge_types = transform_edge_hashmap_to_semantic(&node_types, edge_types)?;
    let semantic_walker_types = transform_walker_hashmap_to_semantic(&node_types, walker_types)?;

    Ok(SemanticGlobal {
        edges: semantic_edge_types,
        walkers: semantic_walker_types,
    })
}
