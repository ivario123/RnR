#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Lt,
    Gt,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Subtract,
    Borrow,
    BorrowMut,
    Dereff,
}
