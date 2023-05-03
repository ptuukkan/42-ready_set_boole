use crate::{helpers::parse_formula, proposition::Proposition};

fn rewrite(prop: &mut Proposition) {
    match prop {
        Proposition::Negation(inner) => match &**inner {
            Proposition::Negation(inner_inner) => {
                *prop = *(inner_inner).clone();
                rewrite(prop);
            }
            Proposition::Conjunction(a, b) => {
                *prop = Proposition::Disjunction(
                    Box::new(Proposition::Negation(a.clone())),
                    Box::new(Proposition::Negation(b.clone())),
                );
                rewrite(prop);
            }
            Proposition::Disjunction(a, b) => {
                *prop = Proposition::Conjunction(
                    Box::new(Proposition::Negation(a.clone())),
                    Box::new(Proposition::Negation(b.clone())),
                );
                rewrite(prop);
            },
            Proposition::Variable(_) => {
                rewrite(&mut *inner);
            },
            _ => {
                rewrite(&mut *inner);
                rewrite(prop);
            },
        },
        Proposition::MaterialCondition(a, b) => {
            *prop = Proposition::Disjunction(Box::new(Proposition::Negation(a.clone())), b.clone());
            rewrite(prop);
        }
        Proposition::LogicalEquivalence(a, b) => {
            *prop = Proposition::Conjunction(
                Box::new(Proposition::MaterialCondition(a.clone(), b.clone())),
                Box::new(Proposition::MaterialCondition(b.clone(), a.clone())),
            );
            rewrite(prop);
        }
        Proposition::ExclusiveDisjunction(a, b) => {
            *prop = Proposition::Disjunction(
                Box::new(Proposition::Conjunction(
                    a.clone(),
                    Box::new(Proposition::Negation(b.clone())),
                )),
                Box::new(Proposition::Conjunction(
                    Box::new(Proposition::Negation(a.clone())),
                    b.clone(),
                )),
            );
            rewrite(prop);
        }
        Proposition::Conjunction(a, b) | Proposition::Disjunction(a, b) => {
            rewrite(&mut *a);
            rewrite(&mut *b);
        }
        Proposition::Variable(_) => (),
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let variables = ('A'..='Z').collect::<Vec<char>>();
    if let Ok(mut parse_result) = parse_formula(formula, &variables) {
        rewrite(&mut parse_result.proposition);
        return parse_result.proposition.into_iter().collect();
    }
    return String::from("Error");
}

#[test]
fn test_negation_normal_form() {
    assert_eq!("A!B!|", negation_normal_form("AB&!"));
    assert_eq!("A!B!&", negation_normal_form("AB|!"));
    assert_eq!("A!B|", negation_normal_form("AB>"));
    assert_eq!("A!B|B!A|&", negation_normal_form("AB="));
    assert_eq!("A!B!&C!|", negation_normal_form("AB|C&!"));
    assert_eq!("AB|", negation_normal_form("A!B>"));
    assert_eq!("A!B!&AB&|", negation_normal_form("A!B^"));
}
