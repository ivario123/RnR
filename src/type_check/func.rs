use std::collections::HashMap;

use super::{FunctionMeta, TypeCheck, TypeEnv, TypeErr, ValueMeta};
use crate::ast::func::{Func, FuncCall};
use crate::ast::{Expr, Type};

impl TypeCheck for FuncCall {
    type ReturnType = Type;
    fn check(&self, env: &mut TypeEnv, idx: usize) -> Result<Self::ReturnType, TypeErr> {
        let mut args: Vec<Type> = vec![];
        for arg in self.args.iter() {
            args.push(arg.check(env, idx)?)
        }

        let id = match (*self.id).clone() {
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
}

impl TypeCheck for Func {
    type ReturnType = Option<Type>;
    fn check(&self, env: &mut TypeEnv, idx: usize) -> Result<Self::ReturnType, TypeErr> {
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
        let mut new_env = TypeEnv::new();
        let mut idx = 0;
        while let Some(env) = env.get(idx) {
            let t_env = if idx == 0 {
                env.0.clone()
            } else {
                HashMap::new()
            };

            new_env.push((t_env, env.1.clone()));
            idx += 1;
        }
        new_env.push((HashMap::new(), HashMap::new()));
        let len = new_env.len();
        for (id, ty, mutable) in args {
            let id = match id {
                Expr::Ident(id) => id,
                _ => unreachable!(),
            };
            new_env.get_mut(len - 1).unwrap().0.insert(
                id.clone(),
                ValueMeta {
                    ty: Some(ty.clone()),
                    assigned: true,
                    mutable,
                },
            );
        }
        let ret_ty = self.body.check(&mut new_env, idx)?;
        // Allow mutable access to global scope
        env.get_mut(0).unwrap().0 = new_env.get(0).unwrap().0.clone();
        if ret_ty != self.ty {
            return Err(format!("Expected {} but got {ret_ty}", self.ty));
        }
        Ok(Some(ret_ty))
    }
}
