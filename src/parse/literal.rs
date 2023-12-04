use super::{Literal, Parse, ParseStream, Result};

impl Parse for Literal {
    fn parse(input: ParseStream) -> Result<Self> {
        // Use the "built in" syn parser for literals
        let l: syn::Lit = input.parse()?;

        Ok(match l {
            syn::Lit::Int(l) => Literal::Int(l.base10_parse().unwrap()),
            syn::Lit::Bool(b) => Literal::Bool(b.value),
            // for now only Int and Bool are covered
            _ => unimplemented!(),
        })
    }
}
