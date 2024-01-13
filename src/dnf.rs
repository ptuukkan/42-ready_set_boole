use crate::{helpers::*, proposition::Proposition};

pub fn disjunctive_normal_form(formula: &str) -> String {
    let variables = ('A'..='Z').collect::<Vec<char>>();
    if let Ok(parse_result) = parse_formula(formula, &variables) {
        let high_mask: u32 = 1 << parse_result.operands.len();

        let dnf = (0..high_mask)
            .map(|mask| {
                let value_map = get_value_map(&parse_result.operands, &mask);
                let result = evaluate(&parse_result.proposition, &value_map);
                (result, value_map)
            })
            .filter(|(result, _value_map)| *result)
            .map(|(_result, value_map)| value_map)
            .map(|value_map| {
                value_map
                    .iter()
                    .filter(|v| *v.1)
                    .fold(None, |acc, e| match acc {
                        Some(prop) => Some(Proposition::Conjunction(
                            Box::new(Proposition::Variable(*e.0)),
                            Box::new(prop),
                        )),
                        None => Some(Proposition::Variable(*e.0)),
                    })
            })
            .flatten()
            .reduce(|acc, e| Proposition::Disjunction(Box::new(e), Box::new(acc)));
        if let Some(formula) = dnf {
            formula.into_iter().collect()
        } else {
            String::from("Error")
        }
    } else {
        String::from("Error")
    }
}

#[test]
fn test_disjunctive_normal_form() {}
