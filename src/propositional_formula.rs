use std::collections::{HashSet, BTreeMap};

use crate::proposition::Proposition;

pub enum FormulaError {
    InvalidFormula,
}

pub struct PropositionalFormula {
    pub variables: HashSet<char>,
    pub formula: Proposition,
}

impl PropositionalFormula {
    pub fn evaluate(&self, variable_map: &BTreeMap<char, bool>) -> bool {
        self.formula.evaluate(variable_map)
    }
}

impl TryFrom<&str> for PropositionalFormula {
    type Error = FormulaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut stack: Vec<Proposition> = Vec::new();
        let mut variables: HashSet<char> = HashSet::new();
        for c in value.chars() {
            match c {
                '&' => {
                    let q = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                    let p = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                    stack.push(Proposition::Conjunction(Box::new(p), Box::new(q)));
                }
                '|' => {
                    let q = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                    let p = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                    stack.push(Proposition::Disjunction(Box::new(p), Box::new(q)));
                }
                '^' => {
                    let q = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                    let p = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                    stack.push(Proposition::ExclusiveDisjunction(Box::new(p), Box::new(q)));
                }
                '=' => {
                    let q = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                    let p = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                    stack.push(Proposition::LogicalEquivalence(Box::new(p), Box::new(q)));
                }
                '>' => {
                    let q = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                    let p = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                    stack.push(Proposition::MaterialCondition(Box::new(p), Box::new(q)));
                }
                '!' => {
                    let p = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                    stack.push(Proposition::Negation(Box::new(p)));
                }
                'A'..='Z' | '0'..='1' => {
                    variables.insert(c);
                    stack.push(Proposition::Variable(c));
                }
                _ => {}
            }
        }

        let formula = stack.pop().ok_or(FormulaError::InvalidFormula)?;
        if !stack.is_empty() {
            return Err(FormulaError::InvalidFormula);
        }
        Ok(Self { variables, formula })
    }
}
