use super::TypeCheck;
use crate::ast::program::Prog;
use crate::intrinsics::vm_println;

impl TypeCheck for Prog {
    type ReturnType = ();
    fn check(
        &self,
        env: &mut super::TypeEnv,
        idx: usize,
    ) -> Result<Self::ReturnType, super::TypeErr> {
        let mut global_scope = (
            crate::type_check::Scope::new(),
            crate::type_check::FunctionScope::new(),
        );
        // Introduce compile builtins
        let (f, _body) = vm_println();
        match &f.id {
            crate::ast::Expr::Ident(id) => global_scope.1.insert(id.clone(), f.into()),
            e => return Err(format!("Malformed compiler built in {e}")),
        };
        env.push(global_scope);
        for el in self.statements.iter() {
            match el.check(env, idx)?{
                crate::ast::Type::Unit => {},
                t => return Err(format!("All top level expressions should return unit type, got {t} when evaluting {el}")),
            };
        }
        Ok(())
    }
}
