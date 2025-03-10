use lalrpop_util::lalrpop_mod;
mod base_type_pim;

lalrpop_mod!(pub dspim); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(dspim::TermParser::new().parse("22").is_ok());
    assert!(dspim::TermParser::new().parse("(22)").is_ok());
    assert!(dspim::TermParser::new().parse("((((22))))").is_ok());
    assert!(dspim::TermParser::new().parse("((22)").is_err());
    assert!(dspim::BaseTypeParser::new().parse("int8").is_ok())
}

fn main() {

}
