use super::{Arg, Func, FuncCall, Prog, Static};
use crate::ast::{BinaryOp, Block, Expr, Literal, Statement, Type, UnaryOp};
use std::fmt::{self};

enum KeyWords {
    Let,
    Fn,
    While,
    Static,
}

#[cfg(test)]
mod color_test {

    impl std::fmt::Display for super::KeyWords {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                super::KeyWords::Let => "let",
                super::KeyWords::Fn => "fn",
                super::KeyWords::While => "while",
                super::KeyWords::Static => "static",
            }
            .to_string();
            write!(f, "{}", s)
        }
    }
    pub fn identifier(id: &str) -> String {
        id.to_string()
    }
    pub fn fn_identifier(id: &str) -> String {
        id.to_string()
    }
    pub fn ty(id: String) -> String {
        id
    }
    pub fn lit(id: String) -> String {
        id
    }
}
#[cfg(not(test))]
mod color_normal {
    use ansi_term::Colour::Blue;
    use ansi_term::Colour::Cyan;
    use ansi_term::Colour::Purple;
    use ansi_term::Colour::Red;
    use ansi_term::Colour::Yellow;

    impl std::fmt::Display for super::KeyWords {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                super::KeyWords::Let => Purple.paint("let"),
                super::KeyWords::Fn => Purple.paint("fn"),
                super::KeyWords::While => Purple.paint("while"),
                super::KeyWords::Static => Purple.paint("static"),
            }
            .to_string();
            write!(f, "{}", s)
        }
    }

    pub fn identifier(id: &str) -> String {
        Cyan.paint(id).to_string()
    }
    pub fn fn_identifier(id: &str) -> String {
        Blue.paint(id).to_string()
    }
    pub fn ty(id: String) -> String {
        Yellow.paint(id).to_string()
    }
    pub fn lit(id: String) -> String {
        Red.paint(id).to_string()
    }
}
#[cfg(not(test))]
use color_normal::*;
#[cfg(test)]
use color_test::*;
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

impl InteralFormat for Static {
    fn fmt_internal(&self, indent: usize) -> String {
        format!(
            "{}{} {} {}:{}={};",
            " ".repeat(indent),
            KeyWords::Static,
            match self.mutable {
                true => ty("mut".to_string()).to_string(),
                _ => "".to_string(),
            },
            identifier(&self.id),
            ty(self.ty.to_string()),
            self.value
        )
    }
}

impl InteralFormat for Statement {
    fn fmt_internal(&self, indent: usize) -> String {
        let str = match self {
            Statement::Let(lhs, mutable, typ, rhs) => {
                format!(
                    "{} {}{}{}{}",
                    KeyWords::Let,
                    match mutable {
                        true => ty("mut ".to_owned()),
                        _ => "".to_owned(),
                    }
                    .to_owned(),
                    identifier(format!("{}", lhs).as_str()),
                    match typ {
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
                format!(
                    "{} {condition} {}",
                    KeyWords::While,
                    block.fmt_internal(indent + 1)
                )
            }
            Statement::Assign(lhs, rhs) => {
                format!("{lhs} = {}", rhs.fmt_internal(indent))
            }
            Statement::Block(b) => b.fmt_internal(indent + 1),
            Statement::FnDecleration(func) => func.fmt_internal(indent),
        };
        format!("{}{};", " ".repeat(indent), str)
    }
}

impl InteralFormat for Func {
    fn fmt_internal(&self, indent: usize) -> String {
        let id = match self.id.clone() {
            Expr::Ident(i) => i,
            _ => unreachable!(),
        };
        format!(
            "{} {}({}) -> {} {}",
            KeyWords::Fn,
            fn_identifier(id.as_str()),
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
            strings.push(statement.fmt_internal(indent + 4));
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
            Expr::Ident(a) => identifier(a),
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
fmt!(Prog, Block, Func, Expr, Statement, Static,);

impl fmt::Display for FuncCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = match *self.id.clone() {
            Expr::Ident(i) => i,
            _ => unreachable!(),
        };
        write!(
            f,
            "{}({})",
            fn_identifier(id.as_str()),
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
            identifier(format!("{}", self.id).as_str()),
            match self.mutable {
                true => ty("mut".to_owned()),
                false => "".to_owned(),
            },
            self.ty
        )
    }
}
impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnaryOp::Not => "!".to_string(),
            UnaryOp::Subtract => "- ".to_string(),
            UnaryOp::Borrow => "&".to_string(),
            UnaryOp::BorrowMut => format!("&{} ", ty("mut".to_string())),
            UnaryOp::Dereff => "*".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Literal::Bool(b) => lit(b.to_string()),
            Literal::Int(i) => lit(i.to_string()),
            Literal::Unit => lit("()".to_string()),
            Literal::Array(arr) => format!(
                "[{}]",
                arr.iter()
                    .map(|el| lit(format!("{}", *el).to_owned()))
                    .collect::<Vec<String>>()
                    .join(",")
            )
            .to_owned(),
            Literal::String(str) => lit(str.to_string()),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Type::I32 => ty("i32".to_owned()),
            Type::Bool => ty("bool".to_owned()),
            Type::Unit => ty("()".to_owned()),
            Type::Usize => ty("usize".to_owned()),
            Type::Array(typ, size) => format!("[{};{size}]", ty(typ.to_string())),
            Type::Ref(crate::ast::types::Ref(ty, _, _)) => format!("& {ty}"),
            Type::String => ty("String".to_string()),
            Type::MutRef(crate::ast::types::Ref(ty, _, _)) => format!("&mut {ty}"),
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
        assert_eq!(format!("{}", e), "if a > 5 {\n     5\n}");
    }
}
