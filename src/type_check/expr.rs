use super::{Operation, TypeEnv, TypeErr};
use crate::ast::{Expr, Literal, Type};

impl super::TypeCheck for Expr {
    type ReturnType = Type;
    // check_expr
    // recursively checks an expression for type correctness
    // on success: the expression type is returned
    // on failure, an expression type error is returned
    fn check(&self, env: &mut TypeEnv, idx: usize) -> Result<Self::ReturnType, TypeErr> {
        if env.len() < idx || env.len() == 0 {
            return Err("No scope decleared".to_owned());
        }
        let ret = match self.clone() {
            Expr::Ident(id) => {
                let res = env.get(idx);
                let res = res.unwrap().0.get(&id);

                match (res, idx) {
                    (Some(t), _) => match &t.ty {
                        Some(t) => Ok(t.clone()),
                        _ => Err(format!("Type of variable {id} must be known at this point")),
                    },
                    // Look for identifier in earlier scopes
                    (_, 0) => Err("variable not found".to_string()),
                    (_, _) => return self.check(env, idx - 1),
                }
            }
            Expr::Lit(l) => l.check(env, env.len() - 1),
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
                Ok(op.return_type((lhs, rhs))?)
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
                let got = (*e).check(env, env.len() - 1)?;
                let expected = op.return_type(got.clone())?;

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
            Expr::Index(id, arr_index) => index(*id, *arr_index, false, env, idx),
            Expr::IndexMut(id, arr_index) => index(*id, *arr_index, true, env, idx),
            Expr::FuncCall(fncall) => fncall.check(env, idx),
            Expr::Block(b) => b.check(env, env.len() - 1),
        };
        match (ret, idx) {
            (Ok(value), _) => Ok(value),
            (Err(e), 0) => Err(e),
            (Err(_), idx) => self.check(env, idx - 1),
        }
    }
}
fn index(
    id: Expr,
    index: Expr,
    mutable: bool,
    env: &mut TypeEnv,
    idx: usize,
) -> Result<Type, TypeErr> {
    let id = match id {
        Expr::Ident(id) => id,
        ty => return Err(format!("{ty} does not implement index")),
    };
    match env.get(idx).unwrap().0.get(&id) {
        Some(meta) => {
            if !meta.mutable && mutable {
                return Err("Cannot get a mutable element from immutable value".to_string());
            }
            if !meta.assigned {
                return Err("Cannot index type that has not been initialized".to_string());
            };
            match meta.ty.clone() {
                Some(Type::Array(ty, size)) => {
                    // If the idx is a constant we can check it
                    if let Expr::Lit(Literal::Int(idx)) = index {
                        if idx as usize >= size {
                            return Err(format!("Cannot access element at index {idx} since array is of size {size}"));
                        }
                    }
                    Ok(*ty)
                }
                Some(ty) => Err(format!("{ty} does not implement index")),
                None => Err("Type must be known at this point".to_string()),
            }
        }
        _ => Err(format!("Usage of undecleared variable {id}")),
    }
}
