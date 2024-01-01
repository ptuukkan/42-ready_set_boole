use crate::helpers::*;

pub fn sat(formula: &str) -> bool {
    let variables = ('A'..='Z').collect::<Vec<char>>();
    let mut result = false;
    if let Ok(parse_result) = parse_formula(formula, &variables) {
        let high_mask: u32 = 1 << parse_result.operands.len();
        for mask in 0..high_mask {
            let value_map = get_value_map(&parse_result.operands, &mask);
            if evaluate(&parse_result.proposition, &value_map) {
                result = true;
                break;
            }
        }
    } else {
        println!("Error");
    }
    result
}

#[test]
fn test_sat() {
    assert_eq!(sat("AB|"), true);
    assert_eq!(sat("AB&"), true);
    assert_eq!(sat("AA!&"), false);
    assert_eq!(sat("AA^"), false);
}

