#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub id: super::Expr,
    pub ty: super::Type,
    pub mutable: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    pub id: super::Expr,
    pub ty: super::Type,
    /// A vector of argument identifiers
    pub args: Vec<Arg>,
    pub body: super::Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncCall {
    pub id: Box<super::Expr>,
    pub args: Box<Vec<super::Expr>>,
}
