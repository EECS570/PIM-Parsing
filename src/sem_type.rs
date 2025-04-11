use crate::base_type::NamedBlock;
use std::collections::HashMap;
use std::rc::Rc;

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
    pub weight: i64,
}

pub struct SemanticWalkerInst {
    pub walker_type: Rc<SemanticWalker>,
    pub start_node: Rc<SemanticNodeInst>,
}

#[derive(Clone)]
pub struct SemanticGraph {
    pub node_insts: Vec<Rc<SemanticNodeInst>>,
    pub edge_insts: Vec<Rc<SemanticEdgeInst>>,
    pub walker_insts: Vec<Rc<SemanticWalkerInst>>,
}

#[derive(Clone)]
pub struct SemanticGlobal {
    pub edges: HashMap<String, Rc<SemanticEdge>>,
    pub walkers: HashMap<String, Rc<SemanticWalker>>,
    pub graphs: Vec<SemanticGraph>,
}
