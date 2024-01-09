use std::collections::BTreeMap;

use crate::{helpers::evaluate, proposition::Proposition};

pub fn union<T: Clone>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut res = a.clone();
    res.append(&mut b.clone());
    res
}

pub fn relative_complement(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let res = b.clone();
    res.into_iter().filter(|e| !a.contains(&e)).collect()
}

// pub fn evaluate(proposition: &Proposition, variables: &BTreeMap<char, Vec<i32>>) -> Vec<i32> {
//     match proposition {
//         Proposition::Conjunction(p, q) => union(
//             &evaluate(p, variables),
//             &evaluate(q, variables)
//         ),
//         Proposition::Disjunction(p, q) => todo!(),
//         Proposition::ExclusiveDisjunction(p, q) => todo!(),
//         Proposition::LogicalEquivalence(p, q) => todo!(),
//         Proposition::MaterialCondition(p, q) => todo!(),
//         Proposition::Negation(p) => todo!(),
//         Proposition::Variable(x) => {
//             let b = *variables.get(x).unwrap();
//             b
//         }
//     }
// }
