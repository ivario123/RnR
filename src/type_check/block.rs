use super::{FunctionScope, Scope, TypeEnv, TypeErr};
use crate::ast::{Block, Type};
impl super::TypeCheck for Block {
    fn check(&self, env: &mut TypeEnv, _: usize) -> Result<Type, TypeErr> {
        // Push a new scope for the block.
        let scope = Scope::new();
        env.push((scope, FunctionScope::new()));
        let len = env.len() - 1;

        let mut return_ty = Type::Unit;
        for stmt in &self.statements {
            // update the return type for each iteration
            return_ty = stmt.check(env, len)?
        }
        for (id, meta) in env.pop().unwrap().0.iter() {
            match (meta.ty.clone(),meta.assigned)  {
                (Some(_),true) => {}
                _ => {
                    return Err(format!(
                        "Type of {id} must be known at the end of the block and it must allso be assigned"
                    ))
                }
            }
        }

        if self.semi {
            Ok(Type::Unit)
        } else {
            Ok(return_ty)
        }
    }
}
