use std::collections::BTreeMap;

use crate::{proposition::Proposition, propositional_formula::PropositionalFormula};

impl Proposition {
    pub fn evaluate(&self, variable_map: &BTreeMap<char, bool>) -> bool {
        match self {
            Proposition::Conjunction(p, q) => p.evaluate(variable_map) && q.evaluate(variable_map),
            Proposition::Disjunction(p, q) => p.evaluate(variable_map) || q.evaluate(variable_map),
            Proposition::ExclusiveDisjunction(p, q) => {
                p.evaluate(variable_map) ^ q.evaluate(variable_map)
            }
            Proposition::LogicalEquivalence(p, q) => {
                p.evaluate(variable_map) == q.evaluate(variable_map)
            }
            Proposition::MaterialCondition(p, q) => {
                !p.evaluate(variable_map) || q.evaluate(variable_map)
            }
            Proposition::Negation(p) => !p.evaluate(variable_map),
            Proposition::Variable(x) => *variable_map.get(x).unwrap(),
        }
    }
}

pub fn eval_formula(formula: &str) -> bool {
    let mut variable_map = BTreeMap::new();
    variable_map.insert('0', false);
    variable_map.insert('1', true);

    if let Ok(pf) = PropositionalFormula::try_from(formula) {
        pf.evaluate(&variable_map)
    } else {
        println!("Error");
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
    assert_eq!(eval_formula("00|"), false);
    assert_eq!(eval_formula("01|"), true);
    assert_eq!(eval_formula("11|"), true);
    assert_eq!(eval_formula("11&"), true);
    assert_eq!(eval_formula("11^"), false);
    assert_eq!(eval_formula("10^"), true);
    assert_eq!(eval_formula("00>"), true);
    assert_eq!(eval_formula("01>"), true);
    assert_eq!(eval_formula("10>"), false);
    assert_eq!(eval_formula("11>"), true);
    assert_eq!(eval_formula("00="), true);
    assert_eq!(eval_formula("11="), true);
    assert_eq!(eval_formula("10="), false);
    assert_eq!(eval_formula("01="), false);
    assert_eq!(eval_formula("11&0|"), true);
    assert_eq!(eval_formula("10&1|"), true);
    assert_eq!(eval_formula("11&1|"), true);
    assert_eq!(eval_formula("11&1|1^"), false);
    assert_eq!(eval_formula("01&1|1="), true);
    assert_eq!(eval_formula("01&1&1&"), false);
    assert_eq!(eval_formula("0111&&&"), false);
}
