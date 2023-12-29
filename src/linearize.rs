//! This modlue is responsible for linearization of programs, this requires that each node can be
//! found recursivly in an enviornment.
//!
//!
//!
//!
#![allow(unused)]
use std::collections::HashMap;

use syn::Meta;

use crate::{
    ast::{Arg, Block, Expr, Func, FuncCall, Statement},
    type_check::FunctionMeta,
    vm::statement,
    AstNode,
};

#[derive(Debug)]
pub enum BCError {
    EnvError(EnvErr),
    InvalidIdentifierType(Expr),
    NeverUsed(String),
}

#[derive(Debug)]
pub struct BCMeta<'a> {
    source: Box<&'a mut dyn Rename>,
    // Tracks, scope depth, scope count, declaration counter.
    unique_id: (String, usize, usize, usize),
    usage_counter: usize,
}

pub trait Rename: crate::AstNode {
    fn rename(&mut self, new_id: String) -> Result<(), BCError>;
    fn name(&self) -> Result<String, BCError>;
}

pub trait MetaVariable {
    fn access(&mut self) {}
}

pub trait Scope: Sized + Default {
    type Meta: MetaVariable;
    fn validate(&mut self) -> Result<(), String>;
    fn get(&self, id: &String) -> Option<&Self::Meta>;
    fn get_mut(&mut self, id: &String) -> Option<&mut Self::Meta>;
    fn insert(&mut self, id: String, value: Self::Meta) -> Option<Self::Meta>;
}

#[derive(Default, Debug)]
pub struct BCScope<'a> {
    scope: HashMap<String, BCMeta<'a>>,
}

#[derive(Debug)]
pub struct Env<Meta: Scope> {
    vars: Vec<Meta>,
    fns: Vec<HashMap<String, FunctionMeta>>,
    scope_counter: usize,
}

#[derive(Debug)]
pub enum EnvErr {
    NoSuchIdentifier(String),
    ScopeError(String),
}

