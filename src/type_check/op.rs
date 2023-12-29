use crate::ast::{BinaryOp, UnaryOp, Type};

/// A generic operation allows the type checker to know expected return types and
/// operands
pub trait Operation {
    type Operands;
    fn return_type(&self, operands: Self::Operands) -> Result<super::Type, super::TypeErr>;
    fn type_check(&self, operands: Self::Operands) -> bool;
}

impl Operation for BinaryOp {
    type Operands = (super::Type, super::Type);
    
    fn return_type(&self, _operands: Self::Operands) -> Result<super::Type, super::TypeErr> {
        match self {
            BinaryOp::And | BinaryOp::Or | BinaryOp::Eq | BinaryOp::Lt | BinaryOp::Gt => {
                Ok(super::Type::Bool)
            }
            _ => Ok(super::Type::I32),
        }
    }
    
    fn type_check(&self, operands: Self::Operands) -> bool {
        match self {
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Lt | Self::Gt => matches!(operands, (Type::I32, Type::I32)),
            Self::Eq => operands.0 == operands.1,
            Self::And | Self::Or => operands == (Type::Bool, Type::Bool),
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
            Self::BorrowMut => Ok(super::Type::MutRef(operands.into())),
            Self::Dereff => match operands {
                super::Type::Ref(crate::ast::types::Ref(ty, _, _)) => Ok(*ty),
                super::Type::MutRef(crate::ast::types::Ref(ty, _, _)) => Ok(*ty),
                ty => Err(format!("Cannot treat {} as a reference", ty)),
            },
        }
    }
    
    fn type_check(&self, operands: Self::Operands) -> bool {
        match self {
            Self::Not => operands == super::Type::Bool,
            Self::Subtract => operands == super::Type::I32,
            Self::Borrow => true,
            Self::BorrowMut => true,
            Self::Dereff => {
                matches!(operands, super::Type::Ref(_))
                    || matches!(operands, super::Type::MutRef(_))
            }
        }
    }
}
