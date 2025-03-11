use base_type_pim::{PIMBaseType, PIMType, GeneralBlock};
use lalrpop_util::lalrpop_mod;
mod base_type_pim;
use anyhow::Result;
use clap::Parser;
use std::fs;

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
        .parse("{hello: int16; goodbye: float;}")
        .expect("Parsing error");
    println!("{:?}", block);

    let namedblock = dspim::NamedBlockRuleParser::new()
        .parse("nd {hello: int16; goodbye: float;}")
        .expect("Parsing error");
    println!("{:?}", namedblock);

    let node = dspim::NodeRuleParser::new()
        .parse("node nd {hello: int16; goodbye: float;}")
        .expect("Parsing error");
    println!("{:?}", node);
}

fn parse_str(content: &str) -> Result<Vec<GeneralBlock>> {
    let input = dspim::GeneralRuleParser::new()
        .parse(content)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(input)
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Reading from: {}", args.file);
    let file_content = fs::read_to_string(args.file)?;
    println!("File content: {}", file_content);
    let _ = parse_str(&file_content)?;
    Ok(())
}
