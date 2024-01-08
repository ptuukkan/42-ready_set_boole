fn union<T: Clone>(s1: &Vec<T>, s2: &Vec<T>) -> Vec<T> {
    let mut res = s1.clone();
    res.append(&mut s2.clone());
    res
}

pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    match set[..] {
        [] => vec![vec![]],
        _ => {
            let mut s = set.clone();
            let e = s.pop().unwrap();
            let t = powerset(s);
            union(&t, &t.iter().map(|sub| union(sub, &vec![e])).collect())
        }
    }
}

#[test]
fn test_powerset() {
    assert_eq!(powerset(vec![]), vec![vec![]]);
    assert_eq!(powerset(vec![1]), vec![vec![], vec![1]]);
    assert_eq!(
        powerset(vec![1, 2]),
        vec![vec![], vec![1], vec![2], vec![1, 2]]
    );
    assert_eq!(
        powerset(vec![1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ]
    );
}
