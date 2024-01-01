use crate::{helpers::parse_formula, proposition::Proposition};

fn rewrite(prop: Proposition) -> Proposition {
    match prop {
        Proposition::Negation(inner) => match *inner {
            Proposition::Negation(inner_inner) => rewrite(*inner_inner),
            Proposition::Conjunction(a, b) => {
                let p = rewrite(Proposition::Negation(a));
                let q = rewrite(Proposition::Negation(b));
                Proposition::Disjunction(Box::new(p), Box::new(q))
            }
            Proposition::Disjunction(a, b) => {
                let p = rewrite(Proposition::Negation(a));
                let q = rewrite(Proposition::Negation(b));
                Proposition::Conjunction(Box::new(p), Box::new(q))
            }
            Proposition::Variable(x) => Proposition::Negation(Box::new(Proposition::Variable(x))),
            _ => {
                let new_inner = rewrite(*inner);
                rewrite(Proposition::Negation(Box::new(new_inner)))
            }
        },
        Proposition::MaterialCondition(a, b) => {
            let p = rewrite(Proposition::Negation(a));
            let q = rewrite(*b);
            Proposition::Disjunction(Box::new(p), Box::new(q))
        }
        Proposition::LogicalEquivalence(a, b) => {
            // let p = Proposition::MaterialCondition(
            //     Box::new(rewrite(*a.clone())),
            //     Box::new(rewrite(*b.clone())),
            // );
            // let q = Proposition::MaterialCondition(Box::new(rewrite(*b)), Box::new(rewrite(*a)));
            let p = rewrite(Proposition::MaterialCondition(
                Box::new(*a.clone()),
                Box::new(*b.clone()),
            ));
            let q = rewrite(Proposition::MaterialCondition(Box::new(*b), Box::new(*a)));
            Proposition::Conjunction(Box::new(p), Box::new(q))
        }
        Proposition::ExclusiveDisjunction(a, b) => {
            let p = rewrite(Proposition::Conjunction(
                a.clone(),
                Box::new(Proposition::Negation(b.clone())),
            ));

            let q = rewrite(Proposition::Conjunction(
                Box::new(Proposition::Negation(a)),
                b,
            ));

            Proposition::Disjunction(Box::new(p), Box::new(q))
        }
        Proposition::Conjunction(a, b) => {
            let p = rewrite(*a);
            let q = rewrite(*b);
            Proposition::Conjunction(Box::new(p), Box::new(q))
        }
        Proposition::Disjunction(a, b) => {
            let p = rewrite(*a);
            let q = rewrite(*b);
            Proposition::Disjunction(Box::new(p), Box::new(q))
        }
        Proposition::Variable(x) => Proposition::Variable(x),
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let variables = ('A'..='Z').collect::<Vec<char>>();
    if let Ok(parse_result) = parse_formula(formula, &variables) {
        let nnf = rewrite(parse_result.proposition);
        return nnf.into_iter().collect();
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