impl MetaVariable for BCMeta<'_> {
    fn access(&mut self) {
        self.usage_counter += 1;
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
impl<'a> Scope for BCScope<'a> {
    type Meta = BCMeta<'a>;
    fn validate(&mut self) -> Result<(), String> {
        // 1. check that all variables have one or more use
        for (_id, meta) in self.scope.iter_mut() {
            println!("Meta : {meta:?}");
            meta.finalize().map_err(|e| format!("{e:?}").to_string())?;
            println!("Meta : {meta:?}");
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

impl<'a> BCMeta<'a> {
    fn hash(&self) -> String {
        let (id, count, depth, reassign) = self.unique_id.clone();
        format!(
            "{}_count_{}_depth_{}_reassign_{}",
            id, count, depth, reassign
        )
        .to_owned()
    }
    fn finalize(&mut self) -> Result<(), BCError> {
        if self.usage_counter == 0 {
            println!("Never used");
            return Err(BCError::NeverUsed(self.unique_id.0.clone()));
        }
        self.source.rename(self.hash())?;
        Ok(())
    }
}

impl<Meta> Env<Meta>
where
    Meta: Scope + std::fmt::Debug,
{
    pub fn traverse<'a>(&'a mut self, target: &String) -> Result<&'a Meta::Meta, EnvErr> {
        let mut len: usize = self.vars.len();
        while let Some(idx) = len.checked_sub(1) {
            if let Some(meta) = self.vars.get(idx).unwrap().get(target) {
                let meta = self.vars.get_mut(idx).unwrap().get_mut(target).unwrap();
                meta.access();
                return Ok(self.vars.get(idx).unwrap().get(target).unwrap());
            }
            len -= 1;
        }
        return Err(EnvErr::NoSuchIdentifier(target.to_owned()));
    }
    pub fn push(&mut self) {
        self.vars.push(Meta::default());
        self.fns.push(HashMap::new());
        self.scope_counter += 1;
    }
    pub fn pop(&mut self) -> Result<(), EnvErr> {
        // No need to decrement the counter here, not doing so allows us to linearize the program
        // much faster.
        if let Some(mut env) = self.vars.pop() {
            return match env.validate() {
                Ok(_) => {
                    println!("Env after validation {env:?}");
                    Ok(())
                }
                Err(e) => Err(EnvErr::ScopeError(e)),
            };
        }
        Ok(())
    }
    pub fn counter(&self) -> (usize, usize) {
        (self.vars.len(), self.scope_counter.clone())
    }
    pub fn new() -> Self {
        Self {
            vars: Vec::new(),
            fns: Vec::new(),
            scope_counter: 0,
        }
    }
    pub fn enter_function(&self) -> Self {
        let fns = self.fns.clone();
        // TODO:
        // Lacks access to global scope
        let vars = vec![];
        Self {
            vars,
            fns,
            scope_counter: self.scope_counter.clone(),
        }
    }
}

impl<'a, Meta> Env<Meta>
where
    Meta: Scope<Meta = BCMeta<'a>> + std::fmt::Debug,
{
    pub fn declare(&mut self, expr: Box<&'a mut Expr>) -> Result<(), BCError> {
        println!("Trying to insert {expr:?} into env {self:?}");
        // First we check if there is any variables with the same name in this scope
        let ident = match (**expr).clone() {
            Expr::Ident(i) => i.clone(),
            _ => unreachable!(),
        };
        let mut id = match self.vars.last().unwrap().get(&ident) {
            Some(meta) => meta.unique_id.clone(),
            None => {
                let scope_info: (usize, usize) = self.counter();
                (ident.clone(), scope_info.0, scope_info.1, 0)
            }
        };
        id.3 += 1;
        let expr: &'a mut dyn Rename = *expr;
        let meta = BCMeta {
            source: Box::new(expr),
            unique_id: id,
            usage_counter: 0,
        };
        println!("{:?}", self.vars.last_mut());
        let last = self.vars.last_mut().unwrap();
        let mut previous = last.insert(ident, meta);

        if let Some(mut previous) = previous {
            previous.finalize()?;
        }

        println!("var_env {last:?}");
        println!("full env : {self:?}");
        Ok(())
    }
}

pub trait Linearize {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError>;
}

impl Linearize for Expr {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        match self {
            Expr::Ident(i) => {
                let meta = env.traverse(&i.clone()).map_err(|e| BCError::EnvError(e))?;
                *i = meta.hash();
                Ok(())
            }
            Expr::BinOp(_op, lhs, rhs) => {
                let _ = lhs.linearize(env)?;
                rhs.linearize(env)
            }
            Expr::UnOp(_op, rhs) => rhs.linearize(env),
            Expr::Par(e) => e.linearize(env),
            Expr::IfThenElse(condition, block, other_block) => {
                let _ = condition.linearize(env)?;
                let _ = block.linearize(env)?;
                if let Some(block) = other_block {
                    let _ = block.linearize(env);
                }
                Ok(())
            }
            Expr::Array(e) => {
                for el in e {
                    let _ = el.linearize(env)?;
                }
                Ok(())
            }
            Expr::Index(id, value) | Expr::IndexMut(id, value) => {
                id.linearize(env)?;
                value.linearize(env)
            }
            Expr::FuncCall(_) => todo!(),
            Expr::Block(b) => b.linearize(env),
            _ => Ok(()),
        }
    }
}
impl Linearize for Statement {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        match self {
            Statement::Let(ident, mutable, _, Some(rhs)) => {
                let _ = match *ident {
                    Expr::Ident(_) => Ok(()),
                    _ => Err(BCError::InvalidIdentifierType(ident.clone())),
                }?;
                rhs.linearize(env)?;
                env.declare(Box::new(ident))?;
                Ok(())
            }
            Statement::Let(ident, mutable, _, None) => {
                let _ = match *ident {
                    Expr::Ident(_) => Ok(()),
                    _ => Err(BCError::InvalidIdentifierType(ident.clone())),
                }?;
                env.declare(Box::new(ident))?;
                Ok(())
            }
            Statement::Assign(_ident, rhs) => todo!(),
            Statement::While(stmt, block) => {
                stmt.linearize(env)?;
                block.linearize(env)?;
                Ok(())
            }
            Statement::Expr(e) => e.linearize(env),
            Statement::Block(b) => b.linearize(env),
            Statement::FnDecleration(f) => f.linearize(env),
        }
    }
}
impl Linearize for FuncCall {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        for arg in self.args.iter_mut() {
            arg.linearize(env)?;
        }
        Ok(())
    }
}
impl Linearize for Func {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        let env = &mut env.enter_function();
        for arg in &mut self.args {
            arg.linearize(env)?;
        }
        Ok(())
    }
}
impl Linearize for Arg {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        self.id.linearize(env)
    }
}
impl Linearize for Block {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        env.push();
        for statement in self.statements.iter_mut() {
            let _ = statement.linearize(env)?;
        }
        env.pop().map_err(|e| BCError::EnvError(e))
    }
}
impl<T: AstNode + Linearize + 'static> crate::Ast<T> {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        self.t.linearize(env)?;
        println!("Resulting env : {env:?}");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{ast::Block, linearize::Linearize, parse, Ast};

    use super::{BCScope, Env};

    #[test]
    fn test() {
        let prog = "{
            let a:i32 = 20;
            let b:i32 = a-2;
            let a:i32 = 20-a;
            let b:i32 = b+a-30;
            b+1
        }"
        .to_string();
        let mut prog: Ast<Block> = parse!(prog, Block);
        println!("{prog}");
        let mut env = Env::new();
        assert!(prog.linearize(&mut env).is_ok());
        println!("env : {env:?}");
        println!("linear prog : {prog}")
    }
}
