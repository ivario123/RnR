#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    Bool,
    String,
    Unit,
    Ref(Box<Type>),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mutable(pub bool);

#[derive(Debug, Clone, PartialEq)]
pub struct Parameters(pub Vec<Parameter>);

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub mutable: Mutable,
    pub id: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclaration {
    pub id: String,
    pub parameters: Parameters,
    pub ty: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prog(pub Vec<FnDeclaration>);

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(Mutable, String, Option<Type>, Option<Expr>),
    Assign(Expr, Expr),
    While(Expr, Block),
    Expr(Expr),
    Fn(FnDeclaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub semi: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arguments(pub Vec<Expr>);

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Ref,
    DeRef,
    Mut,
    Bang,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(String),
    Lit(Literal),
    BinOp(Op, Box<Expr>, Box<Expr>),
    Par(Box<Expr>),
    Call(String, Arguments),
    IfThenElse(Box<Expr>, Block, Option<Block>),
    Block(Block),
    UnOp(UnOp, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Bool(bool),
    Int(i32),
    String(String),
    Unit,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Lt,
    Gt,
}
