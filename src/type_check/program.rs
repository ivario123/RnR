use super::TypeCheck;
use crate::ast::program::Prog;

impl TypeCheck for Prog {
    type ReturnType = ();
    fn check(
        &self,
        env: &mut super::TypeEnv,
        idx: usize,
    ) -> Result<Self::ReturnType, super::TypeErr> {
        todo!()
    }
}
