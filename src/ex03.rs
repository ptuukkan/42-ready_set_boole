use std::collections::BTreeMap;

use crate::helpers::{evaluate, parse_formula};

pub fn eval_formula(formula: &str) -> bool {
    let variables = vec!['0', '1'];
    let mut value_map = BTreeMap::new();
    value_map.insert('0', false);
    value_map.insert('1', true);

    if let Ok(parse_result) = parse_formula(formula, &variables) {
        evaluate(&parse_result.proposition, &value_map)
    } else {
        println!("Error parsing formula");
        false
    }
}
#[test]
fn test_eval_formula() {
    assert_eq!(eval_formula("10&"), false);
    assert_eq!(eval_formula("10|"), true);
    assert_eq!(eval_formula("11>"), true);
    assert_eq!(eval_formula("10="), false);
    assert_eq!(eval_formula("1011||="), true);
    assert_eq!(eval_formula("1011||=!"), false);
    assert_eq!(eval_formula("1!"), false);
    assert_eq!(eval_formula("0!"), true);
}
