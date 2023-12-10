pub mod block;
pub mod expr;
pub mod func;
pub mod literal;
pub mod op;
pub mod program;
pub mod statement;
pub mod types;

pub use block::*;
pub use expr::*;
pub use func::*;
pub use literal::*;
pub use op::*;
pub use program::*;
pub use statement::*;
pub use types::*;

use crate::ast::{BinaryOp, Block, Expr, Literal, Statement, Type, UnaryOp};

use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_lit_int() {
        let ts: proc_macro2::TokenStream = "1".parse().unwrap();
        let l: Literal = syn::parse2(ts).unwrap();
        assert_eq!(l, Literal::Int(1));
    }

    #[test]
    fn parse_lit_bool_false() {
        let ts: proc_macro2::TokenStream = "false".parse().unwrap();
        let l: Literal = syn::parse2(ts).unwrap();
        assert_eq!(l, Literal::Bool(false));
    }

    #[test]
    fn parse_lit_fail() {
        let ts: proc_macro2::TokenStream = "a".parse().unwrap();
        let l: Result<Literal> = syn::parse2(ts);
        assert!(l.is_err());
    }

    #[test]
    fn parse_op_add() {
        let ts: proc_macro2::TokenStream = "+".parse().unwrap();
        let op: BinaryOp = syn::parse2(ts).unwrap();
        println!("op {:?}", op);
        assert_eq!(op, BinaryOp::Add);
    }

    #[test]
    fn parse_op_mul() {
        let ts: proc_macro2::TokenStream = "*".parse().unwrap();
        let op: BinaryOp = syn::parse2(ts).unwrap();
        println!("op {:?}", op);
        assert_eq!(op, BinaryOp::Mul);
    }

    #[test]
    fn parse_op_fail() {
        let ts: proc_macro2::TokenStream = "1".parse().unwrap();
        let err = syn::parse2::<BinaryOp>(ts);
        println!("err {:?}", err);
        assert!(err.is_err());
    }

    #[test]
    fn test_expr_ident() {
        let ts: proc_macro2::TokenStream = "my_best_ident".parse().unwrap();
        println!("{:?}", ts);
        let e: Expr = syn::parse2(ts).unwrap();

        println!("e {:?}", e);

        assert_eq!(e, Expr::Ident("my_best_ident".to_string()));
    }

    #[test]
    fn test_expr_if_then_else() {
        let ts: proc_macro2::TokenStream = "if a > 0 {1} else {2}".parse().unwrap();
        println!("{:?}", ts);
        let e: Expr = syn::parse2(ts).unwrap();

        println!("e {:?}", e);
    }

    // This test is not really a test of our parser
    // Added just a reference to how Rust would treat the nesting.
    #[test]
    #[allow(clippy::all)]
    #[allow(unused_must_use)]
    fn test_if_then_else_nested_rust() {
        if false {
            2;
        } else if true {
            3 + 5;
        };
    }

    #[test]
    fn test_if_then_else_nested() {
        let ts: proc_macro2::TokenStream = "
    if false {
        2;
    } else {
        if true {
            3 + 5;
        }
    }"
        .parse()
        .unwrap();
        println!("{:?}", ts);
        let e: Expr = syn::parse2(ts).unwrap();

        println!("e {:?}", e);
    }

    // This test is not really a test of our parser
    // Added just a reference to how Rust would treat the nesting.
    #[test]
    #[allow(unused_must_use)]
    #[allow(clippy::all)]
    fn test_if_then_else_nested_rust2() {
        if false {
            2;
        } else if true {
            3 + 5;
        };
    }

    #[test]
    fn test_if_then_else_nested2() {
        let ts: proc_macro2::TokenStream = "
    if false {
        2;
    } else if true {
        3 + 5;
    }"
        .parse()
        .unwrap();
        println!("{:?}", ts);
        let e: Expr = syn::parse2(ts).unwrap();

        println!("e {:?}", e);
    }

    #[test]
    fn test_expr_right() {
        let ts: proc_macro2::TokenStream = "2 - 4 - 5".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {:?}", e);
    }

    #[test]
    fn test_expr_par() {
        let ts: proc_macro2::TokenStream = "(2 - 4) - 5".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {:?}", e);
    }

    #[test]
    fn test_expr_mul() {
        let ts: proc_macro2::TokenStream = "2 * 4 - 5".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {:?}", e);
    }

    #[test]
    fn test_expr_par_mul() {
        let ts: proc_macro2::TokenStream = "(2 * 4) - 5".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {:?}", e);
    }

    #[test]
    fn test_expr_fail() {
        let ts: proc_macro2::TokenStream = "(2 * 4) - ".parse().unwrap();
        let e: Result<Expr> = syn::parse2(ts);
        assert!(e.is_err());
    }

    #[test]
    fn test_type_i32() {
        let ts: proc_macro2::TokenStream = "i32".parse().unwrap();
        let e: Type = syn::parse2(ts).unwrap();
        assert_eq!(e, Type::I32);
    }

    #[test]
    fn test_type_bool() {
        let ts: proc_macro2::TokenStream = "bool".parse().unwrap();
        let e: Type = syn::parse2(ts).unwrap();
        assert_eq!(e, Type::Bool);
    }

    #[test]
    fn test_type_unit() {
        let ts: proc_macro2::TokenStream = "()".parse().unwrap();
        let e: Type = syn::parse2(ts).unwrap();
        assert_eq!(e, Type::Unit);
    }

    #[test]
    fn test_type_fail() {
        let ts: proc_macro2::TokenStream = "u32".parse().unwrap();
        let e: Result<Type> = syn::parse2(ts);
        assert!(e.is_err());
    }

    #[test]
    fn test_block_expr_let() {
        let ts: proc_macro2::TokenStream = "let a: i32 = 2".parse().unwrap();
        let be: Statement = syn::parse2(ts).unwrap();
        println!("be {:?}", be);

        assert_eq!(
            be,
            Statement::Let(
                Expr::Ident("a".to_string()),
                false,
                Some(Type::I32),
                Some(Expr::Lit(Literal::Int(2)))
            )
        );
    }

    #[test]
    fn test_block_expr_assign() {
        let ts: proc_macro2::TokenStream = "a = false".parse().unwrap();
        let be: Statement = syn::parse2(ts).unwrap();
        println!("be {:?}", be);

        assert_eq!(
            be,
            Statement::Assign(
                Expr::Ident("a".to_string()),
                Expr::Lit(Literal::Bool(false))
            )
        );
    }

    #[test]
    fn test_block_while() {
        let ts: proc_macro2::TokenStream = "while a {}".parse().unwrap();
        let be: Statement = syn::parse2(ts).unwrap();
        println!("be {:?}", be);

        assert_eq!(
            be,
            Statement::While(
                Expr::Ident("a".to_string()),
                Block {
                    statements: vec![],
                    semi: false
                }
            )
        );
    }

    #[test]
    fn test_block_expr_expr() {
        let ts: proc_macro2::TokenStream = "a".parse().unwrap();
        let be: Statement = syn::parse2(ts).unwrap();
        println!("be {:?}", be);
        assert_eq!(be, Statement::Expr(Expr::Ident("a".to_string())));
    }
    #[test]
    fn test_block_expr_fail() {
        let ts: proc_macro2::TokenStream = "{ let a = }".parse().unwrap();
        let be: Result<Statement> = syn::parse2(ts);
        println!("be {:?}", be);
        assert!(be.is_err());
    }

    #[test]
    fn test_block1() {
        let ts: proc_macro2::TokenStream = "{ let a : i32 = 1; a = 5; a + 5; }".parse().unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        assert_eq!(bl.statements.len(), 3);
        assert!(bl.semi);
    }

    #[test]
    fn test_block2() {
        let ts: proc_macro2::TokenStream = "{ let b : bool = false; b = true }".parse().unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        assert_eq!(bl.statements.len(), 2);
        assert!(!bl.semi);
    }

    #[test]
    fn test_block_fail() {
        let ts: proc_macro2::TokenStream = "{ let a = 1 a = 5 }".parse().unwrap();
        let bl: Result<Block> = syn::parse2(ts);
        println!("bl {:?}", bl);

        assert!(bl.is_err());
    }
}
