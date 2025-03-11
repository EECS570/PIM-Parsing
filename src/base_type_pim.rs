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
pub struct Parse<'a> {
    nodes: Vec<Node<'a>>,
    edges: Vec<Edge<'a>>,
}
