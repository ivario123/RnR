//! Defines the environment used for borrow checking

use std::collections::HashMap;

use crate::{ast::Expr, type_check::FunctionMeta};

use super::{BCError, BCMeta, BorrowMap, BorrowValue, EnvErr, MetaVariable, Rename, Scope};

#[derive(Debug)]
pub struct Env<Meta: Scope> {
    pub(crate) vars: Vec<Meta>,
    fns: Vec<HashMap<String, FunctionMeta>>,
    borrows: BorrowMap,
    borrowers: HashMap<String, String>,
    scope_counter: usize,
}

impl<Meta> Env<Meta>
where
    Meta: Scope + std::fmt::Debug,
{
    pub fn traverse<'a>(&'a mut self, target: &String) -> Result<&'a Meta::Meta, EnvErr> {
        let mut len: usize = self.vars.len();
        while let Some(idx) = len.checked_sub(1) {
            if let Some(_meta) = self.vars.get(idx).unwrap().get(target) {
                let meta = self.vars.get_mut(idx).unwrap().get_mut(target).unwrap();
                meta.access();
                if meta.validate() {
                    return Ok(self.vars.get(idx).unwrap().get(target).unwrap());
                } else {
                    return Err(EnvErr::OutOfScope);
                }
            }
            len -= 1;
        }
        return Err(EnvErr::NoSuchIdentifier(target.to_owned()));
    }
    pub fn traverse_mut(
        &'static mut self,
        target: &String,
    ) -> Result<&'static mut Meta::Meta, EnvErr> {
        let mut len: usize = self.vars.len();
        while let Some(idx) = len.checked_sub(1) {
            if let Some(_meta) = self.vars.get(idx).unwrap().get(target) {
                let meta = self.vars.get_mut(idx).unwrap().get_mut(target).unwrap();
                meta.access();
                if meta.validate() {
                    return Ok(meta);
                } else {
                    return Err(EnvErr::OutOfScope);
                }
            }
            len -= 1;
        }
        return Err(EnvErr::NoSuchIdentifier(target.to_owned()));
    }
    fn traverse_imut<'a>(&'a self, target: &String) -> Result<&'a Meta::Meta, EnvErr> {
        let mut len: usize = self.vars.len();
        while let Some(idx) = len.checked_sub(1) {
            if let Some(meta) = self.vars.get(idx).unwrap().get(target) {
                if meta.validate() {
                    return Ok(meta);
                } else {
                    return Err(EnvErr::OutOfScope);
                }
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

    pub fn counter(&self) -> (usize, usize) {
        (self.vars.len(), self.scope_counter.clone())
    }
    pub fn new() -> Self {
        Self {
            vars: Vec::new(),
            fns: Vec::new(),
            borrows: BorrowMap {
                map: HashMap::new(),
            },
            borrowers: HashMap::new(),
            scope_counter: 0,
        }
    }
    pub fn enter_function(&self) -> Self {
        let fns = self.fns.clone();
        // TODO:
        // Lacks access to global scope
        let vars = vec![];
        let mut new = Self {
            vars,
            fns,
            borrows: self.borrows.clone(),
            borrowers: self.borrowers.clone(),
            scope_counter: self.scope_counter.clone(),
        };
        new.push();
        new
    }
}

impl<'a, Meta> Env<Meta>
where
    Meta: Scope<Meta = BCMeta<'a>> + std::fmt::Debug,
{
    pub fn declare(&mut self, expr: Box<&'a mut Expr>) -> Result<(), BCError> {
        // First we check if there is any variables with the same name in this scope
        let ident = match (**expr).clone() {
            Expr::Ident(i) => i.clone(),
            _ => unreachable!(),
        };
        let usage_counter = match ident.starts_with("_") {
            true => 1,
            _ => 0,
        };
        let mut id = match self.vars.last().unwrap().get(&ident) {
            Some(meta) => meta.unique_id.clone(),
            None => {
                let scope_info: (usize, usize) = self.counter();
                (ident.clone(), scope_info.0, scope_info.1, 0)
            }
        };
        id.3 += 1;
        // Type-system shenanigans
        let expr: &'a mut dyn Rename = *expr;
        let meta = BCMeta {
            source: Box::new(expr),
            unique_id: id,
            usage_counter,
            refs: Vec::new(),
            valid: true,
        };
        let last = self.vars.last_mut().unwrap();
        let previous = last.insert(ident, meta);

        if let Some(mut previous) = previous {
            previous.finalize()?;
        }

        Ok(())
    }

    /// This assumes that the typechecking does some basic borrow checking to see that all borrows
    /// go to valid variables.
    pub(crate) fn borrow(
        &mut self,
        target_id: &String,
        borrow_value: BorrowValue,
    ) -> Result<(), BCError> {
        let is_mutable = borrow_value.mutable.clone();
        let mut targets = Vec::new();
        let mut any_mut = false;
        if let Some((prev_any_mut, borrows)) = self.borrows.map.get(target_id) {
            targets = borrows.clone();
            any_mut = *prev_any_mut || is_mutable;
        }
        println!("{:?}", (is_mutable, any_mut, targets.len()));
        match (is_mutable, any_mut, targets.len()) {
            (_, true, _) => return Err(BCError::MultipleRefWhileMutRefAlive),
            (true, false, 0) => {}
            (true, false, _) => return Err(BCError::MultipleRefWhileMutRefAlive),
            (_, _, _) => {}
        };
        self.borrowers
            .insert(borrow_value.id.clone(), target_id.clone());
        targets.push(Box::new(borrow_value));
        self.borrows
            .map
            .insert(target_id.clone(), (any_mut | is_mutable, targets));
        Ok(())
    }

    fn remove_refferands(&mut self, target_id: &String) {
        match self.borrows.map.remove(target_id) {
            Some((_mutable, borrowers)) => borrowers,
            _ => return,
        };
    }

    pub(crate) fn destroy_ref(&mut self, target_id: &String) {
        // First we check if we are borrowers
        match self.borrowers.remove(target_id) {
            Some(target) => match self.borrows.map.get(&target).clone() {
                Some(targets) => {
                    let mut id = None;
                    for (idx, el) in targets.1.iter().enumerate() {
                        if el.id == *target_id {
                            id = Some(idx);
                        }
                    }
                    let targets = self.borrows.map.get_mut(&target).unwrap();
                    if let Some(id) = id {
                        targets.1.remove(id);
                    }
                    if targets.1.len() == 0 {
                        self.borrows.map.remove(&target);
                    }
                }
                _ => {}
            },
            None => self.remove_refferands(target_id),
        }
    }

    pub(crate) fn format_ident(&self, e: Expr) -> Result<String, EnvErr> {
        match e {
            Expr::Ident(i) => match self.traverse_imut(&i) {
                Ok(meta) => Ok(meta.hash()),
                Err(EnvErr::NoSuchIdentifier(id)) => {
                    let scope_info: (usize, usize) = self.counter();
                    let (id, count, depth, reassign) = (id.clone(), scope_info.0, scope_info.1, 0);

                    Ok(format!(
                        "{}_depth_{}_count_{}_reassign_{}",
                        id, depth, count, reassign
                    )
                    .to_owned())
                }
                Err(e) => Err(e),
            },
            e => Err(EnvErr::CannotTreatAsIdentifier(e)),
        }
    }


    pub fn dereff(&mut self, target_id: &String) -> Result<(), BCError> {
        let ref_id = match self.borrowers.get(target_id) {
            Some(ref_id) => Ok(ref_id),
            None => Err(BCError::DerrefOfOutOfScope),
        }?;
        // Validate on derefference
        match self.borrows.map.get(ref_id) {
            Some((_, r)) => {
                for el in r {
                    if el.id == *target_id {
                        return Ok(());
                    }
                }
                return Err(BCError::DerrefOfOutOfScope);
            }
            None => return Err(BCError::DerrefOfOutOfScope),
        }
    }

}
