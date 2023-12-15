use syn::{parse::Parse, Token};

use crate::ast::globals::Static;
impl Parse for Static {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if !input.peek(Token![static]) {
            return Err(input.error("Expected static"));
        }
        let _: Token![static] = input.parse().unwrap();

        let mutable = match input.peek(Token![mut]) {
            true => {
                let _: Token![mut] = input.parse().unwrap();
                true
            }
            f => f,
        };

        if !input.peek(syn::Ident) {
            return Err(input.error("Expected identifier"));
        }
        let ident: syn::Ident = input.parse().unwrap();

        if !input.peek(Token![:]) {
            return Err(input.error("Expected type"));
        }
        let _: Token![:] = input.parse().unwrap();
        let ty: crate::ast::Type = input.parse()?;

        if !input.peek(Token![=]) {
            return Err(input.error("Expected equal sign"));
        }
        let _: Token![=] = input.parse().unwrap();

        let value = input.parse()?;
        // Global constants are
        let _: Token![;] = input.parse()?;

        Ok(Self {
            id: ident.to_string(),
            ty,
            mutable,
            value,
        })
    }
}
