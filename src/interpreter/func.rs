use crate::{ast::{BType, Block, BlockItem, Decl, FuncDef, FuncFParams, FuncRParams, Stmt}, error::{Error, Result}, interpreter::eval::Evaluate};

use super::{environment::Environment, run::Label, values::{Type, Value}, Execute};



impl<'ast> FuncDef {
    pub fn call(&'ast self, params: &'ast Option<FuncRParams>, env: &mut Environment<'ast>) -> Result<Option<Value>> {
        env.enter();
        let mut res = Ok(None);
        if let (Some(Rparams), Some(Lparams)) = (params, &self.funcfparams) {
            let len = Rparams.exps.len();
            if (Lparams.params.len() != len) || Lparams.params.len() == 0 {
                return Err(Error::CallError(format!("in function: {}", self.ident)));
            } else {
                for (param, exp) in Lparams.params.iter().zip(&Rparams.exps) {
                    let id = &param.ident;
                    let val = Value::Var(Type::from(exp.eval(env)));
                    env.new_value(id, val)?;
                }
            }
        }
        // println!("{:#?}", env.values);
        for item in &self.block.items {
            match item {
                BlockItem::Decl(decl) => {
                    decl.run(env)?;
                }
                BlockItem::Stmt(stmt) => {
                    if let Some(Label::Type(ret)) = stmt.run(env)? {
                        res = match ret {
                            None => Ok(None),
                            Some(typ) => Ok(Some(Value::new(true, typ))),
                        };
                        break;
                    }
                }
            }
            env.run_loop()?;
        }
        env.exit();
        res
    }
}
