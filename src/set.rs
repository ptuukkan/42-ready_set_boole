use std::{collections::BTreeMap, fmt::Debug};

pub fn union<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T>
where
    T: PartialEq,
    T: Clone,
    T: Debug,
{
    let mut res = a.clone();
    for x in b {
        if !res.contains(x) {
            res.push(x.clone());
        }
    }
    res.to_vec()
}

pub fn complement(a: &Vec<i32>, variable_map: &BTreeMap<char, Vec<i32>>) -> Vec<i32> {
    let union_of_sets = variable_map
        .clone()
        .into_iter()
        .map(|(_c, v)| v)
        .reduce(|acc, e| union(&acc, &e))
        .unwrap();
    union_of_sets
        .into_iter()
        .filter(|e| !a.contains(e))
        .collect()
}

pub fn intersection(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    a.clone().into_iter().filter(|e| b.contains(e)).collect()
}
