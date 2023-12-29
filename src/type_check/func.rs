use std::collections::HashMap;

use super::{FunctionMeta, Scope, TypeCheck, TypeEnv, TypeErr, ValueMeta};
use crate::ast::func::{Arg, Func, FuncCall};
use crate::ast::{Expr, Type};

impl From<Arg> for ValueMeta {
    fn from(value: Arg) -> Self {
        Self {
            ty: Some(value.ty),
            assigned: true,
            mutable: value.mutable,
            shadowable: true,
            ref_counter: None,
        }
    }
}

fn reconstruct_evn(env: &TypeEnv, args: Vec<Arg>) -> TypeEnv {
    let global = env.get(0).unwrap().0.clone();
    let mut local_scope: HashMap<String, ValueMeta> = HashMap::new();
    for arg in args {
        if let Expr::Ident(i) = arg.id.clone() {
            local_scope.insert(i, arg.into());
        }
    }
    let blank_scope: Scope = Scope::new();
    let mut new_env: TypeEnv = env
        .iter()
        .map(|(_var, func)| (blank_scope.clone(), func.clone()))
        .collect();
    new_env.push((blank_scope.clone(), HashMap::new()));
    new_env.get_mut(0).unwrap().0 = global;
    let len = new_env.len();
    new_env.get_mut(len - 1).unwrap().0 = local_scope;
    new_env
}
impl TypeCheck for FuncCall {
    fn check(&self, env: &mut TypeEnv, idx: usize) -> Result<Type, TypeErr> {
        let mut args: Vec<Type> = vec![];
        for arg in self.args.iter() {
            args.push(arg.check(env, idx)?)
        }

        let id = match (*self.id).clone() {
            Expr::Ident(id) => id,
            e => return Err(format!("Cannot treat {e} as a function identifier.")),
        };

        let mut index = env.len() - 1;
        let mut fndec = None;
        while let Some(scope) = env.get(index) {
            if let Some(dec) = scope.1.get(&id) {
                fndec = Some(dec.clone());
                break;
            };
            index -= 1;
        }

        let fndec = match fndec {
            Some(fndec) => fndec,
            _ => return Err(format!("Tried to call undefined function {id}")),
        };
        if id == *"println!" {
            return Ok(Type::Unit);
        }
        if fndec.args.len() != args.len() {
            return Err(format!(
                "Expected {} arguments but got {}",
                fndec.args.len(),
                args.len()
            ));
        }
        type ArgTuple<'a> = &'a (Type, bool);
        let mut args: Vec<(usize, (ArgTuple, &Type))> =
            fndec.args.iter().zip(args.iter()).enumerate().collect();

        while let Some((idx, ((expected_ty, _expected_mutable), got))) = args.pop() {
            // Check them in order
            if *got != *expected_ty {
                return Err(format!(
                    "Expected argument nr {idx} to be of type {expected_ty} but got {got}"
                ));
            }
        }
        Ok(fndec.ty)
    }
}

impl TypeCheck for Func {
    fn check(&self, env: &mut TypeEnv, idx: usize) -> Result<Type, TypeErr> {
        // We have a function decleration, this should be inserted into the fn env and then
        // the 0th env and a new function env should be used to check wether or not the
        // internal code is valid
        let id = match &self.id {
            Expr::Ident(id) => Ok(id),
            exp => Err(format!("Cannot treat {exp} as a function identifier")),
        }?;
        let args: Vec<(Expr, Type, bool)> = self
            .args
            .iter()
            .map(|arg| (arg.id.clone(), arg.ty.clone(), arg.mutable))
            .collect();

        if env.get(idx).unwrap().1.get(id).is_some() {
            return Err(format!("Duplicate definition of function {id}"));
        }
        // Add in the new function and assume correctly typed for now
        env.get_mut(idx).unwrap().1.insert(
            id.clone(),
            FunctionMeta {
                ty: self.ty.clone(),
                args: args
                    .iter()
                    .map(|(_id, ty, mutable)| (ty.clone(), *mutable))
                    .collect(),
            },
        );

        // Give function scope access to global scope and all of the accessible functions
        let mut new_env = reconstruct_evn(env, self.args.clone());
        let ret_ty = self.body.check(&mut new_env, idx)?;
        // Allow mutable access to global scope
        env.get_mut(0).unwrap().0 = new_env.get(0).unwrap().0.clone();
        if ret_ty != self.ty {
            return Err(format!("Expected {} but got {ret_ty}", self.ty));
        }
        Ok(Type::Unit)
    }
}
