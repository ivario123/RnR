use crate::ast::Literal;

use super::{Parse, ParseStream, Result, Type};
use quote::quote;
use syn::Token;
impl Parse for Type {
    fn parse(input: ParseStream) -> Result<Type> {
        // The syn::Type is very complex and overkill
        // Types in Rust involve generics, paths
        // etc., etc., etc. ...
        //
        // To make things simple, we just turn the syn::Type
        // to a token stream (`quote`) and turn that into a String
        // and turn that into an &str (`as_str`)
        if input.peek(syn::token::Bracket) {
            // This is an array type
            let content;
            syn::bracketed!(content in input);
            let t: Type = content.parse()?;
            let _: syn::Token![;] = content.parse()?;
            let count: Literal = content.parse()?;
            let count = match count {
                Literal::Int(i) => i as usize,
                _ => panic!("Expected usize"),
            };
            return Ok(Type::Array(Box::new(t), count));
        } else if input.peek(Token![&]) {
            let _: Token![&] = input.parse()?;
            let t: Type = input.parse()?;
            return Ok(Type::Ref(t.into()));
        }
        let t: syn::Type = input.parse()?;

        let ts = quote! {#t}.to_string();
        match ts.as_str() {
            "i32" => Ok(Type::I32),
            "bool" => Ok(Type::Bool),
            "usize" => Ok(Type::Usize),
            "()" => Ok(Type::Unit),
            "String" => Ok(Type::String),
            _ =>
            // to explicitly create an error at the current position
            {
                input.step(|cursor| Err(cursor.error("expected operator")))
            }
        }
    }
}
