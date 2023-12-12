use super::{Eval, ValueMeta, Values, VarEnv, VmErr};
use crate::ast::{Expr, Literal, Statement};
impl Statement {
    fn eval_internal(
        &self,
        env: &mut VarEnv,
        scope: usize,
        max_iter: usize,
        iter_counter: &mut usize,
    ) -> Result<Values, VmErr> {
        if env.len() < scope {
            return Err(VmErr::Err(
                "Trying to read from undecleared scope".to_owned(),
            ));
        }
        let ret = match self.clone() {
            // The type is unused in the VM as the type checker should
            // already have validated that the type is correct.
            Statement::Let(id, _mutable, _t, e) => {
                // let a: i32 = 5 + 2
                // for now just accept an ident
                let expr_type = match e {
                    Some(e) => match e {
                        Expr::Lit(l) => Some(Values::Lit(l)),

                        e => Some(e.eval(env, env.len() - 1, max_iter, iter_counter)?),
                    },
                    _ => None,
                };
                let meta = ValueMeta { value: expr_type };

                env.get_mut(scope)
                    .unwrap()
                    .0
                    .insert(format!("{}", id), meta.clone());
                Ok(Values::Lit(Literal::Unit))
            }
            Statement::Expr(e) => {
                // the type of an Expr is returned
                e.eval(env, env.len() - 1, max_iter, iter_counter)
            }
            Statement::Assign(id, e) => {
                let len = env.len();
                let rhs = match (e.eval(env, len - 1, max_iter, iter_counter), scope) {
                    // If we can't eval in this scope go one lower
                    (Ok(val), _) => Ok(val),
                    (Err(e), 0) => Err(e),
                    (_, idx) => e.eval(env, idx - 1, max_iter, iter_counter),
                }?;

                match (
                    id.clone()
                        .assign(env, len - 1, rhs.clone(), max_iter, iter_counter),
                    scope,
                ) {
                    // If we can't eval in this scope go one lower
                    (Ok(_), _) => Ok(()),
                    (Err(e), 0) => Err(e),
                    (_, idx) => id.clone().assign(env, idx - 1, rhs, max_iter, iter_counter),
                }?;

                Ok(Values::Lit(Literal::Unit))
            }
            Statement::While(e, b) => {
                // First actual VM thingie.
                //
                // This is a quite simple case, we just loop while e
                // is true.
                let mut ret: Values = Values::Lit(Literal::Unit);
                while let Ok(Values::Lit(Literal::Bool(true))) =
                    e.eval(env, env.len() - 1, max_iter, iter_counter)
                {
                    ret = match b.eval(env, env.len() - 1, max_iter, iter_counter) {
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
                    *iter_counter += 1;
                }
                Ok(ret)
            }
            Statement::Block(b) => match b.eval(env, scope, max_iter, iter_counter) {
                Ok(ty) => Ok(ty),
                Err(e) => Err(e),
            },
            Statement::FnDecleration(func) => func.eval(env, scope, max_iter, iter_counter),
        };
        match (ret, scope) {
            (Ok(value), _) => Ok(value),
            (Err(e), 0) => Err(e),
            (Err(_), scope) => self.eval(env, scope - 1, max_iter, iter_counter),
        }
    }
}

impl super::Eval for Statement {
    fn eval(
        &self,
        env: &mut VarEnv,
        scope: usize,
        max_iter: usize,
        iter_counter: &mut usize,
    ) -> Result<Values, VmErr> {
        *iter_counter += 1;
        if *iter_counter > max_iter {
            return Err(VmErr::Err(format!("Itteration roof {max_iter} reached ")));
        }
        let ret = self.eval_internal(env, scope, max_iter, iter_counter);
        match ret {
            Err(VmErr::Err(e)) => {
                let pretty = if e != "!e.is_empty()" {
                    format!("Error : {e}\nOccured during execution of statement {self} @ instruction count {}",*iter_counter)
                } else {
                    format!("Occured during execution of statement {self}")
                };
                eprintln!("{}", pretty);
                // This is a bit ugly, ideally we should have some queue here.
                // But this interface makes for a nice stack trace
                Err(VmErr::Handled("".to_string()))
            }
            value => value,
        }
    }
}
