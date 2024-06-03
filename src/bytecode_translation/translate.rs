use crate::ast::*;
use crate::error::{Result, Error};
use crate::vm::OpCode;

use super::environment::Environment;
use super::TransByteCode;

const PCMASK: usize = 0x0000FFFF;
const EXTMASK:usize = 0xFFFF0000;
const SHIFT:  usize = 16;

impl TransByteCode for CompUnit {
    fn translate_byte(
        &mut self,
        env: &mut Environment,
        extension: usize,
    ) -> Result<Vec<OpCode>> {
        let mut func_def_code = Vec::new();
        let mut global_decl_code = Vec::new();
        let mut totle_ins = 0;
        func_def_code.push(OpCode::Call(0, 0));
        // self.globaldefs.sort_by(|a, b| {
        //     match (a, b) {
        //         (GlobalDef::Decl(_), GlobalDef::FuncDef(_)) => std::cmp::Ordering::Less,
        //         (GlobalDef::FuncDef(_), GlobalDef::Decl(_)) => std::cmp::Ordering::Greater,
        //         _ => std::cmp::Ordering::Equal
        //     }
        // });
        for global_def in self.globaldefs.clone() {
            match global_def {
                GlobalDef::FuncDef(mut funcdef) => {
                    if funcdef.ident == "main" {
                        func_def_code[0] = OpCode::Call(totle_ins, 0);
                        env.push_pc(usize::MAX);
                        env.new_scope();
                    }
                    func_def_code.extend(funcdef.translate_byte(env, extension + totle_ins)?);
                    if funcdef.ident == "main" && !env.pc_stack_empty() {
                        env.leave_scope();
                        func_def_code.push(OpCode::LoadConst(env.pop_pc() as i32));
                        func_def_code.push(OpCode::Ret);
                    }
                }
                GlobalDef::Decl(mut decl) => {
                    global_decl_code.extend(decl.translate_byte(env, extension | (1 << SHIFT))?);
                }
            }
            totle_ins = global_decl_code.len() + func_def_code.len();
        }
        let mut result_code = Vec::new();
        result_code.extend(global_decl_code);
        result_code.extend(func_def_code);
        Ok(result_code)
    }
}

impl TransByteCode for FuncDef {
    fn translate_byte(&mut self, env: &mut Environment, mut extension: usize) -> Result<Vec<OpCode>> {
        extension += 2 << SHIFT;
        let mut res = Vec::new();
        let id = self.ident.clone();
        let addr = extension & PCMASK;
        let dep = extension >> (SHIFT + 1);
        env.new_scope();
        let mut args = vec![];
        if let Some(params) = &self.funcfparams {
            for arg in params.params.clone() {
                args.push(arg.ident.clone());
                env.new_val(arg.ident.clone(), dep)
            }
        }
        let temp = self.block.translate_byte(env, extension)?;
        env.new_func(id, addr, args);
        env.leave_scope();
        res.extend(temp);
        Ok(res)
    }
}

impl TransByteCode for Block {
    fn translate_byte(&mut self, env: &mut Environment, mut extension: usize) -> Result<Vec<OpCode>> {
        extension += (2 << SHIFT) + 1;
        let mut res = vec![OpCode::EnterScope(0)];
        env.new_scope();
        for mut item in self.items.clone() {
            let temp = item.translate_byte(env, extension)?;
            extension += temp.len();
            res.extend(temp)
        }
        res.push(OpCode::LeaveScope);
        env.leave_scope();
        Ok(res)
    }
}

impl TransByteCode for BlockItem {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        match self {
            BlockItem::Decl(decl) => decl.translate_byte(env, extension),
            BlockItem::Stmt(stmt) => stmt.translate_byte(env, extension),
        }
    }
}

