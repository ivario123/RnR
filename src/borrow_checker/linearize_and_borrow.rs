use crate::{
    ast::{Arg, Block, Expr, Func, FuncCall, Statement, Static, UnaryOp},
    prelude::Prog,
    AstNode,
};

use super::{BCError, BCScope, BorrowValue, Env, Linearize};
impl Expr {
    fn linearize<'a>(
        &'a mut self,
        env: &mut Env<BCScope<'a>>,
        dereff_depth: &mut usize,
    ) -> Result<Option<(String, BorrowValue)>, BCError> {
        match self {
            Expr::Ident(i) => {
                let meta = env.traverse(&i.clone()).map_err(BCError::EnvError)?;
                *i = meta.hash();
            }
            Expr::BinOp(_op, lhs, rhs) => {
                let _ = lhs.linearize(env, dereff_depth)?;
                let _ = rhs.linearize(env, dereff_depth)?;
            }
            Expr::UnOp(UnaryOp::BorrowMut, e) => {
                let e_clone = e.clone();
                let _ = e.linearize(env, dereff_depth)?;
                let id = match *e_clone.clone() {
                    Expr::Ident(_id) => env.format_ident(*e_clone).map_err(BCError::EnvError)?,
                    e => Err(BCError::InvalidIdentifierType(e.clone()))?,
                };
                return Ok(Some((
                    id,
                    BorrowValue {
                        id: "".to_string(),
                        mutable: true,
                    },
                )));
            }
            Expr::UnOp(UnaryOp::Borrow, e) => {
                let e_clone = e.clone();
                e.linearize(env, dereff_depth)?;
                let id = env.format_ident(*e_clone).map_err(BCError::EnvError)?;

                return Ok(Some((
                    id,
                    BorrowValue {
                        id: "".to_string(),
                        mutable: false,
                    },
                )));
            }
            Expr::UnOp(UnaryOp::Dereff, rhs) => {
                let e_clone = rhs.clone();
                rhs.linearize(env, dereff_depth)?;
                match *e_clone.clone() {
                    Expr::Ident(_) => {
                        let id = env.format_ident(*e_clone).map_err(BCError::EnvError)?;
                        *dereff_depth += 1;
                        env.dereff(&id, *dereff_depth)?;
                    }
                    // This is already handled
                    Expr::UnOp(UnaryOp::Borrow, _)
                    | Expr::UnOp(UnaryOp::BorrowMut, _)
                    | Expr::UnOp(UnaryOp::Dereff, _) => {}
                    e => return Err(BCError::InvalidIdentifierType(e)),
                }
            }
            Expr::UnOp(_op, rhs) => {
                rhs.linearize(env, dereff_depth)?;
            }
            Expr::Par(e) => {
                e.linearize(env, dereff_depth)?;
            }
            Expr::IfThenElse(condition, block, other_block) => {
                let _ = condition.linearize(env, dereff_depth)?;
                let _ = block.linearize(env)?;
                if let Some(block) = other_block {
                    let _ = block.linearize(env);
                }
            }
            Expr::Array(e) => {
                for el in e {
                    let _ = el.linearize(env, dereff_depth)?;
                }
            }
            Expr::Index(id, value) | Expr::IndexMut(id, value) => {
                id.linearize(env, dereff_depth)?;
                value.linearize(env, dereff_depth)?;
            }
            Expr::FuncCall(f) => f.linearize(env)?,
            Expr::Block(b) => {
                b.linearize(env)?;
            }
            _ => {}
        };
        Ok(None)
    }
}

impl Linearize for Statement {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        match self {
            Statement::Let(ident, _mutable, _, Some(rhs)) => {
                let is_borrow = rhs.linearize(env, &mut 0)?;
                let ident_clone = ident.clone();
                env.declare(Box::new(ident))?;
                if let Some((target, mut borrow_value)) = is_borrow {
                    let id = env.format_ident(ident_clone).map_err(BCError::EnvError)?;
                    borrow_value.id = id;
                    env.borrow(&target, borrow_value)?;
                }
                Ok(())
            }
            Statement::Let(ident, _mutable, _, None) => {
                let _ = match *ident {
                    Expr::Ident(_) => Ok(()),
                    _ => Err(BCError::InvalidIdentifierType(ident.clone())),
                }?;
                env.declare(Box::new(ident))?;
                Ok(())
            }
            Statement::Assign(ident, rhs) => {
                let is_borrow = rhs.linearize(env, &mut 0)?;
                let ident_clone = ident.clone();
                ident.linearize(env, &mut 0)?;
                if let Some((target, mut borrow_value)) = is_borrow {
                    let id = env.format_ident(ident_clone).map_err(BCError::EnvError)?;
                    borrow_value.id = id;
                    env.borrow(&target, borrow_value)?;
                }
                Ok(())
            }
            Statement::While(stmt, block) => {
                stmt.linearize(env, &mut 0)?;
                block.linearize(env)?;
                Ok(())
            }
            Statement::Expr(e) => {
                e.linearize(env, &mut 0)?;
                Ok(())
            }
            Statement::Block(b) => b.linearize(env),
            Statement::FnDecleration(f) => f.linearize(env),
        }
    }
}

impl Linearize for FuncCall {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        for arg in self.args.iter_mut() {
            arg.linearize(env, &mut 0)?;
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

        self.body.linearize(env)?;
        env.pop().map_err(BCError::EnvError)
    }
}
impl Linearize for Arg {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        // 1. Function arguments should not be linearised, they are per definition unique, given
        //    that they do not interfere with globals
        //
        env.declare(Box::new(&mut self.id))
    }
}

impl Linearize for Block {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        env.push();
        for statement in self.statements.iter_mut() {
            statement.linearize(env)?;
        }
        env.pop().map_err(BCError::EnvError)
    }
}
impl Linearize for Static {
    fn linearize<'a>(&'a mut self, _env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        todo!()
    }
}

impl Linearize for Prog {
    fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        for el in self.statements.iter_mut() {
            el.linearize(env)?;
        }
        Ok(())
    }
}

impl<T: AstNode + Linearize + 'static> crate::Ast<T> {
    pub fn linearize<'a>(&'a mut self, env: &mut Env<BCScope<'a>>) -> Result<(), BCError> {
        self.t.linearize(env)?;
        Ok(())
    }
}
