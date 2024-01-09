use crate::{proposition::Proposition, propositional_formula::PropositionalFormula};

impl PropositionalFormula {
    pub fn to_cnf(&self) -> Self {
        fn rewrite(prop: Proposition) -> Proposition {
            match prop {
                Proposition::Disjunction(ref a, ref b) => match (&**a, &**b) {
                    (Proposition::Conjunction(a_a, a_b), Proposition::Conjunction(b_a, b_b)) => {
                        let p = rewrite(Proposition::Conjunction(
                            Box::new(Proposition::Disjunction(a_a.clone(), b_a.clone())),
                            Box::new(Proposition::Disjunction(a_a.clone(), b_b.clone())),
                        ));
                        let q = rewrite(Proposition::Conjunction(
                            Box::new(Proposition::Disjunction(a_b.clone(), b_a.clone())),
                            Box::new(Proposition::Disjunction(a_b.clone(), b_b.clone())),
                        ));
                        Proposition::Conjunction(Box::new(p), Box::new(q))
                    }
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
                            (Proposition::Disjunction(_, _), Proposition::Variable(_)) => {
                                Proposition::Disjunction(Box::new(q), Box::new(p))
                            }
                            (Proposition::Disjunction(_, _), Proposition::Negation(_)) => {
                                Proposition::Disjunction(Box::new(q), Box::new(p))
                            }
                            _ => Proposition::Disjunction(Box::new(p), Box::new(q)),
                        }
                    }
                },
                Proposition::Conjunction(a, b) => {
                    let p = rewrite(*a);
                    let q = rewrite(*b);
                    match (&p, &q) {
                        (Proposition::Conjunction(_, _), Proposition::Variable(_)) => {
                            Proposition::Conjunction(Box::new(q), Box::new(p))
                        }
                        (Proposition::Conjunction(_, _), Proposition::Negation(_)) => {
                            Proposition::Conjunction(Box::new(q), Box::new(p))
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
        let nnf = self.to_nnf();
        Self {
            variables: nnf.variables,
            formula: rewrite(nnf.formula),
        }
    }
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    if let Ok(pf) = PropositionalFormula::try_from(formula) {
        pf.to_cnf().formula.to_string()
    } else {
        String::from("Error")
    }
}

#[test]
fn test_conjuctive_normal_form() {
    assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
    assert_eq!(conjunctive_normal_form("ABCD&|&"), "ABC|BD|&&");
    assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
    assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
    assert_eq!(conjunctive_normal_form("AB|C|D|"), "DCAB|||");
    assert_eq!(conjunctive_normal_form("AB&C&D&"), "DCAB&&&");
    assert_eq!(conjunctive_normal_form("AB&!C!|"), "C!A!B!||");
    assert_eq!(conjunctive_normal_form("AB|!C!&"), "C!A!B!&&");
}
