#![deny(clippy::all)]
#![deny(warnings)]

// Once you have back-ported your code
// you should remove the above attributes.
// Strive to keep your code free of warnings.

use prelude::TypeCheck;
use syn::parse::Parse;
use vm::Eval;

// common definitions
//pub mod common;
pub mod error;

// AST related
pub mod ast;
pub mod climb;
pub mod parse;

// type generic environment
//pub mod env;
// intrinsic functions
pub mod intrinsics;

// semantic analysis
pub mod type_check;
// natural interpretation
pub mod codegen;
pub mod vm;
// borrow checking
// pub mod bc;
pub mod prelude {
    pub use super::ast::Prog;
    pub use super::type_check::{TypeCheck, TypeEnv};
    pub use super::vm::{Eval, VarEnv};
    pub use super::Ast;
}

/// Ast wrapper for improved error messages
pub struct Ast<T: Parse + TypeCheck + Eval> {
    t: T,
}

impl<T: Parse + TypeCheck + Eval> From<String> for Ast<T> {
    fn from(value: String) -> Self {
        let map = |el: Vec<String>, r: std::ops::Range<usize>| {
            let r_clone = r.clone();
            let intermediate = el
                .iter()
                .enumerate()
                .map(|(idx, el)| match r.contains(&idx) {
                    true => Some(el),
                    false => None,
                });
            let mut ret = vec![];
            for (el, idx) in intermediate.zip(r_clone.into_iter()) {
                if el.is_some() {
                    ret.push(format!("{idx}|\t{}", el.unwrap().clone()));
                }
            }
            ret.join("\n")
        };

        let ts: proc_macro2::TokenStream = match value.parse() {
            Ok(ts) => ts,
            Err(e) => {
                let line = e.span().start().line;
                let lines = map(
                    value
                        .lines()
                        .into_iter()
                        .map(|el| el.to_string())
                        .collect::<Vec<String>>(),
                    line - 4..line + 5,
                );

                eprintln!("Error {e} occured on line {} \n{lines}", line);

                panic!("Invalid input");
            }
        };
        let t = match syn::parse2(ts) {
            Ok(ts) => ts,
            Err(e) => {
                let line = e.span().start().line;
                let lines = map(
                    value
                        .lines()
                        .into_iter()
                        .map(|el| el.to_string())
                        .collect::<Vec<String>>(),
                    line - 4..line + 5,
                );

                eprintln!("Error {e} occured on line {} \n{lines}", line);

                panic!("Invalid input");
            }
        };

        Self { t }
    }
}
impl<T: Parse + TypeCheck + Eval> Eval for Ast<T> {
    fn eval(
        &self,
        env: &mut prelude::VarEnv,
        scope: usize,
        max_iter: usize,
        iter_counter: &mut usize,
    ) -> Result<vm::Values, vm::VmErr> {
        self.t.eval(env, scope, max_iter, iter_counter)
    }
}

impl<T: Parse + TypeCheck + Eval> TypeCheck for Ast<T> {
    type ReturnType = T::ReturnType;
    fn check(
        &self,
        env: &mut prelude::TypeEnv,
        idx: usize,
    ) -> Result<Self::ReturnType, type_check::TypeErr> {
        self.t.check(env, idx)
    }
}
impl<T: Parse + TypeCheck + Eval + std::fmt::Display> std::fmt::Display for Ast<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.t)
    }
}
// ========================================
//              Helper macros
// ========================================
#[macro_export]
macro_rules! discard {
    ($($ty:literal)+) => {
        $(
            let _: Token![$ty] = input.parse()?;
        )+
    };
}
#[macro_export]
macro_rules! check {
    ($id:ident) => {
        $id.check(&mut TypeEnv::new(), 0)
    };
}
#[macro_export]
macro_rules! eval {
    ($id:ident,$iter:ident) => {
        $id.eval(&mut VarEnv::new(), 0, $iter, &mut 0)
    };
}
#[macro_export]
macro_rules! parse {
    ($text:ident,$t:ty) => {{
        let ret: Ast<$t> = $text.into();
        ret
    }};
}

// optional backend goes here..
