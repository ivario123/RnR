use crate::{
    ast::{Block, Expr, Func, Statement, Static, UnaryOp},
    prelude::Prog,
    Ast, AstNode,
};

use super::{EnvErr, PreDeclareTop, UnOpPreDeclaration};

impl UnOpPreDeclaration for Expr {
    fn pre_declare<'a>(
        &mut self,
        counter: &mut usize,
        block: &mut Vec<Statement>,
        index: &mut usize,
    ) -> Result<(), EnvErr> {
        match self {
            // These do not have to be redlecared
            Expr::UnOp(op, e) => {
                if let Expr::Ident(_) = **e {
                    return Ok(());
                }
                e.pre_declare(counter, block, index)?;
                let new_ident = Expr::Ident(format!("#{}_unary_op", *counter).to_string());
                let needs_mut = match op {
                    UnaryOp::BorrowMut => true,
                    _ => false,
                };
                let new_declaration =
                    Statement::Let(new_ident.clone(), needs_mut, None, Some(*e.clone()));
                block.insert(*index, new_declaration);
                *self = Expr::UnOp(op.clone(), Box::new(new_ident));

                *counter += 1;
                *index += 1;
                Ok(())
            }
            Expr::Block(b) => b.pre_declare(counter, block, index),
            Expr::BinOp(_op, l, r) => {
                l.pre_declare(counter, block, index)?;
                r.pre_declare(counter, block, index)
            }
            _ => Ok(()),
        }
    }
}
impl UnOpPreDeclaration for Statement {
    fn pre_declare<'a>(
        &mut self,
        counter: &mut usize,
        block: &mut Vec<Statement>,
        index: &mut usize,
    ) -> Result<(), EnvErr> {
        let ret = match self {
            Statement::Let(_, _, _, Some(rhs)) | Statement::Expr(rhs) => {
                rhs.pre_declare(counter, block, index)
            }
            Statement::While(cond, b) => {
                cond.pre_declare(counter, block, index)?;
                b.pre_declare(counter, block, index)
            }
            Statement::Block(b) => b.pre_declare(counter, block, index),
            Statement::Assign(_id, rhs) => rhs.pre_declare(counter, block, index),
            _ => Ok(()),
        };
        ret
    }
}
impl Block {
    fn _pre_declare_private<'a>(
        &mut self,
        counter: &mut usize,
        index: &mut usize,
    ) -> Result<(), EnvErr> {
        // This clone is rather symbolic, it might be better to create a new empty block here
        self.pre_declare(counter, &mut self.statements.clone(), index)
    }
}
impl UnOpPreDeclaration for Block {
    fn pre_declare<'a>(
        &mut self,
        counter: &mut usize,
        _block: &mut Vec<Statement>,
        _index: &mut usize,
    ) -> Result<(), EnvErr> {
        let mut new_index = 0;
        while let Some(mut statement) = self.statements.get_mut(new_index).cloned() {
            statement.pre_declare(counter, &mut self.statements, &mut new_index)?;
            self.statements[new_index] = statement;
            new_index += 1;
        }
        Ok(())
    }
}

impl PreDeclareTop for Static {
    fn pre_declare_top<'a>(
        &mut self,
        _counter: &mut usize,
        _index: &mut usize,
    ) -> Result<(), EnvErr> {
        todo!()
    }
}
impl PreDeclareTop for Func {
    fn pre_declare_top<'a>(
        &mut self,
        counter: &mut usize,
        index: &mut usize,
    ) -> Result<(), EnvErr> {
        self.body
            .pre_declare(counter, &mut self.body.statements.clone(), index)
    }
}
impl<T: PreDeclareTop + AstNode> PreDeclareTop for Ast<T> {
    fn pre_declare_top<'a>(
        &mut self,
        counter: &mut usize,
        index: &mut usize,
    ) -> Result<(), EnvErr> {
        self.t.pre_declare_top(counter, index)
    }
}

impl PreDeclareTop for Prog {
    fn pre_declare_top<'a>(
        &mut self,
        counter: &mut usize,
        index: &mut usize,
    ) -> Result<(), EnvErr> {
        for stmt in self.statements.iter_mut() {
            stmt.pre_declare_top(counter, index)?;
        }
        Ok(())
    }
}
