

use super::{TypeEnv, TypeErr, ValueMeta};
use crate::{
    ast::{Expr, Statement, Type},
};

impl super::TypeCheck for Statement {
    type ReturnType = Type;
    fn check(&self, env: &mut TypeEnv, idx: usize) -> Result<Self::ReturnType, TypeErr> {
        if env.len() < idx {
            return Err("Trying to read from undecleared scope".to_owned());
        }
        let last_scope = match env.len() {
            0 => 0,
            l => l - 1,
        };
        let ret = match self.clone() {
            Statement::Let(id, mutable, t, e) => {
                // let a: i32 = 5 + 2
                // for now just accept an ident

                let assigned = e.is_some();

                let ty = match (t, e) {
                    (Some(t), Some(e)) => {
                        let expr_ty = e.check(env, last_scope)?;
                        if t != expr_ty {
                            Err(format!(
                                "Cannot assign expression of type {expr_ty} to value of type {t}"
                            ))
                        } else {
                            Ok(Some(t))
                        }
                    }
                    (None, Some(e)) => Ok(Some(e.check(env, last_scope)?)),
                    (Some(t), None) => Ok(Some(t)),
                    (None, None) => Ok(None),
                }?;
                let meta = ValueMeta {
                    assigned,
                    ty,
                    mutable,
                };
                env.get_mut(idx)
                    .unwrap()
                    .0
                    .insert(format!("{}", id), meta.clone());
                Ok(Some(Type::Unit))
            }

            Statement::Expr(e) => {
                // the type of an Expr is returned
                match e.check(env, last_scope) {
                    Ok(ty) => Ok(Some(ty)),
                    Err(e) => Err(e),
                }
            }
            Statement::Assign(id, e) => {
                // a = 5
                let ret = match id {
                    Expr::Ident(id) => {
                        let expected = env.get_mut(idx).unwrap().0.get_mut(&id);
                        match expected {
                            Some(t) => {
                                if !t.mutable && t.assigned {
                                    return Err("Cannot assing to a immutable value".to_owned());
                                } else {
                                    t.assigned = true;
                                    Ok((id, t.ty.clone()))
                                }
                            }
                            _ => Err(format!("Use of undecleared variable {}", id)),
                        }
                    }
                    Expr::IndexMut(id, idx) => {
                        let intermediate =
                            Some(Expr::IndexMut(id.clone(), idx).check(env, last_scope)?);
                        match *id {
                            Expr::Ident(id) => Ok((id, intermediate)),
                            ty => Err(format!("Cannot use {ty} as identifier")),
                        }
                    }

                    ty => return Err(format!("Cannot assign to non identifier type {ty}")),
                };
                match ret {
                    Ok((id, mut expected)) => {
                        let rhs = e.check(env, last_scope)?;

                        match expected {
                            Some(t) => match rhs == t {
                                true => Ok(Some(Type::Unit)),
                                _ => Err(format!(
                                    "Invalid return type for expression got {rhs} expected {t}"
                                )),
                            },
                            _ => {
                                expected = Some(rhs.clone());
                                // Re assign the new expected
                                // Unwrapping here is ok since the value must exist at this point
                                env.get_mut(idx).unwrap().0.get_mut(&id).unwrap().ty = expected;
                                Ok(Some(Type::Unit))
                            }
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Statement::While(e, b) => {
                let expr_type = e.check(env, last_scope)?;
                if expr_type != Type::Bool {
                    Err(format!(
                        "Itterator condition must be a bool. Recived {}",
                        expr_type
                    ))
                } else {
                    match b.check(env, idx) {
                        Ok(ty) => Ok(Some(ty)),
                        Err(e) => Err(e),
                    }
                }
            }
            Statement::Block(b) => match b.check(env, idx) {
                Ok(ty) => Ok(Some(ty)),
                Err(e) => Err(e),
            },
            Statement::FnDecleration(func) => func.check(env, idx),
        };
        match (ret, idx) {
            (Ok(Some(value)), _) => Ok(value),
            (Err(e), 0) => Err(e),
            (Err(_), idx) => self.check(env, idx - 1),
            (Ok(None), _) => Err("Type must be known at this point".to_owned()),
        }
    }
}
