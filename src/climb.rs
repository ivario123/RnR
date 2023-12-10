pub mod expritem;

use crate::ast::{
    BinaryOp,
    Expr::{self, *},
    UnaryOp,
};
use expritem::ExprItems;

use std::{
    convert::TryInto,
    iter::{Peekable, Rev},
    panic,
};

// A Peekable iterator allows allows to peek next item
// Rev reverses the scanning order
// std::slice::Iter is the iterator implementation for Vec
type Scanner<'a> = Peekable<Rev<std::slice::Iter<'a, ExprItems>>>;

impl BinaryOp {
    // The operator priority
    // Mul/Div has higher priority (binds harder) than Add/Sub
    pub fn priority(&self) -> u8 {
        match self {
            BinaryOp::Add => 0,
            BinaryOp::Sub => 0,
            BinaryOp::Mul => 1,
            BinaryOp::Div => 1,
            BinaryOp::And => 1,
            BinaryOp::Or => 0,
            BinaryOp::Eq => 2,
            BinaryOp::Lt => 2,
            BinaryOp::Gt => 2,
        }
    }
}
impl UnaryOp {
    pub fn priority(&self) -> u8 {
        match self {
            UnaryOp::Not => 3,
            UnaryOp::Subtract => 3,
            UnaryOp::Borrow => 4,
        }
    }
}

// Flattens an Expr into a vector of ExprItems
fn to_vec(e: Expr) -> Vec<ExprItems> {
    match e {
        Lit(l) => vec![ExprItems::Lit(l)],
        BinOp(op, l, r) => {
            let mut r = to_vec(*r);
            match *l {
                Lit(l) => {
                    r.push(ExprItems::Op(op));
                    r.push(ExprItems::Lit(l));
                    r
                }
                // should never occur due to the Expr structure
                Par(block) => {
                    let mut ret = to_vec(*block);
                    ret.push(ExprItems::Op(op));
                    ret.extend(r);
                    ret
                }
                _ => unreachable!(),
            }
        }
        UnOp(op, operand) => {
            let mut ret = to_vec(*operand);
            ret.push(ExprItems::UnOp(op));
            ret
        }
        Par(block) => vec![ExprItems::Par(to_vec(*block))],
        // not yet implemented
        _ => unimplemented!(),
    }
}

fn peek_precedence<F>(scanner: &mut Scanner, f: F) -> bool
where
    F: Fn(u8) -> bool,
{
    match scanner.peek() {
        Some(ExprItems::Op(op)) => f(op.priority()),
        Some(ExprItems::UnOp(op)) => f(op.priority()),
        _ => false,
    }
}

fn climb_rec(mut lhs: Expr, min_precedence: u8, scanner: &mut Scanner) -> Expr {
    while peek_precedence(scanner, |op_precedence| op_precedence >= min_precedence) {
        // op := lookahead
        let expr_item = scanner.next().unwrap();
        let bin_op: Result<&BinaryOp, ()> = expr_item.try_into();

        if let Ok(op) = bin_op {
            let mut rhs = expr(scanner);

            // advance to next token
            // rhs := parse_primary ()
            // while lookahead is a binary operator whose precedence is greater
            //                  than op's, or a right-associative operator
            //                  whose precedence is equal to op's
            while peek_precedence(scanner, |op_precedence| op_precedence > op.priority()) {
                // rhs := parse_expression_1 (rhs, min_precedence + 1)
                rhs = climb_rec(rhs, min_precedence + 1, scanner);
                // lookahead := peek next token
                // scanner will be updated since we passed it recursively
            }
            // lhs := the result of applying op with operands lhs and rhs
            lhs = BinOp(*op, Box::new(lhs), Box::new(rhs));
            continue;
        }
        continue;
    }
    lhs
}
// gets next whole expression from the scanner
fn expr(scanner: &mut Scanner) -> Expr {
    let rhs = match scanner.next() {
        Some(expr) => expr.to_owned(),
        _ => panic!("Invalid syntax expected one more literal"),
    };
    println!("{:?}", rhs);
    let expr: Expr = {
        let op: Result<&UnaryOp, ()> = rhs.try_into();
        match op {
            Ok(op) => {
                // This is the only real edgecase here.
                // Now we gotta include the result of the next expr extraction to the right of this.
                let rhs = expr(scanner);
                Expr::UnOp(*op, Box::new(rhs))
            }
            Err(_) => match rhs.try_into() {
                Ok(expr) => expr,
                Err(_) => {
                    panic!("Cannot convert {:?} into an expression", rhs);
                }
            },
        }
    };

    // This modification is a bit strange, since the entire tree is reversed
    // we need to manage the change being rhs ! lhs,
    // the and since the rhs has already been peeked we get
    // ! false | true so we peek the
    // true first, then we peek the or and then we peek the false
    // after that we peek the !. This is a problem so the hacky workaround
    // is that any UnaryOp is checked in any state, i.e. when managing a
    // expr we peek the next token and if that is a unary op we move that
    // op and manage it before the literal

    match scanner.peek() {
        Some(&syntax_element) => {
            let operator: Result<&UnaryOp, ()> = syntax_element.try_into();
            if let Ok(op) = operator {
                // In this case the expr must be a literal
                Expr::UnOp(*op, Box::new(expr))
            } else {
                expr
            }
        }
        _ => expr,
    }
}

