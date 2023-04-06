
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
