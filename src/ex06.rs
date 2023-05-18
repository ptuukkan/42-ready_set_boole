use crate::{proposition::Proposition, ex05::negation_normal_form, helpers::parse_formula};

fn rewrite(prop: &mut Proposition) {
    match prop {
        Proposition::Disjunction(a, b) => {
            match &**a {
                Proposition::Conjunction(&mut inner_a, &mut inner_b) => todo!(),
                Proposition::Disjunction(&mut inner_a, &mut inner_b)
                | Proposition::ExclusiveDisjunction(&mut inner_a, &mut inner_b)
                | Proposition::LogicalEquivalence(&mut inner_a, &mut inner_b)
                | Proposition::MaterialCondition(&mut inner_a, &mut inner_b) => {
                    rewrite(&mut *inner_a);
                    rewrite(&mut *inner_b);
                }
                Proposition::Negation(inner) => todo!(),
                Proposition::Variable(_) => todo!(),
            }
        }
        Proposition::Conjunction(a, b)
        | Proposition::ExclusiveDisjunction(a, b)
        | Proposition::LogicalEquivalence(a, b)
        | Proposition::MaterialCondition(a, b) => {
            rewrite(&mut *a);
            rewrite(&mut *b);
        },
        Proposition::Negation(inner) => {
            rewrite(&mut *inner);
        },
        Proposition::Variable(_) => (),
    }
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let nnf_formula = negation_normal_form(formula);
    let variables = ('A'..='Z').collect::<Vec<char>>();
    if let Ok(mut parse_result) = parse_formula(&nnf_formula, &variables) {
        rewrite(&mut parse_result.proposition);
        return parse_result.proposition.into_iter().collect();
    }
    return String::from("Error");
}

#[test]
fn test_conjuctive_normal_form() {
    assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
    assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
    assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
    assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
    assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
    assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
    assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
}
