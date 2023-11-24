use crate::ast::{
    Arguments, Block, Expr, FnDeclaration, Literal, Mutable, Op, Parameter, Parameters, Prog,
    Statement, Type, UnOp,
};

use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

// Back-port your parser
// You may want to put the tests in a module.
// See e.g., the vm.rs

impl Parse for Literal {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!("not implemented {:?}", input)
    }
}

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
fn parse_lit_string() {
    let ts: proc_macro2::TokenStream = "\"abba\"".parse().unwrap();
    let l: Literal = syn::parse2(ts).unwrap();
    assert_eq!(l, Literal::String("abba".to_string()));
}

#[test]
fn parse_lit_fail() {
    let ts: proc_macro2::TokenStream = "a".parse().unwrap();
    let l: Result<Literal> = syn::parse2(ts);
    assert!(l.is_err());
}

impl Parse for Op {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!("not implemented {:?}", input)
    }
}

impl Parse for UnOp {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!("not implemented {:?}", input)
    }
}

#[test]
fn parse_op_add() {
    let ts: proc_macro2::TokenStream = "+".parse().unwrap();
    let op: Op = syn::parse2(ts).unwrap();
    println!("op {:?}", op);
    assert_eq!(op, Op::Add);
}

#[test]
fn parse_op_mul() {
    let ts: proc_macro2::TokenStream = "*".parse().unwrap();
    let op: Op = syn::parse2(ts).unwrap();
    println!("op {:?}", op);
    assert_eq!(op, Op::Mul);
}

#[test]
fn parse_op_fail() {
    let ts: proc_macro2::TokenStream = "1".parse().unwrap();
    let err = syn::parse2::<Op>(ts);
    println!("err {:?}", err);
    assert!(err.is_err());
}

// Render a "right associative" AST
impl Parse for Expr {
    // Use a custom parser for expressions
    fn parse(input: ParseStream) -> Result<Self> {
        todo!("not implemented {:?}", input)
    }
}

//
// We want to parse strings like
// `if expr { then block }`
// and
// `if expr { then block } else { else block }
//
// The else arm is optional
struct IfThenOptElse(Expr, Block, Option<Block>);

impl Parse for IfThenOptElse {
    fn parse(input: ParseStream) -> Result<IfThenOptElse> {
        todo!("not implemented {:?}", input)
    }
}

