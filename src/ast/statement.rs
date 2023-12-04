use super::{block::Block, expr::Expr, types::Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(Expr, bool, Option<Type>, Option<Expr>),
    Assign(Expr, Expr),
    While(Expr, Block),
    Expr(Expr),
    Block(Block),
    FnDecleration(super::Func),
}
