use std::{fmt, collections::BTreeMap};

#[derive(Clone)]
pub enum Proposition {
    Conjunction(Box<Proposition>, Box<Proposition>),
    Disjunction(Box<Proposition>, Box<Proposition>),
    ExclusiveDisjunction(Box<Proposition>, Box<Proposition>),
    LogicalEquivalence(Box<Proposition>, Box<Proposition>),
    MaterialCondition(Box<Proposition>, Box<Proposition>),
    Negation(Box<Proposition>),
    Variable(char),
}

fn proposition_to_char(proposition: &Proposition) -> char {
    match proposition {
        Proposition::Conjunction(_, _) => '&',
        Proposition::Disjunction(_, _) => '|',
        Proposition::ExclusiveDisjunction(_, _) => '^',
        Proposition::LogicalEquivalence(_, _) => '=',
        Proposition::MaterialCondition(_, _) => '>',
        Proposition::Negation(_) => '!',
        Proposition::Variable(x) => x.to_owned(),
    }
}

impl Proposition {
    fn postorder(&self) -> Vec<char> {
        let mut result = vec![];

        match self {
            Proposition::Conjunction(p, q) => {
                result.append(&mut p.postorder());
                result.append(&mut q.postorder());
                result.push(proposition_to_char(self))
            }
            Proposition::Disjunction(p, q) => {
                result.append(&mut p.postorder());
                result.append(&mut q.postorder());
                result.push(proposition_to_char(self))
            }
            Proposition::ExclusiveDisjunction(p, q) => {
                result.append(&mut p.postorder());
                result.append(&mut q.postorder());
                result.push(proposition_to_char(self))
            }
            Proposition::LogicalEquivalence(p, q) => {
                result.append(&mut p.postorder());
                result.append(&mut q.postorder());
                result.push(proposition_to_char(self))
            }
            Proposition::MaterialCondition(p, q) => {
                result.append(&mut p.postorder());
                result.append(&mut q.postorder());
                result.push(proposition_to_char(self))
            }
            Proposition::Negation(p) => {
                result.append(&mut p.postorder());
                result.push(proposition_to_char(self));
            }
            Proposition::Variable(_) => result.push(proposition_to_char(self)),
        }

        result
    }
}

impl IntoIterator for Proposition {
    type Item = char;
    type IntoIter = std::vec::IntoIter<char>;

    fn into_iter(self) -> Self::IntoIter {
        self.postorder().into_iter()
    }
}

impl fmt::Display for Proposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = self.clone().into_iter().collect::<String>();
        write!(f, "{}", str)
    }
}
