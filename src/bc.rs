use crate::ast::{Block, Expr, FnDeclaration, Prog};
use crate::common::Eval;
use crate::env::{Env, Ref};
use crate::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Loan {
    Unique(Ref),
    Shared(Ref),
}

type Stack = Vec<Loan>;
#[derive(Debug, Clone, PartialEq)]
pub struct Loans(Stack);

impl Loans {
    //
}

// Borrow check
#[derive(Debug, Clone, PartialEq)]
pub enum Bc {
    Lit(Loans),
    Ref(bool, Ref),
}

impl Eval<Bc> for Expr {
    fn eval(&self, env: &mut Env<Bc>) -> Result<(Bc, Option<Ref>), Error> {
        todo!("not implemented {:?}", self)
    }
}

impl Eval<Bc> for Block {
    fn eval(&self, env: &mut Env<Bc>) -> Result<(Bc, Option<Ref>), Error> {
        todo!("not implemented {:?}", self)
    }
}

impl Eval<Bc> for FnDeclaration {
    fn eval(&self, env: &mut Env<Bc>) -> Result<(Bc, Option<Ref>), Error> {
        todo!("not implemented {:?}", self)
    }
}
impl Eval<Bc> for Prog {
    fn eval(&self, env: &mut Env<Bc>) -> Result<(Bc, Option<Ref>), Error> {
        todo!("not implemented {:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::Bc;
    use crate::ast::{Block, Prog};
    use crate::common::parse_test;

    // Tests for the Bc specific handling
    // of Loans etc.
    #[test]
    fn loan() {}

    // Tests for borrow checking of Block
    #[test]
    fn test_block_let() {
        let _v = parse_test::<Block, Bc>(
            "
        {
            let a: i32 = 1;
            let b: i32 = 2;

            a + b
        }",
        );
        // Suitable assertion
    }

    // Come up with your own set of tests for Block

    // Tests for borrow checking of Prog
    #[test]
    fn test_prog_fn_sig() {
        let _v = parse_test::<Prog, Bc>(
            "
        fn main() {
            let mut a = 0;
            let b = &mut a;
            let c = &a;
            *b = 4;
            let d = *c; // <- error here, with stacked borrows
        }
            ",
        );
        // suitable assertion
    }

    // Come up with your own set of test for Prog
}
