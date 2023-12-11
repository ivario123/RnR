use crate::ast::{Arg, Block, Expr, Func, FuncCall, Type};
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        // We have another argument
        let mutable = match input.peek(Token![mut]) {
            true => {
                let _: Token![mut] = input.parse()?;
                true
            }
            f => f,
        };
        let id: Expr = input.parse()?;
        let id = match id {
            Expr::Ident(i) => Expr::Ident(i),
            e => panic!(
                "{}",
                format!("fn argument has to be a valid identifier, got {e}").to_owned()
            ),
        };
        let _: Token![:] = input.parse()?;
        let ty: Type = input.parse()?;
        Ok(Arg { id, mutable, ty })
    }
}
impl Parse for Func {
    /// Parses the input stream in to a [function definition](Func)
    ///
    fn parse(input: ParseStream) -> Result<Self> {
        println!("parsing function definition {:?}", input);
        let _: Token![fn] = input.parse()?;
        let ident: syn::Ident = input.parse()?;
        let ident = Expr::Ident(ident.to_string());
        let content;
        syn::parenthesized!(content in input);
        let args = content.parse_terminated(Arg::parse, syn::token::Comma)?;
        let ty: Type = if input.peek(Token![->]) {
            let _: Token![->] = input.parse()?;
            input.parse()?
        } else {
            Type::Unit
        };
        let body: Block = input.parse()?;
        Ok(Func {
            id: ident,
            ty,
            body,
            args: args.into_iter().collect(),
        })
    }
}
impl Parse for FuncCall {
    /// Parses the input stream in to a [function call](FuncCall)
    ///
    /// ## Deviations from rust syntax
    ///
    /// This parser deviates from the rust parser in that it treats macro invocations as a function
    /// call.
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: syn::Ident = input.parse()?;
        let mut inner = ident.to_string();

        //
        //  Deviation from rust syntax
        //  discard the ! as we have not implemented macro invocations
        //
        //
        if input.peek(Token![!]) {
            let _: Token![!] = input.parse()?;
            inner.push('!');
        }

        let ident = Expr::Ident(inner);

        let content;
        syn::parenthesized!(content in input);
        let args = content.parse_terminated(Expr::parse, syn::token::Comma)?;
        Ok(FuncCall {
            id: Box::new(ident),
            args: Box::new(args.into_iter().collect()),
        })
    }
}
