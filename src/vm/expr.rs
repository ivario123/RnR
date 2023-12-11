use std::convert::TryInto;

use super::{op::Operation, Eval, ValueMeta, VarEnv, VmErr};
use crate::ast::{Expr, Literal, UnaryOp};

impl super::Eval for Expr {
    //.eval_expr
    // recursively.evals an expression for type correctness
    // on success: the expression type is returned
    // on failure, an expression type error is returned
    fn eval(&self, env: &mut VarEnv, scope: usize) -> Result<Literal, VmErr> {
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
                    (_, scope) => return self.eval(env, scope - 1),
                }
            }

            Expr::Lit(l) => Ok(l),
            Expr::BinOp(op, l, r) => {
                let lhs = (*l).eval(env, last_scope)?;
                let rhs = (*r).eval(env, last_scope)?;
                Ok(op.eval(lhs, rhs)?)
            }
            Expr::Par(e) => (*e).eval(env, last_scope),
            Expr::IfThenElse(cond, t, e) => {
                let cond = (*cond).eval(env, last_scope)?;

                // Another actual VM part
                if let Literal::Bool(true) = cond {
                    t
                } else if let Literal::Bool(false) = cond {
                    match e {
                        Some(block) => block,
                        _ => return Ok(Literal::Unit),
                    }
                } else {
                    return Err(VmErr::Err(format!("Invalid expression {self}")));
                }
                .eval(env, last_scope)
            }
            Expr::UnOp(op, e) => {
                let got = (*e).eval(env, last_scope)?;
                op.eval(got)
            }
            Expr::Array(elements) => {
                let mut inner = vec![];
                for el in elements {
                    inner.push(Box::new(el.eval(env, last_scope)?));
                }
                return Ok(Literal::Array(inner));
            }
            Expr::Index(id, index) => {
                let id = match *id {
                    Expr::Ident(id) => id,
                    ty => return Err(VmErr::Err(format!("{ty} does not implement index"))),
                };
                let meta: ValueMeta = (id, &env, last_scope).try_into()?;

                let val = match meta.value.clone() {
                    Some(Literal::Array(values)) => Ok(values),
                    Some(ty) => return Err(VmErr::Err(format!("{ty} does not implement index"))),
                    None => return Err(VmErr::Err("Type must be known at this point".to_owned())),
                }?;

                let idx = match *index {
                    // If the idx is a constant we can.eval it
                    Expr::Lit(Literal::Int(idx)) => Ok(idx),
                    idx => match (idx.eval(env, scope - 1), scope - 1) {
                        (Ok(Literal::Int(val)), _) => Ok(val),
                        (_, 0) => Err(VmErr::Err(format!("Cannot convert {idx} into usize"))),
                        (_, index) => Ok(match idx.eval(env, index - 1) {
                            Ok(Literal::Int(val)) => Ok(val),
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
                Ok(*val[idx as usize].clone())
            }
            Expr::IndexMut(id, index) => {
                match Expr::IndexMut(id.clone(), index).as_mut(env, scope)? {
                    Some(lit) => Ok((*lit).clone()),
                    _ => Err(VmErr::Err(format!("Value {id} is unsagined"))),
                }
            }
            Expr::FuncCall(call) => {
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
                    (_, _) => return self.eval(env, scope - 1),
                }?;
                let mut args = vec![];
                let mut values = vec![];
                for (arg, value) in fndec.args.iter().zip(call.args.iter()) {
                    let intermediate = value.eval(env, last_scope)?;
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
                    false => fndec.body.eval(&mut new_env, last_scope)?,
                }; //fndec.rec_count -= 1;
                   // Allow mutable access to global scope
                env.get_mut(0).unwrap().0 = new_env.get(0).unwrap().0.clone();
                Ok(ret)
            }
            Expr::Block(b) => b.eval(env, env.len() - 1),
        };
        match (ret, scope) {
            (Ok(value), _) => Ok(value),
            (Err(e), 0) => Err(e),
            (Err(_), idx) => self.eval(env, idx - 1),
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
    pub fn assign(self, env: &mut VarEnv, scope: usize, value: Literal) -> Result<(), VmErr> {
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
                    (_, _) => self.assign(env, scope - 1, value),
                }
            }
            Expr::IndexMut(id, idx) => {
                let idx = match *idx {
                    Expr::Lit(Literal::Int(idx)) => idx as usize,
                    expr => match ((expr).eval(env, env.len() - 1), env.len() - 1) {
                        (Ok(Literal::Int(idx)), _) => idx as usize,
                        (_, 0) => {
                            return Err(VmErr::Err(format!("Cannot convert {expr} into usize")));
                        }
                        (_, idx) => match expr.eval(env, idx - 1) {
                            Ok(Literal::Int(idx)) => idx as usize,
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
                        Some(Literal::Array(el)) => {
                            *el[idx] = value;
                            Ok(())
                        }
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
    pub fn as_mut(self, env: &mut VarEnv, scope: usize) -> Result<Option<&mut Literal>, VmErr> {
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
                            Some(val) => Ok(Some(val)),
                            _ => Ok(None),
                        }
                    }
                    (_, 0) => Err(VmErr::Err(format!("No such variable {i}"))),
                    (_, _) => self.as_mut(env, scope - 1),
                }
            }
            Expr::IndexMut(id, idx) => {
                let idx = match *idx {
                    Expr::Lit(Literal::Int(idx)) => idx as usize,
                    expr => match ((expr).eval(env, env.len() - 1), env.len() - 1) {
                        (Ok(Literal::Int(idx)), _) => idx as usize,
                        (_, 0) => {
                            return Err(VmErr::Err(format!("Cannot convert {expr} into usize")));
                        }
                        (_, idx) => match expr.eval(env, idx - 1) {
                            Ok(Literal::Int(idx)) => idx as usize,
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
                        Some(Literal::Array(el)) => Ok(Some(&mut *el[idx])),
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
