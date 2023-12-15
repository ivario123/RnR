pub mod block;
pub mod expr;
pub mod func;
pub mod globals;
pub mod literal;
pub mod op;
pub mod program;
pub mod statement;

pub use block::*;
pub use expr::*;
pub use func::*;
pub use globals::*;
pub use literal::*;
pub use op::*;
pub use program::*;
pub use statement::*;

use crate::ast::{Expr, Func, Type};

use std::collections::HashMap;

// So let's implement a type checker
// Here we go!!!!
#[derive(Debug, Clone, Copy)]
pub enum Ref {
    Mutable,
    Immutable(usize),
}
/// Describes all of the needed data for a value.
#[derive(Debug, Clone)]
pub struct ValueMeta {
    ty: Option<Type>,
    assigned: bool,
    mutable: bool,
    shadowable: bool,
    ref_counter: Option<Ref>,
}
#[derive(Debug, Clone)]
pub struct FunctionMeta {
    ty: Type,
    /// The variable scope, this should include
    /// all arguments and their types
    args: Vec<(Type, bool)>,
}

/// Represents the functions accessible in the current scope
pub type FunctionScope = HashMap<String, FunctionMeta>;

/// Represents a specific scope.
/// For example a block has it's own scope.
pub type Scope = HashMap<String, ValueMeta>;
/// Represents all program [`Scope`]s
pub type TypeEnv = Vec<(Scope, FunctionScope)>;
pub type TypeErr = String;

/// Denotes that a type is simply TypeCheckable.
///
/// This means that given the current vec of all
/// [`Scope`]s and the index of the current scope.
pub trait TypeCheck {
    type ReturnType;
    fn check(&self, env: &mut TypeEnv, idx: usize) -> Result<Self::ReturnType, TypeErr>;
}

impl From<Func> for FunctionMeta {
    fn from(value: Func) -> Self {
        let ty = value.ty;
        let args = value
            .args
            .iter()
            .map(|arg| (arg.ty.clone(), arg.mutable))
            .collect();
        Self { ty, args }
    }
}

