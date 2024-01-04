use crate::AstNode;

use super::{block::Block, expr::Expr, types::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Let(Expr, bool, Option<Type>, Option<Expr>),
    Assign(Expr, Expr),
    While(Expr, Block),
    Expr(Expr),
    Block(Block),
    FnDecleration(super::Func),
}
impl AstNode for Statement {}
