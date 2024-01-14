use std::{collections::BTreeMap, fmt::Debug};

// OR
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

// NOT
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

// NOT
pub fn relative_complement(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    b.clone().into_iter().filter(|e| !a.contains(e)).collect()
}

// AND
pub fn intersection(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    a.clone().into_iter().filter(|e| b.contains(e)).collect()
}

// XOR
pub fn symmetric_difference(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    union(&relative_complement(a, b), &relative_complement(b, a))
}

pub fn material_conditional(
    a: &Vec<i32>,
    b: &Vec<i32>,
    variable_map: &BTreeMap<char, Vec<i32>>,
) -> Vec<i32> {
    union(&complement(a, variable_map), b)
}

pub fn logical_equivalence(
    a: &Vec<i32>,
    b: &Vec<i32>,
    variable_map: &BTreeMap<char, Vec<i32>>,
) -> Vec<i32> {
    union(&intersection(a, b), &complement(&union(a, b), variable_map))
}
