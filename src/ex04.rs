use crate::helpers::*;
use itertools::Itertools;

fn parse_symbol(c: &char) -> Result<Symbol<char>, FormulaError> {
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

fn parse_formula(formula: &str) -> Result<Node<char>, FormulaError> {
    let mut stack: Vec<Node<char>> = Vec::new();

    for c in formula.chars() {
        let symbol = parse_symbol(&c)?;
        match symbol {
            Symbol::Operand(operand) => stack.push(Node::Operand(operand)),
            Symbol::UnaryOperator(operator) => {
                let child = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                let node = Node::UnaryExpr {
                    op: operator,
                    child: Box::new(child),
                };
                stack.push(node)
            }
            Symbol::BinaryOperator(operator) => {
                let left = stack.pop().ok_or(FormulaError::InvalidFormula)?;
                let right = stack.pop().ok_or(FormulaError::InvalidFormula)?;
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
    Ok(btree)
}

fn evaluate(node: &Node<bool>) -> bool {
    fn evaluateInner(node: &Node<bool>) -> bool {
        match node {
            Node::Operand(n) => *n,
            Node::UnaryExpr { op, child } => match op {
                UnaryOperator::Negation => !evaluate(child),
            },
            Node::BinaryExpr { op, left, right } => {
                let p = evaluate(left);
                let q = evaluate(right);
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
    evaluateInner(node)
}
pub fn print_truth_table(formula: &str) -> bool {
    let perms = (0..2).multi_cartesian_product(); 
    for p in perms.into_iter() {
        println!("{:?}", p);
    }
    true
}
