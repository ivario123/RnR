use super::Eval;
use crate::ast::program::*;

impl Eval for Prog {
    fn eval(
        &self,
        env: &mut super::VarEnv,
        scope: usize,
    ) -> Result<crate::ast::Literal, super::VmErr> {
        todo!()
    }
}
