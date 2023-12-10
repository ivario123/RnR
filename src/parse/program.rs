use crate::ast::{program::*, Func};

use syn::parse::Parse;

impl Parse for Prog {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // This is a cool top level thingie, this means that at the end of parsing
        // the input stream should be empty.
        let mut statements: Vec<Box<dyn TopLevel>> = vec![];
        // The things we have that we can parse here are all statements.
        while !input.is_empty() {
            let stmt: Func = input.parse()?;
            statements.push(Box::new(stmt));
        }
        Ok(statements.into())
    }
}
