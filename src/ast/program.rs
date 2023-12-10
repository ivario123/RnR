use std::fmt::Display;

pub trait Prio {
    fn prio(&self) -> usize
    where
        Self: Sized,
    {
        0
    }
}

pub trait TopLevel:
    crate::vm::Eval + crate::type_check::TypeCheck<ReturnType = crate::ast::Type> + Display + Prio
{
}

impl Prio for super::Func {
    fn prio(&self) -> usize
    where
        Self: Sized,
    {
        2
    }
}
impl TopLevel for super::Func {}

impl std::fmt::Debug for dyn TopLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Prog {
    pub statements: Vec<Box<dyn TopLevel>>,
}

impl From<Vec<Box<dyn TopLevel>>> for Prog {
    fn from(value: Vec<Box<dyn TopLevel>>) -> Self {
        Self { statements: value }
    }
}
