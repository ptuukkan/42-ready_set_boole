use std::collections::BTreeMap;

use crate::helpers::*;

struct ParseResult {
    tree: Node<char>,
    operands: BTreeMap<char, bool>,
}

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

fn parse_formula(formula: &str) -> Result<ParseResult, FormulaError> {
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
    Ok(ParseResult {
        tree: btree,
        operands,
    })
}

fn evaluate(node: &Node<char>, value_map: &BTreeMap<char, bool>) -> bool {
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

fn get_value_map(value_map: &BTreeMap<char, bool>, mask: &u32) -> BTreeMap<char, bool> {
    let mut masked_value_map = BTreeMap::new();

    for (i, c) in value_map.iter().rev().map(|v| v.0).enumerate() {
        masked_value_map.insert(c.to_owned(), (mask >> i & 1) == 1);
    }
    masked_value_map
}

fn print_header(value_map: &BTreeMap<char, bool>) {
    let header = value_map
        .iter()
        .map(|v| v.0.to_string())
        .collect::<Vec<String>>()
        .join(" | ");

    println!("| {} | = |", header);

    let border = value_map
        .iter()
        .map(|_v| "-".to_string())
        .collect::<Vec<String>>()
        .join("-|-");

    println!("|-{}-|---|", border);
}

fn print_row(value_map: &BTreeMap<char, bool>, result: bool) {
    let row = value_map
        .iter()
        .map(|v| match *v.1 {
            true => "1",
            false => "0",
        })
        .collect::<Vec<&str>>()
        .join(" | ");

    println!(
        "| {} | {} |",
        row,
        match result {
            true => "1",
            false => "0",
        }
    );
}

pub fn print_truth_table(formula: &str) {
    if let Ok(parse_result) = parse_formula(formula) {
        let high_mask: u32 = 1 << parse_result.operands.len();

        print_header(&parse_result.operands);

        for mask in 0..high_mask {
            let value_map = get_value_map(&parse_result.operands, &mask);
            let result = evaluate(&parse_result.tree, &value_map);
            print_row(&value_map, result);
        }
    } else {
        println!("Error");
    }
}