fn get_meta<'a>(env: &'a mut TypeEnv, expr: &Expr) -> Result<Option<&'a mut ValueMeta>, TypeErr> {
    let id = match expr {
        Expr::Ident(i) => Ok(i),
        e => Err(format!("Cannot treat {e} as an identifer")),
    }?;
    let mut scope = env.len() - 1;
    while env.get(scope).unwrap().0.get(id).is_none() {
        if scope == 0 {
            return Ok(None);
        }
        scope -= 1;
    }
    Ok(env.get_mut(scope).unwrap().0.get_mut(id))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{Block, Expr, Statement};

    #[test]
    fn test_expr_stmt_while() {
        let ts: proc_macro2::TokenStream = "
        while true {
            let mut a: i32 = 5;
            a = 5;
            a;
        }"
        .parse()
        .unwrap();
        let e: Statement = syn::parse2(ts).unwrap();
        println!("{}", e);
        let mut env = TypeEnv::new();
        let scope = super::Scope::new();
        let fn_scope = super::FunctionScope::new();
        env.push((scope, fn_scope));
        let len = env.len();
        let ty = e.check(&mut env, len).unwrap();
        assert_eq!(ty, Type::Unit);
    }

    #[test]
    fn test_expr_stmt_let() {
        let ts: proc_macro2::TokenStream = "let a: i32 = 5 + a".parse().unwrap();
        let e: Statement = syn::parse2(ts).unwrap();
        println!("{}", e);
        let mut env = TypeEnv::new();
        let mut scope = Scope::new();
        scope.insert(
            "a".to_string(),
            ValueMeta {
                ty: Some(Type::I32),
                assigned: false,
                mutable: false,
                shadowable: true,
                ref_counter: None,
            },
        );
        env.push((scope, HashMap::new()));
        let ty = e.check(&mut env, 0).unwrap();
        assert_eq!(ty, Type::Unit);
    }

    #[test]
    fn test_expr_stmt() {
        let ts: proc_macro2::TokenStream = "a + 1 + (5 - 5) * 8".parse().unwrap();
        let e: Statement = syn::parse2(ts).unwrap();
        println!("{}", e);
        let mut env = TypeEnv::new();
        let mut scope = Scope::new();
        scope.insert(
            "a".to_string(),
            ValueMeta {
                ty: Some(Type::I32),
                assigned: false,
                mutable: true,
                shadowable: true,
                ref_counter: None,
            },
        );
        env.push((scope, HashMap::new()));
        let ty = e.check(&mut env, 0).unwrap();
        assert_eq!(ty, Type::I32);
    }

    #[test]
    fn test_expr_assign() {
        let ts: proc_macro2::TokenStream = "a = 1 + a".parse().unwrap();
        let e: Statement = syn::parse2(ts).unwrap();
        println!("{}", e);
        let mut env = TypeEnv::new();
        let mut scope = Scope::new();
        scope.insert(
            "a".to_string(),
            ValueMeta {
                ty: Some(Type::I32),
                assigned: false,
                mutable: true,
                shadowable: true,
                ref_counter: None,
            },
        );
        env.push((scope, HashMap::new()));
        let ty = e.check(&mut env, 0).unwrap();
        assert_eq!(ty, Type::Unit);
    }

    #[test]
    fn test_expr_assign_fail() {
        let ts: proc_macro2::TokenStream = "a = 1 + false".parse().unwrap();
        let e: Statement = syn::parse2(ts).unwrap();
        println!("{}", e);
        let mut env = TypeEnv::new();
        let mut scope = Scope::new();
        scope.insert(
            "a".to_string(),
            ValueMeta {
                ty: Some(Type::I32),
                assigned: false,
                mutable: true,
                shadowable: true,
                ref_counter: None,
            },
        );
        env.push((scope, HashMap::new()));
        let ty = e.check(&mut env, 0);
        assert!(ty.is_err());
    }
    #[test]
    fn test_block() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 0;
        let a: bool = false;
        a
    }"
        .parse()
        .unwrap();
        let e: Block = syn::parse2(ts).unwrap();
        println!("{}", e);
        let mut env = TypeEnv::new();
        let mut scope = Scope::new();
        scope.insert(
            "a".to_string(),
            ValueMeta {
                ty: Some(Type::I32),
                assigned: false,
                mutable: false,
                shadowable: true,
                ref_counter: None,
            },
        );
        env.push((scope, HashMap::new()));
        let ty = e.check(&mut env, 0).unwrap();
        assert_eq!(ty, Type::Bool);
    }

    #[test]
    fn test_check_if_then_else_shadowing() {
        let ts: proc_macro2::TokenStream = "
        {
            let mut a: i32 = 1 + 2; // a == 3
            let mut a: i32 = 2 + a; // a == 5
            if true { 
                a = a - 1;      // outer a == 4 
                let mut a: i32 = 0; // inner a == 0 
                a = a + 1       // inner a == 1
            } else { 
                a = a - 1 
            };
            a   // a == 4
        }
        "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {}", bl);
        let mut env = TypeEnv::new();
        let mut scope = Scope::new();
        scope.insert(
            "a".to_string(),
            ValueMeta {
                ty: Some(Type::I32),
                assigned: false,
                mutable: false,
                shadowable: true,
                ref_counter: None,
            },
        );
        env.push((scope, HashMap::new()));
        let ty = bl.check(&mut env, 0).unwrap();

        assert_eq!(ty, Type::I32);
    }
    #[test]
    fn test_id() {
        let ts: proc_macro2::TokenStream = "a".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        let mut env = TypeEnv::new();
        let mut scope = Scope::new();
        scope.insert(
            "a".to_string(),
            ValueMeta {
                ty: Some(Type::I32),
                assigned: false,
                mutable: false,
                shadowable: true,
                ref_counter: None,
            },
        );
        env.push((scope, HashMap::new()));
        let ty = e.check(&mut env, 0).unwrap();
        assert_eq!(ty, Type::I32);
    }

    #[test]
    fn test_lit_i32() {
        let ts: proc_macro2::TokenStream = "123".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        let mut env = TypeEnv::new();
        let scope = super::Scope::new();
        let fn_scope = super::FunctionScope::new();
        env.push((scope, fn_scope));
        let ty = e.check(&mut env, 0).unwrap();
        assert_eq!(ty, Type::I32);
    }

    #[test]
    fn test_expr() {
        let ts: proc_macro2::TokenStream = "a + 1 + (5 - 5) * 8".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("{}", e);
        let mut env = TypeEnv::new();
        let mut scope = Scope::new();
        scope.insert(
            "a".to_string(),
            ValueMeta {
                ty: Some(Type::I32),
                assigned: false,
                mutable: false,
                shadowable: true,
                ref_counter: None,
            },
        );
        env.push((scope, HashMap::new()));
        let len = env.len();
        let ty = e.check(&mut env, len).unwrap();
        assert_eq!(ty, Type::I32);
    }

    #[test]
    fn test_expr_if_then_else() {
        let ts: proc_macro2::TokenStream = "
        if true { false } else { b }
        "
        .parse()
        .unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("{}", e);
        let mut env = TypeEnv::new();
        let mut scope = Scope::new();
        scope.insert(
            "b".to_string(),
            ValueMeta {
                ty: Some(Type::Bool),
                assigned: false,
                mutable: false,
                shadowable: true,
                ref_counter: None,
            },
        );
        env.push((scope, HashMap::new()));
        let ty = e.check(&mut env, 0).unwrap();
        assert_eq!(ty, Type::Bool);
    }
}
