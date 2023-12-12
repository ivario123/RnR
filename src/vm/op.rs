use crate::ast::Literal;

use super::{Values, VmErr};

pub trait Operation {
    type Operands;
    fn eval(&self, operands: Self::Operands) -> Result<Values, VmErr>;
}

impl Operation for crate::ast::UnaryOp {
    type Operands = Values;
    fn eval(&self, operands: Self::Operands) -> Result<Values, VmErr> {
        use crate::ast::UnaryOp::*;
        match (self, operands) {
            (Not, Values::Lit(Literal::Bool(val))) => Ok(Values::Lit(Literal::Bool(!val))),
            (Not, Values::Lit(Literal::Int(val))) => Ok(Values::Lit(Literal::Int(!val))),
            (Subtract, Values::Lit(Literal::Int(val))) => Ok(Values::Lit(Literal::Int(-val))),
            (op, expr) => Err(VmErr::Err(format!(
                "Opration {op} can only be applied to literal valued epxressions, got {expr}"
            ))),
        }
    }
}