impl TransByteCode for Stmt {
    fn translate_byte(&mut self, env: &mut Environment, mut extension: usize) -> Result<Vec<OpCode>> {
        let mut res = Vec::new();
        match self {
            Stmt::Assign(lval, exp) => {
                res.extend(exp.translate_byte(env, extension)?);
                let (dep, pos) = env.get_val(lval.ident.clone())?;
                if dep == 0 {
                    res.push(OpCode::StoreGlobal(pos));
                } else {
                    res.push(OpCode::StoreVar(dep, pos));
                }
            },
            Stmt::Block(block) => {
                res.extend(block.translate_byte(env, extension)?);
            },
            Stmt::Exp(exp) => {
                if let Some(exp) = exp {
                    res.extend(exp.translate_byte(env, extension)?);
                }
            },
            Stmt::Ret(ret) => {
                if let Some(exp) = ret {
                    res.extend(exp.translate_byte(env, extension)?);
                }
                res.extend(vec![OpCode::Ret]);
            },
            Stmt::If { condition, then_branch, else_branch } => {
                res.extend(condition.translate_byte(env, extension)?);
                let temp = then_branch.translate_byte(env, extension)?;
                res.push(OpCode::JmpFalse(0));
                extension += temp.len() + res.len();
                let iter = res.len() - 1;
                res.extend(temp);
                let jumpto = extension & PCMASK;
                res[iter] = OpCode::JmpFalse(jumpto);
                if let Some(else_branch) = else_branch {
                    res.extend(else_branch.translate_byte(env, extension)?);
                }
            },
            Stmt::While { condition, loopbody } => {
                env.push_pc(extension & PCMASK);
                res.extend(condition.translate_byte(env, extension)?);
                let temp = loopbody.translate_byte(env, extension)?;
                res.push(OpCode::JmpFalse(0));
                extension += temp.len() + res.len() + 1;
                let iter = res.len() - 1;
                res.extend(temp);
                let jumpto = extension & PCMASK;
                res[iter] = OpCode::JmpFalse(jumpto);
                res.push(OpCode::Jmp(env.pc_stack_top()));
                env.pop_pc();
            },
            Stmt::FuncDef(_) => todo!(),
            Stmt::Continue => {
                assert!(!env.pc_stack_empty());
                res.push(OpCode::LeaveScope);
                res.push(OpCode::Jmp(env.pc_stack_top()));
            },
            Stmt::Break => {
                assert!(!env.pc_stack_empty());
                res.push(OpCode::LeaveScope);
            },
        };
        Ok(res)
    }
}

impl TransByteCode for Decl {
    fn translate_byte(
        &mut self,
        env: &mut Environment,
        ext: usize, // xxx0 局部， 0001 全局
    ) -> Result<Vec<OpCode>> {
        match self {
            Decl::VarDecl(decl) => decl.translate_byte(env, ext),
            Decl::ValDecl(decl) => decl.translate_byte(env, ext),
        }
    }
}

impl TransByteCode for VarDecl {
    fn translate_byte(
        &mut self, 
        env: &mut Environment,
        extension: usize,
    ) -> Result<Vec<OpCode>> {
        let mut res = Vec::new();
        if extension & EXTMASK == (1 << SHIFT) {
            res.extend(self.initval.translate_byte(env, extension)?);
            res.push(OpCode::StoreGlobal(env.get_values_count(0)));
            env.new_val(self.ident.clone(), 0); 
        } else {
            let dep = extension >> (SHIFT + 1);
            while env.get_dep() < dep {
                env.new_scope();
            }
            res.extend(self.initval.translate_byte(env, extension)?);
            // println!("!!! {} {}", self.ident, env.get_values_count(dep));
            res.push(OpCode::StoreVar(dep, env.get_values_count(dep)));
            env.new_val(self.ident.clone(), dep);
        }
        return Ok(res);
    }
}

impl TransByteCode for ValDecl {
    fn translate_byte(
        &mut self,
        env: &mut Environment,
        extension: usize,
    ) -> Result<Vec<OpCode>> {
        let mut res = Vec::new();
        if extension & EXTMASK == (1 << SHIFT) {
            res.extend(self.initval.translate_byte(env, extension)?);
            res.push(OpCode::StoreGlobal(env.get_values_count(0)));
            env.new_val(self.ident.clone(), 0);
        } else {
            let dep = extension >> (SHIFT + 1);
            while env.get_dep() < dep {
                env.new_scope();
            }
            res.extend(self.initval.translate_byte(env, extension)?);
            res.push(OpCode::StoreVar(dep, env.get_values_count(dep)));
            env.new_val(self.ident.clone(), dep);
        }
        return Ok(res);
    }
}

impl TransByteCode for Exp {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        self.lor_exp.translate_byte(env, extension)
    }
}

impl TransByteCode for InitVal {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        self.exp.translate_byte(env, extension)
    }
}

impl TransByteCode for LOrExp {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        match self {
            LOrExp::And(and) => and.translate_byte(env, extension),
            LOrExp::Or(lhs , rhs) => {
                let mut res = lhs.translate_byte(env, extension)?;
                let rexp = rhs.translate_byte(env, extension)?;
                res.extend(rexp);
                res.push(OpCode::BinOpOr);
                Ok(res)
            },
        }
    }
}

impl TransByteCode for LAndExp {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        match self {
            LAndExp::Eq(eq) => eq.translate_byte(env, extension),
            LAndExp::And(lhs, rhs) => {
                let mut res = lhs.translate_byte(env, extension)?;
                let rexp = rhs.translate_byte(env, extension)?;
                res.extend(rexp);
                res.push(OpCode::BinOpAnd);
                Ok(res)
            },
        }
    }
}

