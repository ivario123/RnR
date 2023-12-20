// codegen for a simple MIPS 3k in single cycle mode.
#![allow(dead_code)]
use crate::ast::*;
use crate::{ast::BinaryOp, Ast};

use mips::{
    asm::*,
    instrs::Instrs,
    rf::Reg::{self, *},
};

use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
#[derive(Debug)]
pub enum CompileTarget {
    Mips,
}
impl FromStr for CompileTarget {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mips" => Ok(Self::Mips),
            s => Err(format!("{s} is not a valid identifier")),
        }
    }
}

#[derive(Debug, Clone)]
enum Target {
    Var(i16), // offset on stack
    Fn,       // address to function in memory
}

fn name_space_n(block_number: usize, block_type: &str) -> String {
    format!("{}_{}", block_number, block_type)
}

#[derive(Debug, Clone)]
pub struct Env {
    offset: i16,
    // (name_space, HashMap)
    scope: VecDeque<(String, HashMap<String, Target>)>,
}

impl Env {
    fn new() -> Self {
        Env {
            offset: 0,
            scope: VecDeque::new(),
        }
    }

    fn get_var_offset(&self, id: &str) -> i16 {
        match self.get_var(id) {
            Some(Target::Var(offset)) => offset,
            _ => panic!("ICE, cannot find identifier {}", id),
        }
    }

    // returns true if id already in current scope
    fn push_var(&mut self, id: &str) -> bool {
        if self.scope.is_empty() {
            self.scope
                .push_front(("GLOBAL_SCOPE".to_string(), HashMap::new()))
        }
        match self.scope[0].1.contains_key(id) {
            true => match self.scope[0].1.get(id).unwrap() {
                Target::Var(_) => true,
                Target::Fn => panic!("expecting var, found fn"),
            },
            false => {
                self.offset -= 4;
                self.scope[0]
                    .1
                    .insert(id.to_owned(), Target::Var(self.offset));
                false
            }
        }
    }

    fn insert_fn(&mut self, id: &str) {
        let scope = match self.scope.get_mut(0) {
            Some(scope) => scope,
            None => {
                self.push_scope("GLOBAL_SCOPE");
                self.scope.get_mut(0).unwrap()
            }
        };
        if scope.1.contains_key(id) {
            panic!("fn `{}` already defined in scope", id)
        } else {
            scope.1.insert(id.to_owned(), Target::Fn);
        }
    }

    // set argument offset relative to fp
    fn set_arg_offset(&mut self, id: &str, offset: i16) {
        self.scope[0].1.insert(id.to_owned(), Target::Var(offset));
    }

    fn push_scope(&mut self, name_space: &str) {
        self.scope.push_front((name_space.into(), HashMap::new()));
    }

    fn pop_scope(&mut self) {
        self.scope.pop_front();
    }

    // get_var, traverse the scopes
    fn get_var(&self, id: &str) -> Option<Target> {
        for (_, frame) in self.scope.iter() {
            if let Some(target) = frame.get(id) {
                return Some(target.clone());
            }
        }
        None
    }

    // get_fn, traverse the scopes
    fn get_fn(&self, id: &str) -> Option<String> {
        // get_fn_scope, traverse the scopes
        let mut found = false;
        let mut name = String::new();

        for (path, s) in &self.scope {
            if let Some(Target::Fn) = s.get(id) {
                found = true;
            };
            if found {
                // only include name space if fn found
                name = format!("{}_{}", name, path);
            }
        }

        if found {
            Some(format!("{}{}", id, name))
        } else {
            None
        }
    }
}

fn push(r: Reg) -> Instrs {
    Instrs(vec![addiu(sp, sp, -4), sw(r, 0, sp)]).comment(&format!("push {:?}", r))
}

fn pop(r: Reg) -> Instrs {
    Instrs(vec![lw(r, 0, sp), addiu(sp, sp, 4)]).comment(&format!("pop {:?}", r))
}

