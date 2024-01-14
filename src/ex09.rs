use std::collections::BTreeMap;

use crate::{proposition::Proposition, propositional_formula::PropositionalFormula, set::*};

impl Proposition {
    pub fn evaluate_set(&self, variable_map: &BTreeMap<char, Vec<i32>>) -> Vec<i32> {
        match self {
            Proposition::Conjunction(p, q) => {
                intersection(&p.evaluate_set(variable_map), &q.evaluate_set(variable_map))
            }
            Proposition::Disjunction(p, q) => {
                union(&p.evaluate_set(variable_map), &q.evaluate_set(variable_map))
            }
            Proposition::LogicalEquivalence(p, q) => logical_equivalence(
                &p.evaluate_set(variable_map),
                &q.evaluate_set(variable_map),
                variable_map,
            ),
            Proposition::ExclusiveDisjunction(p, q) => {
                symmetric_difference(&p.evaluate_set(variable_map), &q.evaluate_set(variable_map))
            }
            Proposition::MaterialCondition(p, q) => material_conditional(
                &p.evaluate_set(variable_map),
                &q.evaluate_set(variable_map),
                variable_map,
            ),
            Proposition::Negation(p) => complement(&p.evaluate_set(variable_map), variable_map),
            Proposition::Variable(x) => variable_map.get(x).unwrap().to_vec(),
        }
    }
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    match PropositionalFormula::try_from(formula) {
        Ok(pf) => {
            if sets.len() != pf.variables.len() {
                println!("Error");
                return vec![];
            }

            let mut variables = pf.variables.clone().into_iter().collect::<Vec<char>>();
            variables.sort();
            let variable_map: BTreeMap<char, Vec<i32>> = variables.into_iter().zip(sets).collect();
            let mut result = pf.evaluate_set(&variable_map);
            result.sort();
            result
        }
        Err(_) => {
            println!("Error");
            vec![]
        }
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

    assert_eq!(eval_set("A", vec![vec![]]), vec![]);
    assert_eq!(eval_set("A!", vec![vec![]]), vec![]);
    assert_eq!(eval_set("A", vec![vec![42]]), vec![42]);
    assert_eq!(eval_set("A!", vec![vec![42]]), vec![]);

    assert_eq!(
        eval_set("A!B&", vec![vec![1, 2, 3], vec![2, 3, 4]]),
        vec![4]
    );
    assert_eq!(eval_set("AB|", vec![vec![0, 1, 2], vec![]]), vec![0, 1, 2]);
    assert_eq!(eval_set("AB&", vec![vec![0, 1, 2], vec![]]), vec![]);
    assert_eq!(eval_set("AB&", vec![vec![0, 1, 2], vec![0]]), vec![0]);
    assert_eq!(eval_set("AB&", vec![vec![0, 1, 2], vec![42]]), vec![]);
    assert_eq!(eval_set("AB^", vec![vec![0, 1, 2], vec![0]]), vec![1, 2]);
    assert_eq!(eval_set("AB>", vec![vec![0], vec![1, 2]]), vec![1, 2]);
    assert_eq!(eval_set("AB>", vec![vec![0], vec![0, 1, 2]]), vec![0, 1, 2]);

    assert_eq!(eval_set("ABC||", vec![vec![], vec![], vec![]]), vec![]);
    assert_eq!(
        eval_set("ABC||", vec![vec![0], vec![1], vec![2]]),
        vec![0, 1, 2]
    );
    assert_eq!(eval_set("ABC||", vec![vec![0], vec![0], vec![0]]), vec![0]);
    assert_eq!(eval_set("ABC&&", vec![vec![0], vec![0], vec![]]), vec![]);
    assert_eq!(eval_set("ABC&&", vec![vec![0], vec![0], vec![0]]), vec![0]);
    assert_eq!(eval_set("ABC^^", vec![vec![0], vec![0], vec![0]]), vec![0]);
    assert_eq!(eval_set("ABC>>", vec![vec![0], vec![0], vec![0]]), vec![0]);
    assert_eq!(eval_set("AB=C&", vec![vec![0,1,2], vec![1,2,3], vec![0,1,2,3,4,5]]), vec![1,2,4,5]);
}
