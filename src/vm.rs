pub mod block;
pub mod expr;
pub mod func;
pub mod op;
pub mod program;
pub mod statement;

use std::collections::HashMap;

use crate::ast::{
    op::BinaryOp,
    Block,
    Expr::{self},
    Func, Literal,
};

#[derive(Debug)]
pub enum VmErr {
    Err(String),
    Handled(String),
}
impl std::fmt::Display for VmErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmErr::Err(e) => write!(f, "{}", e),
            VmErr::Handled(e) => write!(f, "{}", e),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Values {
    Lit(Literal),
    Ref((String, usize)),
}

impl Values {
    pub fn lit(self) -> Literal {
        match self {
            Values::Lit(l) => l,
            _ => panic!(),
        }
    }
}
impl std::fmt::Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Values::Lit(l) => l.to_string(),
            Values::Ref((id, _)) => format!("&{id}"),
        };
        write!(f, "{}", s)
    }
}
/// Describes all of the needed data for a value.
#[derive(Debug, Clone)]
pub struct ValueMeta {
    value: Option<Values>,
}
#[derive(Debug, Clone)]
pub struct FunctionMeta {
    /// The variable scope, this should include
    /// all arguments and their type info
    ///
    /// in the vm all type information is discarded
    args: Vec<String>,
    body: Block,
}
impl From<Func> for FunctionMeta {
    fn from(value: Func) -> Self {
        Self {
            args: value
                .args
                .iter()
                .map(|el| match &el.id {
                    Expr::Ident(i) => i.clone(),
                    e => panic!("Cannot treat {e} as an expression"),
                })
                .collect(),
            body: value.body,
        }
    }
}
/// Represents the functions accessible in the current scope
pub type FunctionScope = HashMap<String, FunctionMeta>;

/// Represents a specific scope.
/// For example a block has it's own scope.
pub type Scope = HashMap<String, ValueMeta>;
/// Represents all program [`Scope`]s
pub type VarEnv = Vec<(Scope, FunctionScope)>;

pub trait Eval {
    fn eval(
        &self,
        env: &mut VarEnv,
        scope: usize,
        max_iter: usize,
        iter_counter: &mut usize,
    ) -> Result<Values, VmErr>;
}

impl Literal {
    pub fn get_int(&self) -> Result<i32, VmErr> {
        match self {
            Literal::Int(i) => Ok(*i),
            _ => Err(VmErr::Err(format!("cannot get integer from {:?}", self))),
        }
    }

    pub fn get_bool(&self) -> Result<bool, VmErr> {
        match self {
            Literal::Bool(b) => Ok(*b),
            _ => Err(VmErr::Err(format!("cannot get Bool from {:?}", self))),
        }
    }
}

impl BinaryOp {
    // Evaluate operator to literal
    pub fn eval(&self, left: Values, right: Values) -> Result<Values, VmErr> {
        use BinaryOp::*;
        use Literal::{Bool, Int};
        let (left, right) = match (left, right) {
            (Values::Lit(left), Values::Lit(right)) => (left, right),
            (l, r) => {
                return Err(VmErr::Err(format!(
                    "Cannot peform operations on refferences. {l:?} and {r:?} should be literals"
                )))
            }
        };
        Ok(Values::Lit(match self {
            Add => Int(left.get_int()? + right.get_int()?),
            Sub => Int(left.get_int()?) - Int(right.get_int()?),
            Mul => Int(left.get_int()? * right.get_int()?),
            Div => Int(left.get_int()? / right.get_int()?),
            And => Bool(left.get_bool()? && right.get_bool()?),
            Or => Bool(left.get_bool()? || right.get_bool()?),
            Eq => Bool(left == right), // overloading
            Lt => Bool(left.get_int()? < right.get_int()?),
            Gt => Bool(left.get_int()? > right.get_int()?),
        }))
    }
}

impl Expr {
    pub fn get_id(&self) -> Result<String, VmErr> {
        match self {
            Expr::Ident(s) => Ok(s.to_owned()),
            _ => Err(VmErr::Err(format!("cannot get id from {:?}", self))),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::ast::Block;
    #[test]
    fn test_check_block1() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        a = a + 1;
        a
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0, 100, &mut 0).unwrap();
        println!("l {:?}", l);
        assert_eq!(l.lit().get_int().unwrap(), 4);
    }

    #[test]
    fn test_check_if_then_else() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        if false {
            a = a + 1
        } else {
            a = a - 1
        };
        if true {
            a = a + 3
        };
        a
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0, 100, &mut 0).unwrap();
        println!("l {:?}", l);
        assert_eq!(l.lit().get_int().unwrap(), 5);
    }

    #[test]
    fn test_check_if_then_else_shadowing() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        if true {
            let a: i32 = 0;
            a = a + 1
        } else {
            a = a - 1
        };
        a
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0, 100, &mut 0).unwrap();
        println!("l {:?}", l);
        // notice this will fail
        assert_eq!(l.lit().get_int().unwrap(), 3);
    }

    #[test]
    fn test_check_while() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        let b: i32 = 0;
        while a > 0 {
            a = a - 1;
            b = b + 1;
        };
        b
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0, 100, &mut 0).unwrap();
        println!("l {:?}", l);
        assert_eq!(l.lit().get_int().unwrap(), 3);
    }

    #[test]
    fn test_check_if() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        let b: i32 = 0;
        if a > 0 { b = 1 };
        b
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0, 100, &mut 0).unwrap();
        println!("l {:?}", l);
        assert_eq!(l.lit().get_int().unwrap(), 1);
    }

    #[test]
    fn test_check_if_else() {
        let ts: proc_macro2::TokenStream = "
    {
        let a: i32 = 1 + 2;
        let b: i32 = 0;
        if a < 1 { b = 1 } else { b = 2 };
        b
    }
    "
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        let l = bl.eval(&mut VarEnv::new(), 0, 100, &mut 0).unwrap();
        println!("l {:?}", l);
        assert_eq!(l.lit().get_int().unwrap(), 2);
    }
}
