use std::collections::HashMap;

use super::{Scope, Values, VarEnv, VmErr};
use crate::ast::{Block, Literal};
impl super::Eval for Block {
    fn eval(
        &self,
        env: &mut VarEnv,
        _: usize,
        max_iter: usize,
        iter_counter: &mut usize,
    ) -> Result<Values, VmErr> {
        // Push a new scope for the block.
        let scope = Scope::new();
        env.push((scope, HashMap::new()));
        let len = env.len() - 1;

        let mut return_value = Values::Lit(Literal::Unit);
        for stmt in &self.statements {
            // update the return type for each iteration
            return_value = stmt.eval(env, len, max_iter, iter_counter)?
        }
        // Instead we simply drop the latest scope
        let _ = env.pop();

        if self.semi {
            Ok(Values::Lit(Literal::Unit))
        } else {
            Ok(return_value)
        }
    }
}
