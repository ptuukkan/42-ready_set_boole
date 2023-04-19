use crate::{ast::Node, helpers::*};

enum RewriteRule {
    DoubleNegation,
}

fn is_negation(node: &Box<Node<char>>) -> Option<&Box<Node<char>>> {
    match **node {
        Node::UnaryExpr { ref op, ref child } => match op {
            UnaryOperator::Negation => Some(child),
        },
        _ => None,
    }
}

fn rewrite_rule(node: &mut Node<char>) -> &mut Node<char> {
    match node {
        Node::Operand(_) => node,
        Node::UnaryExpr { op, child } => match op {
            UnaryOperator::Negation => match is_negation(&*child) {
                Some(grand_child) => {
                    child = grand_child;
                    node
                }
                None => node,
            },
        },
        Node::BinaryExpr { op, left, right } => node,
    }
}

fn joku(tree: &mut Node<char>) -> Node<char> {
    match rewrite_rule(tree) {
        Node::Operand(_) => todo!(),
        Node::UnaryExpr { op, child } => todo!(),
        Node::BinaryExpr { op, left, right } => todo!(),
    };
}

pub fn negation_normal_form(formula: &str) -> String {
    if let Ok(parse_result) = parse_formula(formula) {
        let pla = parse_result.tree.into_iter().collect::<String>();
        return pla;
    }
    String::new()
}

#[test]
fn test_negation_normal_form() {
    assert_eq!("A!B!|", negation_normal_form("AB&!"));
    assert_eq!("A!B!&", negation_normal_form("AB|!"));
    assert_eq!("A!B|", negation_normal_form("AB>"));
    assert_eq!("AB&A!B!&|", negation_normal_form("AB="));
    assert_eq!("A!B!&C!|", negation_normal_form("AB|C&!"));
}
