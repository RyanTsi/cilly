use crate::ast::*;
use crate::error::{Result, Error};
use crate::vm::OpCode;

use super::environment::Environment;
use super::TransByteCode;

const PCMASK: usize = 0x0000FFFF;
const SHIFT:  usize = 16;


pub fn get_ext(dep: usize, in_global: usize, totle_ins: usize) -> usize{
    (dep << (SHIFT + 1)) | (in_global << SHIFT) | totle_ins
}

pub fn split(extension: usize) -> (usize, usize, usize) {
    (extension >> (SHIFT + 1), (extension >> SHIFT) & 1, extension & PCMASK)
}

impl TransByteCode for CompUnit {
    fn translate_byte(
        &mut self,
        env: &mut Environment,
        extension: usize,
    ) -> Result<Vec<OpCode>> {
        let mut func_def_code = Vec::new();
        let mut global_decl_code = Vec::new();
        let mut result_code = Vec::new();
        let (mut dep, mut in_global, mut totle_ins) = split(extension);

        for global_def in self.globaldefs.clone() {
            match global_def {
                GlobalDef::FuncDef(mut funcdef) => {
                    in_global = 0;
                    dep = 1;
                    if func_def_code.is_empty() { // main 入口
                        func_def_code.push(OpCode::Call(usize::MAX, 0));
                        totle_ins += 1;
                    }
                    if funcdef.ident == "main" {
                        func_def_code[0] = OpCode::Call(totle_ins, 0);
                        env.new_scope();
                    }
                    func_def_code.extend(funcdef.translate_byte(env, get_ext(dep, in_global, totle_ins))?);
                    if funcdef.ident == "main"{
                        env.leave_scope();
                    }
                }
                GlobalDef::Decl(mut decl) => {
                    in_global = 1;
                    dep = 0;
                    global_decl_code.extend(decl.translate_byte(env, get_ext(dep, in_global, totle_ins))?);
                }
            }
            totle_ins = global_decl_code.len() + func_def_code.len();
        }
        result_code.extend(global_decl_code);
        result_code.extend(func_def_code);
        // println!("{:?}", env);
        Ok(result_code)
    }
}

impl TransByteCode for FuncDef {
    fn translate_byte(&mut self, env: &mut Environment, mut extension: usize) -> Result<Vec<OpCode>> {
        let (mut dep, mut in_global, mut addr) = split(extension);
        let mut res = Vec::new();
        let id = self.ident.clone();
        env.new_scope(); // args scope
        dep = 0;
        let mut args = vec![];
        if let Some(params) = &self.funcfparams {
            for arg in params.params.clone() {
                args.push(arg.ident.clone());
                env.new_val(arg.ident.clone(), dep)
            }
        }
        dep = 1;
        env.new_func(id, addr, args);
        let temp = self.block.translate_byte(env, get_ext(dep, in_global, addr))?;
        env.leave_scope();
        res.extend(temp);
        Ok(res)
    }
}

impl TransByteCode for Block {
    fn translate_byte(&mut self, env: &mut Environment, mut extension: usize) -> Result<Vec<OpCode>> {
        let (mut dep, mut in_global, mut addr) = split(extension);
        let mut res = Vec::new();
        if dep == 1 {
            res.push(OpCode::EnterScope(0));
            env.new_scope();
        }
        for mut item in self.items.clone() {
            let temp = item.translate_byte(env, get_ext(0, in_global, addr))?;
            addr += temp.len();
            res.extend(temp)
        }
        if dep == 1 {
            res.push(OpCode::LeaveScope);
            env.leave_scope();
        }
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
                if env.is_bottom(dep) {
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
                extension += temp.len() + res.len() + 1;
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
                extension += temp.len() + res.len() + 2;
                let iter = res.len() - 1;
                res.extend(temp);
                let jumpto = extension & PCMASK;
                res[iter] = OpCode::JmpFalse(jumpto);
                res.push(OpCode::Jmp(env.pc_stack_top() + 1));
                env.pop_pc();
            },
            Stmt::FuncDef(_) => todo!(),
            Stmt::Continue => {
            },
            Stmt::Break => {
            },
        };
        Ok(res)
    }
}

