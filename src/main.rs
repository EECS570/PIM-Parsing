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

fn main() {
}
