use crate::{ast::{CompUnit, Decl, FuncDef, GlobalDef, ValDecl, VarDecl}, error::Result};

use super::{environment::Environment, eval::Evaluate, values::{Type, Value}, Execute};

impl Execute for CompUnit {
    fn run(&self, env: &mut Environment) -> Result<()> {
        for global_def in &self.globaldefs {
            match global_def {
                GlobalDef::FuncDef(funcdef) => {
                    interprete_funcdef(&env, funcdef)?
                }
                GlobalDef::Decl(decl) => {
                    decl.run(env)?;
                }
            }
        }
        Ok(())
    }
}

impl Execute for Decl {
    fn run(&self, env: &mut Environment) -> Result<()> {
        match &self {
            Decl::ValDecl(decl) => {
                decl.run(env)?
            }
            Decl::VarDecl(decl) => {
                decl.run(env)?
            }
        }
        Ok(())
    }
}

// pub fn interprete_decl(env: &mut Environment<'a>, decl: &'a Decl) -> Result<()> {
//     match decl {
//         Decl::ValDecl(decl) => {
//             Self::interprete_val_decl(env, decl)?
//         }
//         Decl::VarDecl(decl) => {
//             Self::interprete_var_decl(env, decl)?
//         }
//     }
//     Ok(())
// }

impl<'ast> Execute for ValDecl {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<()> {
        env.new_value(&self.ident, Value::new(true, Type::from(self.initval.eval(env))))?;
        Ok(())
    }
}

impl Execute for VarDecl {
    fn run(&self, env: &mut Environment) -> Result<()> {
        env.new_value(&self.ident, Value::new(false, Type::from(self.initval.eval(env))))?;
        Ok(())
    }
}

fn interprete_funcdef(env: &Environment, funcdef: &FuncDef) -> Result<()> {
    Ok(())
}
