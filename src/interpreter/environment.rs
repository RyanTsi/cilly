/**
 * 表示代码运行的环境，stack 维护上下文（函数栈，当前函数的 loop 栈）
 */

use std::collections::HashMap;

use crate::{ast::{FuncDef, FuncRParams, Stmt}, error::{Error, Result}};

use super::{eval::Evaluate, values::Value, Execute};

#[derive(Debug)]
pub struct Environment<'ast> {
    funcs: HashMap<&'ast str, &'ast FuncDef>,
    pub values: Vec<HashMap<&'ast str, Value>>,
    stack: Vec<(&'ast FuncDef, Vec<&'ast Stmt>)>,
}

impl<'ast> Environment<'ast> {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            values: vec![HashMap::new()],
            stack: Vec::new(),
        }
    }
    pub fn new_value(&mut self, ident: &'ast str, v: Value) -> Result<()> {
        let cur = self.values.last_mut().unwrap();
        if let Some(Value::Const(_)) = cur.get(&ident) {
            return Err(Error::DuplicatedDef);
        }
        cur.insert(ident, v);
        Ok(())
    }
    pub fn update_value(&mut self, ident: &'ast str, v: Value) -> Result<()> {
        for scope in self.values.iter_mut().rev() {
            if let Some(value) = scope.get_mut(ident) {
                *value = v; // 更新值为新的值
                return Ok(());
            }
        }
        Err(Error::SymbolNotFound)
    }
    pub fn new_func(&mut self, ident: &'ast str, func: &'ast FuncDef) -> Result<()> {
        if self.funcs.contains_key(ident) {
            return Err(Error::DuplicatedDef);
        }
        self.funcs.insert(ident, func);
        Ok(())
    }
    pub fn value(&self, ident: &'ast str) -> Result<&Value> {
        let mut cur = self.values.len() as i32 - 1;
        while cur >= 0 {
            if let Some(v) = self.values[cur as usize].get(ident) {
                return Ok(v);
            }
            cur -= 1;
        }
        Err(Error::SymbolNotFound)
    }
    pub fn func(&self, ident: &'ast str) -> Result<&'ast FuncDef> {
        if let Some(func) = self.funcs.get(ident) {
            return Ok(func);
        }
        Err(Error::SymbolNotFound)
    }
    pub fn enter(&mut self) {
        self.values.push(HashMap::new());
    }
    pub fn exit(&mut self) {
        self.values.pop();
    }
    pub fn push_func(&mut self, ident: &'ast str) -> Result<()> {
        self.stack.push((self.func(ident)?, vec![]));
        Ok(())
    }
    pub fn pop_func(&mut self) -> Result<()> {
        self.stack.pop();
        Ok(())
    }
    pub fn call_func(&mut self, params: &'ast Option<FuncRParams>) -> Result<Option<Value>> {
        let (curfunc, _) = self.stack.last().unwrap();
        curfunc.call(params, self)
    }
    pub fn push_loop(&mut self, stmt: &'ast Stmt) -> Result<()> {
        self.stack.last_mut().unwrap().1.push(stmt);
        Ok(())
    }
    pub fn pop_loop(&mut self) -> Result<()> {
        self.stack.last_mut().unwrap().1.pop();
        Ok(())
    }
    pub fn loop_is_empty(&self) -> bool {
        self.stack.last().unwrap().1.is_empty()
    }
    pub fn run_loop(&mut self) -> Result<()> {
        loop {
            if let Some(Stmt::While { condition, loopbody }) = &self.stack.last().unwrap().1.last() {
                if let Some(condition) = condition.eval(self) {
                    if condition == 0 {
                        self.pop_loop()?;
                        continue;
                    } else {
                        let loopbody: &Stmt = loopbody;
                        loopbody.run(self)?;
                    }
                } else {
                    return Err(Error::MissingExpression);
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}