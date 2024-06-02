use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    LoadConst(i32),             // 1
    LoadTrue,                   // 2
    LoadFalse,                  // 3
    LoadNull,                   // 4
    LoadGlobal(usize),          // 5
    StoreGlobal,                // 6

    BinOpAdd,                   // 100
    BinOpSub,                   // 101
    BinOpMul,                   // 102
    BinOpDiv,                   // 103
    BinOpGt,                    // 104
    BinOpGe,                    // 105
    BinOpLt,                    // 106
    BinOpLe,                    // 107
    BinOpEq,                    // 108
    BinOpNe,                    // 109
    BinOpOr,                    // 110
    BinOpAnd,                   // 111

    Jmp(usize),                 // 10
    JmpTrue(usize),             // 11
    JmpFalse(usize),            // 12
    PrintItem,                  // 13
    PrintNewline,               // 14
    Pop,                        // 15
    UniOpNot,                   // 16
    UniOpNeg,                   // 17
    StoreVar(usize),            // 18
    LoadVar(usize, usize),      // 19
    EnterScope(usize),          // 20
    LeaveScope,                 // 21
    MakeClosure,                // 22
    Call(usize, usize),         // 23
    Ret,                        // 24
}

pub struct VM {
    stack: Vec<i32>,
    scpoes: Vec<Vec<i32>>,  // 返回地址 | 参数个数 | 参数 | Local 变量
    pc: usize,
    code: Vec<OpCode>
}

impl VM {
    pub fn new(code: Vec<OpCode>) -> Self {
        Self {
            stack: Vec::new(),
            scpoes: vec![vec![-1, 0]],
            pc: 0,
            code,
        }
    }
    pub fn run(&mut self) -> Result<()> {
        while self.pc < self.code.len() {
            let index = self.code[self.pc];
            self.pc += 1;
            match index {
                OpCode::LoadConst(v) => {
                    self.push(v);
                },
                OpCode::LoadTrue => {
                    self.push(1);
                },
                OpCode::LoadFalse => {
                    self.push(0);
                },
                OpCode::LoadNull => {
                    self.push(0);
                },
                OpCode::LoadGlobal(pos) => {
                    self.push(self.scpoes[0][pos]);
                },
                OpCode::StoreGlobal => {
                    let v = self.pop();
                    self.scpoes[0].push(v);
                },
                OpCode::BinOpAdd => {
                    self.binop(index)?;
                },
                OpCode::BinOpSub => {
                    self.binop(index)?;
                },
                OpCode::BinOpMul => {
                    self.binop(index)?;
                },
                OpCode::BinOpDiv => {
                    self.binop(index)?;
                },
                OpCode::BinOpGt => {
                    self.binop(index)?;
                },
                OpCode::BinOpGe => {
                    self.binop(index)?;
                },
                OpCode::BinOpLt => {
                    self.binop(index)?;
                },
                OpCode::BinOpLe => {
                    self.binop(index)?;
                },
                OpCode::BinOpEq => {
                    self.binop(index)?;
                },
                OpCode::BinOpNe => {
                    self.binop(index)?;
                },
                OpCode::BinOpOr => {
                    self.binop(index)?;
                }
                OpCode::BinOpAnd => {
                    self.binop(index)?;
                }
                OpCode::Jmp(next) => {
                    self.pc = next.clone();
                },
                OpCode::JmpTrue(next) => {
                    let condition = self.pop();
                    if condition == 1 {
                        self.pc = next;
                    }
                },
                OpCode::JmpFalse(next) => {
                    let condition = self.pop();
                    if condition == 1 {
                        self.pc = next;
                    }
                },
                OpCode::PrintItem => {
                    let c = self.pop();
                    print!("{c}");
                },
                OpCode::PrintNewline => {
                    println!("");
                },
                OpCode::Pop => {
                    self.pop();
                },
                OpCode::UniOpNot => {
                    let v = !self.pop();
                    self.push(v);
                },
                OpCode::UniOpNeg => {
                    let v = -self.pop();
                    self.push(v);
                },
                OpCode::StoreVar(scope_i) => {
                    let v = self.pop();
                    self.scpoes[scope_i].push(v);
                },
                OpCode::LoadVar(scope_i, pos) => {
                    let v = self.scpoes[scope_i][pos];
                    self.push(v);
                },
                OpCode::EnterScope(sz) => {
                    self.enter_scope(sz);
                },
                OpCode::LeaveScope => {
                    self.leave_scope();
                },
                OpCode::MakeClosure => todo!(),
                OpCode::Call(next, args_count) => {
                    self.enter_scope(args_count + 2);
                    self.scpoes.last_mut().unwrap()[0] = self.pc as i32;
                    self.scpoes.last_mut().unwrap()[1] = args_count as i32;
                    for i in 2..args_count + 2 {
                        let v = self.pop();
                        self.scpoes.last_mut().unwrap()[i] = v;
                    }
                    self.pc = next;
                },
                OpCode::Ret => {
                    self.leave_scope();
                    self.pc = self.pop() as usize;
                },
            }
        }
        Ok(())
    }
    fn enter_scope(&mut self, var_count: usize) {
        self.scpoes.push(vec![0; var_count]);
    }
    fn leave_scope(&mut self) {
        self.scpoes.pop();
    }
    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }
    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
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
            OpCode::BinOpOr => {
                if v1 != 0 || v2 != 0 {
                    v = Some(1);
                } else {
                    v = Some(0);
                }
            }
            OpCode::BinOpAnd => {
                if v1 == 0 || v2 == 0 {
                    v = Some(0);
                } else {
                    v = Some(1);
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
