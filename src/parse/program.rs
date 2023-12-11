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
        // Sort the items to minimize risk of re evalution being required.
        //
        // Things like
        //
        //
        // pub static a:i32 = 0;
        // pub static b:i32 = c + 2;
        // pub static c:i32 = 2;
        //
        // are valid in the rustc compiler, so our language should reflect this.
        //
        statements.sort_by(|el1, el2| order(el1, el2));
        Ok(statements.into())
    }
}
