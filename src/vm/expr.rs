use std::convert::TryInto;

use super::{op::Operation, Eval, ValueMeta, Values, VarEnv, VmErr};
use crate::ast::{Expr, Literal, UnaryOp};

impl super::Eval for Expr {
    //.eval_expr
    // recursively.evals an expression for type correctness
    // on success: the expression type is returned
    // on failure, an expression type error is returned
    fn eval(
        &self,
        env: &mut VarEnv,
        scope: usize,
        max_iter: usize,
        iter_counter: &mut usize,
    ) -> Result<Values, VmErr> {
        if env.len() < scope {
            return Err(VmErr::Err("No scope decleared".to_owned()));
        }
        // This only makes sense for evaluating single expressions out side of a block.
        let last_scope = match env.len() {
            0 => 0,
            l => l - 1,
        };
        let ret = match self.clone() {
            Expr::Ident(id) => {
                let this_env = match env.get(scope) {
                    Some(scope) => &scope.0,
                    _ => {
                        return Err(VmErr::Err(format!(
                            "Tried to get scope {scope} when there were only {} scopes",
                            env.len()
                        )))
                    }
                };
                match (this_env.get(&id), scope) {
                    (Some(value), _) => match &value.value {
                        Some(value) => Ok(value.clone()),
                        _ => Err(VmErr::Err(format!(
                            "Value of variable {id} must be known at this point"
                        ))),
                    },
                    (None, 0) => Err(VmErr::Err("variable not found".to_string())),
                    (_, scope) => return self.eval(env, scope - 1, max_iter, iter_counter),
                }
            }

            Expr::Lit(l) => Ok(Values::Lit(l)),
            Expr::BinOp(op, l, r) => {
                let lhs = (*l).eval(env, last_scope, max_iter, iter_counter)?;
                let rhs = (*r).eval(env, last_scope, max_iter, iter_counter)?;
                Ok(op.eval(lhs, rhs)?)
            }
            Expr::Par(e) => (*e).eval(env, last_scope, max_iter, iter_counter),
            Expr::IfThenElse(cond, t, e) => {
                let cond = (*cond).eval(env, last_scope, max_iter, iter_counter)?;

                // Another actual VM part
                if let Values::Lit(Literal::Bool(true)) = cond {
                    t
                } else if let Values::Lit(Literal::Bool(false)) = cond {
                    match e {
                        Some(block) => block,
                        _ => return Ok(Values::Lit(Literal::Unit)),
                    }
                } else {
                    return Err(VmErr::Err(format!("Invalid expression {self}")));
                }
                .eval(env, last_scope, max_iter, iter_counter)
            }
            Expr::UnOp(UnaryOp::BorrowMut, e) | Expr::UnOp(UnaryOp::Borrow, e) => match *e {
                Expr::Ident(i) => {
                    let mut last_scope = env.len();
                    let mut scope = None;
                    while let Some(_) = last_scope.checked_sub(1) {
                        println!("Checking {:?} for {i}", env.get(last_scope));
                        last_scope -= 1;
                        let res = env.get(last_scope).unwrap().0.get(&i);
                        if res.is_some() {
                            scope = Some(last_scope);
                            break;
                        }
                    }
                    if scope.is_none() {
                        return Err(VmErr::Err(format!("Cannot find identifier {i}")));
                    }
                    let scope = scope.unwrap();
                    Ok(Values::Ref((i, scope as usize)))
                }
                e => Err(VmErr::Err(format!("Cannot borrow {e} mutably"))),
            },
            Expr::UnOp(UnaryOp::Dereff, e) => {
                let meta = e.eval(env, env.len() - 1, max_iter, iter_counter)?;
                let (id, idx) = match meta {
                    Values::Ref((id, idx)) => Ok((id, idx)),
                    e => Err(VmErr::Err(format!("Cannot derreference {e}"))),
                }?;

                let target_env = match env.get(idx) {
                    Some(env) => Ok(env),
                    _ => Err(VmErr::Err(format!("Invalid refference {self}"))),
                }?;

                match target_env.0.get(&id) {
                    Some(meta) => match &meta.value {
                        Some(value) => Ok(value.clone()),
                        _ => Err(VmErr::Err(format!("Cannot derreference unassigned value"))),
                    },
                    _ => Err(VmErr::Err(format!("Invalid refference {self}"))),
                }
            }
            Expr::UnOp(op, e) => {
                let got = (*e).eval(env, last_scope, max_iter, iter_counter)?;
                op.eval(got)
            }
            Expr::Array(elements) => {
                let mut inner = vec![];
                for el in elements {
                    inner.push(Box::new(
                        match el.eval(env, last_scope, max_iter, iter_counter)? {
                            Values::Lit(l) => l,
                            e => {
                                return Err(VmErr::Err(format!(
                                    "Cannot have an array of refferences {e:?}"
                                )))
                            }
                        },
                    ));
                }
                return Ok(Values::Lit(Literal::Array(inner)));
            }
            Expr::Index(id, index) => {
                let id = match *id {
                    Expr::Ident(id) => id,
                    ty => return Err(VmErr::Err(format!("{ty} does not implement index"))),
                };
                let meta: ValueMeta = (id, &env, last_scope).try_into()?;

                let val = match meta.value.clone() {
                    Some(Values::Lit(Literal::Array(values))) => Ok(values),
                    Some(ty) => return Err(VmErr::Err(format!("{ty:?} does not implement index"))),
                    None => return Err(VmErr::Err("Type must be known at this point".to_owned())),
                }?;

                let idx = match *index {
                    // If the idx is a constant we can.eval it
                    Expr::Lit(Literal::Int(idx)) => Ok(idx),
                    idx => match (idx.eval(env, scope - 1, max_iter, iter_counter), scope - 1) {
                        (Ok(Values::Lit(Literal::Int(val))), _) => Ok(val),
                        (_, 0) => Err(VmErr::Err(format!("Cannot convert {idx} into usize"))),
                        (_, index) => Ok(match idx.eval(env, index - 1, max_iter, iter_counter) {
                            Ok(Values::Lit(Literal::Int(val))) => Ok(val),
                            _ => Err(VmErr::Err(format!("Cannot convert {idx} into usize"))),
                        }?),
                    },
                }?;
                if idx as usize >= val.len() {
                    return Err(VmErr::Err(format!(
                        "Cannot access element at index {idx} since array is of size {}",
                        val.len()
                    )));
                }
                Ok(Values::Lit(*val[idx as usize].clone()))
            }
            Expr::IndexMut(id, index) => {
                match Expr::IndexMut(id.clone(), index).as_mut(
                    env,
                    scope,
                    max_iter,
                    iter_counter,
                )? {
                    Some(lit) => Ok(Values::Lit((*lit).clone())),
                    _ => Err(VmErr::Err(format!("Value {id} is unsagined"))),
                }
            }
            Expr::FuncCall(call) => {
                println!("Trying to call {call}");
                let curr_scope = match env.get(scope) {
                    Some(env) => Ok(env.clone()),
                    _ => Err(VmErr::Err("Invalid scope usage".to_owned())),
                }?;
                let id = match *call.id {
                    Expr::Ident(id) => Ok(id),
                    exp => Err(VmErr::Err(format!("Cannot treat {exp} as a function id"))),
                }?;
                let func_name = id.clone();

                let fndec = match (curr_scope.1.get(&id), scope) {
                    (Some(fndec), _) => Ok(fndec),
                    (_, 0) => Err(VmErr::Err(format!("Cannot find function {id}"))),
                    (_, _) => return self.eval(env, scope - 1, max_iter, iter_counter),
                }?;
                let mut args = vec![];
                let mut values = vec![];
                for (arg, value) in fndec.args.iter().zip(call.args.iter()) {
                    println!("Trying to locate {value}");
                    let intermediate = value.eval(env, last_scope, max_iter, iter_counter)?;
                    values.push(intermediate.clone());
                    args.push((arg.clone(), intermediate));
                }

                // Give function scope access to global scope and all of the accessible functions
                // Do not keep it on the stack
                let mut new_env = Box::<VarEnv>::default();
                let mut idx = 0;
                while let Some(env) = env.get(idx) {
                    let t_env = if idx == 0 {
                        env.0.clone()
                    } else {
                        std::collections::HashMap::new()
                    };
                    new_env.push((t_env, env.1.clone()));
                    idx += 1;
                }
                new_env.push((
                    std::collections::HashMap::new(),
                    std::collections::HashMap::new(),
                ));
                for (id, val) in args {
                    new_env
                        .get_mut(last_scope)
                        .unwrap()
                        .0
                        .insert(id.clone(), ValueMeta { value: Some(val) });
                }

                // Now we need to check if the call is intrinsic
                let ret = match func_name == *"println!" {
                    true => {
                        let (_f, body) = crate::intrinsics::vm_println();
                        body(values)
                    }
                    false => fndec
                        .body
                        .eval(&mut new_env, last_scope, max_iter, iter_counter)?,
                }; //fndec.rec_count -= 1;
                   // Allow mutable access to global scope
                env.get_mut(0).unwrap().0 = new_env.get(0).unwrap().0.clone();
                Ok(ret)
            }
            Expr::Block(b) => b.eval(env, env.len() - 1, max_iter, iter_counter),
        };
        match (ret, scope) {
            (Ok(value), _) => Ok(value),
            (Err(e), 0) => Err(e),
            (Err(_), idx) => self.eval(env, idx - 1, max_iter, iter_counter),
        }
    }
}
impl TryInto<ValueMeta> for (String, &&mut VarEnv, usize) {
    type Error = VmErr;
    fn try_into(self) -> Result<ValueMeta, Self::Error> {
        let (id, env, scope) = self;

        let this_env = match env.get(scope) {
            Some(scope) => scope.0.clone(),
            _ => {
                return Err(VmErr::Err(format!(
                    "Tried to get scope {scope} when there were only {} scopes",
                    env.len()
                )))
            }
        };
        match (this_env.get(&id), scope) {
            (Some(value), _) => Ok(value.clone()),
            (None, 0) => Err(VmErr::Err("variable not found".to_string())),
            (_, scope) => {
                let new_query = (id, env, scope - 1);
                new_query.try_into()
            }
        }
    }
}