// A trampoline to start of the precedence climbing
pub fn climb(e: Expr) -> Expr {
    // flatten the Expr into a vector
    let v: Vec<ExprItems> = to_vec(e);
    // turn the vector into a Scanner
    let mut scanner = v.iter().rev().peekable();
    // This is problematic when working with unops.
    // The first token does not need to be a literal it can be an operator on a literal

    let lhs: Expr = expr(&mut scanner);
    // now call the climber
    climb_rec(lhs, 0, &mut scanner)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{Expr, Literal};
    use crate::vm::{Eval, VarEnv};
    #[test]
    fn climb_test1() {
        let ts: proc_macro2::TokenStream = "2 - 3 - 4 - 5".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {:?}", e);
        let e = climb(e);
        println!("e {:?}", e);
        let o = e.eval(&mut VarEnv::new(), 0).unwrap();
        println!("evaluation {:?}", o);
        assert_eq!(o, Literal::Int(2 - 3 - 4 - 5));
    }

    #[test]
    fn climb_test2() {
        let ts: proc_macro2::TokenStream = "2 - 3 * 4 - 5".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {:?}", e);
        let e = climb(e);
        println!("e {:?}", e);
        let o = e.eval(&mut VarEnv::new(), 0).unwrap();
        println!("evaluation {:?}", o);
        assert_eq!(o, Literal::Int(2 - 3 * 4 - 5));
    }

    #[test]
    fn climb_test3() {
        let ts: proc_macro2::TokenStream = "4 - 5 - 2 * 8 * 3 - 1 - 2 * 5".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {:?}", e);
        let e = climb(e);
        println!("e {:?}", e);
        let o = e.eval(&mut VarEnv::new(), 0).unwrap();
        println!("evaluation {:?}", o);
        assert_eq!(o, Literal::Int(4 - 5 - 2 * 8 * 3 - 1 - 2 * 5));
    }

    #[test]
    fn climb_test4() {
        let ts: proc_macro2::TokenStream = "8 - 7 - 6 * 5 - 4 * 3".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {:?}", e);
        let e = climb(e);
        println!("e {:?}", e);
        let o = e.eval(&mut VarEnv::new(), 0).unwrap();
        println!("evaluation {:?}", o);
        assert_eq!(o, Literal::Int(8 - 7 - 6 * 5 - 4 * 3));
    }
    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn climb_test_not() {
        let ts: proc_macro2::TokenStream = "true && !false".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {:?}", e);
        let e = climb(e);
        println!("e {:?}", e);
        let o = e.eval(&mut VarEnv::new(), 0).unwrap();
        println!("evaluation {:?}", o);
        assert_eq!(o, Literal::Bool(true && !false));
    }
    #[test]
    fn climb_test_minus() {
        let ts: proc_macro2::TokenStream = "1 - -a".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        assert_eq!(
            e,
            Expr::BinOp(
                BinaryOp::Sub,
                Box::new(Expr::Lit(Literal::Int(1))),
                Box::new(Expr::UnOp(
                    UnaryOp::Subtract,
                    Box::new(Expr::Ident("a".to_owned()))
                ))
            )
        );
    }
}
