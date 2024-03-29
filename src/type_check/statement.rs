use super::{TypeEnv, TypeErr, ValueMeta};
use crate::ast::{Expr, Statement, Type, UnaryOp};

impl super::TypeCheck for Statement {
    fn check(&self, env: &mut TypeEnv, idx: usize) -> Result<Type, TypeErr> {
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
                    shadowable: true,
                    ref_counter: None,
                };
                let id = match id {
                    Expr::Ident(i) => i,
                    e => return Err(format!("Cannot use {e} as an identifier")),
                };
                for env in env.iter().rev() {
                    if let Some(val) = env.0.get(&id) {
                        if !val.shadowable {
                            return Err(format!("{self} cannot shadow static {id}"));
                        }
                    }
                }
                let env = match env.get_mut(idx) {
                    Some(env) => env,
                    None => return Err(format!("Invalid scope when typechecking {self}")),
                };
                env.0.insert(id, meta.clone());
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
                    Expr::UnOp(UnaryOp::Dereff, e) => {
                        //let ty = e.check(env, env.len() - 1)?;
                        fn get_base_expression(expr: Expr) -> Expr {
                            match expr {
                                Expr::UnOp(_op, e) => get_base_expression(*e.clone()),
                                e => e,
                            }
                        }
                        let ty = get_base_expression(*e.clone()).check(env, idx)?;
                        fn check_mut(ty: Type) -> Result<(), TypeErr> {
                            match ty.clone() {
                                Type::MutRef(r) => {
                                    if let Type::Ref(_) = *r.0.clone() {
                                        return Err(format!(
                                            "Cannot assign to immutable refference {:?}",
                                            r.0
                                        ));
                                    }
                                    check_mut(*r.0)
                                }
                                Type::Ref(_e) => Err(format!("Cannot treat {ty} as &mut")),
                                _ => Ok(()),
                            }
                        }
                        check_mut(ty)?;
                        fn refferand(e: Expr, env: &mut TypeEnv) -> Result<String, TypeErr> {
                            let id = e;
                            match id {
                                Expr::Ident(i) => Ok(i),
                                Expr::UnOp(UnaryOp::Dereff, e) => refferand(*e, env),
                                e => Err(format!("Cannot treat {e} as an expression")),
                            }
                        }
                        let id = match *e.clone() {
                            Expr::Ident(i) => i,
                            e => refferand(e, env)?,
                        };
                        let ty = e.check(env, last_scope)?;
                        match ty {
                            Type::MutRef(crate::ast::types::Ref(ty, _, _)) => Ok((id, Some(*ty))),
                            e => Err(format!("Cannot treat {e} as a mutable borrow")),
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
                                    "Invalid return type for expression {e} got {rhs} expected {t}\noccured in:\n\t{self}"
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
                    match b.check(env, last_scope) {
                        Ok(ty) => Ok(Some(ty)),
                        Err(e) => Err(e),
                    }
                }
            }
            Statement::Block(b) => match b.check(env, last_scope) {
                Ok(ty) => Ok(Some(ty)),
                Err(e) => Err(e),
            },
            Statement::FnDecleration(func) => Ok(Some(func.check(env, last_scope)?)),
        };
        match ret {
            Ok(Some(value)) => Ok(value),
            Err(e) => Err(e),
            Ok(None) => Err("Type must be known at this point".to_owned()),
        }
    }
}