fn li(r: Reg, d: u32) -> Instrs {
    let imm = d as u16;
    match imm as u32 == d {
        true => Instrs(vec![ori(r, zero, imm)]).comment("16 bit constant"),
        false => Instrs(vec![lui(t0, (d >> 16) as u16), ori(t0, t0, imm)]),
    }
}
impl Ast<Prog> {
    pub fn codegen(&self) -> Instrs {
        let mut env = Env::new();
        let mut asm = Instrs::new();
        asm.push(mov(fp, sp).comment("move sp to frame pointer"));
        let mut code = self.t.codegen(&mut env);
        asm.append(&mut code);
        asm.push(halt());
        asm
    }
}
impl Prog {
    fn codegen(&self, env: &mut Env) -> Instrs {
        let statements = &self.statements;
        let mut fns = Instrs::new();

        for statement in statements.iter() {
            statement.codegen(env, &mut fns)
        }
        let entry_point = Expr::FuncCall(FuncCall {
            id: Box::new(Expr::Ident("main".to_string())),
            args: Box::<Vec<Expr>>::default(),
        });
        let mut entry_point = entry_point.codegen(env, &mut Instrs::new());
        entry_point.push(halt().comment("Main exit"));
        entry_point.append(&mut fns);
        entry_point
    }
}
impl CodeGen for Static {
    fn codegen(&self, env: &mut Env, fns: &mut Instrs) {
        let replacement = Statement::Let(
            Expr::Ident(self.id.clone()),
            self.mutable,
            Some(self.ty.clone()),
            Some(self.value.clone()),
        );
        let mut new_instrs = replacement.codegen(env, fns, 0, &mut false);
        fns.append(&mut new_instrs)
    }
}

// Code generated ensures that result will be at top of stack
impl Expr {
    fn codegen(&self, env: &mut Env, fns: &mut Instrs) -> Instrs {
        match self {
            Expr::Ident(id) => {
                let offset = env.get_var_offset(id);
                let mut asm = Instrs::new();
                asm.push(lw(t0, offset, fp));
                asm.append(&mut push(t0));
                asm.comment(&format!("load '{}' at offset {}", id, offset))
            }
            Expr::Lit(l) => match l {
                Literal::Bool(b) => {
                    let mut v = li(t0, *b as u32);
                    v.append(&mut push(t0));
                    v.comment(&format!("boolean constant {}", l))
                }
                // for now we don't check if fits in immediate
                Literal::Int(i) => {
                    let mut v = li(t0, *i as u32);
                    v.append(&mut push(t0));
                    v.comment(&format!("integer constant {}", l))
                }
                Literal::String(_) => todo!(),
                Literal::Unit => todo!(),
                _ => todo!(),
            },
            Expr::BinOp(op, lhs, rhs) => {
                let mut bin_op_asm = lhs.codegen(env, fns); // lhs on stack
                bin_op_asm.append(&mut rhs.codegen(env, fns)); // rhs on stack
                bin_op_asm.append(&mut pop(t1)); // rhs
                bin_op_asm.append(&mut pop(t0)); // lhs
                match op {
                    // for now we treat as unsigned to avoid overflow exception
                    BinaryOp::Add => bin_op_asm.push(addu(t0, t0, t1)),
                    BinaryOp::Sub => bin_op_asm.push(subu(t0, t0, t1)),
                    BinaryOp::Mul => todo!(),
                    BinaryOp::Div => todo!(),
                    BinaryOp::And => bin_op_asm.push(and(t0, t0, t1)),
                    BinaryOp::Or => bin_op_asm.push(or(t0, t0, t1)),
                    BinaryOp::Eq => {
                        // t0 = 1 if lhs == rhs else 0
                        bin_op_asm.push(slt(t2, t0, t1)); // t2 = lhs < rhs
                        bin_op_asm.push(slt(t0, t1, t0)); // t0 = rhs > lhs
                        bin_op_asm.push(xor(t0, t0, t2)); // t0 = lhs != rhs
                        bin_op_asm.push(xori(t0, t0, 1));
                    }
                    BinaryOp::Lt => bin_op_asm.push(slt(t0, t0, t1)), // t2 = lhs < rhs
                    BinaryOp::Gt => {
                        bin_op_asm.push(slt(t0, t1, t0)); // rhs < lhs
                    }
                };
                bin_op_asm.append(&mut push(t0));
                bin_op_asm.comment(&format!("op {}", op))
            }
            Expr::Par(e) => e.codegen(env, fns),
            Expr::FuncCall(call) => {
                let (id, args) = (call.id.clone(), call.args.clone());
                let id = match *id.clone() {
                    Expr::Ident(i) => i,
                    _ => unreachable!(),
                };
                let mut call_asm = Instrs::new();

                for arg in args.iter() {
                    call_asm.append(&mut arg.codegen(env, fns).comment(&format!("arg {}", arg)));
                }
                match env.get_fn(&id) {
                    Some(ns) => {
                        call_asm.push(bal_label(&ns).comment(&format!("call {}", id)));
                        if !args.is_empty() {
                            // remove arguments
                            call_asm.append(&mut pop(t0).comment("pop result"));
                            call_asm.push(
                                addiu(sp, sp, 4 * args.len() as i16).comment("remove arguments"),
                            );
                            call_asm.append(&mut push(t0).comment("push back result"));
                        }
                        call_asm
                    }
                    None => panic!("fn {} not found", id),
                }
            }
            Expr::IfThenElse(cond, then_block, else_block) => {
                let mut then_block_asm = then_block.codegen(env, fns, "then").comment("then arm");
                let mut else_block_asm = if let Some(else_block) = else_block {
                    else_block.codegen(env, fns, "else").comment("else arm")
                } else {
                    Instrs::new() // empty else block
                };

                if else_block_asm.len() > 0 {
                    then_block_asm.push(b(else_block_asm.len() as i16))
                }

                let mut ite_asm = Instrs(Vec::new());
                ite_asm.append(&mut cond.codegen(env, fns).comment("condition"));
                ite_asm.append(&mut pop(t0));
                ite_asm.push(beq(t0, zero, then_block_asm.len() as i16));
                ite_asm.append(&mut then_block_asm);
                ite_asm.append(&mut else_block_asm);

                ite_asm
            }
            Expr::Block(b) => b.codegen(env, fns, "expr"),
            // Since we assume type checking has been done before this we simply
            // treat mut an imutable borrows equally
            #[allow(unreachable_code, unused_variables)]
            Expr::UnOp(UnaryOp::Borrow, expr) | Expr::UnOp(UnaryOp::BorrowMut, expr) => {
                todo!();
                let mut asm = Instrs::new();

                let id = match *expr.clone() {
                    Expr::Ident(id) => id,
                    e => {
                        let mut id = 0;
                        while env.get_var(&format!("{id}_borrow")).is_some() {
                            id += 1;
                        }
                        let mut stmt = Statement::Let(
                            Expr::Ident(format!("{id}_borrow")),
                            true,
                            Some(Type::Unit),
                            Some(e),
                        )
                        .codegen(env, fns, 0, &mut true);

                        asm.append(&mut stmt);
                        format!("{id}_borrow")
                    }
                };
                let meta = env.get_var_offset(&id);
                asm.append(&mut li(t0, meta as u32));
                asm
            }
            _ => todo!(),
        }
    }
}

