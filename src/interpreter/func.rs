use crate::{ast::{BType, Block, BlockItem, Decl, FuncDef, FuncFParams, FuncRParams, Stmt}, error::{Error, Result}, interpreter::eval::Evaluate};

use super::{environment::Environment, values::{Type, Value}, Execute};


impl<'ast> FuncDef {
    pub fn call(&'ast self, params: &Option<FuncRParams>, env: &mut Environment<'ast>) -> Result<Option<Value>> {
        env.enter();
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
        let mut curitem_ith = 0;
        while curitem_ith < self.block.items.len() {
            let item = &self.block.items[curitem_ith];
            match item {
                BlockItem::Decl(decl) => {
                    decl.run(env)?;
                }
                BlockItem::Stmt(stmt) => {
                    
                }
            }
            curitem_ith += 1;
        }
        env.exit();
        Ok(None)
    }
}
