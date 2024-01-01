use crate::{ex05::negation_normal_form, helpers::parse_formula, proposition::Proposition};

fn rewrite(prop: Proposition) -> Proposition {
    match prop {
        Proposition::Disjunction(ref a, ref b) => match (&**a, &**b) {
            // (Proposition::Conjunction(a_a, a_b), Proposition::Conjunction(b_a, b_b)) => todo!(),
            (Proposition::Conjunction(a_a, a_b), _) => {
                let p = rewrite(Proposition::Disjunction(a_a.clone(), b.clone()));
                let q = rewrite(Proposition::Disjunction(a_b.clone(), b.clone()));
                Proposition::Conjunction(Box::new(p), Box::new(q))
            }
            (_, Proposition::Conjunction(b_a, b_b)) => {
                let p = rewrite(Proposition::Disjunction(a.clone(), b_a.clone()));
                let q = rewrite(Proposition::Disjunction(a.clone(), b_b.clone()));
                Proposition::Conjunction(Box::new(p), Box::new(q))
            }
            _ => {
                let p = rewrite(*a.clone());
                let q = rewrite(*b.clone());
                match (&p, &q) {
                    (Proposition::Conjunction(_, _), Proposition::Conjunction(_, _)) => todo!(),
                    (Proposition::Conjunction(_, _), Proposition::Disjunction(_, _)) => todo!(),
                    (Proposition::Conjunction(_, _), Proposition::ExclusiveDisjunction(_, _)) => todo!(),
                    (Proposition::Conjunction(_, _), Proposition::LogicalEquivalence(_, _)) => todo!(),
                    (Proposition::Conjunction(_, _), Proposition::MaterialCondition(_, _)) => todo!(),
                    (Proposition::Conjunction(_, _), Proposition::Negation(_)) => todo!(),
                    (Proposition::Conjunction(_, _), Proposition::Variable(_)) => todo!(),
                    (Proposition::Disjunction(_, _), Proposition::Conjunction(_, _)) => todo!(),
                    (Proposition::Disjunction(_, _), Proposition::Disjunction(_, _)) => todo!(),
                    (Proposition::Disjunction(_, _), Proposition::ExclusiveDisjunction(_, _)) => todo!(),
                    (Proposition::Disjunction(_, _), Proposition::LogicalEquivalence(_, _)) => todo!(),
                    (Proposition::Disjunction(_, _), Proposition::MaterialCondition(_, _)) => todo!(),
                    (Proposition::Disjunction(_, _), Proposition::Negation(_)) => todo!(),
                    (Proposition::Disjunction(_, _), Proposition::Variable(_)) => todo!(),
                    (Proposition::ExclusiveDisjunction(_, _), Proposition::Conjunction(_, _)) => todo!(),
                    (Proposition::ExclusiveDisjunction(_, _), Proposition::Disjunction(_, _)) => todo!(),
                    (Proposition::ExclusiveDisjunction(_, _), Proposition::ExclusiveDisjunction(_, _)) => todo!(),
                    (Proposition::ExclusiveDisjunction(_, _), Proposition::LogicalEquivalence(_, _)) => todo!(),
                    (Proposition::ExclusiveDisjunction(_, _), Proposition::MaterialCondition(_, _)) => todo!(),
                    (Proposition::ExclusiveDisjunction(_, _), Proposition::Negation(_)) => todo!(),
                    (Proposition::ExclusiveDisjunction(_, _), Proposition::Variable(_)) => todo!(),
                    (Proposition::LogicalEquivalence(_, _), Proposition::Conjunction(_, _)) => todo!(),
                    (Proposition::LogicalEquivalence(_, _), Proposition::Disjunction(_, _)) => todo!(),
                    (Proposition::LogicalEquivalence(_, _), Proposition::ExclusiveDisjunction(_, _)) => todo!(),
                    (Proposition::LogicalEquivalence(_, _), Proposition::LogicalEquivalence(_, _)) => todo!(),
                    (Proposition::LogicalEquivalence(_, _), Proposition::MaterialCondition(_, _)) => todo!(),
                    (Proposition::LogicalEquivalence(_, _), Proposition::Negation(_)) => todo!(),
                    (Proposition::LogicalEquivalence(_, _), Proposition::Variable(_)) => todo!(),
                    (Proposition::MaterialCondition(_, _), Proposition::Conjunction(_, _)) => todo!(),
                    (Proposition::MaterialCondition(_, _), Proposition::Disjunction(_, _)) => todo!(),
                    (Proposition::MaterialCondition(_, _), Proposition::ExclusiveDisjunction(_, _)) => todo!(),
                    (Proposition::MaterialCondition(_, _), Proposition::LogicalEquivalence(_, _)) => todo!(),
                    (Proposition::MaterialCondition(_, _), Proposition::MaterialCondition(_, _)) => todo!(),
                    (Proposition::MaterialCondition(_, _), Proposition::Negation(_)) => todo!(),
                    (Proposition::MaterialCondition(_, _), Proposition::Variable(_)) => todo!(),
                    (Proposition::Negation(_), Proposition::Conjunction(_, _)) => todo!(),
                    (Proposition::Negation(_), Proposition::Disjunction(_, _)) => todo!(),
                    (Proposition::Negation(_), Proposition::ExclusiveDisjunction(_, _)) => todo!(),
                    (Proposition::Negation(_), Proposition::LogicalEquivalence(_, _)) => todo!(),
                    (Proposition::Negation(_), Proposition::MaterialCondition(_, _)) => todo!(),
                    (Proposition::Negation(_), Proposition::Negation(_)) => todo!(),
                    (Proposition::Negation(_), Proposition::Variable(_)) => todo!(),
                    (Proposition::Variable(_), Proposition::Conjunction(_, _)) => todo!(),
                    (Proposition::Variable(_), Proposition::Disjunction(_, _)) => todo!(),
                    (Proposition::Variable(_), Proposition::ExclusiveDisjunction(_, _)) => todo!(),
                    (Proposition::Variable(_), Proposition::LogicalEquivalence(_, _)) => todo!(),
                    (Proposition::Variable(_), Proposition::MaterialCondition(_, _)) => todo!(),
                    (Proposition::Variable(_), Proposition::Negation(_)) => todo!(),
                    (Proposition::Variable(_), Proposition::Variable(_)) => todo!(),
                }
                Proposition::Disjunction(Box::new(p), Box::new(q))
            }
        },
        Proposition::Conjunction(a, b) => {
            let p = rewrite(*a);
            let q = rewrite(*b);
            match (&p, &q) {
                (Proposition::Conjunction(_, _), Proposition::Variable(_)) => {
                    Proposition::Conjunction(Box::new(q), Box::new(p))
                }
                (Proposition::Variable(_), Proposition::Conjunction(_, _)) => {
                    Proposition::Conjunction(Box::new(p), Box::new(q))
                }
                _ => Proposition::Conjunction(Box::new(p), Box::new(q)),
            }
        }
        Proposition::Negation(inner) => {
            let p = Box::new(rewrite(*inner));
            Proposition::Negation(p)
        }
        Proposition::Variable(x) => Proposition::Variable(x),
        _ => todo!(),
    }
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let nnf_formula = negation_normal_form(formula);
    let variables = ('A'..='Z').collect::<Vec<char>>();
    if let Ok(parse_result) = parse_formula(&nnf_formula, &variables) {
        let cnf = rewrite(parse_result.proposition);
        return cnf.into_iter().collect();
    }
    return String::from("Error");
}

#[test]
fn test_conjuctive_normal_form() {
    assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
    assert_eq!(conjunctive_normal_form("ABCD&|&"), "ABC|BD|&&");
    assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
    assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
    assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
    assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
    assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
    assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
}
