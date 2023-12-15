use std::fmt::Display;

use crate::codegen::CodeGen;

pub trait Prio {
    fn prio(&self) -> usize {
        0
    }
}

pub trait TopLevel:
    crate::vm::Eval
    + crate::type_check::TypeCheck<ReturnType = crate::ast::Type>
    + Display
    + Prio
    + CodeGen
{
    fn is_main(&self) -> bool;
}
/// Used to sort the program in to usable chunks
pub(crate) fn order<T1: TopLevel + ?Sized, T2: TopLevel + ?Sized>(
    el1: &T1,
    el2: &T2,
) -> std::cmp::Ordering {
    match (
        el1.prio() >= el2.prio(),
        el1.prio() == el2.prio(),
        el1.is_main(),
        el2.is_main(),
    ) {
        (_, _, true, _) => std::cmp::Ordering::Greater,
        (_, _, _, true) => std::cmp::Ordering::Less,
        (true, true, _, _) => std::cmp::Ordering::Equal,
        (true, false, _, _) => std::cmp::Ordering::Greater,
        (false, _, _, _) => std::cmp::Ordering::Less,
    }
}

impl Prio for super::Func {
    fn prio(&self) -> usize
    where
        Self: Sized,
    {
        2
    }
}
impl TopLevel for super::Func {
    fn is_main(&self) -> bool {
        match &self.id {
            crate::ast::Expr::Ident(s) => s == &"main".to_string(),
            _ => false,
        }
    }
}

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