#[test]
fn test_println() {
    let ts: proc_macro2::TokenStream = "
    println!(\"{}\", 1)"
        .parse()
        .unwrap();
    println!("{:?}", ts);
    let e: Expr = syn::parse2(ts).unwrap();

    println!("e {:?}", e);
    println!("e {}", e);
}

#[test]
fn test_expr_block() {
    let ts: proc_macro2::TokenStream = "
    {
        12
    }
    "
    .parse()
    .unwrap();
    println!("{:?}", ts);
    let e: Expr = syn::parse2(ts).unwrap();

    println!("e {:?}", e);
    println!("e {}", e);
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
    println!("e {}", e);
}

#[test]
fn test_if_then_else_nested3() {
    let ts: proc_macro2::TokenStream = "
    if false {
        2;
    } else if true {
        3 + 5;
    } else if false {
        let a : i32 = 0;
    } else {
        5
    }
    "
    .parse()
    .unwrap();
    println!("{:?}", ts);
    let e: Expr = syn::parse2(ts).unwrap();

    println!("e {:?}", e);
    println!("e {}", e);
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
#[allow(unused_must_use)]
fn test_if_then_else_nested_rust() {
    if false {
        2;
    } else {
        if true {
            3 + 5;
        }
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
fn test_if_then_else_nested_rust2() {
    if false {
        2;
    } else if true {
        3 + 5;
    };
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
fn test_expr_call() {
    let ts: proc_macro2::TokenStream = "ident(1, 2 + 2)".parse().unwrap();
    let e: Expr = syn::parse2(ts).unwrap();
    println!("e {}", e);
}

#[test]
fn test_expr_call_comma() {
    let ts: proc_macro2::TokenStream = "ident(1, 2 + 2,)".parse().unwrap();
    let e: Expr = syn::parse2(ts).unwrap();
    println!("e {}", e);
}

#[test]
fn test_expr_call_block() {
    let ts: proc_macro2::TokenStream = "ident({1}, {let a = 6; a },)".parse().unwrap();
    let e: Expr = syn::parse2(ts).unwrap();
    println!("e {}", e);
}

#[test]
fn test_expr_fail() {
    let ts: proc_macro2::TokenStream = "(2 * 4) - ".parse().unwrap();
    let e: Result<Expr> = syn::parse2(ts);
    assert!(e.is_err());
}

#[test]
fn test_expr_call_fail() {
    let ts: proc_macro2::TokenStream = "call(2 * 4, -)".parse().unwrap();
    let e: Result<Expr> = syn::parse2(ts);
    assert!(e.is_err());
}

use quote::quote;

impl Parse for Type {
    fn parse(input: ParseStream) -> Result<Type> {
        // The syn::Type is very complex and overkill
        // Types in Rust involve generics, paths
        // etc., etc., etc. ...
        //
        // To make things simple, we just turn the syn::Type
        // to a token stream (`quote`) and turn that into a String
        // and turn that into an &str (`as_str`)
        Ok(match input.parse::<Token![&]>() {
            Ok(_) => {
                let t: Type = input.parse()?;
                Type::Ref(Box::new(t))
            }
            Err(_) => {
                let t: syn::Type = input.parse()?;
                let ts = quote! {#t}.to_string();
                match ts.as_str() {
                    "i32" => Type::I32,
                    "bool" => Type::Bool,
                    "()" => Type::Unit,
                    _ =>
                    // to explicitly create an error at the current position
                    {
                        input.step(|cursor| Err(cursor.error("expected operator")))?
                    }
                }
            }
        })
    }
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
    assert_eq!(e.is_err(), true);
}

impl Parse for Parameter {
    fn parse(input: ParseStream) -> Result<Parameter> {
        let mutable = if input.peek(syn::token::Mut) {
            let _m: syn::token::Mut = input.parse()?;
            Mutable(true)
        } else {
            Mutable(false)
        };
        let id: syn::Ident = input.parse()?;
        let id = id.to_string();

        let _colon: syn::Token![:] = input.parse()?;

        let ty: Type = input.parse()?;
        Ok(Parameter { mutable, id, ty })
    }
}

#[test]
fn test_arg() {
    let ts: proc_macro2::TokenStream = "a: i32".parse().unwrap();
    let arg: Parameter = syn::parse2(ts).unwrap();
    println!("{}", arg);
}

// Here we take advantage of the parser function `parse_terminated`
impl Parse for Parameters {
    fn parse(input: ParseStream) -> Result<Parameters> {
        let content;
        let _ = syn::parenthesized!(content in input);
        let bl: Punctuated<Parameter, Token![,]> =
            content.parse_terminated(Parameter::parse, Token![,])?;
        Ok(Parameters(bl.into_iter().collect()))
    }
}

#[test]
fn test_args() {
    let ts: proc_macro2::TokenStream = "(a: i32, b: bool)".parse().unwrap();
    let arg: Parameters = syn::parse2(ts).unwrap();
    println!("{}", arg);
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> Result<Arguments> {
        let content;
        let _ = syn::parenthesized!(content in input);
        let bl: Punctuated<Expr, Token![,]> = content.parse_terminated(Expr::parse, Token![,])?;
        Ok(Arguments(bl.into_iter().collect()))
    }
}

#[test]
fn test_pars() {
    let ts: proc_macro2::TokenStream = "(1, 2, 3 + 4)".parse().unwrap();
    let pars: Arguments = syn::parse2(ts).unwrap();
    println!("{}", pars);
}

#[test]
fn test_call() {
    let ts: proc_macro2::TokenStream = "a(1, 2, 3 + 4)".parse().unwrap();
    let expr: Expr = syn::parse2(ts).unwrap();
    println!("{}", expr);
}

impl Parse for FnDeclaration {
    fn parse(input: ParseStream) -> Result<FnDeclaration> {
        // fn ident
        let _fn: syn::token::Fn = input.parse()?;
        let id: syn::Ident = input.parse()?;
        let id = id.to_string();

        // fn ident(...)
        let args: Parameters = input.parse()?;
        // fn ident(...) -> i32

        let ty = if input.peek(syn::Token![->]) {
            let _: syn::Token![->] = input.parse()?;
            let ty: Type = input.parse()?;
            Some(ty)
        } else {
            None
        };

        // fn ident() ... { ... }
        let body: Block = input.parse()?;

        Ok(FnDeclaration {
            id,
            parameters: args,
            ty,
            body,
        })
    }
}

#[test]
fn test_fn_no_type() {
    let ts: proc_macro2::TokenStream = "fn a(a: i32, b: bool) {}".parse().unwrap();
    let fn_: FnDeclaration = syn::parse2(ts).unwrap();
    println!("{}", fn_);
}

#[test]
fn test_fn_type() {
    let ts: proc_macro2::TokenStream = "fn a(a: i32, b: bool) -> i32 {}".parse().unwrap();
    let fn_: FnDeclaration = syn::parse2(ts).unwrap();
    println!("{}", fn_);
}

impl Parse for Statement {
    fn parse(input: ParseStream) -> Result<Statement> {
        if input.peek(syn::token::Fn) {
            // fn
            let fn_: FnDeclaration = input.parse()?;
            Ok(Statement::Fn(fn_))
        } else if input.peek(syn::token::Let) {
            // let ...
            let _let: syn::token::Let = input.parse()?;

            // let mut ...
            let m = if input.peek(syn::token::Mut) {
                let _mut: syn::token::Mut = input.parse()?;
                Mutable(true)
            } else {
                Mutable(false)
            };

            // let mut a ...
            let id: syn::Ident = input.parse()?;
            let id = id.to_string();

            // let a: i32 ...
            let ty = if input.peek(Token![:]) {
                let _colon: Token![:] = input.parse()?;

                Some(input.parse()?)
            } else {
                None
            };

            // let a: i32 = 1 + 2
            let right = if input.peek(Token![=]) {
                let _eq: syn::token::Eq = input.parse()?;
                Some(input.parse()?)
            } else {
                None
            };

            Ok(Statement::Let(m, id, ty, right))
        } else if input.peek(syn::token::While) {
            // while a {}
            let _while: syn::token::While = input.parse()?;

            let e: Expr = input.parse()?;

            let bl: Block = input.parse()?;
            Ok(Statement::While(e, bl))
        } else {
            // a = 1 + 2, as a assignment
            // 1 + 2, as an expression
            let left: Expr = input.parse()?;

            if input.peek(syn::token::Eq) {
                // a = 1 + 2
                let _eq: syn::token::Eq = input.parse()?;
                let right: Expr = input.parse()?;

                Ok(Statement::Assign(left, right))
            } else {
                // 1 + 2
                Ok(Statement::Expr(left))
            }
        }
    }
}

#[test]
fn test_statement_let_ty_expr() {
    let ts: proc_macro2::TokenStream = "let a: i32 = 2".parse().unwrap();
    let stmt: Statement = syn::parse2(ts).unwrap();
    println!("stmt {:?}", stmt);

    assert_eq!(
        stmt,
        Statement::Let(
            Mutable(false),
            "a".to_string(),
            Some(Type::I32),
            Some(Expr::Lit(Literal::Int(2)))
        )
    );
}

#[test]
fn test_statement_let_mut_ty_expr() {
    let ts: proc_macro2::TokenStream = "let mut a: i32 = 2".parse().unwrap();
    let stmt: Statement = syn::parse2(ts).unwrap();
    println!("stmt {:?}", stmt);

    assert_eq!(
        stmt,
        Statement::Let(
            Mutable(true),
            "a".to_string(),
            Some(Type::I32),
            Some(Expr::Lit(Literal::Int(2)))
        )
    );
}

#[test]
fn test_statement_let() {
    let ts: proc_macro2::TokenStream = "let a".parse().unwrap();
    let stmt: Statement = syn::parse2(ts).unwrap();
    println!("stmt {:?}", stmt);

    assert_eq!(
        stmt,
        Statement::Let(Mutable(false), "a".to_string(), None, None,)
    );
}

#[test]
fn test_statement_assign() {
    let ts: proc_macro2::TokenStream = "a = false".parse().unwrap();
    let stmt: Statement = syn::parse2(ts).unwrap();
    println!("stmt {:?}", stmt);

    assert_eq!(
        stmt,
        Statement::Assign(
            Expr::Ident("a".to_string()),
            Expr::Lit(Literal::Bool(false))
        )
    );
}

#[test]
fn test_statement_while() {
    let ts: proc_macro2::TokenStream = "while a {}".parse().unwrap();
    let stmt: Statement = syn::parse2(ts).unwrap();
    println!("stmt {:?}", stmt);

    assert_eq!(
        stmt,
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
fn test_statement_expr() {
    let ts: proc_macro2::TokenStream = "a".parse().unwrap();
    let stmt: Statement = syn::parse2(ts).unwrap();
    println!("stmt {:?}", stmt);
    assert_eq!(stmt, Statement::Expr(Expr::Ident("a".to_string())));
}

use syn::punctuated::Punctuated;

// Here we take advantage of the parser function `parse_terminated`
// impl Parse for Block {
//     fn parse(input: ParseStream) -> Result<Block> {
//         let content;
//         let _ = syn::braced!(content in input);

//         let bl: Punctuated<Statement, Token![;]> = content.parse_terminated(Statement::parse, Token![;])?;

//         // We need to retrieve the semi before we collect into a vector
//         // as into_iter consumes the value.
//         let semi = bl.trailing_punct();

//         Ok(Block {
//             // turn the Punctuated into a vector
//             statements: bl.into_iter().collect(),
//             semi,
//         })
//     }
// }

// Here we take advantage of the parser function `parse_terminated`
impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Block> {
        let content;
        let _ = syn::braced!(content in input);

        let mut semi = false;
        let mut statements = vec![];
        while !content.is_empty() {
            let stmt: Statement = content.parse()?;

            let mut has_semi = false;
            while content.peek(Token![;]) {
                let _: Token![;] = content.parse()?;
                has_semi = true;
            }
            match stmt {
                Statement::Let(_, _, _, _) => {
                    if !has_semi {
                        // generate an error (we know that it is not a ";")
                        let _: Token![;] = content.parse()?;
                    }
                }
                Statement::Assign(_, _) | Statement::Expr(_) => {
                    if !content.is_empty() {
                        if !has_semi {
                            // generate an error (we know that it is not a ";")
                            let _: Token![;] = content.parse()?;
                        }
                    }
                }

                Statement::While(_, _) | Statement::Fn(_) => {
                    // these may or may not be trailed by ";"
                    // so we do nothing
                }
            }
            semi = has_semi;
            statements.push(stmt);
        }

        Ok(Block {
            // turn the Punctuated into a vector
            statements,
            semi,
        })
    }
}

#[test]
fn test_block_expr_fail() {
    let ts: proc_macro2::TokenStream = "{ let a = }".parse().unwrap();
    let stmt: Result<Statement> = syn::parse2(ts);
    println!("stmt {:?}", stmt);
    assert_eq!(stmt.is_err(), true);
}

#[test]
fn test_block_semi() {
    let ts: proc_macro2::TokenStream = "
    { 
        let a : i32 = 1; 
        a = 5; 
        a + 5; 
    }"
    .parse()
    .unwrap();
    let bl: Block = syn::parse2(ts).unwrap();
    println!("bl {}", bl);
    assert_eq!(bl.statements.len(), 3);
    assert_eq!(bl.semi, true);
}

#[test]
fn test_block_no_semi() {
    let ts: proc_macro2::TokenStream = "
    { 
        let a : i32 = 1; 
        a = 5; 
        a + 5 
    }"
    .parse()
    .unwrap();
    let bl: Block = syn::parse2(ts).unwrap();
    println!("bl {}", bl);
    assert_eq!(bl.statements.len(), 3);
    assert_eq!(bl.semi, false);
}

#[test]
fn test_block_fn() {
    let ts: proc_macro2::TokenStream = "
    { 
        let a : i32 = 1; 
        fn t() {}
        a = 5; 
        a + 5 
    }"
    .parse()
    .unwrap();
    let bl: Block = syn::parse2(ts).unwrap();
    println!("bl {}", bl);
    assert_eq!(bl.statements.len(), 4);
    assert_eq!(bl.semi, false);
}

#[test]
fn test_block_while() {
    let ts: proc_macro2::TokenStream = "
    { 
        let a : i32 = 1;
        while true {} 
        a = 5;
        a + 5 
    }"
    .parse()
    .unwrap();
    let bl: Block = syn::parse2(ts).unwrap();
    println!("bl {}", bl);
    assert_eq!(bl.statements.len(), 4);
    assert_eq!(bl.semi, false);
}

#[test]
fn test_block2() {
    let ts: proc_macro2::TokenStream = "{ let b : bool = false; b = true }".parse().unwrap();
    let bl: Block = syn::parse2(ts).unwrap();
    println!("bl {:?}", bl);
    assert_eq!(bl.statements.len(), 2);
    assert_eq!(bl.semi, false);
}

#[test]
fn test_block_fail() {
    let ts: proc_macro2::TokenStream = "{ let a = 1 a = 5 }".parse().unwrap();
    let bl: Result<Block> = syn::parse2(ts);
    println!("bl {:?}", bl);

    assert_eq!(bl.is_err(), true);
}

impl Parse for Prog {
    fn parse(input: ParseStream) -> Result<Prog> {
        let mut fns = vec![];
        while input.peek(syn::token::Fn) {
            let fn_: FnDeclaration = input.parse()?;
            fns.push(fn_);
        }

        Ok(Prog(fns))
    }
}

#[test]
fn test_prog() {
    let ts: proc_macro2::TokenStream = "
    fn a(a: i32) { let b = a; }
    fn b() -> i32 { 3 }

    fn main() {

    }
    "
    .parse()
    .unwrap();
    let pr: Result<Prog> = syn::parse2(ts);
    println!("prog\n{}", pr.unwrap());
}

#[test]
fn test_ref_de_ref() {
    let ts: proc_macro2::TokenStream = "
    fn main() {
        let a = &1;
        let mut a = &mut 1;
        *a = *a + 1;
        println!(\"{}\", *a);
    }
    "
    .parse()
    .unwrap();
    let pr: Result<Prog> = syn::parse2(ts);
    println!("prog\n{}", pr.unwrap());
}
