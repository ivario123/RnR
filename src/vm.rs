pub mod block;
pub mod expr;
pub mod func;
pub mod op;
pub mod program;
pub mod statement;

use std::collections::HashMap;

use crate::ast::{
    op::BinaryOp,
    Block,
    Expr::{self},
    Literal,
};

#[derive(Debug)]
pub enum VmErr {
    Err(String),
    InvalidIdentifier(Expr),
}

/// Describes all of the needed data for a value.
#[derive(Debug, Clone)]
pub struct ValueMeta {
    value: Option<Literal>,
}
#[derive(Debug, Clone)]
pub struct FunctionMeta {
    /// The variable scope, this should include
    /// all arguments and their type info
    ///
    /// in the vm all type information is discarded
    args: Vec<String>,
    body: Block,
}

/// Represents the functions accessible in the current scope
pub type FunctionScope = HashMap<String, FunctionMeta>;

/// Represents a specific scope.
/// For example a block has it's own scope.
pub type Scope = HashMap<String, ValueMeta>;
/// Represents all program [`Scope`]s
pub type VarEnv = Vec<(Scope, FunctionScope)>;

pub trait Eval {
    fn eval(&self, env: &mut VarEnv, scope: usize) -> Result<Literal, VmErr>;
}

impl Literal {
    pub fn get_int(&self) -> Result<i32, VmErr> {
        match self {
            Literal::Int(i) => Ok(*i),
            _ => Err(VmErr::Err(format!("cannot get integer from {:?}", self))),
        }
    }

    pub fn get_bool(&self) -> Result<bool, VmErr> {
        match self {
            Literal::Bool(b) => Ok(*b),
            _ => Err(VmErr::Err(format!("cannot get Bool from {:?}", self))),
        }
    }
}

impl BinaryOp {
    // Evaluate operator to literal
    pub fn eval(&self, left: Literal, right: Literal) -> Result<Literal, VmErr> {
        use BinaryOp::*;
        use Literal::{Bool, Int};
        match self {
            Add => Ok(Int(left.get_int()? + right.get_int()?)),
            Sub => Ok(Int(left.get_int()? - right.get_int()?)),
            Mul => Ok(Int(left.get_int()? * right.get_int()?)),
            Div => Ok(Int(left.get_int()? / right.get_int()?)),
            And => Ok(Bool(left.get_bool()? && right.get_bool()?)),
            Or => Ok(Bool(left.get_bool()? || right.get_bool()?)),
            Eq => Ok(Bool(left == right)), // overloading
            Lt => Ok(Bool(left.get_int()? < right.get_int()?)),
            Gt => Ok(Bool(left.get_int()? > right.get_int()?)),
        }
    }
}

impl Expr {
    pub fn get_id(&self) -> Result<String, VmErr> {
        match self {
            Expr::Ident(s) => Ok(s.to_owned()),
            _ => Err(VmErr::Err(format!("cannot get id from {:?}", self))),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::ast::Block;
    #[test]
    fn test_check_block1() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        a = a + 1;
        a
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0).unwrap();
        println!("l {:?}", l);
        assert_eq!(l.get_int().unwrap(), 4);
    }

    #[test]
    fn test_check_if_then_else() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        if false {
            a = a + 1
        } else {
            a = a - 1
        };
        if true {
            a = a + 3
        };
        a
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0).unwrap();
        println!("l {:?}", l);
        assert_eq!(l.get_int().unwrap(), 5);
    }

    #[test]
    fn test_check_if_then_else_shadowing() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        if true {
            let a: i32 = 0;
            a = a + 1
        } else {
            a = a - 1
        };
        a
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0).unwrap();
        println!("l {:?}", l);
        // notice this will fail
        assert_eq!(l.get_int().unwrap(), 3);
    }

    #[test]
    fn test_check_while() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        let b: i32 = 0;
        while a > 0 {
            a = a - 1;
            b = b + 1;
        };
        b
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0).unwrap();
        println!("l {:?}", l);
        assert_eq!(l.get_int().unwrap(), 3);
    }

    #[test]
    fn test_check_if() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        let b: i32 = 0;
        if a > 0 { b = 1 };
        b
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0).unwrap();
        println!("l {:?}", l);
        assert_eq!(l.get_int().unwrap(), 1);
    }

    #[test]
    fn test_check_if_else() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        let b: i32 = 0;
        if a < 1 { b = 1 } else { b = 2 };
        b
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0).unwrap();
        println!("l {:?}", l);
        assert_eq!(l.get_int().unwrap(), 2);
    }
}
