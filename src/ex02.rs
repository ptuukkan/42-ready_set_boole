use crate::ex01::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut result: u32 = 0;
    let mut mut_a = a;
    let mut mut_b = b;

    while mut_b > 0 {
        if mut_b & 1 == 1 {
            result = adder(result, mut_a);
        }
        mut_a <<= 1;
        mut_b >>= 1;
    }
    result
}

#[test]
fn test_multiplier() {
    for i in 0..100 {
        for j in 0..100 {
            assert_eq!(multiplier(i, j), i * j);
        }
    }
}
