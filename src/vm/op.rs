use crate::ast::Literal;

use super::VmErr;

pub trait Operation {
    type Operands;
    fn eval(&self, operands: Self::Operands) -> Result<Literal, VmErr>;
}

impl Operation for crate::ast::UnaryOp {
    type Operands = Literal;
    fn eval(&self, operands: Self::Operands) -> Result<Literal, VmErr> {
        use crate::ast::UnaryOp::*;
        match (self, operands) {
            (Not, Literal::Bool(val)) => Ok(Literal::Bool(!val)),
            (Not, Literal::Int(val)) => Ok(Literal::Int(!val)),
            (Subtract, Literal::Int(val)) => Ok(Literal::Int(-val)),
            (op, expr) => Err(VmErr::Err(format!(
                "Opration {op} can only be applied to literal valued epxressions, got {expr}"
            ))),
        }
    }
}
