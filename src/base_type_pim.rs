pub trait Size {
    fn size_byte(&self) -> i32;
}
#[derive(Debug, PartialEq, Clone)]
pub enum PIMBaseType {
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
    Char,
}

impl Size for PIMBaseType {
    fn size_byte(&self) -> i32 {
        match self {
            PIMBaseType::Int8 => 1,
            PIMBaseType::Int16 => 2,
            PIMBaseType::Int32 => 4,
            PIMBaseType::Int64 => 8,
            PIMBaseType::Float => 4,
            PIMBaseType::Double => 8,
            PIMBaseType::Char => 1,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PIMType {
    Basic(PIMBaseType),
    Array(PIMBaseType, i32),
}

impl Size for PIMType {
    fn size_byte(&self) -> i32 {
        match self {
            PIMType::Basic(t) => t.size_byte(),
            PIMType::Array(t, num) => t.size_byte() * num,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PIMField {
    pub varname: String,
    pub pim_type: PIMType,
}

impl Size for PIMField {
    fn size_byte(&self) -> i32 {
        self.pim_type.size_byte()
    }
}

#[derive(Debug, Clone)]
pub struct NamedBlock {
    pub name: String,
    pub fields: Vec<PIMField>,
}

impl Size for NamedBlock {
    fn size_byte(&self) -> i32 {
        self.fields.iter().map(|field| field.size_byte()).sum()
    }
}

#[derive(Debug, Clone)]
pub struct Node(pub NamedBlock);

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub named_block: NamedBlock,
}

#[derive(Debug, Clone)]
pub struct Walker {
    pub name: String,
    pub node_type: String,
}

#[derive(Debug, Clone)]
// Instantiation of Node
pub struct NodeInst {
    pub node_type: String,
    pub varname: String,
}

pub fn transform_node_inst(node_type: &str, token_list: &Vec<String>) -> Vec<NodeInst> {
    token_list
        .into_iter()
        .map(|a| NodeInst {
            node_type: String::from(node_type),
            varname: a.to_string(),
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct EdgeInst {
    pub edge_type: String,
    pub from_varname: String,
    pub to_varname: String,
    pub weight: i32,
}

#[derive(Debug, Clone)]
pub struct WalkerInst {
    pub walker_type: String,
    pub start_node: String,
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub node_insts: Vec<NodeInst>,
    pub edge_insts: Vec<EdgeInst>,
    pub walker_insts: Vec<WalkerInst>,
}

#[derive(Debug, Clone)]
pub enum GeneralBlock {
    NodeBlock(Node),
    EdgeBlock(Edge),
    WalkerBlock(Walker),
    GraphBlock(Graph),
}
