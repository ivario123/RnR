#![allow(unused_variables)]
#![allow(dead_code)]
// Once you have back-ported your code
// you should remove the above attributes.
// Strive to keep your code free of warnings.

// common definitions
pub mod common;
pub mod error;

// AST related
pub mod ast;
pub mod ast_traits;
pub mod climb;
pub mod parse;

// type generic environment
pub mod env;
// intrinsic functions
pub mod intrinsics;

// semantic analysis
pub mod type_check;
// natural interpretation
pub mod vm;
// borrow checking
pub mod bc;

// optional backend goes here..
