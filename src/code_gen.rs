use crate::base_type::{NamedBlock, PIMBaseType, PIMField, PIMType};
use crate::sem_type;
pub trait TypeCodeGen {
    fn type_code(&self) -> String;
}

impl TypeCodeGen for PIMBaseType {
    fn type_code(&self) -> String {
        String::from(match self {
            PIMBaseType::Int8 => "int8_t",
            PIMBaseType::Int16 => "int16_t",
            PIMBaseType::Int32 => "int32_t",
            PIMBaseType::Int64 => "int64_t",
            PIMBaseType::Char => "char",
            PIMBaseType::Float => "float",
            PIMBaseType::Double => "double",
        })
    }
}

impl TypeCodeGen for PIMField {
    fn type_code(&self) -> String {
        match &self.pim_type {
            PIMType::Basic(t) => format!("{} {};", t.type_code(), self.varname),
            PIMType::Array(t, i) => format!("{} {} [{}];", t.type_code(), self.varname, i),
        }
    }
}

impl TypeCodeGen for NamedBlock {
    fn type_code(&self) -> String {
        let content: Vec<String> = self.fields.iter().map(|field| field.type_code()).collect();

        String::from(format!(
            "typedef struct _{} {{ \n{}\n}} {}",
            self.name,
            content.join("\n"),
            self.name
        ))
    }
}

impl TypeCodeGen for sem_type::SemanticEdge {
    fn type_code(&self) -> String {
        self.named_block.type_code()
    }
}

#[test]
pub fn test_node_code_gen() {
    let _node = NamedBlock {
        name: String::from("TestNode"),
        fields: vec![PIMField {
            varname: String::from("field"),
            pim_type: PIMType::Basic(PIMBaseType::Char),
        }],
    };
    _node.type_code();
}
