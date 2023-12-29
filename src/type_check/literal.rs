use super::{TypeCheck, TypeErr};
use crate::ast::{Expr, Literal, Type};

impl TypeCheck for Literal {
    fn check(&self, env: &mut super::TypeEnv, _idx: usize) -> Result<Type, TypeErr> {
        match self {
            Literal::Unit => Ok(Type::Unit),
            // Default type for Literal ints is i32 this can be coerced in expressions.
            Literal::Int(_) => Ok(Type::I32),
            Literal::Bool(_) => Ok(Type::Bool),
            Literal::String(_) => Ok(Type::String),
            Literal::Array(arr) => {
                let types: Vec<Result<Type, TypeErr>> = arr
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
        }
    }
}
