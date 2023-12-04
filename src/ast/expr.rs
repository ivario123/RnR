use super::{BinaryOp, Block, FuncCall, Literal, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Identifier
    ///
    /// Represents fetching a variables value from memory and inserting the current
    /// value in its place.
    Ident(String),
    /// Representation of a value known at compiletime
    ///
    /// ```rust
    /// // RHS is a literal
    /// let a = 2;
    /// ```
    Lit(Literal),
    /// Represents a [`binary operation`](BinaryOp)
    ///
    /// ```rust
    /// let a:u32 = 2;
    /// let b:u32 = 3;
    ///
    /// // RHS here is a binary operation
    /// let c = a+b;
    ///
    /// ```
    BinOp(BinaryOp, Box<Expr>, Box<Expr>),
    /// Represents a [`unary operation`](UnaryOp)
    ///
    /// ```rust
    /// let a = !false;
    /// ```
    UnOp(UnaryOp, Box<Expr>),
    /// Represents an expression wrapped in parentheses
    ///
    /// ```rust
    /// let a = (2+2);
    /// ```
    Par(Box<Expr>),
    /// Represents an if statement
    ///
    /// Either
    /// ```rust
    /// if 0 == 0{
    ///     println!("Some things");
    /// };
    /// ```
    /// or with an else statement
    /// ```rust
    /// if 0 == 0{
    ///     println!("Some things");
    /// }else{
    ///     println!("Some other things");
    /// }
    /// ```
    /// Also supports nesting
    ///
    /// ```rust
    /// if 0 == 0{
    ///     println!("Some things");
    /// }else if 1 == 0{
    ///     println!("This should not happen");
    /// }else {
    ///     println!("Nor should this");
    /// }
    /// ```
    ///
    IfThenElse(Box<Expr>, Block, Option<Block>),
    /// Used when declaring arrays
    /// ```rust
    /// let a:[u8;3] = [1,2,3];
    /// ```
    Array(Vec<Box<Expr>>),
    /// Returns a immutable reference to the
    /// data contained in the array at the
    /// specified index
    Index(Box<Expr>, Box<Expr>),
    /// Returns a mutable reference to the data
    /// contained in the array at the specified index
    IndexMut(Box<Expr>, Box<Expr>),
    /// Runs the given function with the given arguments
    FuncCall(FuncCall),
}

impl Expr {
    pub fn bin_op(o: BinaryOp, left: Expr, right: Expr) -> Self {
        Expr::BinOp(o, Box::new(left), Box::new(right))
    }
}

impl From<Literal> for Expr {
    fn from(lit: Literal) -> Self {
        Expr::Lit(lit)
    }
}

impl From<i32> for Expr {
    fn from(i: i32) -> Self {
        Expr::Lit(Literal::Int(i))
    }
}
