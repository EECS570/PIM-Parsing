use lalrpop_util::lalrpop_mod;
mod base_type_pim;

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
    let field = dspim::FieldRuleParser::new().parse("hello : int8");
    let name = field.expect("msg").varname;
    eprintln!("varname: {name}");
}

fn main() {}
