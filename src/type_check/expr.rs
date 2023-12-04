use super::{Scope, TypeEnv, TypeErr};
use crate::ast::{Expr, Literal, Operation, Type};

impl super::TypeCheck for Expr {
    type ReturnType = Type;
    // check_expr
    // recursively checks an expression for type correctness
    // on success: the expression type is returned
    // on failure, an expression type error is returned
    fn check(&self, env: &mut TypeEnv, idx: usize) -> Result<Self::ReturnType, TypeErr> {
        if env.len() < idx {
            return Err("No scope decleared".to_owned());
        }
        let ret = match self.clone() {
            Expr::Ident(id) => match (env.get(idx).unwrap().0.get(&id), idx) {
                (Some(t), _) => match &t.ty {
                    Some(t) => Ok(t.clone()),
                    _ => Err(format!("Type of variable {id} must be known at this point")),
                },
                // Look for identifier in earlier scopes
                (_, 0) => Err("variable not found".to_string()),
                (_, _) => return self.check(env, idx - 1),
            },
            Expr::Lit(Literal::Int(_)) => Ok(Type::I32),
            Expr::Lit(Literal::Bool(_)) => Ok(Type::Bool),
            Expr::Lit(Literal::Unit) => Ok(Type::Unit),
            Expr::Lit(Literal::Array(arr)) => {
                let types: Vec<Result<Self::ReturnType, TypeErr>> = arr
                    .iter()
                    .map(|el| Expr::Lit((**el).clone()).check(env, env.len() - 1))
                    .collect();
                let first_type: Type = match arr.first() {
                    Some(el) => Expr::Lit((**el).clone()).check(env, env.len() - 1)?,
                    None => Type::Unit,
                };
                for ty in types {
                    match ty {
                        Ok(ty) => {
                            if ty != first_type {
                                return Err(format!(
                                    "Array has inconcistent types, expected {first_type} got {ty}"
                                ));
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }

                Ok(first_type)
            }

            Expr::BinOp(op, l, r) => {
                let lhs = (*l).check(env, env.len() - 1)?;
                let rhs = (*r).check(env, env.len() - 1)?;
                let ret_type = op.type_check((lhs.clone(), rhs.clone()));
                if !ret_type {
                    return Err(format!(
                        "Oprands is invalid for {}, would result in {} {} {}",
                        op, lhs, op, rhs
                    ));
                }
                Ok(op.return_type())
            }

            Expr::Par(e) => (*e).check(env, env.len() - 1),

            Expr::IfThenElse(cond, t, e) => {
                let cond = (*cond).check(env, env.len() - 1)?;
                if cond != Type::Bool {
                    Err(format!(
                        "Condition expression must be boolean type, got {}",
                        cond
                    ))
                } else {
                    let t = t.check(env, idx)?;
                    match e {
                        Some(b) => {
                            let b = b.check(env, idx)?;
                            if t != b {
                                Err(format!("Else block return type did not match then block, expected : {} got : {}",t,b))
                            } else {
                                Ok(t)
                            }
                        }
                        _ => Ok(t),
                    }
                }
            }
            Expr::UnOp(op, e) => {
                let expected = op.return_type();
                let got = (*e).check(env, env.len() - 1)?;
                match op.type_check(got.clone()) {
                    true => Ok(expected),
                    false => Err(format!("Cannot perform {op} on {got}")),
                }
            }
            Expr::Array(elements) => {
                let mut elements = elements;
                let len = elements.len();
                if len == 0 {
                    return Ok(Type::Array(Box::new(Type::Unit), 0));
                };
                let ty = (*elements.pop().unwrap()).check(env, env.len() - 1)?;
                let mut ret = Ok(Type::Array(Box::new(ty.clone()), len));
                while let Some(expr) = elements.pop() {
                    let found_ty = (*expr).check(env, env.len() - 1)?;
                    if ty != found_ty {
                        ret = Err(format!("Expected {:?} but found {:?}", ty, found_ty));
                    }
                }
                ret
            }
            Expr::Index(id, index) => {
                let id = match *id {
                    Expr::Ident(id) => id,
                    ty => return Err(format!("{ty} does not implement index")),
                };
                let env: &Scope = &env.get(idx).unwrap().0;
                match env.get(&id) {
                    Some(meta) => {
                        if !meta.assigned {
                            return Err(
                                "Cannot index type that has not been initialized".to_string()
                            );
                        };
                        match meta.ty.clone() {
                            Some(Type::Array(ty, size)) => {
                                // If the idx is a constant we can check it
                                if let Expr::Lit(Literal::Int(idx)) = *index {
                                    if idx as usize >= size {
                                        return Err(format!("Cannot access element at index {idx} since array is of size {size}"));
                                    }
                                }
                                Ok(*ty)
                            }
                            Some(ty) => return Err(format!("{ty} does not implement index")),
                            None => return Err("Type must be known at this point".to_string()),
                        }
                    }
                    _ => Err(format!("Usage of undecleared variable {id}")),
                }
            }
            Expr::IndexMut(id, index) => {
                let id = match *id {
                    Expr::Ident(id) => id,
                    ty => return Err(format!("{ty} does not implement index")),
                };
                match env.get(idx).unwrap().0.get(&id) {
                    Some(meta) => {
                        if !meta.mutable {
                            return Err(
                                "Cannot get a mutable element from immutable value".to_string()
                            );
                        }
                        if !meta.assigned {
                            return Err(
                                "Cannot index type that has not been initialized".to_string()
                            );
                        };
                        match meta.ty.clone() {
                            Some(Type::Array(ty, size)) => {
                                // If the idx is a constant we can check it
                                if let Expr::Lit(Literal::Int(idx)) = *index {
                                    if idx as usize >= size {
                                        return Err(format!("Cannot access element at index {idx} since array is of size {size}"));
                                    }
                                }
                                Ok(*ty)
                            }
                            Some(ty) => return Err(format!("{ty} does not implement index")),
                            None => return Err("Type must be known at this point".to_string()),
                        }
                    }
                    _ => Err(format!("Usage of undecleared variable {id}")),
                }
            }
            Expr::FuncCall(fncall) => {
                let mut args: Vec<Type> = vec![];
                for arg in fncall.args.iter() {
                    args.push(arg.check(env, idx)?)
                }

                let id = match *fncall.id {
                    Expr::Ident(id) => id,
                    e => return Err(format!("Cannot treat {e} as a function identifier.")),
                };

                let this_fnenv: &super::FunctionScope = match env.get(idx) {
                    Some(envs) => &envs.1,
                    None => return Err("Invalid scoping".to_owned()),
                };
                let fndec = match (this_fnenv.get(&id), idx) {
                    (Some(fndec), _) => fndec,
                    (_, 0) => return Err(format!("Tried to call undefined function {id}")),
                    (_, idx) => return self.check(env, idx),
                };
                if fndec.args.len() != args.len() {
                    return Err(format!(
                        "Expected {} arguments but got {}",
                        fndec.args.len(),
                        args.len()
                    ));
                }
                let mut args: Vec<(usize, (&(Type, bool), &Type))> =
                    fndec.args.iter().zip(args.iter()).enumerate().collect();
                while let Some((idx, ((expected_ty, _expected_mutable), got))) = args.pop() {
                    // Check them in order
                    if *got != *expected_ty {
                        return Err(format!(
                            "Expected argument nr {idx} to be of type {expected_ty} but got {got}"
                        ));
                    }
                }
                Ok(fndec.ty.clone())
            }
        };
        match (ret, idx) {
            (Ok(value), _) => Ok(value),
            (Err(e), 0) => Err(e),
            (Err(_), idx) => self.check(env, idx - 1),
        }
    }
}
