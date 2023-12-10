use std::fmt::Display;

pub(crate) mod sealed {
    pub trait Prio {
        fn prio(&self) -> usize
        where
            Self: Sized,
        {
            0
        }
    }
}

pub trait TopLevel:
    crate::vm::Eval
    + crate::type_check::TypeCheck<ReturnType = crate::ast::Type>
    + Display
    + sealed::Prio
{
}

impl std::fmt::Debug for dyn TopLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Prog {
    pub(crate) statements: Vec<Box<dyn TopLevel>>,
}
