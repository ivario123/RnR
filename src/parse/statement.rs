use syn::Token;

use crate::ast::{BinaryOp, Func};

use crate::parse::Peek;

use super::{Block, Expr, Parse, ParseStream, Result, Statement, Type};

fn parse_let(input: ParseStream) -> Result<Statement> {
    // let a : u32 = 1 + 2
    let _let: syn::token::Let = input.parse()?;
    let mutable: bool = match input.peek(Token![mut]) {
        true => {
            let _: Token![mut] = input.parse()?;
            true
        }
        false => false,
    };
    let left: Expr = input.parse()?;

    // Check if the next token is a :
    let ty: Option<Type> = match input.peek(Token![:]) {
        true => {
            let _: Token![:] = input.parse()?;
            Some(input.parse()?)
        }
        _ => None,
    };

    // Check if next token is a =
    let right: Option<Expr> = match input.peek(Token![=]) {
        true => {
            let _: Token![=] = input.parse()?;
            let rhs = input.parse()?;
            Some(rhs)
        }
        _ => None,
    };
    Ok(Statement::Let(left, mutable, ty, right))
}
impl Statement {
    fn parse_inner(input: ParseStream) -> Result<Statement> {
        if input.peek(syn::token::Let) {
            parse_let(input)
        } else if input.peek(syn::token::Fn) {
            let func: Func = input.parse()?;
            Ok(Statement::FnDecleration(func))
        } else if input.peek(syn::token::While) {
            let _while: syn::token::While = input.parse()?;

            let condition: Expr = input.parse()?;
            println!("Parsed while {condition}");
            println!("Trying to parse {input:?} as a block");
            let block: Block = input.parse()?;

            Ok(Statement::While(condition, block))
        } else if input.peek(syn::token::Brace) {
            let block: Block = input.parse()?;
            return Ok(Statement::Block(block));
        }
        // This sollution has a few quirks, the main one is that
        // it allows for syntax like
        // a === b;
        // Which would result in
        // a = a == b;
        // This is not valid rust syntax but a workaround would be much uglier
        else if BinaryOp::peek::<2>(input) && input.peek3(Token![=]) {
            // We have to check if it is add assign or such first.
            let id: syn::Ident = input.parse()?;
            let id = Expr::Ident(id.to_string());
            let op: BinaryOp = input.parse()?;
            let _: Token![=] = input.parse()?;
            let rhs = input.parse()?;
            let rhs = Expr::BinOp(op, Box::new(id.clone()), Box::new(rhs));

            return Ok(Statement::Assign(id, rhs));
        } else {
            let left = if input.peek(Token![*]) {
                let left: Expr = input.parse()?;
                if BinaryOp::peek::<1>(input) && input.peek2(Token![=]) {
                    // We have to check if it is add assign or such first
                    let id = left;
                    let op: BinaryOp = input.parse()?;
                    let _: Token![=] = input.parse()?;
                    let rhs = input.parse()?;
                    let rhs = Expr::BinOp(op, Box::new(id.clone()), Box::new(rhs));

                    return Ok(Statement::Assign(id, rhs));
                } else {
                    left
                } /*
                  #[allow(unreachable_code)]
                  let _: Token![*] = input.parse()?;
                  let left: syn::Ident = input.parse()?;
                  Expr::UnOp(
                      crate::ast::UnaryOp::Dereff,
                      Box::new(Expr::Ident(left.to_string())),
                  )*/
            } else {
                let left: Expr = input.parse()?;
                left
            };
            if input.peek(syn::token::Eq) {
                // a = 1 + 2
                let _eq: syn::token::Eq = input.parse()?;
                let right: Expr = input.parse()?;

                Ok(Statement::Assign(left, right))
            } else if input.peek2(Token![=]) && input.peek(Token![+]) {
                // Add assign,
                // RHS here is an expression we should rebuild this to a = a + b
                let _: Token![+=] = input.parse()?;
                let rhs = input.parse()?;
                let right = Expr::bin_op(crate::ast::BinaryOp::Add, left.clone(), rhs);
                Ok(Statement::Assign(left, right))
            } else {
                // 1 + 5
                Ok(Statement::Expr(left))
            }
        }
    }
}
impl Parse for Statement {
    fn parse(input: ParseStream) -> Result<Statement> {
        let outer = input.span();
        match Statement::parse_inner(input) {
            Ok(statement) => Ok(statement),
            Err(e) => {
                let span = e.span();
                //let file = span.source_file().to_string();
                //println!("{}", file);
                let start = span.start();
                let row = start.line;
                let col = start.column;
                let end = span.end();
                let end_row = end.line;
                let end_col = end.column;
                eprintln!(
                    "Error {e} occured while parsing token {} on line {} @ col {} until line {} @ col {}",
                    //e.span().source_text().unwrap_or("".to_string()),
                    e.span().located_at(outer).source_text().unwrap_or("".to_string()),
                    row,
                    col,
                    end_row,
                    end_col
                );
                Err(e)
            }
        }
    }
}
