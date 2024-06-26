use std::collections::HashMap;
use crate::error::{Result, Error};
#[derive(Debug, Clone)]
pub struct Environment {
    values: Vec<HashMap<String, usize>>,
    func_entry_addr: HashMap<String, (usize, Vec<String>)>,
    pc_stack: Vec<usize>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: vec![HashMap::new()],
            func_entry_addr: HashMap::new(),
            pc_stack: Vec::new(),
        }
    }
    pub fn push_pc(&mut self, pc: usize) {
        self.pc_stack.push(pc);
    }
    pub fn pc_stack_top(&self) -> usize {
        self.pc_stack.last().unwrap().clone()
    }
    pub fn pc_stack_empty(&self) -> bool {
        self.pc_stack.is_empty()
    }
    pub fn pop_pc(&mut self) -> usize {
        self.pc_stack.pop().unwrap()
    }
    pub fn new_scope(&mut self) {
        self.values.push(HashMap::new());
    }
    pub fn leave_scope(&mut self) {
        self.values.pop();
    }
    pub fn is_bottom(&self, dep: usize) -> bool {
        dep + 1 == self.values.len()
    }
    pub fn new_val(&mut self, id: String, dep: usize) {
        let dep = self.values.len() - 1 - dep;
        let x = self.values[dep].len();
        self.values[dep].insert(id, x);
    }
    // (dep, pos) 
    pub fn get_val(&self, id: String) -> Result<(usize, usize)> {
        let mut cur = self.values.len() as i32 - 1;
        let mut dep = 0;
        while cur >= 0 {
            if let Some(v) = self.values[cur as usize].get(&id) {
                return Ok((dep, v.clone()));
            }
            cur -= 1;
            dep += 1;
        }
        Err(Error::TranslateError(format!("val: {} is not existed !", id)))
    }
    pub fn get_values_count(&self, dep: usize) -> usize {
        let dep = self.values.len() - 1 - dep;
        self.values[dep].len()
    }
    pub fn new_func(&mut self, id: String, addr: usize, args: Vec<String>) {
        self.func_entry_addr.insert(id, (addr, args));
    }
    pub fn get_func_addr(&self, id: String) -> Result<(usize, Vec<String>)> {
        if let Some(addr_and_args) = self.func_entry_addr.get(&id) {
            return Ok(addr_and_args.clone());
        }
        Err(Error::TranslateError(format!("func {} is not existed !", id)))
    }
    pub fn get_dep(&self) -> usize {
        self.values.len() - 1
    }
}