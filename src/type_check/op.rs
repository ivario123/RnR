use crate::ast::{BinaryOp, UnaryOp};

/// A generic operation allows the type checker to know expected return types and
/// operands
pub trait Operation {
    type Operands;
    fn return_type(&self, operands: Self::Operands) -> Result<super::Type, super::TypeErr>;
    fn type_check(&self, operands: Self::Operands) -> bool;
}

impl Operation for BinaryOp {
    type Operands = (super::Type, super::Type);
    fn return_type(&self, operands: Self::Operands) -> Result<super::Type, super::TypeErr> {
        match self {
            BinaryOp::And | BinaryOp::Or | BinaryOp::Eq | BinaryOp::Lt | BinaryOp::Gt => {
                Ok(super::Type::Bool)
            }
            _ => Ok(super::Type::I32),
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
    fn return_type(&self, operands: Self::Operands) -> Result<super::Type, super::TypeErr> {
        match self {
            Self::Not => Ok(super::Type::Bool),
            Self::Subtract => Ok(super::Type::I32),
            Self::Borrow => Ok(super::Type::Ref(operands.into())),
            Self::BorrowMut => Ok(super::Type::Ref(operands.into())),
            Self::Dereff => match operands {
                super::Type::Ref(crate::ast::types::Ref(ty)) => Ok(*ty),
                ty => Err(format!("Cannot treat {} as a refference", ty)),
            },
        }
    }
    fn type_check(&self, operands: Self::Operands) -> bool {
        match self {
            Self::Not => operands == super::Type::Bool,
            Self::Subtract => operands == super::Type::I32,
            Self::Borrow => true,
            Self::BorrowMut => true,
            Self::Dereff => match operands {
                super::Type::Ref(_) => true,
                _ => false,
            },
        }
    }
}
