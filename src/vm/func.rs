use super::{VmErr};
use crate::ast::{Expr, Literal};

impl super::Eval for crate::ast::func::Func {
    fn eval(&self, env: &mut super::VarEnv, scope: usize) -> Result<Literal, VmErr> {
        // We have a function decleration, this should be inserted into the fn env and then
        // the 0th env and a new function env should be used to check wether or not the
        // internal code is valid

        let id = match &self.id {
            Expr::Ident(id) => Ok(id.clone()),
            exp => Err(VmErr::Err(format!(
                "Cannot treat {exp} as a function identifier"
            ))),
        }?;

        let mut tmp_idx = env.len();
        while let Some(idx) = tmp_idx.checked_sub(1) {
            tmp_idx = idx;
            if let Some(_) = env.get(idx).unwrap().1.get(&id) {
                return Err(VmErr::Err(format!("Function {id} already defined.")));
            }
        }

        let args: Vec<Expr> = self.args.iter().map(|arg| arg.id.clone()).collect();

        // Add in the new function and assume correctly typed for now
        let meta = super::FunctionMeta {
            args: args
                .iter()
                .map(|id| match id {
                    Expr::Ident(id) => id.clone(),
                    _ => unreachable!(),
                })
                .collect(),
            // I really do not like this, this should probably be moved to some
            // external function that pre processes all function declarions in to a jagged array.
            body: self.body.clone(),
        };

        env.get_mut(scope).unwrap().1.insert(id, meta);
        Ok(Literal::Unit)
    }
}
