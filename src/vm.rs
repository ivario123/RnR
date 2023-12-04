use crate::ast::{Block, Expr, FnDeclaration, Literal, Op, Prog};
use crate::common::Eval;
use crate::env::{Env, Ref};
use crate::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Lit(Literal),
    Ref(Ref),
    UnInit,
    Mut(Box<Val>),
}

// Helpers for Val
// Alternatively implement the TryFrom trait
impl Val {
    pub fn get_bool(&self) -> Result<bool, Error> {
        match self {
            Val::Lit(Literal::Bool(b)) => Ok(*b),
            _ => Err(format!("cannot get Bool from {:?}", self)),
        }
    }

    pub fn get_int(&self) -> Result<i32, Error> {
        match self {
            Val::Lit(Literal::Int(i)) => Ok(*i),
            _ => Err(format!("cannot get integer from {:?}", self)),
        }
    }
}

// Helper for Op
impl Op {
    // Evaluate operator to literal
    pub fn eval(&self, left: Val, right: Val) -> Result<Val, Error> {
        todo!();
    }
}

impl Eval<Val> for Expr {
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> {
        todo!("not implemented {:?}", self)
    }
}

impl Eval<Val> for Block {
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> {
        todo!("not implemented {:?}", self)
    }
}

impl Eval<Val> for FnDeclaration {
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> {
        todo!("not implemented {:?}", self)
    }
}

impl Eval<Val> for Prog {
    fn eval(&self, env: &mut Env<Val>) -> Result<(Val, Option<Ref>), Error> {
        todo!("not implemented {:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::Val;
    use crate::ast::Literal;
    use crate::ast::{Block, Prog};
    use crate::common::parse_test;

    #[test]
    fn test_block_let() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a: i32 = 1;
        let b: i32 = 2;

        a + b
    }",
        );
        assert_eq!(v.unwrap().get_int().unwrap(), 3);
    }

    #[test]
    fn test_block_let_shadow() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a: i32 = 1;
        let b: i32 = 2;
        let a: i32 = 3;
        let b: i32 = 4;

        a + b
    }",
        );
        assert_eq!(v.unwrap().get_int().unwrap(), 7);
    }

    #[test]
    fn test_block_assign() {
        let v = parse_test::<Block, Val>(
            "
    {
        let mut a: i32 = 1;
        a = a + 2;
        a
    }",
        );
        assert_eq!(v.unwrap().get_int().unwrap(), 3);
    }

    #[test]
    fn test_expr_if_then_else() {
        let v = parse_test::<Block, Val>(
            "
    {
        let mut a: i32 = 1;
        a = if a > 0 { a + 1 } else { a - 2 };
        a
    }",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 2);
    }

    #[test]
    fn test_expr_if_then_else2() {
        let v = parse_test::<Block, Val>(
            "
    {
        let mut a: i32 = 1;
        a = if a < 0 { a + 1 } else { a - 2 };
        a
    }",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), -1);
    }

    #[test]
    fn test_ref_deref() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 1;
        let b = &a;
        *b
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 1);
    }

    #[test]
    fn test_ref_deref_indirect() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 1;
        let b = &a;
        let c = b;
        *c
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 1);
    }

    #[test]
    fn test_deref_assign() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 1;
        let b = &a;
        *b = 7;
        a
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 7);
    }

    #[test]
    fn test_while() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 2;
        let b = 0;
        while a > 0 {
            a = a - 1;
            b = b + 1;
        }
        b
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 2);
    }

    #[test]
    fn test_while_ref() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 2;
        let b = 0;
        let c = &b;
        while a > 0 {
            a = a - 1;
            *c = (*c) + 1;
        }
        *c
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 2);
    }

    #[test]
    fn test_while_ref2() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 2;
        let b = 0;
        let c = &b;
        let d = &a;
        
        while (*d) > 0 {
            *d = (*d) - 1;
            *c = (*c) + 1;
        }
        *c
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 2);
    }

    #[test]
    fn test_bool() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = true && false;
        a
    }
    ",
        );

        assert!(!v.unwrap().get_bool().unwrap());
    }

    #[test]
    fn test_bool_bang() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = true && !false;
        a
    }
    ",
        );

        assert!(v.unwrap().get_bool().unwrap());
    }

    #[test]
    fn test_bool_bang2() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = (!true) && false;
        a
    }
    ",
        );

        assert!(!v.unwrap().get_bool().unwrap());
    }

    #[test]
    fn test_local_block() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 1;
        { 
            let b = &a;
            *b = 2;
        };
        a
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 2);
    }

    #[test]
    fn test_local_block_assign() {
        let v = parse_test::<Block, Val>(
            "
    {
        let a = 6;
        let b = { 
            let b = &a;
            *b = (*b) + 1;
            *b
        };
        b
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 7);
    }

    #[test]
    fn test_prog() {
        let v = parse_test::<Prog, Val>(
            "
    fn main() {
        let a = 1;
        a
    }
    ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 1);
    }

    #[test]
    fn test_local_fn() {
        let v = parse_test::<Prog, Val>(
            "
    fn main() {
        fn f(i: i32, j: i32) -> i32 {
            i + j
        }
        let a = f(1, 2);
        println!(\"a = {} and another a = {}\", a, a);
    }
    ",
        );

        assert_eq!(v.unwrap(), Val::Lit(Literal::Unit));
    }

    #[test]
    fn test_check_if_then_else_shadowing() {
        let v = parse_test::<Block, Val>(
            "
        {
            let a: i32 = 1 + 2; // a == 3
            let a: i32 = 2 + a; // a == 5
            if true {
                a = a - 1;      // outer a == 4
                let a: i32 = 0; // inner a == 0
                a = a + 1       // inner a == 1
            } else {
                a = a - 1
            };
            a   // a == 4
        }
        ",
        );

        assert_eq!(v.unwrap().get_int().unwrap(), 4);
    }
    #[test]
    fn test_ref() {
        let v = parse_test::<Block, Val>(
            "
        {
            let a = &1;
            *a
        }
        ",
        );
        assert_eq!(v.unwrap().get_int().unwrap(), 1);
    }
}
