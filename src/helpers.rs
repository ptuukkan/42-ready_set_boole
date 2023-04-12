use std::collections::BTreeMap;

use crate::ast::Node;
pub enum FormulaError {
    UnexpectedSymbol,
    InvalidFormula,
}

pub enum UnaryOperator {
    Negation,
}

pub enum BinaryOperator {
    Conjunction,
    Disjunction,
    ExclusiveDisjunction,
    MaterialCondition,
    LogicalEquivalence,
}

pub enum Symbol<T> {
    Operand(T),
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
}

pub struct ParseResult {
    pub tree: Node<char>,
    pub operands: BTreeMap<char, bool>,
}

pub fn parse_symbol(c: &char) -> Result<Symbol<char>, FormulaError> {
    match c {
        'A'..='Z' => Ok(Symbol::Operand(c.to_owned())),
        '!' => Ok(Symbol::UnaryOperator(UnaryOperator::Negation)),
        '&' => Ok(Symbol::BinaryOperator(BinaryOperator::Conjunction)),
        '|' => Ok(Symbol::BinaryOperator(BinaryOperator::Disjunction)),
        '^' => Ok(Symbol::BinaryOperator(BinaryOperator::ExclusiveDisjunction)),
        '>' => Ok(Symbol::BinaryOperator(BinaryOperator::MaterialCondition)),
        '=' => Ok(Symbol::BinaryOperator(BinaryOperator::LogicalEquivalence)),
        _ => Err(FormulaError::UnexpectedSymbol),
    }
}

pub fn parse_formula(formula: &str) -> Result<ParseResult, FormulaError> {
    let mut stack: Vec<Node<char>> = Vec::new();
    let mut operands: BTreeMap<char, bool> = BTreeMap::new();

    for c in formula.chars() {
        let symbol = parse_symbol(&c)?;
        match symbol {
            Symbol::Operand(operand) => {
                stack.push(Node::Operand(operand));
                operands.insert(operand, false);
            }
            Symbol::UnaryOperator(operator) => {
                let child = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                let node = Node::UnaryExpr {
                    op: operator,
                    child: Box::new(child),
                };
                stack.push(node)
            }
            Symbol::BinaryOperator(operator) => {
                let right = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                let left = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                let node = Node::BinaryExpr {
                    op: operator,
                    left: Box::new(left),
                    right: Box::new(right),
                };
                stack.push(node)
            }
        }
    }

    let btree = stack.pop().ok_or(FormulaError::InvalidFormula)?;
    if !stack.is_empty() {
        return Err(FormulaError::InvalidFormula);
    }
    Ok(ParseResult {
        tree: btree,
        operands,
    })
}

pub fn evaluate(node: &Node<char>, value_map: &BTreeMap<char, bool>) -> bool {
    match node {
        Node::Operand(n) => *value_map.get(n).unwrap(),
        Node::UnaryExpr { op, child } => match op {
            UnaryOperator::Negation => !evaluate(child, value_map),
        },
        Node::BinaryExpr { op, left, right } => {
            let p = evaluate(left, value_map);
            let q = evaluate(right, value_map);
            match op {
                BinaryOperator::Conjunction => p && q,
                BinaryOperator::Disjunction => p || q,
                BinaryOperator::ExclusiveDisjunction => p ^ q,
                BinaryOperator::MaterialCondition => !(p && !q),
                BinaryOperator::LogicalEquivalence => p == q,
            }
        }
    }
}

pub fn get_value_map(value_map: &BTreeMap<char, bool>, mask: &u32) -> BTreeMap<char, bool> {
    let mut masked_value_map = BTreeMap::new();

    for (i, c) in value_map.iter().rev().map(|v| v.0).enumerate() {
        masked_value_map.insert(c.to_owned(), (mask >> i & 1) == 1);
    }
    masked_value_map
}
