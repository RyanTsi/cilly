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
        let mut result_code = Vec::new();
        self.globaldefs.sort_by(|a, b| {
            match (a, b) {
                (GlobalDef::Decl(_), GlobalDef::FuncDef(_)) => std::cmp::Ordering::Less,
                (GlobalDef::FuncDef(_), GlobalDef::Decl(_)) => std::cmp::Ordering::Greater,
                (GlobalDef::FuncDef(FuncDef { ident: id_a, btype:_, funcfparams:_, block:_}), 
                 GlobalDef::FuncDef(FuncDef { ident: id_b, btype:_, funcfparams:_, block:_})
                ) => {
                    if id_a == "main" {
                        std::cmp::Ordering::Less
                    } else if id_b == "main" {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                },
                _ => std::cmp::Ordering::Equal
                
            }
        });
        for global_def in self.globaldefs.clone() {
            match global_def {
                GlobalDef::FuncDef(mut funcdef) => {
                    result_code.extend(funcdef.translate_byte(env, extension + result_code.len() + (2 << SHIFT))?);
                }
                GlobalDef::Decl(mut decl) => {
                    result_code.extend(decl.translate_byte(env, extension | (1 << SHIFT))?);
                }
            }
        }
        Ok(result_code)
    }
}

impl TransByteCode for FuncDef {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        let id = self.ident.clone();
        let addr = extension & PCMASK;
        let mut args_count = 0;
        if let Some(params) = &self.funcfparams {
            args_count += params.params.len();
        }
        env.new_func(id, addr, args_count);
        self.block.translate_byte(env, extension)
    }    
}

impl TransByteCode for Block {
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        let mut res = Vec::new();
        for mut item in self.items.clone() {
            res.extend(item.translate_byte(env, extension)?)
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
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>> {
        match self {
            Stmt::Assign(lval, exp) => todo!(),
            Stmt::Block(block) => todo!(),
            Stmt::Exp(exp) => todo!(),
            Stmt::Ret(ret) => todo!(),
            Stmt::If { condition, then_branch, else_branch } => todo!(),
            Stmt::While { condition, loopbody } => todo!(),
            Stmt::FuncDef(_) => todo!(),
            Stmt::Continue => todo!(),
            Stmt::Break => todo!(),
        };
        todo!()
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
            env.new_val(self.ident.clone(), 0);
            res.extend(self.initval.translate_byte(env, extension)?);
            res.push(OpCode::StoreGlobal);
        } else {
            let dep = extension >> 1;
            env.new_val(self.ident.clone(), dep);
            res.extend(self.initval.translate_byte(env, extension)?);
            res.push(OpCode::StoreVar(dep));
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
            env.new_val(self.ident.clone(), 0);
            res.extend(self.initval.translate_byte(env, extension)?);
            res.push(OpCode::StoreGlobal);
        } else {
            let dep = extension >> 1;
            env.new_val(self.ident.clone(), dep);
            res.extend(self.initval.translate_byte(env, extension)?);
            res.push(OpCode::StoreVar(dep));
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
                    _ => return Err(Error::TranslateError(format!("UnaryExp Error")))
                }
                Ok(res)
            },
            UnaryExp::FuncCall { ident, funcrparams } => todo!(),
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
        Ok(vec![OpCode::LoadVar(dep, pos)])
    }
}