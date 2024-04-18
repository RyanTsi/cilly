use crate::{ast::{Block, BlockItem, CompUnit, Decl, FuncDef, GlobalDef, LVal, Stmt, ValDecl, VarDecl}, error::{Error, Result}};

use super::{environment::Environment, eval::Evaluate, values::{Type, Value}, Execute};

impl<'ast> Execute<'ast> for CompUnit {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<Option<Label>> {
        for global_def in &self.globaldefs {
            match global_def {
                GlobalDef::FuncDef(funcdef) => funcdef.run(env)?,
                GlobalDef::Decl(decl) => decl.run(env)?,
            };
        }
        if let Ok(_) = env.push_func("main") {
            env.call_func(&None)?;
            env.pop_func()?;
        }
        Ok(None)
    }
}

impl<'ast> Execute<'ast> for Decl {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<Option<Label>> {
        match &self {
            Decl::ValDecl(decl) => decl.run(env)?,
            Decl::VarDecl(decl) => decl.run(env)?,
        };
        Ok(None)
    }
}

impl<'ast> Execute<'ast> for ValDecl {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<Option<Label>> {
        env.new_value(&self.ident, Value::new(true, Type::from(self.initval.eval(env))))?;
        Ok(None)
    }
}

impl<'ast> Execute<'ast> for VarDecl {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<Option<Label>> {
        env.new_value(&self.ident, Value::new(false, Type::from(self.initval.eval(env))))?;
        Ok(None)
    }
}

impl<'ast> Execute<'ast> for FuncDef {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<Option<Label>> {
        env.new_func(&self.ident, self)?;
        Ok(None)
    }
}

impl<'ast> Execute<'ast> for BlockItem {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<Option<Label>> {
        match &self {
            BlockItem::Decl(decl) => decl.run(env)?,
            BlockItem::Stmt(stmt) => stmt.run(env)?,
        };
        Ok(None)
    }
}

pub enum Label {
    Continue,
    Break,
    Type(Type),
}

impl From<Type> for Label {
    fn from(value: Type) -> Self {
        Self::Type(value)
    }
}

impl<'ast> Execute<'ast> for Stmt {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<Option<Label>> {
        match &self {
            Stmt::Assign(lval, exp) => {
                env.new_value(&lval.ident, Value::new(false, Type::from(exp.eval(env))))?;
            }
            Stmt::Block(block) => {
                env.enter();
                for item in &block.items {
                    item.run(env)?;
                }
                env.exit();
            }
            Stmt::Exp(exp) => {
                if let Some(exp) = exp {
                    exp.eval(env);
                }
            }
            Stmt::Ret(exp) => {
                if let Some(exp) = exp {
                    return Ok(Some(Label::Type(Type::from(exp.eval(env)))))
                } else {
                    return Ok(None);
                }
            },
            Stmt::If { condition, then_branch, else_branch } => {
                if let Some(condition) = condition.eval(env) {
                    if condition != 0 {
                        then_branch.run(env)?;
                    } else {
                        if let Some(else_branch) = else_branch {
                            else_branch.run(env)?;
                        }
                    }
                } else {
                    return Err(Error::MissingExpression);
                }
            },
            Stmt::While { condition, loopbody } => {
                loop {
                    if let Some(condition) = condition.eval(env) {
                        if condition == 0 {
                            break;
                        } else {
                            loopbody.run(env)?;
                        }
                    } else {
                        return Err(Error::MissingExpression);
                    }
                }
            }
            Stmt::FuncDef(_) => todo!(),
            Stmt::Continue => todo!(),
            Stmt::Break => todo!(),
        }
        Ok(None)
    }
}