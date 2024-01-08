#![deny(clippy::all)]
#![deny(warnings)]
#![deny(clippy::panic)]

use ast::{HirNode, Type};
use prelude::TypeCheck;
use syn::parse::Parse;
use vm::Eval;

#[cfg(not(test))]
use ast::color_normal::*;
#[cfg(test)]
use ast::color_test::*;

// common definitions
//pub mod common;
pub mod error;

// AST related
pub mod ast;
pub mod borrow_checker;
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
    pub use super::borrow_checker::{BCError, Env, Linearize, PreDeclareTop};
    pub use super::type_check::{TypeCheck, TypeEnv};
    pub use super::vm::{Eval, VarEnv};
    pub use super::Ast;
    pub use super::{borrow_check, check, eval, parse};
}

pub trait AstNode: Eval + TypeCheck + std::fmt::Debug {}

/// Ast wrapper for improved error messages
#[derive(Clone, Debug)]
pub struct Ast<T: AstNode> {
    t: T,
}

/// High level representation, returned after typechecking.
pub struct HIR<T: ast::HIR> {
    root: T,
}

impl<T: ast::HIR> ast::HIR for HIR<T> {
    fn get_type(&self) -> Type {
        self.root.get_type()
    }
}

impl<T: AstNode + Parse> From<String> for Ast<T> {
    fn from(value: String) -> Self {
        let map = |el: Vec<String>,
                   r: std::ops::Range<usize>,
                   line: usize,
                   end_line: usize,
                   cols: (usize, usize)| {
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
                    match (idx > line, idx < end_line, idx == line, idx == end_line) {
                        (true, true, _, _) => {
                            ret.push(error(format!("{idx}|\t{}", el.unwrap().clone()), true))
                            // Add some cool new processing to highlight errors on multiple lines
                        }
                        (_, _, true, _) | (_, _, _, true) => {
                            ret.push(format!(
                                "{idx}|\t{}<-- Occured here",
                                error(el.unwrap().clone(), true)
                            ));
                            // Assume errors are 1 line in this case.
                            ret.push(format!(
                                "\t{}{}",
                                " ".repeat(cols.0),
                                error(format!("|{}|", "-".repeat(cols.1 - cols.0 - 2)), false)
                            ))
                        }
                        //(_, _, true, err) => {
                        //    let str = format!("{idx}|\t{}", el.unwrap().clone());
                        //    ret.push(error())
                        //}
                        _ => ret.push(format!("{idx}|\t{}", el.unwrap().clone())),
                    }
                }
            }
            ret.join("\n")
        };

        let ts: proc_macro2::TokenStream = match value.parse() {
            Ok(ts) => ts,
            Err(e) => {
                let line = e.span().start().line;
                let cols = (e.span().start().column, e.span().end().column);
                let line_offset = match line.checked_sub(4) {
                    Some(e) => e,
                    None => 0,
                };
                let rel_line = match line.checked_sub(line_offset) {
                    Some(e) => e,
                    None => 0,
                };
                let end_line = e.span().end().line;
                let line_offset_end = match end_line.checked_sub(4) {
                    Some(e) => e,
                    None => 0,
                };
                let rel_line_end = match end_line.checked_sub(line_offset_end) {
                    Some(e) => e,
                    None => 0,
                };

                let lines = map(
                    value
                        .lines()
                        .map(|el| el.to_string())
                        .collect::<Vec<String>>(),
                    line_offset..line + 5,
                    rel_line,
                    rel_line_end,
                    cols,
                );

                eprintln!("Error {e} ocurred on line {} \n{lines}", line + 1);

                panic!("Invalid input");
            }
        };
        let t = match syn::parse2(ts) {
            Ok(ts) => ts,
            Err(e) => {
                let line = e.span().start().line;
                let cols = (e.span().start().column, e.span().end().column);
                let line_offset = match line.checked_sub(4) {
                    Some(e) => e,
                    None => 0,
                };
                let rel_line = match line.checked_sub(line_offset) {
                    Some(e) => e,
                    None => 0,
                };
                let end_line = e.span().end().line;
                let line_offset_end = match end_line.checked_sub(4) {
                    Some(e) => e,
                    None => 0,
                };
                let rel_line_end = match end_line.checked_sub(line_offset_end) {
                    Some(e) => e,
                    None => 0,
                };
                let lines = map(
                    value
                        .lines()
                        .map(|el| el.to_string())
                        .collect::<Vec<String>>(),
                    line_offset..line + 5,
                    rel_line,
                    rel_line_end,
                    cols,
                );

                eprintln!("Error {e} ocurred on line {} \n{lines}", line + 1);

                panic!("Invalid input");
            }
        };

        Self { t }
    }
}

impl<T: AstNode> Eval for Ast<T> {
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

impl<T: AstNode> TypeCheck for Ast<T> {
    fn check(&self, env: &mut prelude::TypeEnv, idx: usize) -> Result<Type, type_check::TypeErr> {
        self.t.check(env, idx)
    }
}

impl<T: AstNode + std::fmt::Display> std::fmt::Display for Ast<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.t)
    }
}

impl<T> Ast<T>
where
    T: AstNode + std::fmt::Display,
{
    #[allow(dead_code)]
    /// Performs type checking on the AST
    /// Reduces it down in to a High level intermediate representation
    fn into_hir(self) -> HIR<HirNode<T>> {
        todo!()
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
macro_rules! borrow_check {
    ($id:ident) => {{
        match $id.pre_declare_top(&mut 0, &mut 0) {
            Ok(_) => {
                let mut env = Env::new();
                $id.linearize(&mut env)
            }
            Err(e) => Err(BCError::EnvError(e)),
        }
    }};
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
        use crate::Ast;
        let ret: Ast<$t> = $text.into();
        ret
    }};
}

// optional backend goes here..
