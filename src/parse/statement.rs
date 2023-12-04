

use syn::Token;

use crate::ast::{Func};

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
            Some(input.parse()?)
        }
        _ => None,
    };
    Ok(Statement::Let(left, mutable, ty, right))
}

impl Parse for Statement {
    fn parse(input: ParseStream) -> Result<Statement> {
        if input.peek(syn::token::Let) {
            parse_let(input)
        } else if input.peek(syn::token::Fn) {
            let func: Func = input.parse()?;
            Ok(Statement::FnDecleration(func))
        } else if input.peek(syn::token::While) {
            let _while: syn::token::While = input.parse()?;
            let condition: Expr = input.parse()?;
            let block: Block = input.parse()?;
            Ok(Statement::While(condition, block))
        } else if input.peek(syn::token::Brace) {
            let block: Block = input.parse()?;
            return Ok(Statement::Block(block));
        } else {
            let left = input.parse()?;

            if input.peek(syn::token::Eq) {
                // a = 1 + 2
                let _eq: syn::token::Eq = input.parse()?;
                let right: Expr = input.parse()?;

                Ok(Statement::Assign(left, right))
            } else {
                // 1 + 5
                Ok(Statement::Expr(left))
            }
        }
    }
}
