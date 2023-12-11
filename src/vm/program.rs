use super::Eval;
use super::VmErr;
use crate::ast::program::*;
use crate::ast::Expr;
use crate::ast::FuncCall;
use crate::ast::Literal;
use crate::intrinsics::vm_println;

impl Eval for Prog {
    fn eval(
        &self,
        env: &mut super::VarEnv,
        scope: usize,
    ) -> Result<crate::ast::Literal, super::VmErr> {
        let mut global_scope = (crate::vm::Scope::new(), crate::vm::FunctionScope::new());
        // Introduce compile builtins
        let (f, _body) = vm_println();
        match &f.id {
            crate::ast::Expr::Ident(id) => global_scope.1.insert(id.clone(), f.into()),
            e => return Err(VmErr::Err(format!("Malformed compiler built in {e}"))),
        };
        env.push(global_scope);
        for el in self.statements.iter() {
            match el.eval(env, scope)?{
                crate::ast::Literal::Unit => {},
                t => return Err(VmErr::Err(format!("All top level statements should return unit value, got {t} when evaluting {el}"))),
            };
        }
        Expr::FuncCall(FuncCall {
            id: Box::new(Expr::Ident("main".to_owned())),
            args: Box::default(),
        })
        .eval(env, scope)?;
        Ok(Literal::Unit)
    }
}
