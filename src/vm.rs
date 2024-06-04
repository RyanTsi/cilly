use std::io;

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    LoadConst(i32),             // 1   加载一个常数到栈顶。
    LoadTrue,                   // 2   加载True到栈顶。
    LoadFalse,                  // 3   加载False到栈顶。
    LoadNull,                   // 4   加载NULL到栈顶。
    LoadGlobal(usize),          // 5   从全局变量中加载一个变量到栈顶。
    StoreGlobal(usize),         // 6   将栈顶的值储存到全局变量表中。

    BinOpAdd,                   // 100 栈顶两个值相加。 
    BinOpSub,                   // 101 栈顶两个值相减。
    BinOpMul,                   // 102 栈顶两个值相乘。
    BinOpDiv,                   // 103 栈顶两个值相除。
    BinOpGt,                    // 104 比较栈顶两个值，大于则结果为true。
    BinOpGe,                    // 105 比较栈顶两个值，大于等于则结果为true。
    BinOpLt,                    // 106 比较栈顶两个值，小于则结果为true。
    BinOpLe,                    // 107 比较栈顶两个值，小于等于则结果为true。
    BinOpEq,                    // 108 比较栈顶两个值，相等则结果为true。
    BinOpNe,                    // 109 比较栈顶两个值，不相等则结果为true。
    BinOpOr,                    // 110 栈顶两个值或。
    BinOpAnd,                   // 111 栈顶两个值与。
    // 跳转的地址
    Jmp(usize),                 // 10  无条件跳转到指定位置。
    JmpTrue(usize),             // 11  如果栈顶值为true，跳转到指定位置。
    JmpFalse(usize),            // 12  如果栈顶值为false，跳转到指定位置。
    
    PrintItem,                  // 13  打印栈顶的值。
    PrintNewline,               // 14  打印一个换行符。
    GetInt,                     // 15  输入一个整数
    Pop,                        // 16  弹出栈顶值
    UniOpNot,                   // 17  对栈顶的布尔值取反。
    UniOpNeg,                   // 18  对栈顶的值取负。
    StorePC,                    // 19  存储当前 PC。
    LoadPC,                     // 20  从 PC 栈中加载。
    // dep, pos
    StoreVar(usize, usize),     // 21  将栈顶的值存储到局部变量表中。
    // dep, pos
    LoadVar(usize, usize),      // 22  从局部变量表中加载一个变量到栈顶。
    // args个数
    EnterScope(usize),          // 23  进入一个新的作用域。
    LeaveScope,                 // 24  离开当前作用域。
    MakeClosure,                // 25  创建一个闭包。
    // pc_addr, args个数 
    Call(usize, usize),         // 26  调用一个函数。
    Ret,                        // 27  从当前函数返回。
}

#[derive(Debug)]
pub struct VM {
    stack: Vec<i32>,
    scpoes: Vec<Vec<i32>>,
    pc_stack: Vec<(usize, usize)>,
    pc: usize,
    code: Vec<OpCode>
}

impl VM {
    pub fn new(code: Vec<OpCode>) -> Self {
        Self {
            stack: Vec::new(),
            scpoes: vec![Vec::new()],
            pc_stack: Vec::new(),
            pc: 0,
            code,
        }
    }
    fn del_addone(&mut self) {
        if let Some((c, _)) = self.pc_stack.last_mut() {
            *c += 1;
        }
    }
    pub fn run(&mut self) -> Result<()> {
        while self.pc < self.code.len() {
            // print!("stack: {:?}\nscpoes: {:?}\npc_stack: {:?}\npc: {}\n\n", &self.stack, &self.scpoes, &self.pc_stack, &self.pc);
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
                OpCode::StoreGlobal(pos) => {
                    let v = self.pop();
                    while self.scpoes[0].len() < pos + 1 {
                        self.scpoes[0].push(0);
                    }
                    self.scpoes[0][pos] = v;
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
                    if condition != 1 {
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
                OpCode::GetInt => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input: i32 = input.trim().parse().unwrap();
                    self.stack.push(input);
                }
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
                OpCode::StoreVar(scope_i, pos) => {
                    let v = self.pop();
                    let totle = self.scpoes.len();
                    let scope_i = totle - scope_i - 1;
                    while self.scpoes[scope_i].len() < pos + 1 {
                        self.scpoes[scope_i].push(0);
                    }
                    self.scpoes[scope_i][pos] = v;
                },
                OpCode::LoadVar(scope_i, pos) => {
                    let totle = self.scpoes.len();
                    let scope_i = totle - scope_i - 1;
                    let v = self.scpoes[scope_i][pos];
                    self.push(v);
                },
                OpCode::EnterScope(sz) => {
                    self.enter_scope(sz);
                    self.del_addone();
                },
                OpCode::LeaveScope => {
                    self.leave_scope();
                },
                OpCode::MakeClosure => todo!(),
                OpCode::Call(next, args_count) => {
                    self.enter_scope(args_count);
                    for i in 0..args_count {
                        let v = self.pop();
                        self.scpoes.last_mut().unwrap()[i] = v;
                    }
                    self.pc_stack.push((0, self.pc));
                    self.pc = next;
                },
                OpCode::Ret => {
                    self.leave_scope();
                    let (mut dep, pc) = self.pc_stack.pop().unwrap();
                    self.pc = pc;
                    while dep > 0 {
                        self.leave_scope();
                        dep -= 1;
                    }
                },
                OpCode::StorePC => {
                    self.pc_stack.push((0, self.pc));
                },
                OpCode::LoadPC => {
                    let (mut dep, pc) = self.pc_stack.pop().unwrap();
                    self.pc = pc;
                    while dep > 0 {
                        self.leave_scope();
                        dep -= 1;
                    }
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
        let v2 = self.pop();
        let v1 = self.pop();
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