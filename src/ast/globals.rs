use crate::AstNode;

use super::{Expr, Prio, TopLevel, Type};

#[derive(Debug)]
pub struct Static {
    pub(crate) ty: Type,
    pub(crate) mutable: bool,
    pub(crate) value: Expr,
    pub(crate) id: String,
}
impl Prio for Static {
    fn prio(&self) -> usize {
        1
    }
}
impl TopLevel for Static {
    fn is_main(&self) -> bool {
        false
    }
}

impl AstNode for Static {}
