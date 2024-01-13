use std::collections::BTreeMap;

use crate::proposition::*;
pub enum FormulaError {
    UnexpectedSymbol,
    InvalidFormula,
}

pub struct ParseResult {
    pub proposition: Proposition,
    pub operands: BTreeMap<char, bool>,
}

pub fn parse_formula(
    formula: &str,
    allowed_variables: &Vec<char>,
) -> Result<ParseResult, FormulaError> {
    let mut stack: Vec<Proposition> = Vec::new();
    let mut operands: BTreeMap<char, bool> = BTreeMap::new();

    for c in formula.chars() {
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
            _ => {
                if allowed_variables.contains(&c) {
                    operands.insert(c, false);
                    stack.push(Proposition::Variable(c));
                } else {
                    return Err(FormulaError::UnexpectedSymbol);
                }
            }
        }
    }

    let tree = stack.pop().ok_or(FormulaError::InvalidFormula)?;
    if !stack.is_empty() {
        return Err(FormulaError::InvalidFormula);
    }
    Ok(ParseResult {
        proposition: tree,
        operands,
    })
}

pub fn evaluate(proposition: &Proposition, value_map: &BTreeMap<char, bool>) -> bool {
    match proposition {
        Proposition::Conjunction(p, q) => evaluate(p, value_map) && evaluate(q, value_map),
        Proposition::Disjunction(p, q) => evaluate(p, value_map) || evaluate(q, value_map),
        Proposition::ExclusiveDisjunction(p, q) => evaluate(p, value_map) ^ evaluate(q, value_map),
        Proposition::LogicalEquivalence(p, q) => evaluate(p, value_map) == evaluate(q, value_map),
        Proposition::MaterialCondition(p, q) => {
            !(evaluate(p, value_map) || evaluate(q, value_map))
        }
        Proposition::Negation(p) => !evaluate(p, value_map),
        Proposition::Variable(x) => *value_map.get(x).unwrap(),
    }
}

pub fn get_value_map(value_map: &BTreeMap<char, bool>, mask: &u32) -> BTreeMap<char, bool> {
    let mut masked_value_map = BTreeMap::new();

    for (i, c) in value_map.iter().rev().map(|v| v.0).enumerate() {
        masked_value_map.insert(c.to_owned(), (mask >> i & 1) == 1);
    }
    masked_value_map
}

