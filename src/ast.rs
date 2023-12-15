pub mod block;
pub mod expr;
pub mod format;
pub mod func;
pub mod globals;
pub mod literal;
pub mod op;
pub mod program;
pub mod statement;
pub mod types;

pub use block::*;
pub use expr::*;
pub use format::*;
pub use func::*;
pub use globals::*;
pub use literal::*;
pub use op::*;
pub use program::*;
pub use statement::*;
pub use types::*;

use crate::prelude::TypeCheck;

#[allow(dead_code)]
pub struct AstNode<T: TypeCheck> {
    node: T,
    span: proc_macro2::Span,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn display_literal() {
        println!("{}", Literal::Int(3));
        println!("{}", Literal::Bool(false));
        println!("{}", Literal::Unit);
        assert_eq!(format!("{}", Literal::Int(3)), "3");
        assert_eq!(format!("{}", Literal::Bool(false)), "false");
        assert_eq!(format!("{}", Literal::Unit), "()");
    }

    #[test]
    fn display_type() {
        assert_eq!(format!("{}", Type::I32), "i32");
        assert_eq!(format!("{}", Type::Bool), "bool");
        assert_eq!(format!("{}", Type::Unit), "()");
    }

    #[test]
    fn display_expr() {
        println!("{}", Expr::Ident("a".to_string()));
        println!("{}", Expr::Lit(Literal::Int(7)));
        println!("{}", Expr::Lit(Literal::Bool(false)));
        let e = Expr::BinOp(
            BinaryOp::Add,
            Box::new(Expr::Ident("a".to_string())),
            Box::new(Expr::Lit(Literal::Int(7))),
        );
        println!("{}", e);
        assert_eq!(format!("{}", e), "a + 7");
    }

    // As you see it becomes cumbersome to write tests
    // if you have to construct the Expr by hand.
    //
    // Instead we might use our parser

    #[test]
    fn parse_display_expr() {
        let ts: proc_macro2::TokenStream = "a + 7".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {}", e);
    }

    // This one will fail (Display for `if` is not yet implemented).
    // Implement it as an optional assignment
    //
    // Hint: You need to implement Display for Statement and Block

    #[test]
    fn parse_display_if() {
        let ts: proc_macro2::TokenStream = "if a > 5 {5}".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {}", e);
    }

    #[test]
    fn display_if_then_else() {
        let ts: proc_macro2::TokenStream = "
        if a { 
            let a : i32 = false; 
            0
        } else { 
            if a == 5 { b = 8 };
            while b {
                e;
            };
            b 
        }
        "
        .parse()
        .unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("ast:\n{:?}", e);

        println!("pretty:\n{}", e);
    }

    #[test]
    fn display_while() {
        let ts: proc_macro2::TokenStream = "
        while a == 9 {
            let b : i32 = 7;
        }
        "
        .parse()
        .unwrap();
        let e: Statement = syn::parse2(ts).unwrap();
        println!("ast:\n{:?}", e);

        println!("pretty:\n{}", e);
    }
    #[test]
    fn array_definition() {
        let ts: proc_macro2::TokenStream = "let a = [42, 42]".parse().unwrap();
        let arr1: Statement = syn::parse2(ts).unwrap();
        let ts: proc_macro2::TokenStream = "let a = [42; 2]".parse().unwrap();
        let arr2: Statement = syn::parse2(ts).unwrap();
        println!("{:?} {:?}", arr1, arr2);
        assert_eq!(arr1, arr2);
    }
}
