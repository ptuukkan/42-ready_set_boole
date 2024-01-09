use std::collections::BTreeMap;

use crate::{
    proposition::Proposition,
    propositional_formula::PropositionalFormula,
    set::{complement, intersection, union},
};

impl Proposition {
    pub fn evaluate_set(&self, variable_map: &BTreeMap<char, Vec<i32>>) -> Vec<i32> {
        match self {
            Proposition::Conjunction(p, q) => {
                intersection(&p.evaluate_set(variable_map), &q.evaluate_set(variable_map))
            }
            Proposition::Disjunction(p, q) => {
                union(&p.evaluate_set(variable_map), &q.evaluate_set(variable_map))
            }
            Proposition::ExclusiveDisjunction(p, q) => todo!(),
            Proposition::LogicalEquivalence(p, q) => todo!(),
            Proposition::MaterialCondition(p, q) => todo!(),
            Proposition::Negation(p) => complement(&p.evaluate_set(variable_map), variable_map),
            Proposition::Variable(x) => variable_map.get(x).unwrap().to_vec(),
        }
    }
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    if let Ok(pf) = PropositionalFormula::try_from(formula) {
        assert_eq!(
            sets.len(),
            pf.variables.len(),
            "Amount of sets must equal to the amount of variables in formula."
        );

        let mut variables = pf.variables.clone().into_iter().collect::<Vec<char>>();
        variables.sort();
        let variable_map: BTreeMap<char, Vec<i32>> = variables.into_iter().zip(sets).collect();
        dbg!(&variable_map);
        let mut result = pf.evaluate_set(&variable_map);
        result.sort();
        result
    } else {
        println!("Error");
        vec![]
    }
}

#[test]
fn test_eval_set() {
    assert_eq!(eval_set("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]]), vec![0]);
    assert_eq!(
        eval_set("AB|", vec![vec![0, 1, 2], vec![3, 4, 5]]),
        vec![0, 1, 2, 3, 4, 5]
    );
    assert_eq!(
        eval_set("A!B!|", vec![vec![0, 1, 2], vec![3, 4, 5]]),
        vec![0, 1, 2, 3, 4, 5]
    );
    assert_eq!(
        eval_set("AB!|", vec![vec![0, 1, 2], vec![3, 4, 5]]),
        vec![0, 1, 2]
    );
    assert_eq!(
        eval_set("A!B|", vec![vec![0, 1, 2], vec![3, 4, 5]]),
        vec![3, 4, 5]
    );
    assert_eq!(eval_set("A!", vec![vec![0, 1, 2]]), vec![]);
}
