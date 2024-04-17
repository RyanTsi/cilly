use crate::{ast::{BType, Block, BlockItem, Decl, FuncDef, FuncFParams, FuncRParams}, error::{Error, Result}, interpreter::eval::Evaluate};

use super::{environment::Environment, values::{Type, Value}};

pub struct Function {
    ret: Option<BType>,
    params: Option<FuncFParams>,
    block: Block,
}

impl From<FuncDef> for Function {
    fn from(func: FuncDef) -> Self {
        Self {
            ret: func.btype,
            params: func.funcfparams,
            block: func.block,
        }
    }
}

impl<'ast> Function {
    fn call(&'ast self, params: &Option<FuncRParams>, env: &mut Environment<'ast>) -> Result<Option<Value>> {
        env.enter();
        if let (Some(Rparams), Some(Lparams)) = (params, &self.params) {
            let len = Rparams.exps.len();
            if (Lparams.params.len() != len) || Lparams.params.len() == 0 {
                return Err(Error::CallError);
            } else {
                for (param, exp) in Lparams.params.iter().zip(&Rparams.exps) {
                    let id = &param.ident;
                    let val = Value::Var(Type::from(exp.eval(env)));
                    env.new_value(id, val)?;
                }
            }
        }
        for item in &self.block.items {

        }
        env.exit();
        todo!()
    }
}

impl BlockItem {
    fn execute(&self, env: &Environment) {
        match &self {
            BlockItem::Decl(decl) => {
                
            }
            BlockItem::Stmt(stmt) => {

            }
        }
    }
}