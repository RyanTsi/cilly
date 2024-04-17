use std::collections::HashMap;

use crate::error::{Error, Result};

use super::{func::Function, values::Value};

pub struct Environment<'ast> {
    funcs: HashMap<&'ast str, Function>,
    values: Vec<HashMap<&'ast str, Value>>,
}

impl<'ast> Environment<'ast> {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            values: vec![HashMap::new()],
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
    pub fn new_func(&mut self, ident: &'ast str, func: Function) -> Result<()> {
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
    pub fn func(&self, ident: &'ast str) -> Result<&Function> {
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
}