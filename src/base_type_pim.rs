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
pub struct PIMField<'a> {
    pub varname: &'a str,
    pub pim_type: PIMType,
}

impl Size for PIMField<'_> {
    fn size_byte(&self) -> i32 {
        self.pim_type.size_byte()
    }
}

#[derive(Debug, Clone)]
pub struct NamedBlock<'a> {
    pub name: &'a str,
    pub fields: Vec<PIMField<'a>>,
}

impl Size for NamedBlock<'_> {
    fn size_byte(&self) -> i32 {
        self.fields.iter().map(|field| field.size_byte()).sum()
    }
}

#[derive(Debug, Clone)]
pub struct Node<'a>(pub NamedBlock<'a>);

#[derive(Debug, Clone)]
pub struct Edge<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub named_block: NamedBlock<'a>,
}

#[derive(Debug, Clone)]
pub struct Walker<'a> {
    pub name: &'a str,
    pub node_type: &'a str,
}

#[derive(Debug, Clone)]
// Instantiation of Node
pub struct NodeInst<'a> {
    pub node_type: &'a str,
    pub varname: &'a str,
}

pub fn transform_node_inst<'input>(
    node_type: &'input str,
    token_list: Vec<&'input str>,
) -> Vec<NodeInst<'input>> {
    token_list
        .into_iter()
        .map(|a| NodeInst {
            node_type,
            varname: a,
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct EdgeInst<'a> {
    pub edge_type: &'a str,
    pub from_varname: &'a str,
    pub to_varname: &'a str,
    pub weight: i32,
}

#[derive(Debug, Clone)]
pub struct Graph<'a> {
    pub node_insts: Vec<NodeInst<'a>>,
    pub edge_insts: Vec<EdgeInst<'a>>,
}

#[derive(Debug, Clone)]
pub enum GeneralBlock<'a> {
    NodeBlock(Node<'a>),
    EdgeBlock(Edge<'a>),
    WalkerBlock(Walker<'a>),
    GraphBlock(Graph<'a>),
}
