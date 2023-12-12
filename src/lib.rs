#![deny(clippy::all)]
#![deny(warnings)]

// Once you have back-ported your code
// you should remove the above attributes.
// Strive to keep your code free of warnings.

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
    ($text:ident) => {
        syn::parse2($text)
    };
}

// optional backend goes here..
