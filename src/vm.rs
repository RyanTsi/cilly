use crate::error::{Error, Result};

#[derive(Debug)]
enum OpCode {
    LoadConst(i32),      // 1
    LoadTrue,            // 2
    LoadFalse,           // 3
    LoadNull,            // 4
    LoadGlobal(usize),   // 5
    StoreGlobal(usize),  // 6
    BinOpAdd,            // 10
    BinOpSub,            // 11
    BinOpMul,            // 12
    BinOpDiv,            // 13
    BinOpGt,             // 14
    BinOpGe,             // 15
    BinOpLt,             // 16
    BinOpLe,             // 17
    BinOpEq,             // 18
    BinOpNe,             // 19
    Jmp(usize),          // 20
    JmpTrue(usize),      // 21
    JmpFalse(usize),     // 22
    PrintItem,           // 23
    PrintNewline,        // 24
    Pop,                 // 25
    UniOpNot,            // 26
    UniOpNeg,            // 27
    StoreVar(usize),     // 28
    LoadVar(usize),      // 29
    EnterScope,          // 30
    LeaveScope,          // 31
    MakeClosure,         // 32
    Call(usize),         // 33
    Ret,                 // 34
}

pub struct VM {
    stack: Vec<i32>,
    call_stack: Vec<i32>,           // 函数调用栈
    scpoes: Vec<Vec<Option<i32>>>,
    pc: usize,
    code: Vec<OpCode>
}

impl VM {
    pub fn new(code: Vec<OpCode>) -> Self {
        Self {
            stack: Vec::new(),
            call_stack: Vec::new(),
            scpoes: Vec::new(),
            pc: 0,
            code,
        }
    }
    pub fn run(&mut self) {
        while self.pc < self.code.len() {
            let index = &self.code[self.pc];
            self.pc += 1;
            
        }
    }
    fn push_call_stack(&mut self, c: i32) {
        self.call_stack.push(c);
    }

    fn pop_call_stack(&mut self) -> i32 {
        self.call_stack.pop().unwrap()
    }
    fn enter_scope(&mut self, var_count: usize) {
        self.scpoes.push(vec![None; var_count]);
    }
    fn leave_scope(&mut self) {
        self.scpoes.pop();
    }
    fn load_var(&mut self, scope_i: usize, i: usize) {
        self.push(self.scpoes[scope_i][i].unwrap());
    }
    fn store_var(&mut self, scope_i: usize, i: usize) {
        self.scpoes[scope_i][i] = Some(self.pop());
    }
    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }
    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    fn binop(&mut self, op: OpCode) -> Result<()>{
        let v1 = self.pop();
        let v2 = self.pop();
        let mut v = None;
        match op {
            OpCode::BinOpAdd => {
                v = Some(v1 + v2);
            },
            OpCode::BinOpSub => {
                v = Some(v1 - v2);
            },
            OpCode::BinOpMul => {
                v = Some(v1 * v2);
            },
            OpCode::BinOpDiv => {
                v = Some(v1 / v2);
            },
            OpCode::BinOpGt => {
                if v1 > v2 {
                    v = Some(1);
                } else {
                    v = Some(0);
                }
            },
            OpCode::BinOpGe => {
                if v1 >= v2 {
                    v = Some(1);
                } else {
                    v = Some(0);
                }
            },
            OpCode::BinOpLt => {
                if v1 < v2 {
                    v = Some(1);
                } else {
                    v = Some(0);
                }
            },
            OpCode::BinOpLe => {
                if v1 <= v2 {
                    v = Some(1);
                } else {
                    v = Some(0);
                }
            },
            OpCode::BinOpEq => {
                if v1 == v2 {
                    v = Some(1);
                } else {
                    v = Some(0);
                }
            },
            OpCode::BinOpNe => {
                if v1 != v2 {
                    v = Some(1);
                } else {
                    v = Some(0);
                }
            }
            _ => {
                return Err(Error::VMError(String::from(format!("非法二元运算符 {:?}", op))));
            }
        }
        self.push(v.unwrap());
        Ok(())
    }
}