impl Statement {
    fn codegen(&self, env: &mut Env, fns: &mut Instrs, _n: usize, last_expr: &mut bool) -> Instrs {
        fn assign(id: &String, e: &Expr, env: &mut Env, fns: &mut Instrs) -> Instrs {
            let mut asm = e.codegen(env, fns);
            asm.append(&mut pop(t0));
            let offset = env.get_var_offset(id);
            asm.push(sw(t0, offset, fp).comment(&format!("store '{}' at offset {}", id, offset)));
            asm.comment(&format!("'{} = {}'", id, e))
        }

        match self {
            // for now we don't support assignments on references
            Statement::Assign(Expr::Ident(id), e) => assign(id, e, env, fns),
            Statement::Assign(_, _) => {
                panic!("only assignments directly to variables supported")
            }

            Statement::Let(id, _mut, _type, opt_e) => {
                // allocate stack and get expression
                let mut let_asm = Instrs::new();
                let id = match id {
                    Expr::Ident(i) => i,
                    _ => unreachable!(),
                };

                if !env.push_var(id) {
                    // update env with new allocation
                    let_asm.push(addiu(sp, sp, -4).comment(&format!("allocate '{}'", id,)));
                }

                // assign
                if let Some(e) = opt_e {
                    // evaluate expression in old environment
                    let_asm.append(&mut assign(id, e, env, fns));
                }
                let_asm
            }
            Statement::While(while_cond, while_body) => {
                let mut while_asm = Instrs::new();
                let mut cond_asm = while_cond.codegen(env, fns).comment("while cond");
                let mut body_asm = while_body
                    .codegen(env, fns, "while_body")
                    .comment("while body");

                body_asm.append(&mut pop(t0).comment("pop, body is not a result"));
                let body_len = body_asm.len() as i16;

                body_asm.push(b(-body_len - cond_asm.len() as i16 - 2 - 2));
                // branch to end in case while condition is false
                cond_asm.append(&mut pop(t0));
                cond_asm.push(beq(t0, zero, body_asm.len() as i16));

                while_asm.append(&mut cond_asm);
                while_asm.append(&mut body_asm);

                while_asm
            }

            Statement::Expr(e) => {
                *last_expr = true;
                e.codegen(env, fns).comment(&format!("{}", e))
            }
            Statement::FnDecleration(f) => {
                f.codegen(env, fns);
                Instrs::new()
            }
            Statement::Block(b) => {
                *last_expr = true;
                b.codegen(env, fns, "")
            }
        }
    }
}

