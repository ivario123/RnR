#[derive(Debug, Copy, Clone, PartialEq)]
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Subtract,
    Borrow,
}

/// A generic operation allows the type checker to know expected return types and
/// operands
pub trait Operation {
    type Operands;
    fn return_type(&self, operands: Self::Operands) -> super::Type;
    fn type_check(&self, operands: Self::Operands) -> bool;
}

impl Operation for BinaryOp {
    type Operands = (super::Type, super::Type);
    fn return_type(&self, operands: Self::Operands) -> super::Type {
        match self {
            BinaryOp::And | BinaryOp::Or | BinaryOp::Eq | BinaryOp::Lt | BinaryOp::Gt => {
                super::Type::Bool
            }
            _ => super::Type::I32,
        }
    }
    fn type_check(&self, operands: Self::Operands) -> bool {
        use super::Type;
        use BinaryOp::*;
        match self {
            Add | Sub | Mul | Div | Lt | Gt => matches!(operands, (Type::I32, Type::I32)),
            Eq => operands.0 == operands.1,
            And | Or => operands == (Type::Bool, Type::Bool),
        }
    }
}

impl Operation for UnaryOp {
    type Operands = super::Type;
    fn return_type(&self, operands: Self::Operands) -> super::Type {
        match self {
            Self::Not => super::Type::Bool,
            Self::Subtract => super::Type::I32,
            Self::Borrow => super::Type::Ref(operands.into()),
        }
    }
    fn type_check(&self, operands: Self::Operands) -> bool {
        match self {
            Self::Not => operands == super::Type::Bool,
            Self::Subtract => operands == super::Type::I32,
            Self::Borrow => true,
        }
    }
}
