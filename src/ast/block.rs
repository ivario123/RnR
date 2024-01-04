use crate::AstNode;

use super::Statement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub semi: bool,
}
impl AstNode for Block {}
