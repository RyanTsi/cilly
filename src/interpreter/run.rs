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
            // println!("{:?}", env.values);
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

        println!("{:?}", env.values);
        Ok(None)
    }
}

impl<'ast> Execute<'ast> for ValDecl {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<Option<Label>> {
        let val = Type::from(self.initval.eval(env));
        env.new_value(&self.ident, Value::new(true, val))?;
        Ok(None)
    }
}

impl<'ast> Execute<'ast> for VarDecl {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<Option<Label>> {
        let val = Type::from(self.initval.eval(env));
        env.new_value(&self.ident, Value::new(false, val))?;
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
    Type(Option<Type>),
    Continue,
    Break,
}

impl From<Option<Type>> for Label {
    fn from(value: Option<Type>) -> Self {
        Self::Type(value)
    }
}

impl<'ast> Execute<'ast> for Stmt {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<Option<Label>> {
        match &self {
            Stmt::Assign(lval, exp) => {
                let val = Type::from(exp.eval(env));
                env.update_value(&lval.ident, Value::new(false, val))?;
                // println!("{:?}", env.value(&lval.ident)?)
            }
            Stmt::Block(block) => {
                env.enter();
                for item in &block.items {
                    if let Some(label) = item.run(env)? {
                        return Ok(Some(label));
                    }                    
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
                    return Ok(Some(Label::Type(Some(Type::from(exp.eval(env))))))
                } else {
                    return Ok(Some(Label::Type(None)));
                }
            },
            Stmt::If { condition, then_branch, else_branch } => {
                if let Some(condition) = condition.eval(env) {
                    if condition != 0 {
                        if let Some(label) = then_branch.run(env)? {
                            return Ok(Some(label));
                        }                        
                    } else {
                        if let Some(else_branch) = else_branch {
                            if let Some(label) = else_branch.run(env)? {
                                return Ok(Some(label));
                            }
                        }
                    }
                } else {
                    return Err(Error::MissingExpression);
                }
            },
            Stmt::While {..} => {
                env.push_loop(self)?;
            },
            Stmt::FuncDef(_) => {
                todo!()
            },
            Stmt::Continue => {
                return Ok(Some(Label::Continue));
            },
            Stmt::Break => {
                env.pop_loop()?;
                return Ok(Some(Label::Break));
            },
        };
        Ok(None)
    }
}