use crate::ast::{BinaryOp, Expr, Literal, UnaryOp};
use std::convert::TryInto;

use super::climb_rec;

// As we seen, the simple parse implementation for Expr
// renders us a right associative representation of Expr
// and thus its evaluation does not adhere to the
// mathematical intuition of expressions.
//
// There are many ways to fix this problem, among these
// precedence climbing is a common approach, see
// https://en.wikipedia.org/wiki/Operator-precedence_parser
//
// parse_expression_1(lhs, min_precedence)
//     lookahead := peek next token
//     while lookahead is a binary operator whose precedence is >= min_precedence
//         op := lookahead
//         advance to next token
//         rhs := parse_primary ()
//         lookahead := peek next token
//         while lookahead is a binary operator whose precedence is greater
//                  than op's, or a right-associative operator
//                  whose precedence is equal to op's
//             rhs := parse_expression_1 (rhs, min_precedence + 1)
//             lookahead := peek next token
//         lhs := the result of applying op with operands lhs and rhs
//     return lhs
//
// In order to implement the algorithm for our Expr
// we first want to turn the right associated graph to
// to a flat vector of elements ExprItems.
#[derive(Debug)]
pub enum ExprItems {
    Op(BinaryOp),
    UnOp(UnaryOp),
    Lit(Literal),
    Par(Vec<ExprItems>),
    Ident(String),
    Array((Vec<ExprItems>, usize)),
}

impl<'a> TryInto<&'a BinaryOp> for &'a ExprItems {
    type Error = ();
    fn try_into(self) -> Result<&'a BinaryOp, Self::Error> {
        match self {
            ExprItems::Op(op) => Ok(op),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<&'a UnaryOp> for &'a ExprItems {
    type Error = ();
    fn try_into(self) -> Result<&'a UnaryOp, Self::Error> {
        match self {
            ExprItems::UnOp(op) => Ok(op),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<&'a Literal> for &'a ExprItems {
    type Error = ();
    fn try_into(self) -> Result<&'a Literal, Self::Error> {
        match self {
            ExprItems::Lit(lit) => Ok(lit),
            _ => Err(()),
        }
    }
}
impl TryInto<Expr> for &ExprItems {
    type Error = ();
    fn try_into(self) -> Result<Expr, Self::Error> {
        match self {
            ExprItems::Lit(lit) => Ok(Expr::Lit(lit.clone())),
            ExprItems::Par(exprs) => {
                // Borrowed ever so kindly from  Axel Johansson.
                let mut scanner = exprs.iter().rev().peekable();
                let lhs: Expr = super::expr(&mut scanner);
                Ok(Expr::Par(Box::new(climb_rec(lhs, 0, &mut scanner))))
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::ExprItems;
    use crate::ast::{BinaryOp, Literal, UnaryOp};

    use std::convert::TryInto;

    #[test]
    fn test_valid_into() {
        let _: &UnaryOp = (&ExprItems::UnOp(UnaryOp::Not)).try_into().unwrap();
        // No need to test all cases here, this covers all the added code.
        let _: &BinaryOp = (&ExprItems::Op(BinaryOp::Add)).try_into().unwrap();
        let _: &Literal = (&ExprItems::Lit(Literal::Int(1))).try_into().unwrap();
    }
    #[test]
    #[should_panic]
    fn test_invalid_into_literal() {
        let _: &Literal = (&ExprItems::Op(BinaryOp::Add)).try_into().unwrap();
    }
    #[test]
    #[should_panic]
    fn test_invalid_into_unop() {
        let _: &UnaryOp = (&ExprItems::Lit(Literal::Int(1))).try_into().unwrap();
    }
    #[test]
    #[should_panic]
    fn test_invalid_into_binop() {
        let _: &BinaryOp = (&ExprItems::UnOp(UnaryOp::Not)).try_into().unwrap();
    }
}