impl Ast<Block> {
    fn codegen(&self, env: &mut Env, fns: &mut Instrs, ns: &str) -> Instrs {
        self.t.codegen(env, fns, ns)
    }
}

impl Block {
    fn codegen(&self, env: &mut Env, fns: &mut Instrs, ns: &str) -> Instrs {
        //
        // When exiting the block, all locals should be gone
        // The top of the stack will contain the block return value

        if self.statements.is_empty() {
            // an empty block returns the unit value, encoded as 0
            let mut empty_block_asm = Instrs(Vec::new());
            empty_block_asm.append(&mut li(t0, 0).comment("empty block, () return value"));
            empty_block_asm.append(&mut push(t0));
            empty_block_asm
        } else {
            // Each block has a local (nested) scope.
            env.push_scope(ns);
            let enter_offset = env.offset;

            let mut stmts_asm = Instrs(Vec::new());
            let mut last_expr = false;
            for (n, s) in self.statements.iter().enumerate() {
                if last_expr {
                    stmts_asm.append(&mut pop(t0).comment("pop non-last expression"));
                }
                last_expr = false;
                stmts_asm.append(&mut s.codegen(env, fns, n, &mut last_expr));
            }

            if self.semi || !last_expr {
                // ensure we have a unit () value on top of stack
                if last_expr {
                    stmts_asm.append(&mut pop(t0).comment("exit block semi, pop last result"));
                }
                stmts_asm.append(&mut li(t0, 0).comment("exit block semi, () return value"));
                stmts_asm.append(&mut push(t0));
            }

            if enter_offset != env.offset {
                // we have local variables
                stmts_asm.append(&mut pop(t0).comment("exit block, pop block result"));
                stmts_asm.push(
                    addiu(sp, sp, enter_offset - env.offset).comment("exit block, remove locals"),
                );
                stmts_asm.append(&mut push(t0).comment("exit block, push back block result"));
            }

            env.pop_scope();
            env.offset = enter_offset;

            stmts_asm
        }
    }
}
pub trait CodeGen {
    fn codegen(&self, env: &mut Env, fns: &mut Instrs);
}

impl crate::ast::func::Func {
    fn enter(&self) -> Instrs {
        let mut enter_asm = Instrs::new();
        enter_asm.append(&mut push(ra)); //
        enter_asm.append(&mut push(fp)); //
        enter_asm.push(mov(fp, sp)); //
        enter_asm
    }

    fn exit(&self) -> Instrs {
        let mut asm = Instrs::new();
        asm.append(&mut pop(t0).comment("pop return value"));

        asm.push(mov(sp, fp));
        asm.append(&mut pop(fp)); //
        asm.append(&mut pop(ra)); //
        asm.append(&mut push(t0).comment("push back return value"));
        asm.push(jr(ra)); //
        asm.comment(&format!("exit frame 'fn {}'", self.id))
    }
}
impl CodeGen for Func {
    // stack frame layout
    //
    // 16[fp]    arg 1
    // 12[fp]    arg 2
    //  8[fp]    arg 3
    //  4[fp]    ra
    //  0[fp]    old_fp
    // -4[fp]    local 1
    // -8[fp]    local 2, etc.

