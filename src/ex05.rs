use crate::{proposition::Proposition, propositional_formula::PropositionalFormula};

impl PropositionalFormula {
    pub fn to_nnf(&self) -> Self {
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
                    Proposition::Variable(x) => {
                        Proposition::Negation(Box::new(Proposition::Variable(x)))
                    }
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
        Self {
            variables: self.variables.clone(),
            formula: rewrite(self.formula.clone()),
        }
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    if let Ok(pf) = PropositionalFormula::try_from(formula) {
        pf.to_nnf().formula.to_string()
    } else {
        String::from("Error")
    }
}

#[test]
fn test_negation_normal_form() {

    test_nnf("A", "A");
    test_nnf("A!", "A!");
    test_nnf("AB&!", "A!B!|");
    test_nnf("AB|!", "A!B!&");
    test_nnf("AB>!", "AB!&");
    test_nnf("AB=!", "AB!&BA!&|");
    test_nnf("AB>", "A!B|");
    test_nnf("AB=", "A!B|B!A|&");
    test_nnf("AB|C&!", "A!B!&C!|");
    test_nnf("A!B>", "AB|");
    test_nnf("A!B^", "A!B!&AB&|");
    test_nnf("ABC||", "ABC||");
    test_nnf("ABC||!", "A!B!C!&&");
    test_nnf("ABC|&", "ABC|&");
    test_nnf("ABC&|", "ABC&|");
    test_nnf("ABC&|!", "A!B!C!|&");
    test_nnf("ABC^^", "AB!C|BC!|&&A!BC!&B!C&|&|");
    test_nnf("ABC>>", "A!B!C||");


    fn test_nnf(formula: &str, expected: &str) {
        let pf = PropositionalFormula::try_from(formula).unwrap();
        let nnf = pf.to_nnf();
        assert_eq!(expected, nnf.formula.to_string());
        assert_eq!(pf.get_truth_table(), nnf.get_truth_table());
    }
}
