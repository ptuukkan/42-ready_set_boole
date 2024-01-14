use std::fmt;

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

impl From<&Proposition> for char {
    fn from(value: &Proposition) -> Self {
        match value {
            Proposition::Conjunction(_, _) => '&',
            Proposition::Disjunction(_, _) => '|',
            Proposition::ExclusiveDisjunction(_, _) => '^',
            Proposition::LogicalEquivalence(_, _) => '=',
            Proposition::MaterialCondition(_, _) => '>',
            Proposition::Negation(_) => '!',
            Proposition::Variable(x) => x.to_owned(),
        }
    }
}

impl Proposition {
    fn postorder(&self) -> Vec<char> {
        let mut result = vec![];

        match self {
            Proposition::Conjunction(p, q)
            | Proposition::Disjunction(p, q)
            | Proposition::ExclusiveDisjunction(p, q)
            | Proposition::LogicalEquivalence(p, q)
            | Proposition::MaterialCondition(p, q) => {
                result.append(&mut p.postorder());
                result.append(&mut q.postorder());
                result.push(char::from(self))
            }
            Proposition::Negation(p) => {
                result.append(&mut p.postorder());
                result.push(char::from(self));
            }
            Proposition::Variable(_) => result.push(char::from(self)),
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
