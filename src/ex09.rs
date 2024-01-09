use std::{iter::zip, collections::BTreeMap};

use crate::helpers::parse_formula;

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let variables = ('A'..='Z').collect::<Vec<char>>();
    if let Ok(parse_result) = parse_formula(formula, &variables) {
        if parse_result.operands.len() == sets.len() {
            let variables = parse_result
                .operands
                .into_iter()
                .map(|(c, _v)| c)
                .zip(sets).collect::<BTreeMap<char, Vec<i32>>>();
                
            
            vec![]
        } else {
            println!("Error");
            vec![]
        }
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
    assert_eq!(eval_set("A!", vec![vec![0, 1, 2]]), vec![]);
}
