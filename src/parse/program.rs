use crate::ast::{program::*, Func, Static};

use syn::{parse::Parse, Token};

impl Parse for Prog {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // This is a cool top level thingie, this means that at the end of parsing
        // the input stream should be empty.
        let mut statements: Vec<Box<dyn TopLevel>> = vec![];
        // The things we have that we can parse here are all statements.
        while !input.is_empty() {
            if input.peek(Token![fn]) {
                let stmt: Func = input.parse()?;
                statements.push(Box::new(stmt));
            } else {
                let stmt: Static = input.parse()?;
                statements.push(Box::new(stmt));
            }
        }

        let mut main_defined = false;
        for el in statements.iter() {
            main_defined = main_defined || el.is_main();
        }
        if !main_defined {
            return Err(input.error("Failed to parse something".to_string()));
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
        statements.sort_by(|el1, el2| order(&**el1, &**el2));
        Ok(statements.into())
    }
}