    fn codegen(&self, env: &mut Env, fns: &mut Instrs) {
        let id = match self.id.clone() {
            Expr::Ident(i) => i,
            _ => unreachable!(),
        };
        // insert function in the current environment
        env.insert_fn(&id);
        // enter a new scope for the function
        let fn_ns = &env.get_fn(&id).unwrap();

        for (offset, parameter) in self.args.iter().rev().enumerate() {
            let id = match parameter.id.clone() {
                Expr::Ident(i) => i,
                _ => unreachable!(),
            };
            env.set_arg_offset(&id, (2 + offset as i16) * 4); // last argument at offset + 2
        }

        let mut asm = self.enter().label(fn_ns).comment(&format!(
            "enter frame 'fn {}{}'",
            self.id,
            self.args
                .iter()
                .map(|el| el.to_string())
                .collect::<Vec<String>>()
                .join(",")
        ));

        // generate code for the body
        let offset = env.offset;
        env.offset = 0;
        asm.append(&mut self.body.codegen(env, fns, &id));
        env.offset = offset;
        // the exit block
        asm.append(&mut self.exit());
        fns.append(&mut asm);
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;

    use super::*;
    use mips::{error::Error, vm::Mips};

    // test the frame management
    #[test]
    fn test_frame() {
        let mut env = Env::new();
        println!("push_scope");
        env.push_scope("top");
        println!("env {:?}", env);

        env.insert_fn("a");
        env.insert_fn("b");
        println!("env {:?}", env);

        env.push_scope("a");

        println!("env {:?}", env);
        env.insert_fn("a");
        println!("env {:?}", env);

        let f = env.get_fn("a");
        println!("f {:?}", f);
        let f = env.get_fn("b");
        println!("f {:?}", f);
        let f = env.get_fn("c");
        println!("f {:?}", f);

        env.pop_scope();

        let f = env.get_fn("a");
        println!("f {:?}", f);
        let f = env.get_fn("b");
        println!("f {:?}", f);
        let f = env.get_fn("c");
        println!("f {:?}", f);
    }

    #[test]
    fn run_assert() {
        let mut asm = Instrs::new();
        asm.push(addi(t0, t0, 1));
        asm.push(addi(t0, t0, 1));
        asm = asm.assert(|mips| mips.rf.get(t0) == 2);
        asm.push(halt());

        println!("{}", asm);
        let mut mips = Mips::new(Instrs::new_from_slice(&asm));
        let r = mips.run();
        assert_eq!(r, Err(Error::Halt));
    }
    #[test]
    fn test_simple_program() {
        mips_test_prog(
            "
fn a() -> i32{
    2
}
fn main(){
    a();
}
",
        )
    }

    // helper to test expressions
    fn mips_test_prog(prog: &str) {
        let prog: Ast<Prog> = prog.to_string().into();
        let asm = prog.codegen();
        println!("codegen\n{}", asm);
        let mut mips = Mips::new(Instrs::new_from_slice(&asm));
        let _ = mips.run();
        let t0_v = mips.rf.get(t0) as i32;
        println!("e {}", t0_v);
        assert_eq!(t0_v, 0);
        let sp_v = mips.rf.get(sp);
        println!("sp {:x}", sp_v);
        assert_eq!(sp_v, 0x7fff_fffc);
    }

    // helper to test expressions
    fn mips_test_expr(expr: &str, assert_val: i32) {
        let ts: proc_macro2::TokenStream = expr.parse().unwrap();
        let expr: Expr = syn::parse2(ts).unwrap();
        println!("testing : {expr}");
        let mut env = Env::new();
        let mut asm = Instrs::new();
        let fns = &mut Instrs::new();
        asm.push(mov(fp, sp).comment("move sp to frame pointer"));
        asm.append(&mut expr.codegen(&mut env, fns));
        asm.push(halt());
        println!("codegen\n{}", asm);
        let mut mips = Mips::new(Instrs::new_from_slice(&asm));
        let _ = mips.run();
        let t0_v = mips.rf.get(t0) as i32;
        println!("e {}", t0_v);
        assert_eq!(t0_v, assert_val);
        let sp_v = mips.rf.get(sp);
        println!("sp {:x}", sp_v);
        assert_eq!(sp_v, 0x7fff_fffc);
    }

    #[test]
    fn mips_lit_bool_false() {
        mips_test_expr("false", 0);
    }

    #[test]
    fn mips_lit_bool_true() {
        mips_test_expr("true", 1);
    }

    #[test]
    fn mips_big_i32() {
        mips_test_expr("0x1234_5678", 0x1234_5678);
    }

    #[test]
    fn mips_small_i32() {
        mips_test_expr("0x1234", 0x1234);
    }

    #[test]
    fn mips_expr_int() {
        mips_test_expr("2-5", 2 - 5);
    }

    #[test]
    fn mips_expr_int2() {
        // notice, we do not climb here
        mips_test_expr("(2 - 5) - 7", (2 - 5) - 7);
    }

    #[test]
    fn mips_expr_int_paren() {
        // notice, we do not climb here
        mips_test_expr("2 - (2 + 3)", 2 - (2 + 3));
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn mips_expr_int_paren2() {
        mips_test_expr("(2 - 2) + 3", (2 - 2) + 3);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn mips_expr_bool1() {
        mips_test_expr("false || true", (false || true) as i32);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn mips_expr_bool2() {
        mips_test_expr("false && true", (false && true) as i32);
    }

    #[test]
    fn mips_test_ite1() {
        mips_test_expr("if true { 42 } else { 1337 }", 42);
    }

    #[test]
    fn mips_test_ite2() {
        mips_test_expr("if false { 42 } else { 1337 }", 1337);
    }

    #[test]
    fn mips_test_ite3() {
        // notice, we do not climb here
        mips_test_expr(
            "if false || true { 42 + 15 - 67 } else { (1337 - 2) - 1}",
            42 + 15 - 67,
        );
    }

    #[test]
    fn mips_test_ite4() {
        // notice, we do not climb here
        mips_test_expr(
            "if false && true { 42 + 15 - 67 } else { (1337 - 2) - 1}",
            1337 - 2 - 1,
        );
    }

    // helper function to test block
    fn test_block(block: &str, assert_val: i32) {
        let ts: proc_macro2::TokenStream = block.parse().unwrap();
        let block: Block = syn::parse2(ts).unwrap();
        println!("Evaluating {block}");
        let mut env = Env::new();
        // start in a new scope
        let mut asm = Instrs::new();
        let fns = &mut Instrs::new();
        asm.push(mov(fp, sp).comment("move sp to frame pointer"));
        asm.append(&mut block.codegen(&mut env, fns, "top"));
        asm.push(halt());

        println!("codegen\n{}", asm);
        let mut mips = Mips::new(Instrs::new_from_slice(&asm));
        let _ = mips.run();
        let to_v = mips.rf.get(t0) as i32;
        println!("semi {}", block.semi);
        println!("{:#10X?}", mips.dm);

        println!("e {}", to_v);
        if !block.semi {
            assert_eq!(to_v, assert_val)
        };
        let sp_v = mips.rf.get(sp);
        println!("sp {:x}", sp_v);

        assert_eq!(sp_v, 0x7fff_fffc);
    }

    #[test]
    fn mips_block_return_unit() {
        // even an empty block has a return value of unit type
        test_block(
            "
        {
            
        }",
            0,
        );
    }

    #[test]
    fn mips_block_return_15() {
        test_block(
            "
        {
            15
        }",
            15,
        );
    }

    #[test]
    fn mips_block_return_local_15() {
        test_block(
            "
        {
            let a = 2; // one allocation
            15
        }",
            15,
        );
    }

    #[test]
    fn mips_block_return_local_local_15() {
        test_block(
            "
        {
            let a = 2; // two allocations
            let b = 4;
            15
        }",
            15,
        );
    }

    #[test]
    fn mips_block_return_ite() {
        test_block(
            "
        {
            if true { 15 } else { 42 }
        }",
            15,
        );
    }

    #[test]
    fn mips_block_return_15_unit() {
        test_block(
            "
        {
            15;
        }",
            0,
        );
    }

    #[test]
    fn mips_block_id_fail() {
        test_block(
            "
        {
            let a = 5;
            a
        }",
            5,
        );
    }

    #[test]
    fn mips_block_return_nested() {
        test_block(
            "
        {
            {
                15
            }
        }",
            15,
        );
    }

    #[test]
    fn mips_let_and_get1() {
        test_block(
            "
        {
            let a = 5;
            let b = 10;
            a + b
        }",
            15,
        );
    }

    #[test]
    fn mips_let_and_get2() {
        test_block(
            "
        {
            let a = 5;
            let b = a + 1;
            a + b
        }",
            11,
        );
    }

    #[test]
    fn mips_let_and_get_nested() {
        test_block(
            "
        {
            let a = {
                let a = 5;
                let b = a + 2;
                a + b
            };
            a
        }",
            12,
        );
    }

    #[test]
    fn mips_let_and_get_nested_unit() {
        test_block(
            "
        {
            let a = {
                let a = 5;
                let b = a + 2;
                a + b;
            };
            a
        }",
            0,
        );
    }

    #[test]
    fn mips_let_and_shadow() {
        test_block(
            "
        {
            let a = 5;
            let a = a + 1; // introduce shadow
            a 
        }",
            6,
        );
    }

    #[test]
    fn mips_let_and_assign() {
        test_block(
            "
        {
            let mut a = 5;
            a = a + 1;
            a
        }",
            6,
        );
    }

    #[test]
    fn mips_inner_block() {
        test_block(
            "
        {
            let mut a = {
                1
            };
            let b = a;
            a
        }",
            1,
        );
    }

    #[test]
    fn mips_inner_block2() {
        test_block(
            "
        {
            let mut a = 5;
            let b = {
                let mut b; // shadow
                a = a + 1; // mutate outer scope
                b = a + 2;
                b
            };
            b 
        }",
            8,
        );
    }

    #[test]
    fn mips_block_ite_then() {
        test_block(
            "
        {
            let a = 5;
            if true { a } else { a + 1 }
        }",
            5,
        );
    }

    #[test]
    fn mips_block_ite_else() {
        test_block(
            "
        {
            let a = 5;
            if false { a } else { a + 1 }
        }",
            6,
        );
    }

    #[test]
    fn mips_block_semis() {
        test_block(
            "
        {
            4;
            5;
        }",
            0,
        );
    }

    #[test]
    fn mips_block_ite_5() {
        test_block(
            "
        {
            if false { 1 } else { 2 }; // this is evaluated as expression
            5
        }",
            5,
        );
    }

    #[test]
    fn mips_block_while_once() {
        test_block(
            "
        {
            let mut a = 0;
            let mut b = true;
            while b {
                a = a + 20;
                b = false; // just iterate once
            };
            a
        }",
            20,
        );
    }

    #[test]
    fn mips_block_while_sum() {
        test_block(
            "
        {
            let mut i = 3;
            let mut sum = 0;
            while i > 0 {
                sum = sum + i;
                i = i - 1;
            };
            sum
        }",
            6,
        );
    }

    #[test]
    fn mips_block_while_sum_nesting() {
        test_block(
            "
        {
            let mut i = 3;
            let mut sum = 0;
            while i > 0 {
                sum = { 
                    let b = sum + i;
                    b
                };
                i = i - 1;
            };
            sum
        }",
            6,
        );
    }

    #[test]
    #[allow(clippy::assign_op_pattern)]
    #[allow(clippy::let_and_return)]
    fn rust_block_while_sum_nesting() {
        let res = {
            let mut i = 3;
            let mut sum = 0;
            while i > 0 {
                sum = {
                    let b = sum + i;
                    b
                };
                i = i - 1;
            }
            sum
        };

        println!("res {}", res);
        assert_eq!(res, 3 + 2 + 1);
    }

    #[test]
    fn mips_block_while_simple() {
        test_block(
            "
        {
            let mut sum = 0;
            sum = sum + 3;
            sum = sum + 2;
            sum = sum + 1;
            sum
        }",
            3 + 2 + 1,
        );
    }

    // helper function to test fn
    fn mips_test_fn(block: &str, assert_val: i32) {
        let block = block.to_string();
        let block = parse!(block, Block);
        println!("{block}");
        let mut env = Env::new();
        let fns = &mut Instrs::new();
        let mut asm = Instrs::new();
        asm.push(mov(fp, sp).comment("move sp to frame pointer"));
        asm.append(&mut block.codegen(&mut env, fns, "top"));
        asm.push(halt());
        asm.append(fns);

        println!("codegen mips_test_fn\n{}", asm);

        let mut mips = Mips::new(Instrs::new_from_slice(&asm));
        let _ = mips.run();
        let to_v = mips.rf.get(t0) as i32;

        println!("e {}", to_v);

        assert_eq!(to_v, assert_val);

        let sp_v = mips.rf.get(sp);
        println!("sp {:x}", sp_v);

        assert_eq!(sp_v, 0x7fff_fffc);
    }

    #[test]
    fn mips_test_fn_unit() {
        mips_test_fn(
            "
            {
                fn a() {
                };

                fn b() {
                }              
            }
        ",
            0,
        )
    }

    #[test]
    fn mips_test_fn_42() {
        mips_test_fn(
            "
            {
                fn a() -> i32 {
                    42
                };
            }
        ",
            0,
        )
    }

    #[test]
    fn mips_test_fn_arg_42() {
        mips_test_fn(
            "
            {
                fn a(x: i32) -> i32 {
                    42 + x
                };
            }
        ",
            0,
        )
    }

    #[test]
    fn mips_test_fn_args_42() {
        mips_test_fn(
            "
            {
                fn a(x: i32, y: i32) -> i32 {
                    42 + x + y
                };
            }
        ",
            0,
        )
    }

    #[test]
    fn mips_test_fn_call() {
        mips_test_fn(
            "
            {
                fn a() {
                };

                a()
            }
        ",
            0,
        )
    }

    #[test]
    fn mips_test_fn_call_3() {
        mips_test_fn(
            "
            {
                fn a() -> i32 {
                    3
                };

                a()
            }
        ",
            3,
        )
    }

    #[test]
    fn mips_test_fn_call_5_3() {
        mips_test_fn(
            "
            {
                fn a(x:i32, y: i32) -> i32 {
                    x - y
                };

                a(5, 3)
            }
        ",
            2,
        )
    }

    #[test]
    fn mips_test_fn_call_a_x() {
        mips_test_fn(
            "
            {
                fn a() -> i32 {
                    fn b() -> i32 {
                        4
                    };

                    b()
                };

                a()
            }
        ",
            4,
        )
    }

    #[test]
    fn mips_test_fn_a() {
        mips_test_fn(
            "
            {
                fn a() -> i32 {
                    3
                };

                a()
            }
        ",
            3,
        )
    }

    #[test]
    fn mips_test_fn_a_b() {
        mips_test_fn(
            "
            {
                fn a() {
                    3
                };

                fn b() {
                    4
                };

                a() + b()
            }
        ",
            3 + 4,
        )
    }

    #[test]
    #[should_panic]
    fn mips_test_fn_a_a() {
        mips_test_fn(
            "
            {
                fn a() {
                    3
                };

                fn a() {
                    4
                };

                a() 
            }
        ",
            4,
        )
    }

    #[test]
    fn mips_test_fn_a_call_b() {
        mips_test_fn(
            "
            {
                fn b() {
                    4
                };

                fn a() -> i32 {
                    3 + 5 + b()
                };     

                a()
            }
        ",
            3 + 5 + 4,
        )
    }

    #[test]
    fn mips_test_fn_nested() {
        mips_test_fn(
            "{
                fn a(x: i32) -> i32 {

                    fn b(y: i32) -> i32 {
                        3 + y
                    };

                    fn a() -> i32 {
                        5 + b(1)
                    };

                    x + b(x + 1) + a()
                };

                a(4) 
            }
        ",
            3 + 4 + 1 + 4 + 5 + 4,
        )
    }

    #[test]
    fn mips_test_fn_rec() {
        mips_test_fn(
            "
            {
                fn sum(x: i32) -> i32 {
                    if x > 0 {
                        sum(x - 1) + x
                    } else {
                        0
                    }
                };

                sum(3)
            }
        ",
            6,
        )
    }

    #[test]
    fn mips_test_fn_a_expr() {
        mips_test_fn(
            "
            {
                fn a(x: i32, y: i32) {
                    x + y
                };

                a(3, 4)
            }
        ",
            3 + 4,
        )
    }

    #[test]
    fn rust_fn_nest() {
        let res = {
            fn f(x: i32) -> i32 {
                fn f() -> i32 {
                    fn g() -> i32 {
                        7
                    }

                    g()
                }
                {
                    fn f() {}
                }
                x + f()
            }

            f(3)
        };

        println!("res {}", res);
        assert_eq!(res, 3 + 7);
    }

    #[test]
    fn mips_block_fn_def() {
        mips_test_fn(
            "
        {
            fn f(x: i32) -> i32 {
                fn f() -> i32 { // shadowing outer declaration
                    fn g() -> i32 {
                        7
                    };
    
                    g()
                };
                {
                    fn f() {} // defined in local scope
                };
                3 + f()
            };

            f(3)
        }",
            3 + 7,
        );
    }
}
