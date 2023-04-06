enum Node {
    Operand(bool),
    UnaryExpr {
        op: UnaryOperator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: BinaryOperator,
        left: Box<Node>,
        right: Box<Node>,
    },
}

enum Error {
    UnexpectedSymbol,
    InvalidFormula,
}

enum UnaryOperator {
    Negation,
}

enum BinaryOperator {
    Conjunction,
    Disjunction,
    ExclusiveDisjunction,
    MaterialCondition,
    LogicalEquivalence,
}

enum Symbol {
    Operand(bool),
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
}

fn parse_symbol(c: &char) -> Result<Symbol, Error> {
    match c {
        '0' => Ok(Symbol::Operand(false)),
        '1' => Ok(Symbol::Operand(true)),
        '!' => Ok(Symbol::UnaryOperator(UnaryOperator::Negation)),
        '&' => Ok(Symbol::BinaryOperator(BinaryOperator::Conjunction)),
        '|' => Ok(Symbol::BinaryOperator(BinaryOperator::Disjunction)),
        '^' => Ok(Symbol::BinaryOperator(BinaryOperator::ExclusiveDisjunction)),
        '>' => Ok(Symbol::BinaryOperator(BinaryOperator::MaterialCondition)),
        '=' => Ok(Symbol::BinaryOperator(BinaryOperator::LogicalEquivalence)),
        _ => Err(Error::UnexpectedSymbol),
    }
}

fn parse_formula(formula: &str) -> Result<Node, Error> {
    let mut stack: Vec<Node> = Vec::new();

    for c in formula.chars() {
        let symbol = parse_symbol(&c)?;
        match symbol {
            Symbol::Operand(operand) => stack.push(Node::Operand(operand)),
            Symbol::UnaryOperator(operator) => {
                let child = stack.pop().ok_or(Error::InvalidFormula)?;
                let node = Node::UnaryExpr {
                    op: operator,
                    child: Box::new(child),
                };
                stack.push(node)
            }
            Symbol::BinaryOperator(operator) => {
                let left = stack.pop().ok_or(Error::InvalidFormula)?;
                let right = stack.pop().ok_or(Error::InvalidFormula)?;
                let node = Node::BinaryExpr {
                    op: operator,
                    left: Box::new(left),
                    right: Box::new(right),
                };
                stack.push(node)
            }
        }
    }

    let btree = stack.pop().ok_or(Error::InvalidFormula)?;
    if !stack.is_empty() {
        return Err(Error::InvalidFormula);
    }
    Ok(btree)
}

fn evaluate(node: &Node) -> bool {
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

pub fn eval_formula(formula: &str) -> bool {
    if let Ok(btree) = parse_formula(formula) {
        evaluate(&btree)
    } else {
        println!("Error parsing formula");
        false
    }
}
#[test]
fn test_eval_formula() {
    assert_eq!(eval_formula("10&"), false);
    assert_eq!(eval_formula("10|"), true);
    assert_eq!(eval_formula("11>"), true);
    assert_eq!(eval_formula("10="), false);
    assert_eq!(eval_formula("1011||="), true);
}
