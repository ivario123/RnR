//! Enumerates all of the possible types in this subset of the rust language.

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    Bool,
    Unit,
    Usize,
    Array(Box<Type>, usize),
    Ref(Box<Type>),
}