impl TransByteCode for EqExp {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        match self {
            EqExp::Rel(rel) => rel.translate_byte(env, extension),
            EqExp::Eq(lhs, op, rhs) => {
                let mut res = lhs.translate_byte(env, extension)?;
                let rexp = rhs.translate_byte(env, extension)?;
                res.extend(rexp);
                match op {
                    BinaryOp::Eq  => res.push(OpCode::BinOpEq),
                    BinaryOp::Neq => res.push(OpCode::BinOpNe),
                    _ => return  Err(Error::TranslateError(format!("EqExp Error")))
                }
                Ok(res)
            },
        }
    }
}


impl TransByteCode for RelExp {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        match self {
            RelExp::Add(add) => add.translate_byte(env, extension),
            RelExp::Rel(lhs, op, rhs) => {
                let mut res = lhs.translate_byte(env, extension)?;
                let rexp = rhs.translate_byte(env, extension)?;
                res.extend(rexp);
                match op {
                    BinaryOp::Lt  => res.push(OpCode::BinOpLt),
                    BinaryOp::Leq => res.push(OpCode::BinOpLe),
                    BinaryOp::Gt  => res.push(OpCode::BinOpGt),
                    BinaryOp::Geq => res.push(OpCode::BinOpGe),
                    _ => return  Err(Error::TranslateError(format!("RelExp Error")))
                }
                Ok(res)
            },
        }
    }
}

impl TransByteCode for AddExp {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        match self {
            AddExp::Mul(mul) => mul.translate_byte(env, extension),
            AddExp::Add(lhs, op, rhs) => {
                let mut res = lhs.translate_byte(env, extension)?;
                let rexp = rhs.translate_byte(env, extension)?;
                res.extend(rexp);
                match op {
                    BinaryOp::Add => res.push(OpCode::BinOpAdd),
                    BinaryOp::Sub => res.push(OpCode::BinOpSub),
                    _ => return  Err(Error::TranslateError(format!("AddExp Error")))
                }
                Ok(res)
            },
        }
    }
}

impl TransByteCode for MulExp {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        match self {
            MulExp::Unary(una) => una.translate_byte(env, extension),
            MulExp::Mul(lhs, op, rhs) => {
                let mut res = lhs.translate_byte(env, extension)?;
                let rexp = rhs.translate_byte(env, extension)?;
                res.extend(rexp);
                match op {
                    BinaryOp::Mul => res.push(OpCode::BinOpMul),
                    BinaryOp::Div => res.push(OpCode::BinOpDiv),
                    _ => return  Err(Error::TranslateError(format!("MulExp Error")))
                }
                Ok(res)
            },
        }
    }
}

impl TransByteCode for UnaryExp {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        match self {
            UnaryExp::Pri(pri) => pri.translate_byte(env, extension),
            UnaryExp::Unary(op, una) => {
                let mut res = una.translate_byte(env, extension)?;
                match op {
                    UnaryOp::Neg => res.push(OpCode::UniOpNeg),
                    UnaryOp::Not => res.push(OpCode::UniOpNot),
                }
                Ok(res)
            },
            UnaryExp::FuncCall { ident, funcrparams } => {
                let mut res = Vec::new();
                if ident == "print" {
                    if let Some(funcrparams) = funcrparams {
                        for mut exp in funcrparams.exps.clone() {
                            res.extend(exp.translate_byte(env, extension)?);
                            res.push(OpCode::PrintItem);
                            res.push(OpCode::PrintNewline);
                        }
                    }
                } else if ident == "getint" {

                } else {
                    if let Ok((pc, mut args)) = env.get_func_addr(ident.to_string()) {
                        env.new_scope();
                        let len = args.len();
                        // let curdep = (extension >> (SHIFT + 1)) + 1;
                        // if let Some(funcrparams) = funcrparams {
                        //     funcrparams.exps.reverse();
                        //     for mut exp in funcrparams.exps.clone() {
                        //         res.extend(exp.translate_byte(env, extension)?);
                        //     }
                        // }
                        // for arg in args {
                        //     env.new_val(arg, curdep);

                        // }
                        res.push(OpCode::Call(pc, len));
                    }
                }
                Ok(res)
            },
        }
    }
}

impl TransByteCode for PrimaryExp {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        match self {
            PrimaryExp::Exp(exp) => exp.translate_byte(env, extension),
            PrimaryExp::Number(num) => {
                Ok(vec![OpCode::LoadConst(num.clone())])
            },
            PrimaryExp::LVal(lval) => lval.translate_byte(env, extension),
        }
    }
}


impl TransByteCode for LVal {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        let (dep, pos) = env.get_val(self.ident.clone())?;
        if dep == 0 {
            Ok(vec![OpCode::LoadGlobal(pos)])
        } else {
            Ok(vec![OpCode::LoadVar(dep, pos)])
        }
    }
}