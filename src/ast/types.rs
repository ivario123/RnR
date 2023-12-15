//! Enumerates all of the possible types in this subset of the rust language.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    I32,
    Bool,
    Unit,
    Usize,
    Array(Box<Type>, usize),
    Ref(Ref),
    MutRef(Ref),
    String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ref(pub Box<Type>, pub usize, pub usize);

impl From<Type> for Ref {
    fn from(value: Type) -> Self {
        Ref(Box::new(value), 0, 0)
    }
}
