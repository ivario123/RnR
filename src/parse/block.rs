use super::{Block, Parse, ParseStream, Result, Statement, Token};
use syn::{parse::discouraged::Speculative, punctuated::Punctuated};
extern crate syn;

// Here we take advantage of the parser function `parse_terminated`
impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Block> {
        let content;
        let optional = input.fork();

        let _ = syn::braced!(content in optional);
        let bl: Punctuated<Statement, Token![;]> =
            content.parse_terminated(Statement::parse, Token![;])?;
        input.advance_to(&optional);

        // We need to retrieve the semi before we collect into a vector
        // as into_iter consumes the value.
        let semi = bl.trailing_punct();

        Ok(Block {
            // turn the Punctuated into a vector
            statements: bl.into_iter().collect(),
            semi,
        })
    }
}