impl Expr {
    pub fn assign(
        self,
        env: &mut VarEnv,
        scope: usize,
        value: Values,
        max_iter: usize,
        iter_counter: &mut usize,
    ) -> Result<(), VmErr> {
        match self.clone() {
            Expr::Ident(i) => {
                let this = match env.get_mut(scope) {
                    Some(scope) => &mut scope.0,
                    _ => return Err(VmErr::Err("Invalid scope".to_owned())),
                };
                match (this.get(&i), scope) {
                    (Some(_meta), _) => {
                        // Since we assume that the type checker has been ran before this the
                        // mutability of types is irrelevant, now we just assign the value to the
                        // variable
                        this.insert(i, ValueMeta { value: Some(value) });
                        Ok(())
                    }
                    (_, 0) => Err(VmErr::Err(format!("No such variable {i}"))),
                    (_, _) => self.assign(env, scope - 1, value, max_iter, iter_counter),
                }
            }
            Expr::IndexMut(id, idx) => {
                let idx = match *idx {
                    Expr::Lit(Literal::Int(idx)) => idx as usize,
                    expr => match (
                        (expr).eval(env, env.len() - 1, max_iter, iter_counter),
                        env.len() - 1,
                    ) {
                        (Ok(Values::Lit(Literal::Int(idx))), _) => idx as usize,
                        (_, 0) => {
                            return Err(VmErr::Err(format!("Cannot convert {expr} into usize")));
                        }
                        (_, idx) => match expr.eval(env, idx - 1, max_iter, iter_counter) {
                            Ok(Values::Lit(Literal::Int(idx))) => idx as usize,
                            _ => {
                                return Err(VmErr::Err(format!(
                                    "Cannot convert {expr} into usize"
                                )));
                            }
                        },
                    },
                };
                let id = match *id {
                    Expr::Ident(id) => id,
                    el => return Err(VmErr::Err(format!("Cannot use {el} as an identifier"))),
                };
                let this = match env.get_mut(scope) {
                    Some(scope) => &mut scope.0,
                    _ => return Err(VmErr::Err("Invalid scope".to_owned())),
                };
                // Now we can assume that the value is mutable as type checker should gaurantee
                // this
                match this.get_mut(&id) {
                    Some(meta) => match &mut meta.value {
                        Some(Values::Lit(Literal::Array(el))) => {
                            let value = match value {
                                Values::Lit(l) => l,
                                e => {
                                    return Err(VmErr::Err(format!(
                                        "Cannot assign {e} to an array"
                                    )))
                                }
                            };
                            *el[idx] = value;
                            Ok(())
                        }
                        el => Err(VmErr::Err(format!("{el:?} cannot be indexed"))),
                    },
                    _ => Err(VmErr::Err(format!("Use of undecleared variable {id}"))),
                }
            }
            Expr::UnOp(UnaryOp::Dereff, e) => {
                // First we have a simple way out, the expression is a mutable borrow
                let ret = e.eval(env, scope, max_iter, iter_counter)?;
                match ret {
                    Values::Ref((id, scope)) => {
                        let scope = match env.get_mut(scope) {
                            Some(scope) => scope,
                            _ => {
                                return Err(VmErr::Err("Invallid scope for refference".to_string()))
                            }
                        };
                        match scope.0.get(&id) {
                            Some(meta) => {
                                match meta.value {
                                    Some(Values::Ref(_)) => {
                                        return Err(VmErr::Err(format!(
                                            "Cannot assign to a refference"
                                        )))
                                    }
                                    _ => {}
                                };
                            }
                            _ => return Err(VmErr::Err(format!("Cannot find identifier {id}"))),
                        };
                        (*(scope.0.get_mut(&id).unwrap())).value = Some(value);
                        Ok(())
                    }
                    e => return Err(VmErr::Err(format!("Cannot derreference {e}"))),
                }
            }
            expr => Err(VmErr::Err(format!(
                "Cannot get {expr} as mutable identifiers"
            ))),
        }
    }
    pub fn as_mut<'a>(
        self,
        env: &'a mut VarEnv,
        scope: usize,
        max_iter: usize,
        iter_counter: &'a mut usize,
    ) -> Result<Option<&'a mut Literal>, VmErr> {
        match self.clone() {
            Expr::Ident(i) => {
                let this = match env.get(scope) {
                    Some(scope) => &scope.0,
                    _ => return Err(VmErr::Err("Invalid scope".to_owned())),
                };
                match (this.get(&i), scope) {
                    (Some(_meta), _) => {
                        // Now we can get it as immutable as it is no longer
                        match &mut env.get_mut(scope).unwrap().0.get_mut(&i).unwrap().value {
                            Some(Values::Lit(val)) => Ok(Some(val)),
                            _ => Ok(None),
                        }
                    }
                    (_, 0) => Err(VmErr::Err(format!("No such variable {i}"))),
                    (_, _) => self.as_mut(env, scope - 1, max_iter, iter_counter),
                }
            }
            Expr::IndexMut(id, idx) => {
                let idx = match *idx {
                    Expr::Lit(Literal::Int(idx)) => idx as usize,
                    expr => match (
                        (expr).eval(env, env.len() - 1, max_iter, iter_counter),
                        env.len() - 1,
                    ) {
                        (Ok(Values::Lit(Literal::Int(idx))), _) => idx as usize,

                        (_, 0) => {
                            return Err(VmErr::Err(format!("Cannot convert {expr} into usize")));
                        }
                        (_, idx) => match expr.eval(env, idx - 1, max_iter, iter_counter) {
                            Ok(Values::Lit(Literal::Int(idx))) => idx as usize,
                            _ => {
                                return Err(VmErr::Err(format!(
                                    "Cannot convert {expr} into usize"
                                )));
                            }
                        },
                    },
                };
                let id = match *id {
                    Expr::Ident(id) => id,
                    el => return Err(VmErr::Err(format!("Cannot use {el} as an identifier"))),
                };
                let this = match env.get_mut(scope) {
                    Some(scope) => &mut scope.0,
                    _ => return Err(VmErr::Err("Invalid scope".to_owned())),
                };
                match this.get_mut(&id) {
                    Some(meta) => match &mut meta.value {
                        Some(Values::Lit(Literal::Array(el))) => Ok(Some(&mut *el[idx])),
                        el => Err(VmErr::Err(format!("{el:?} cannot be indexed"))),
                    },
                    _ => Err(VmErr::Err(format!("Use of undecleared variable {id}"))),
                }
            }
            expr => Err(VmErr::Err(format!(
                "Cannot get {expr} as mutable identifiers"
            ))),
        }
    }
}
