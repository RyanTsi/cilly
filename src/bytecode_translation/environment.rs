use std::collections::HashMap;
use crate::error::{Result, Error};
#[derive(Debug, Clone)]
pub struct Environment {
    values: Vec<HashMap<String, usize>>,

    func_entry_addr: HashMap<String, (usize, usize)>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: vec![HashMap::new()],
            func_entry_addr: HashMap::new(),
        }
    }
    pub fn new_val(&mut self, id: String, dep: usize) {
        let x = self.values[dep].len();
        self.values[dep].insert(id, x);
    }
    // (dep, pos) 
    pub fn get_val(&self, id: String) -> Result<(usize, usize)> {
        let mut cur = self.values.len() as i32 - 1;
        while cur >= 0 {
            if let Some(v) = self.values[cur as usize].get(&id) {
                return Ok((cur as usize, v.clone()));
            }
            cur -= 1;
        }
        Err(Error::TranslateError(format!("val: {} is not existed !", id)))
    }
    pub fn get_values_count(&self, dep: usize) -> usize {
        self.values[dep].len()
    }
    pub fn new_func(&mut self, id: String, addr: usize, args_count: usize) {
        self.func_entry_addr.insert(id, (addr, args_count));
    }
    pub fn get_func_addr(&self, id: String) -> Result<(usize, usize)> {
        if let Some(addr_and_args_count) = self.func_entry_addr.get(&id) {
            return Ok(addr_and_args_count.clone());
        }
        Err(Error::TranslateError(format!("func {} is not existed !", id)))
    }
}