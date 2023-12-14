//! Defines parsing rules for [expressions](crate::ast::Expr)
//!

use crate::{ast::FuncCall, parse::Peek};

use super::{
    BinaryOp, Block, Expr, Literal, Parse, ParseStream, Result, Statement, Token, UnaryOp,
};

// Render a "right associative" AST
impl Parse for Expr {
    /// Parses the input stream in to an [expression](Expr)
    ///
    /// ## Deviations from rust syntax
    ///
    /// This parser deviates from the rust syntax in that it treats macro invocations
    /// as function calls simply discarding the ! at the end of function identifiers
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("{:?}", input);
        let left = if input.peek(syn::token::Paren) {
            // we have a left (Expr), e.g., "(1 + 2)"
            let content;
            let _ = syn::parenthesized!(content in input);
            let e: Expr = content.parse()?;
            Expr::Par(Box::new(e))
        } else if input.peek(syn::Ident)
            && (input.peek2(syn::token::Paren)
                || (input.peek2(Token![!]) && input.peek3(syn::token::Paren)))
        {
            //println!("Parsing a function call");
            // This is a function call. Now we simply parse the function call and return that.
            let fncall: FuncCall = input.parse()?;
            Expr::FuncCall(fncall)
        } else if input.peek(syn::Ident) && input.peek2(syn::token::Bracket) {
            //println!("Parsing an array in some interpretation of the word");

            // This might be a bit hacky
            let left: syn::Ident = input.parse()?;
            let left: Expr = Expr::Ident(left.to_string());
            // This is either a Index or a IndexMut
            let content;
            syn::bracketed!(content in input);
            // TODO : Add slice support

            let idx: Expr = content.parse()?;

            if input.peek(Token![=]) {
                Expr::IndexMut(Box::new(left), Box::new(idx))
            } else {
                Expr::Index(Box::new(left), Box::new(idx))
            }
        } else if input.peek(syn::Ident) {
            // we have a left Ident, e.g, "my_best_ident_ever"
            let ident: syn::Ident = input.parse()?;
            println!("Parsed an identifier {ident}");
            Expr::Ident(ident.to_string())
        } else if input.peek(syn::token::If) {
            //println!("Parsing an if statement");
            // we have a left conditional, e.g., "if true {1} else {2}" or
            // if true { 5 }
            let IfThenOptElse(c, t, e) = input.parse()?;
            Expr::IfThenElse(Box::new(c), t, e)
        } else if input.peek(Token![*]) && input.peek2(syn::Ident) {
            let _: Token![*] = input.parse()?;
            let id: syn::Ident = input.parse()?;
            Expr::UnOp(UnaryOp::Dereff, Box::new(Expr::Ident(id.to_string())))
        } else if UnaryOp::peek::<1>(input) {
            // We have a UnaryOp
            let op: UnaryOp = input.parse()?;

            let operand: Expr = if input.peek(syn::Ident) {
                let id: syn::Ident = input.parse()?;
                Expr::Ident(id.to_string())
            } else {
                input.parse()?
            };
            return Ok(Expr::UnOp(op, Box::new(operand)));
        } else if input.peek(syn::token::Bracket) {
            //println!("Parsing an array decleration");
            // This is an array
            let content;
            syn::bracketed!(content in input);
            // One token from now this should either be a ; or a ,

            let bl: Vec<Box<Expr>> = match content.peek2(Token![;]) {
                true => {
                    let exprs = content.parse_terminated(Literal::parse, Token![;])?;
                    let iter: Vec<Literal> = exprs.into_iter().collect();
                    if iter.len() != 2 {
                        panic!("Expected [Expression;usize]");
                    }
                    let mut ret = vec![];
                    let len = match iter[1] {
                        Literal::Int(len) => len as usize,
                        _ => panic!("Second field must be of type usize"),
                    };

                    for _ in 0..len {
                        ret.push(Box::new(Expr::Lit(iter[0].clone())));
                    }
                    ret
                }
                _ => {
                    let intermediate = content.parse_terminated(Expr::parse, Token![,])?;
                    intermediate
                        .iter()
                        .map(|value| Box::new((*value).clone()))
                        .collect()
                }
            };
            Expr::Array(bl)
        } else if input.peek(syn::token::Brace) {
            let bl: Block = input.parse()?;
            Expr::Block(bl)
        } else {
            // else we require a left literal
            let e: Expr = input.parse::<crate::ast::Literal>()?.into();
            e
        };
        // now check if right is an Op Expr
        match (BinaryOp::peek::<1>(input), input.peek2(Token![=])) {
            (true, false) => {
                let op: BinaryOp = input.parse()?;
                let right: Expr = input.parse()?;
                Ok(Expr::BinOp(op, Box::new(left), Box::new(right)))
            }
            // Cover case where we have expr == expr as this would not be coverd by the previous
            // test
            (true, true) => match input.peek(Token![==]) {
                true => {
                    let op: BinaryOp = input.parse()?;
                    let right: Expr = input.parse()?;
                    Ok(Expr::BinOp(op, Box::new(left), Box::new(right)))
                }
                _ => Ok(left),
            },
            // no op, just return the left, no error
            _ => Ok(left),
        }
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
        let _if: syn::token::If = input.parse()?;
        let cond_expr: Expr = input.parse()?;

        let then_block: Block = input.parse()?;

        if input.peek(syn::token::Else) {
            let _else: syn::token::Else = input.parse()?;
        } else {
            return Ok(IfThenOptElse(cond_expr, then_block, None));
        }
        // We now know that the statemet is an if .. else .. statment
        if input.peek(syn::token::If) {
            let nested_if: IfThenOptElse = input.parse()?;
            let semi = nested_if.1.semi;
            let semi = match &nested_if.2 {
                Some(block) => semi | block.semi,
                _ => semi,
            };

            let new_block = Block {
                statements: vec![Statement::Expr(Expr::IfThenElse(
                    Box::new(nested_if.0),
                    nested_if.1,
                    nested_if.2,
                ))],
                semi,
            };
            Ok(IfThenOptElse(cond_expr, then_block, Some(new_block)))
        } else {
            let else_block: Block = input.parse()?;
            Ok(IfThenOptElse(cond_expr, then_block, Some(else_block)))
        }
    }
}
