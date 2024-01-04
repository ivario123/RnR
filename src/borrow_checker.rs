//! This modlue is responsible for linearization of programs, this requires that each node can be
//! found recursivly in an enviornment.

pub mod env;
pub mod linearize_and_borrow;
pub mod pre_decleration;

pub use env::*;
pub use linearize_and_borrow::*;
pub use pre_decleration::*;
use std::collections::HashMap;

use crate::ast::{Expr, Statement};

#[derive(Debug)]
pub enum BCError {
    EnvError(EnvErr),
    InvalidIdentifierType(Expr),
    NeverUsed(String),
    MultipleRefWhileMutRefAlive,
    TryingToBorrowWhileMutBorrow(String),
    TryingToBorrowMutWhileImmut(String),
    DerrefOfOutOfScope,
}

#[derive(Debug)]
pub enum EnvErr {
    NoSuchIdentifier(String),
    CannotTreatAsIdentifier(Expr),
    ScopeError(String),
    OutOfScope,
}

pub trait Rename: crate::AstNode {
    fn rename(&mut self, new_id: String) -> Result<(), BCError>;
    fn name(&self) -> Result<String, BCError>;
}

#[derive(Debug)]
pub struct BCMeta<'a> {
    source: Box<&'a mut dyn Rename>,
    // Tracks, id, scope count, scope depth, declaration counter.
    unique_id: (String, usize, usize, usize),
    refs: Vec<&'a mut BCMeta<'a>>,
    usage_counter: usize,
    valid: bool,
}

pub trait Scope: Sized + Default {
    type Meta: MetaVariable;
    fn validate(&mut self) -> Result<(), String>;
    fn get(&self, id: &String) -> Option<&Self::Meta>;
    fn get_mut(&mut self, id: &String) -> Option<&mut Self::Meta>;
    fn insert(&mut self, id: String, value: Self::Meta) -> Option<Self::Meta>;
}

pub trait MetaVariable {
    fn access(&mut self) {}
    fn borrow(&mut self, source_of_borrow: &'static mut Self);
    fn validate(&self) -> bool;
}

pub trait PreDeclareTop {
    fn pre_declare_top<'a>(&mut self, counter: &mut usize, index: &mut usize)
        -> Result<(), EnvErr>;
}

pub trait Linearize {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError>;
}

pub trait UnOpPreDeclaration {
    fn pre_declare<'a>(
        &mut self,
        counter: &mut usize,
        block: &mut Vec<Statement>,
        index: &mut usize,
    ) -> Result<(), EnvErr>;
}

#[derive(Default, Debug)]
pub struct BCScope<'a> {
    scope: HashMap<String, BCMeta<'a>>,
}

#[derive(Debug, Clone)]
struct BorrowMap {
    map: HashMap<String, (bool, Vec<Box<BorrowValue>>)>,
}

#[derive(Debug, Clone)]
pub struct BorrowValue {
    id: String,
    mutable: bool,
}

impl<'a> MetaVariable for BCMeta<'a> {
    fn access(&mut self) {
        self.usage_counter += 1;
    }
    fn borrow(&mut self, source_of_borrow: &'static mut Self) {
        self.refs.push(source_of_borrow);
    }
    fn validate(&self) -> bool {
        self.valid
    }
}

impl Rename for Expr {
    fn rename(&mut self, new_id: String) -> Result<(), BCError> {
        match self {
            Expr::Ident(s) => Ok(*s = new_id),
            e => Err(BCError::InvalidIdentifierType(e.clone())),
        }
    }
    fn name(&self) -> Result<String, BCError> {
        match self {
            Expr::Ident(s) => Ok(s.clone()),
            e => Err(BCError::InvalidIdentifierType(e.clone())),
        }
    }
}

impl<'a> Env<BCScope<'a>> {
    pub fn pop(&mut self) -> Result<(), EnvErr> {
        // No need to decrement the counter here, not doing so allows us to linearize the program
        // much faster.
        if let Some(mut env) = self.vars.pop() {
            for el in env.to_vec() {
                self.destroy_ref(&el.hash());
            }
            return match env.validate() {
                Ok(_) => Ok(()),
                Err(e) => Err(EnvErr::ScopeError(e)),
            };
        }
        Ok(())
    }
}

impl<'a> Scope for BCScope<'a> {
    type Meta = BCMeta<'a>;
    fn validate(&mut self) -> Result<(), String> {
        // 1. check that all variables have one or more use
        for (_id, meta) in self.scope.iter_mut() {
            meta.finalize().map_err(|e| format!("{e:?}").to_string())?;
        }
        Ok(())
    }

    fn get(&self, id: &String) -> Option<&Self::Meta> {
        self.scope.get(id)
    }

    fn get_mut(&mut self, id: &String) -> Option<&mut Self::Meta> {
        self.scope.get_mut(id)
    }

    fn insert(&mut self, id: String, value: Self::Meta) -> Option<Self::Meta> {
        self.scope.insert(id, value)
    }
}

impl<'a> BCScope<'a> {
    pub fn to_vec(&self) -> Vec<&BCMeta<'a>> {
        self.scope.values().collect()
    }
}

impl<'a> BCMeta<'a> {
    fn hash(&self) -> String {
        let (id, count, depth, reassign) = self.unique_id.clone();
        format!(">{}#{}!{}_{}", count, depth, reassign, id).to_owned()
    }
    fn finalize(&mut self) -> Result<(), BCError> {
        if self.usage_counter == 0 {
            return Err(BCError::NeverUsed(self.unique_id.0.clone()));
        }
        self.source.rename(self.hash())?;
        for el in self.refs.iter_mut() {
            (*el).valid = false;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Env;
    use crate::{
        borrow_checker::PreDeclareTop, check, eval, parse, prelude::*, vm::Eval, vm::VarEnv, Ast,
    };

    #[test]
    fn test() {
        let prog = "fn main(){
            let a:i32 = 20;
            let b:i32 = a-2;
            let a:i32 = 20-a;
            let mut b:i32 = b+a-30;
            {
                let a:i32 = 0;
                {
                    let a:i32 = 2;
                    a;
                };
                a;
            };
            fn a(a:i32) -> i32{
                let b:i32 = a;
                let a = &b;
                b;
                *a
            };
            { 
                let d = &&mut b;
                d;
            };
            {
                let k1 = &{1+1};
                k1;
                let c = &&&b;
                let k = ***c+1;
                k + 1;
            };
            let d = &mut b;
            let c = *d;
            let b = c + 1;
            b+1
        }"
        .to_string();
        let mut prog: Ast<Prog> = parse!(prog, Prog);
        prog.pre_declare_top(&mut 0, &mut 0).unwrap();
        println!("{prog}");
        let mut env = Env::new();
        let l = prog.linearize(&mut env);
        assert!(l.is_ok());
        println!("linear prog : {prog}");
        let iter = 400;
        println!("{:?}", check!(prog));
        println!("{:?}", eval!(prog, iter));
    }
}
