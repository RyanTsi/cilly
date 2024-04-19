/**
 * 函数的调用，内置函数的实现
 */

use std::io;

use crate::{ast::{BlockItem, FuncDef, FuncRParams}, error::{Error, Result}, interpreter::eval::Evaluate};

use super::{environment::Environment, run::Label, values::{Type, Value}, Execute};


impl<'ast> FuncDef {
    pub fn call(&'ast self, params: &'ast Option<FuncRParams>, env: &mut Environment<'ast>) -> Result<Option<Value>> {
        match self.ident.as_str() {
            "print" => {
                if let Some(params) = params {
                    for param in &params.exps {
                        if let Some(res) = param.eval(env) {
                            println!("{}", res);
                        }
                    }
                }
                return Ok(None);
            }
            "getint" => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input: i32 = input.trim().parse().unwrap();
                let input = Type::from(Some(input));
                return Ok(Some(Value::new(true, input)));
            }
            _ => ()
        };
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
        // println!("{:?}", res);
        res
    }
}
