use super::{Arg, Func, FuncCall, Prog};
use crate::ast::{BinaryOp, Block, Expr, Literal, Statement, Type, UnaryOp};

use std::fmt::{self};

mod sealed {

    pub trait InteralFormat {
        fn fmt_internal(&self, indent: usize) -> String;
    }
}
use sealed::*;

macro_rules! fmt {
    ($($id:ident,)+) => {
        $(
            impl std::fmt::Display for $id {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", self.fmt_internal(0))
                }
            }
        )+
    };
}
impl InteralFormat for Prog {
    fn fmt_internal(&self, _indent: usize) -> String {
        self.statements
            .iter()
            .map(|el| format!("{el}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl InteralFormat for Statement {
    fn fmt_internal(&self, indent: usize) -> String {
        let str = match self {
            Statement::Let(lhs, mutable, ty, rhs) => {
                format!(
                    "let {}{}{}{}",
                    match mutable {
                        true => "mut ",
                        _ => "",
                    }
                    .to_owned(),
                    lhs,
                    match ty {
                        Some(ty) => format!(" : {ty}"),
                        _ => "".to_owned(),
                    },
                    match rhs {
                        Some(rhs) => format!(" = {}", rhs.fmt_internal(indent + 1)),

                        _ => "".to_owned(),
                    }
                )
            }

            Statement::Expr(expr) => expr.fmt_internal(indent).to_string(),
            Statement::While(condition, block) => {
                format!("while {condition} {}", block.fmt_internal(indent + 1))
            }
            Statement::Assign(lhs, rhs) => {
                format!("{lhs} = {}", rhs.fmt_internal(indent))
            }
            Statement::Block(b) => b.fmt_internal(indent),
            Statement::FnDecleration(func) => func.fmt_internal(indent + 1),
        };
        format!("{}{};", " ".repeat(indent), str)
    }
}

impl InteralFormat for Func {
    fn fmt_internal(&self, indent: usize) -> String {
        format!(
            "{}fn {}({}) -> {} {}",
            " ".repeat(indent),
            self.id,
            self.args
                .iter()
                .map(|el| format!("{el}"))
                .collect::<Vec<String>>()
                .join(","),
            self.ty,
            self.body.fmt_internal(indent + 1)
        )
    }
}

impl InteralFormat for Block {
    fn fmt_internal(&self, indent: usize) -> String {
        let mut strings = vec![];
        for statement in &self.statements {
            strings.push(statement.fmt_internal(indent + 1));
        }
        if !self.semi && !strings.is_empty() {
            let mut last: String = strings.pop().unwrap();
            last.pop();
            strings.push(last);
        }
        let indent = match indent {
            0 => 0,
            i => i - 1,
        };
        format!("{{\n{}\n{}}}", strings.join("\n"), " ".repeat(indent),)
    }
}
impl InteralFormat for Expr {
    fn fmt_internal(&self, indent: usize) -> String {
        match self {
            Expr::Ident(a) => a.to_owned(),
            Expr::Lit(l) => format!("{}", l),
            Expr::BinOp(op, l, r) => format!("{} {} {}", l, op, r.fmt_internal(indent)),
            Expr::UnOp(op, operand) => format!("{}{}", op, operand.fmt_internal(indent)),
            Expr::Par(e) => format!("({})", e.fmt_internal(indent)),
            Expr::IfThenElse(req, block, base_case) => match base_case {
                Some(other_block) => {
                    format!(
                        "if {} {} else {}",
                        req,
                        block.fmt_internal(indent + 1),
                        other_block.fmt_internal(indent + 1)
                    )
                }
                _ => {
                    format!("if {} {}", req, block.fmt_internal(indent + 1))
                }
            },
            Expr::Array(elements) => {
                let strs = elements
                    .iter()
                    .map(|el| format!("{}", el))
                    .collect::<Vec<String>>()
                    .join(",");
                format!("[{strs}]")
            }
            Expr::Index(id, idx) => format!("{id}[{idx}]"),
            Expr::IndexMut(id, idx) => format!("{id}[{idx}]"),
            Expr::FuncCall(func) => format!("{func}"),
            Expr::Block(block) => block.fmt_internal(indent),
        }
    }
}
impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::Eq => "==",
            BinaryOp::Lt => "<",
            BinaryOp::Gt => ">",
        };
        write!(f, "{}", s)
    }
}
fmt!(Prog, Block, Func, Expr, Statement,);

impl fmt::Display for FuncCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.id,
            self.args
                .iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>()
                .join(","),
        )
    }
}
impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} {}",
            self.id,
            match self.mutable {
                true => "mut",
                false => "",
            },
            self.ty
        )
    }
}
impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnaryOp::Not => "!",
            UnaryOp::Subtract => "- ",
            UnaryOp::Borrow => "&",
            UnaryOp::BorrowMut => "&mut ",
            UnaryOp::Dereff => "*",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Literal::Bool(b) => b.to_string(),
            Literal::Int(i) => i.to_string(),
            Literal::Unit => "()".to_string(),
            Literal::Array(arr) => format!(
                "[{}]",
                arr.iter()
                    .map(|el| format!("{}", *el).to_owned())
                    .collect::<Vec<String>>()
                    .join(",")
            )
            .to_owned(),
            Literal::String(str) => str.to_string(),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Type::I32 => "i32".to_owned(),
            Type::Bool => "bool".to_owned(),
            Type::Unit => "()".to_owned(),
            Type::Usize => "usize".to_owned(),
            Type::Array(ty, size) => format!("[{ty};{size}]"),
            Type::Ref(crate::ast::types::Ref(ty)) => format!("& {ty}"),
            Type::String => "String".to_string(),
        };
        write!(f, "{}", s)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn display_type() {
        assert_eq!(format!("{}", Type::I32), "i32");
        assert_eq!(format!("{}", Type::Bool), "bool");
        assert_eq!(format!("{}", Type::Unit), "()");
    }

    #[test]
    fn display_literal() {
        println!("{}", Literal::Int(3));
        println!("{}", Literal::Bool(false));
        println!("{}", Literal::Unit);
        assert_eq!(format!("{}", Literal::Int(3)), "3");
        assert_eq!(format!("{}", Literal::Bool(false)), "false");
        assert_eq!(format!("{}", Literal::Unit), "()");
    }

    #[test]
    fn display_expr() {
        println!("{}", Expr::Ident("a".to_string()));
        println!("{}", Expr::Lit(Literal::Int(7)));
        println!("{}", Expr::Lit(Literal::Bool(false)));
        let e = Expr::BinOp(
            BinaryOp::Add,
            Box::new(Expr::Ident("a".to_string())),
            Box::new(Expr::Lit(Literal::Int(7))),
        );
        println!("{}", e);
        assert_eq!(format!("{}", e), "a + 7");
    }

    // As you see it becomes cumbersome to write tests
    // if you have to construct the Expr by hand.
    //
    // Instead we might use our parser

    #[test]
    fn parse_display_expr() {
        let ts: proc_macro2::TokenStream = "a + 7".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {}", e);
        assert_eq!(format!("{}", e), "a + 7");
    }

    // This one will fail (Display for `if` is not yet implemented).
    // Implement it as an optional assignment
    //
    // Hint: You need to implement Display for Statement and Block

    #[test]
    fn parse_display_if() {
        let ts: proc_macro2::TokenStream = "if a > 5 { 5}".parse().unwrap();
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {}", e);
        assert_eq!(format!("{}", e), "if a > 5 {\n  5\n}");
    }
}
