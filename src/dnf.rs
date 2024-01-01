use crate::helpers::*;

pub fn disjunctive_normal_form(formula: &str) -> &str {
    let variables = ('A'..='Z').collect::<Vec<char>>();
    if let Ok(parse_result) = parse_formula(formula, &variables) {
        let high_mask: u32 = 1 << parse_result.operands.len();

        (0..high_mask)
            .map(|mask| {
                let value_map = get_value_map(&parse_result.operands, &mask);
                let result = evaluate(&parse_result.proposition, &value_map);
                (result, value_map)
            })
            .filter(|(result, _value_map)| *result)
            .map(|(_result, value_map)| value_map)
            .for_each(|value_map| {
                let row = value_map
                    .iter()
                    .map(|v| match *v.1 {
                        true => "1",
                        false => "0",
                    })
                    .collect::<Vec<&str>>()
                    .join(" | ");

                println!("| {} | {} |", row, 1);
            });
    } else {
        println!("Error");
    }
    "aaa"
}

#[test]
fn test_disjunctive_normal_form() {}
