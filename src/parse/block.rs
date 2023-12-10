use super::{Block, Parse, ParseStream, Result, Statement, Token};
use syn::punctuated::Punctuated;

// Here we take advantage of the parser function `parse_terminated`
impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Block> {
        let content;
        let _ = syn::braced!(content in input);
        println!("content : {:?}", content);
        let bl: Punctuated<Statement, Token![;]> =
            content.parse_terminated(Statement::parse, Token![;])?;
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
