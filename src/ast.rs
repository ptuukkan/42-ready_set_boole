use crate::helpers::{BinaryOperator, UnaryOperator};

pub enum Node<T> {
    Operand(T),
    UnaryExpr {
        op: UnaryOperator,
        child: Box<Node<T>>,
    },
    BinaryExpr {
        op: BinaryOperator,
        left: Box<Node<T>>,
        right: Box<Node<T>>,
    },
}

fn unary_to_char(op: &UnaryOperator) -> char {
         match op {
            UnaryOperator::Negation => '!',
        }
    
}

fn binary_to_char(op: &BinaryOperator) -> char {
         match op {
            BinaryOperator::Conjunction => '&',
            BinaryOperator::Disjunction => '|',
            BinaryOperator::ExclusiveDisjunction => '^',
            BinaryOperator::MaterialCondition => '>',
            BinaryOperator::LogicalEquivalence => '=',
        }
    
}

impl Node<char> {
    fn postorder(&self) -> Vec<char> {
        let mut result = vec![];

        match self {
            Node::Operand(c) => result.push(c.to_owned()),
            Node::UnaryExpr { op, child } => {
                result.append(&mut child.postorder());
                result.push(unary_to_char(op));
            }
            Node::BinaryExpr { op, left, right } => {
                result.append(&mut left.postorder());
                result.append(&mut right.postorder());
                result.push(binary_to_char(op));
            }
        }

        result
    }
}

impl IntoIterator for Node<char> {
    type Item = char;
    type IntoIter = std::vec::IntoIter<char>;

    fn into_iter(self) -> Self::IntoIter {
        self.postorder().into_iter()
    }
}
