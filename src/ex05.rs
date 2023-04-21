
use crate::helpers::parse_formula;

enum RewriteRule {
    DoubleNegation,
}

pub fn negation_normal_form(formula: &str) -> String {
    let variables = ('A'..='Z').collect::<Vec<char>>();
    if let Ok(parse_result) = parse_formula(formula, &variables) {
         
    }
    return String::from("Error");
}

#[test]
fn test_negation_normal_form() {
    assert_eq!("A!B!|", negation_normal_form("AB&!"));
    assert_eq!("A!B!&", negation_normal_form("AB|!"));
    assert_eq!("A!B|", negation_normal_form("AB>"));
    assert_eq!("AB&A!B!&|", negation_normal_form("AB="));
    assert_eq!("A!B!&C!|", negation_normal_form("AB|C&!"));
}
