use std::collections::HashMap;

use super::{Scope, VarEnv, VmErr};
use crate::ast::{Block, Literal};
impl super::Eval for Block {
    fn eval(&self, env: &mut VarEnv, _: usize) -> Result<Literal, VmErr> {
        // Push a new scope for the block.
        let scope = Scope::new();
        env.push((scope, HashMap::new()));
        let len = env.len() - 1;

        let mut return_value = Literal::Unit;
        for stmt in &self.statements {
            // update the return type for each iteration
            return_value = stmt.eval(env, len)?
        }

        // Wether or not each variable has a value or not at the end of the scope should be caught
        // by the type checker so there is no need to check this in the VM.
        /*
        for (id, meta) in env.pop().unwrap().0.iter() {
            match (meta.value.clone(), meta.assigned) {
                (Some(_), true) => {}
                _ => {
                    // This is not really a hard error any more as this should have been caught by
                    // the type checker. So here we shouh
                    return Err(VmErr::Err(format!(
                        "{id} has no value at the end of the block"
                    )))
                }
            }
        }*/
        // Instead we simply drop the latest scope
        let _ = env.pop();

        if self.semi {
            Ok(Literal::Unit)
        } else {
            Ok(return_value)
        }
    }
}