impl TransByteCode for Decl {
    fn translate_byte(
        &mut self,
        env: &mut Environment,
        extension: usize,
    ) -> Result<Vec<OpCode>> {
        
        match self {
            Decl::VarDecl(decl) => decl.translate_byte(env, extension),
            Decl::ValDecl(decl) => decl.translate_byte(env, extension),
        }
    }
}

impl TransByteCode for VarDecl {
    fn translate_byte(
        &mut self, 
        env: &mut Environment,
        extension: usize,
    ) -> Result<Vec<OpCode>> {
        let (mut dep, mut in_global, mut addr) = split(extension);
        dep = 0;
        let mut res = Vec::new();
        if in_global == 1 {
            res.extend(self.initval.translate_byte(env, get_ext(dep, in_global, addr))?);
            res.push(OpCode::StoreGlobal(env.get_values_count(0)));
            env.new_val(self.ident.clone(), 0); 
        } else {
            res.extend(self.initval.translate_byte(env, get_ext(dep, in_global, addr))?);
            // println!("!!! {} {}", self.ident, env.get_values_count(dep));
            res.push(OpCode::StoreVar(dep, env.get_values_count(0)));
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
        let (mut dep, mut in_global, mut addr) = split(extension);
        let mut res = Vec::new();
        if in_global == 1 {
            res.extend(self.initval.translate_byte(env, extension)?);
            res.push(OpCode::StoreGlobal(env.get_values_count(0)));
            env.new_val(self.ident.clone(), 0);
        } else {
            res.extend(self.initval.translate_byte(env, extension)?);
            res.push(OpCode::StoreVar(dep, env.get_values_count(0)));
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
                    res.push(OpCode::GetInt);
                } else {
                    if let Ok((pc, mut args)) = env.get_func_addr(ident.to_string()) {
                        if let Some(funcrparams) = funcrparams {
                            funcrparams.exps.reverse();
                            for mut exp in funcrparams.exps.clone() {
                                res.extend(exp.translate_byte(env, extension)?);
                            }
                        }
                        res.push(OpCode::Call(pc, args.len()));
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
        if env.is_bottom(dep) {
            Ok(vec![OpCode::LoadGlobal(pos)])
        } else {
            Ok(vec![OpCode::LoadVar(dep, pos)])
        }
    }
}

pub fn translate_from(bytes: Vec<usize>) -> Vec<OpCode> {
    let mut res = Vec::new();
    let mut cur = 0;
    while cur < bytes.len() {
        match bytes[cur] {
            1 => {
                cur += 1;
                let val = bytes[cur] as i32;
                res.push(OpCode::LoadConst(val));
            }
            2 => res.push(OpCode::LoadTrue),
            3 => res.push(OpCode::LoadFalse),
            4 => res.push(OpCode::LoadNull),
            5 => {
                cur += 1;
                res.push(OpCode::LoadGlobal(bytes[cur]));
            }
            6 => {
                cur += 1;
                res.push(OpCode::StoreGlobal(bytes[cur]));
            }
            10 => {
                cur += 1;
                res.push(OpCode::Jmp(bytes[cur]));
            }
            11 => {
                cur += 1;
                res.push(OpCode::JmpTrue(bytes[cur]));
            }
            12 => {
                cur += 1;
                res.push(OpCode::JmpFalse(bytes[cur]));
            }
            13 => res.push(OpCode::PrintItem),
            14 => res.push(OpCode::PrintNewline),
            15 => res.push(OpCode::GetInt),
            16 => res.push(OpCode::Pop),
            17 => res.push(OpCode::UniOpNot),
            18 => res.push(OpCode::UniOpNeg),
            19 => res.push(OpCode::StorePC),
            20 => res.push(OpCode::LoadPC),
            21 => {
                cur += 1;
                let dep = bytes[cur];
                cur += 1;
                let pos = bytes[cur];
                res.push(OpCode::StoreVar(dep, pos));
            }
            22 => {
                cur += 1;
                let dep = bytes[cur];
                cur += 1;
                let pos = bytes[cur];
                res.push(OpCode::LoadVar(dep, pos));
            }
            23 => {
                cur += 1;
                res.push(OpCode::EnterScope(bytes[cur]));
            }
            24 => res.push(OpCode::LeaveScope),
            25 => res.push(OpCode::MakeClosure),
            26 => {
                cur += 1;
                let addr = bytes[cur];
                cur += 1;
                let args = bytes[cur];
                res.push(OpCode::Call(addr, args));
            }
            27 => res.push(OpCode::Ret),
            100 => res.push(OpCode::BinOpAdd),
            101 => res.push(OpCode::BinOpSub),
            102 => res.push(OpCode::BinOpMul),
            103 => res.push(OpCode::BinOpDiv),
            104 => res.push(OpCode::BinOpGt),
            105 => res.push(OpCode::BinOpGe),
            106 => res.push(OpCode::BinOpLt),
            107 => res.push(OpCode::BinOpLe),
            108 => res.push(OpCode::BinOpEq),
            109 => res.push(OpCode::BinOpNe),
            110 => res.push(OpCode::BinOpOr),
            111 => res.push(OpCode::BinOpAnd),
            _ => panic!("Unknown OpCode: {}", bytes[cur]),
        }
        cur += 1;
    }
    res
}


pub fn translate_to(opcodes: Vec<OpCode>) -> Vec<usize> {
    let mut res = Vec::new();
    for code in opcodes {
        match code {
            OpCode::LoadConst(c) => res.extend(vec![1, c as usize]),
            OpCode::LoadTrue => res.push(2),
            OpCode::LoadFalse => res.push(3),
            OpCode::LoadNull => res.push(4),
            OpCode::LoadGlobal(v) => res.extend(vec![5, v]),
            OpCode::StoreGlobal(v) => res.extend(vec![6, v]),
            OpCode::BinOpAdd => res.push(100),
            OpCode::BinOpSub => res.push(101),
            OpCode::BinOpMul => res.push(102),
            OpCode::BinOpDiv => res.push(103),
            OpCode::BinOpGt => res.push(104),
            OpCode::BinOpGe => res.push(105),
            OpCode::BinOpLt => res.push(106),
            OpCode::BinOpLe => res.push(107),
            OpCode::BinOpEq => res.push(108),
            OpCode::BinOpNe => res.push(109),
            OpCode::BinOpOr => res.push(110),
            OpCode::BinOpAnd => res.push(111),
            OpCode::Jmp(addr) => res.extend(vec![10, addr]),
            OpCode::JmpTrue(addr) => res.extend(vec![11, addr]),
            OpCode::JmpFalse(addr) => res.extend(vec![12, addr]),
            OpCode::PrintItem => res.push(13),
            OpCode::PrintNewline => res.push(14),
            OpCode::GetInt => res.push(15),
            OpCode::Pop => res.push(16),
            OpCode::UniOpNot => res.push(17),
            OpCode::UniOpNeg => res.push(18),
            OpCode::StorePC => res.push(19),
            OpCode::LoadPC => res.push(20),
            OpCode::StoreVar(dep, pos) => res.extend(vec![21, dep, pos]),
            OpCode::LoadVar(dep, pos) => res.extend(vec![22, dep, pos]),
            OpCode::EnterScope(args) => res.extend(vec![23, args]),
            OpCode::LeaveScope => res.push(24),
            OpCode::MakeClosure => res.push(25),
            OpCode::Call(addr, args) => res.extend(vec![26, addr, args]),
            OpCode::Ret => res.push(27),
        }
    }
    res
}
