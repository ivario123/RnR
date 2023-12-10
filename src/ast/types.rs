//! Enumerates all of the possible types in this subset of the rust language.

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    Bool,
    Unit,
    Usize,
    Array(Box<Type>, usize),
    Ref(Ref),
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ref(pub Box<Type>);

impl From<Type> for Ref {
    fn from(value: Type) -> Self {
        Ref(Box::new(value))
    }
}
