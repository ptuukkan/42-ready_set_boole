use crate::helpers::*;

pub fn negation_normal_form(formula: &str) -> String {
    if let Ok(parse_result) = parse_formula(formula) {
        let pla = parse_result.tree.into_iter().collect::<String>();
        return pla;
    }
    String::new()
}

#[test]
fn test_negation_normal_form() {}
