use super::{BinaryOp, Parse, ParseStream, Result, Token, UnaryOp};

impl super::Peek for BinaryOp {
    fn peek<const DIST: usize>(input: ParseStream) -> bool {
        use syn::token;
        // check if next token is `+`
        Self::peek_buffer(input, token::Plus, DIST)
            || Self::peek_buffer(input, token::Minus, DIST)
            || Self::peek_buffer(input, token::Star, DIST)
            || Self::peek_buffer(input, token::Slash, DIST)
            || Self::peek_buffer(input, token::AndAnd, DIST)
            || Self::peek_buffer(input, token::OrOr, DIST)
            || Self::peek_buffer(input, Token![==], DIST)
            || Self::peek_buffer(input, Token![>], DIST)
            || Self::peek_buffer(input, Token![<], DIST)
    }
}
impl Parse for BinaryOp {
    fn parse(input: ParseStream) -> Result<Self> {
        // check if next token is `+`
        if input.peek(Token![+]) {
            // consume the token
            let _: Token![+] = input.parse()?;
            Ok(BinaryOp::Add)
        } else if input.peek(Token![-]) {
            let _: Token![-] = input.parse()?;
            Ok(BinaryOp::Sub)
        } else if input.peek(Token![*]) {
            let _: Token![*] = input.parse()?;
            Ok(BinaryOp::Mul)
        } else if input.peek(Token![/]) {
            let _: Token![/] = input.parse()?;
            Ok(BinaryOp::Div)
        } else if input.peek(Token![&&]) {
            let _: Token![&&] = input.parse()?;
            Ok(BinaryOp::And)
        } else if input.peek(Token![||]) {
            let _: Token![||] = input.parse()?;
            Ok(BinaryOp::Or)
        } else if input.peek(Token![==]) {
            let _: Token![==] = input.parse()?;
            Ok(BinaryOp::Eq)
        } else if input.peek(Token![>]) {
            let _: Token![>] = input.parse()?;
            Ok(BinaryOp::Gt)
        } else if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;
            Ok(BinaryOp::Lt)
        } else {
            // to explicitly create an error at the current position
            input.step(|cursor| Err(cursor.error("expected operator")))
        }
    }
}

impl Parse for UnaryOp {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![!]) {
            let _: Token![!] = input.parse()?;
            Ok(UnaryOp::Not)
        } else if input.peek(Token![-]) {
            let _: Token![-] = input.parse()?;
            Ok(UnaryOp::Subtract)
        } else if input.peek(Token![&]) {
            let _: Token![&] = input.parse()?;
            if input.peek(Token![mut]) {
                let _: Token![mut] = input.parse()?;
                Ok(UnaryOp::BorrowMut)
            } else {
                Ok(UnaryOp::Borrow)
            }
        } else if input.peek(Token![*]) {
            Ok(UnaryOp::Dereff)
        } else {
            // to explicitly create an error at the current position
            input.step(|cursor| Err(cursor.error("expected operator")))
        }
    }
}
