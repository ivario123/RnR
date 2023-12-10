

use super::{Eval, ValueMeta, VarEnv, VmErr};
use crate::ast::{Expr, Literal, Statement};
impl Statement {
    fn eval_internal(&self, env: &mut VarEnv, scope: usize) -> Result<crate::ast::Literal, VmErr> {
        if env.len() < scope {
            return Err(VmErr::Err(
                "Trying to read from undecleared scope".to_owned(),
            ));
        }
        let ret = match self.clone() {
            // The type is unused in the VM as the type checker should
            // already have validated that the type is correct.
            Statement::Let(id, mutable, _t, e) => {
                // let a: i32 = 5 + 2
                // for now just accept an ident
                let expr_type = match e {
                    Some(e) => match e {
                        Expr::Lit(l) => Some(l),
                        e => Some(e.eval(env, env.len() - 1)?),
                    },
                    _ => None,
                };
                let meta = ValueMeta { value: expr_type };

                env.get_mut(scope)
                    .unwrap()
                    .0
                    .insert(format!("{}", id), meta.clone());
                Ok(Literal::Unit)
            }
            Statement::Expr(e) => {
                // the type of an Expr is returned
                e.eval(env, env.len() - 1)
            }
            Statement::Assign(id, e) => {
                let len = env.len();
                let rhs = match (e.eval(env, len - 1), scope) {
                    // If we can't eval in this scope go one lower
                    (Ok(val), _) => Ok(val),
                    (Err(e), 0) => Err(e),
                    (_, idx) => e.eval(env, idx - 1),
                }?;

                let _ = match (id.clone().assign(env, len - 1, rhs.clone()), scope) {
                    // If we can't eval in this scope go one lower
                    (Ok(_), _) => Ok(()),
                    (Err(e), 0) => Err(e),
                    (_, idx) => id.clone().assign(env, idx - 1, rhs),
                }?;

                Ok(Literal::Unit)
            }
            Statement::While(e, b) => {
                // First actual VM thingie.
                //
                // This is a quite simple case, we just loop while e
                // is true.
                let mut ret: Literal = Literal::Unit;
                let mut iter_counter = 0;
                while let Ok(Literal::Bool(true)) = e.eval(env, env.len() - 1) {
                    ret = match b.eval(env, env.len() - 1) {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(VmErr::Err(
                                format!(
                                    "Error {e:?} occured in iteration {iter_counter} of \n{self}"
                                )
                                .to_owned(),
                            ))
                        }
                    };
                    iter_counter += 1;
                }
                Ok(ret)
            }
            Statement::Block(b) => match b.eval(env, scope) {
                Ok(ty) => Ok(ty),
                Err(e) => Err(e),
            },
            Statement::FnDecleration(func) => {
                // We have a function decleration, this should be inserted into the fn env and then
                // the 0th env and a new function env should be used to check wether or not the
                // internal code is valid

                let id = match func.id {
                    Expr::Ident(id) => Ok(id),
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

                let args: Vec<Expr> = func.args.iter().map(|arg| arg.id.clone()).collect();

                // Add in the new function and assume correctly typed for now
                let meta = super::FunctionMeta {
                    args: args
                        .iter()
                        .map(|id| match id {
                            Expr::Ident(id) => id.clone(),
                            _ => unreachable!(),
                        })
                        .collect(),
                    body: func.body,
                };

                env.get_mut(scope).unwrap().1.insert(id, meta);
                Ok(Literal::Unit)
            }
        };
        match (ret, scope) {
            (Ok(value), _) => Ok(value),
            (Err(e), 0) => Err(e),
            (Err(_), scope) => self.eval(env, scope - 1),
        }
    }
}

impl super::Eval for Statement {
    fn eval(&self, env: &mut VarEnv, scope: usize) -> Result<crate::ast::Literal, VmErr> {
        let ret = self.eval_internal(env, scope);
        match ret {
            Err(VmErr::Err(e)) => {
                let pretty = if e != "!e.is_empty()" {
                    format!("Error : {e}\nOccured during execution of statement {self}")
                } else {
                    format!("Occured during execution of statement {self}")
                };
                eprintln!("{}", pretty);
                // This is a bit ugly, ideally we should have some queue here.
                // But this interface makes for a nice stack trace
                Err(VmErr::Err("".to_owned()))
            }
            value => value,
        }
    }
}
