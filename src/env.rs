// Type generic environment

use crate::error::Error;
use crate::{ast::FnDeclaration, intrinsics::Intrinsic};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ref {
    scope_index: usize,
    scope_offset: usize,
}

impl Ref {
    pub fn new(scope_index: usize, scope_offset: usize) -> Self {
        Ref {
            scope_index,
            scope_offset,
        }
    }
}

type Stack<T> = Vec<T>;
type Var = HashMap<String, Ref>;
#[derive(Debug, Clone)]
pub struct Scope<T> {
    stack: Stack<T>,
    var: Var,
}

impl<T> Scope<T> {
    fn new() -> Self {
        Scope {
            stack: Stack::new(),
            var: Var::new(),
        }
    }
}

type Scopes<T> = Vec<Scope<T>>;
#[derive(Debug, Clone)]
pub struct VarEnv<T>(Scopes<T>);

impl<T> VarEnv<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        VarEnv(Scopes::new())
    }

    pub fn push_scope(&mut self) {
        self.0.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        self.0.pop();
    }

    // allocation in current scope
    // re-use current allocation if identifier shadows old binding
    pub fn alloc(&mut self, id: &str, v: T) -> Ref {
        let (scope_index, scope) = self.0.iter_mut().enumerate().last().unwrap();
        match scope.var.get(id).cloned() {
            Some(r) => {
                self.set_ref(r, v);
                r
            }
            None => {
                scope.stack.push(v);
                let (scope_offset, _) = scope.stack.iter().enumerate().last().unwrap();
                let r = Ref {
                    scope_index,
                    scope_offset,
                };
                scope.var.insert(id.to_owned(), r);
                r
            }
        }
    }

    pub fn stack_val(&mut self, v: T) -> Ref {
        let (scope_index, scope) = self.0.iter_mut().enumerate().last().unwrap();

        scope.stack.push(v);
        let (offset, _) = scope.stack.iter().enumerate().last().unwrap();
        Ref {
            scope_index,
            scope_offset: offset,
        }
    }

    pub fn set_ref(&mut self, r: Ref, v: T) {
        self.0[r.scope_index].stack[r.scope_offset] = v;
    }

    pub fn get(&self, id: &str) -> Option<T> {
        let r = self.get_ref(id)?;
        Some(self.0[r.scope_index].stack[r.scope_offset].clone())
    }

    pub fn get_ref(&self, id: &str) -> Option<Ref> {
        for scope in self.0.iter().rev() {
            if let Some(r) = scope.var.get(id) {
                return Some(r.clone());
            }
        }
        None
    }

    pub fn de_ref(&self, r: Ref) -> T {
        self.0[r.scope_index].stack[r.scope_offset].clone()
    }
}

#[derive(Clone)]
pub struct FnEnv(pub HashMap<String, (FnDeclaration, Option<Intrinsic>)>);

impl Debug for FnEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for f in self.0.iter() {
            s.push_str(&format!("{:?}, ", f.0));
        }
        write!(f, "{}", s)
    }
}

impl FnEnv {
    fn new() -> Self {
        FnEnv(HashMap::new())
    }

    pub fn add_functions_unique(&mut self, new_fns: Vec<FnDeclaration>) -> Result<(), Error> {
        let mut hm = HashSet::new();
        for f in new_fns.clone() {
            match hm.get(&f.id) {
                Some(_) => Err(&format!("Function {} already defined", f.id))?,
                None => {
                    hm.insert(f.id.clone());
                }
            };
        }

        for f in new_fns {
            self.0.insert(f.id.clone(), (f, None));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Env<T>
where
    T: Clone,
{
    pub v: VarEnv<T>,
    pub f: FnEnv,
}

impl<T> Env<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Env {
            v: VarEnv::new(),
            f: FnEnv::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VarEnv;
    use crate::ast::*;

    #[test]
    fn t() {
        let mut env = VarEnv::new();

        env.push_scope();

        env.alloc("a", Literal::Int(1));
        println!("env {:?}", env);

        env.alloc("a", Literal::Int(2));
        println!("env {:?}", env);

        env.push_scope();

        let ref_a = env.get_ref("a").unwrap();
        println!("ref_a {:?}", ref_a);

        println!("de_ref_a {:?}", env.de_ref(ref_a));

        // env.alloc("a", ref_a);
        // println!("env {:?}", env);

        // let ref_a = env.get_ref("a").unwrap();
        // println!("ref_a {:?}", ref_a);

        // let v = env.get("a");
        // println!("v {:?}", v);

        env.pop_scope();

        let v = env.get("a");
        println!("v {:?}", v);
    }
}
