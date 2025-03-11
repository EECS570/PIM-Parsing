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

#[derive(Debug, PartialEq, Clone)]
pub enum PIMType {
    Basic(PIMBaseType),
    Array(PIMBaseType, i32),
}

#[derive(Debug, Clone)]
pub struct PIMField<'a> {
    pub varname: &'a str,
    pub pim_type: PIMType,
}

#[derive(Debug, Clone)]
pub struct NamedBlock<'a> {
    pub name: &'a str,
    pub fields: Vec<PIMField<'a>>,
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
