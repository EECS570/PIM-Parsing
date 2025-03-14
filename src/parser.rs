use crate::base_type_pim::{GeneralBlock, PIMBaseType, PIMType};
use anyhow::Result;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub dspim); // synthesized by LALRPOP

#[test]
fn test_term() {
    assert!(dspim::TermParser::new().parse("22").is_ok());
    assert!(dspim::TermParser::new().parse("(22)").is_ok());
    assert!(dspim::TermParser::new().parse("((((22))))").is_ok());
    assert!(dspim::TermParser::new().parse("((22)").is_err());
}

#[test]
fn test_type() {
    assert!(dspim::PIMBaseTypeRuleParser::new().parse("int8").is_ok());
    assert!(dspim::PIMTypeRuleParser::new().parse("int64 [30]").is_ok());
}

#[test]
fn test_block() {
    let var_name = dspim::TokenRuleParser::new().parse("hello");
    match var_name {
        Ok(s) => assert!(s == "hello"),
        Err(_) => panic!(),
    }
    // assert!(var_name.is_ok());
    let field = dspim::FieldRuleParser::new().parse("hello : int16");
    let value = field.expect("Parsing failed");
    println!("{:?}", value);
    assert!(value.pim_type == PIMType::Basic(PIMBaseType::Int16));
    assert!(value.varname == "hello");

    let list = dspim::FieldListRuleParser::new()
        .parse("hello: int16; goodbye: float;")
        .expect("Parsing error");
    println!("{:?}", list);

    let block = dspim::BlockRuleParser::new()
        .parse("{hello: int16; goodbye: float;};")
        .expect("Parsing error");
    println!("{:?}", block);

    let namedblock = dspim::NamedBlockRuleParser::new()
        .parse("nd {hello: int16; goodbye: float;};")
        .expect("Parsing error");
    println!("{:?}", namedblock);

    let node = dspim::NodeRuleParser::new()
        .parse("node nd {hello: int16; goodbye: float;};")
        .expect("Parsing error");
    println!("{:?}", node);
}

#[test]
pub fn test_graph() {
    let node_list = dspim::NodeInstRuleParser::new()
        .parse("node Hello h1, h2, h3;")
        .expect("Parsing Error");
    assert_eq!(node_list[0].varname, "h1");
    assert_eq!(node_list[1].varname, "h2");
    assert_eq!(node_list[2].varname, "h3");
    assert_eq!(node_list[0].node_type, "Hello");
    println!("{:?}", node_list);

    let graph = dspim::GraphRuleParser::new()
        .parse("graph {};")
        .expect("Parsing Error");
    println!("{:?}", graph);
    let graph = dspim::GraphRuleParser::new()
        .parse("graph { node Hello h1,h2,h3; edge Hedge h1 h2 7; edge Hedge h2 h3 5; node Hello h4;};")
        .expect("Parsing Error");
    println!("{:?}", graph);
}

pub fn parse_str(content: &str) -> Result<Vec<GeneralBlock>> {
    let input = dspim::GeneralRuleParser::new()
        .parse(content)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(input)
}
