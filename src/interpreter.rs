use crate::ast::*;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(comp_unit: CompUnit) {
        let mut env = Environment::new(); // 创建环境

        for global_def in comp_unit.globaldefs {
            match global_def {
                GlobalDef::FuncDef(func_def) => {
                    // 解释函数定义
                    self.interpret_func_def(func_def, &mut env);
                }
                GlobalDef::Decl(decl) => {
                    // 解释声明
                    self.interpret_decl(decl, &mut env);
                }
            }
        }
    }

    fn interpret_decl(decl: Decl, env: &mut Environment) {
        match decl {
            Decl::VarDecl(var_decl) => {
                // 解释变量声明
                self.interpret_var_decl(var_decl, env);
            }
            Decl::ValDecl(val_decl) => {
                // 解释值声明
                self.interpret_val_decl(val_decl, env);
            }
        }
    }

    fn interpret_var_decl(var_decl: VarDecl, env: &mut Environment) {
        // 将变量添加到环境中
        env.add_variable(var_decl.ident, None); // 初始化为 None
    }

    fn interpret_val_decl(val_decl: ValDecl, env: &mut Environment) {
        // 解释值声明的初始化表达式并将其值存储在环境中
        let value = self.interpret_init_val(val_decl.initval, env);
        env.add_variable(val_decl.ident, Some(value));
    }

    fn interpret_init_val(init_val: InitVal, env: &mut Environment) -> i32 {
        // 解释初始化表达式
        self.interpret_exp(init_val.exp, env)
    }

    fn interpret_func_def(func_def: FuncDef, env: &mut Environment) {
        // 解释函数定义的代码块
        // 在这个简单的示例中，我们只是打印函数定义的内容
        println!("Function definition: {:?}", func_def);
    }

    fn interpret_exp(exp: Exp, env: &mut Environment) -> i32 {
        // 解释表达式
        match exp.lor_exp {
            LOrExp::And(and_exp) => {
                // 解释逻辑与表达式
                self.interpret_land_exp(and_exp, env)
            }
            // 在这个简单的示例中，我们只解释了逻辑与表达式
            // 你需要根据你的语言语义和需要来实现其他类型的表达式解释
            _ => unimplemented!(),
        }
    }

    fn interpret_land_exp(land_exp: LAndExp, env: &mut Environment) -> i32 {
        // 解释逻辑与表达式
        match land_exp {
            LAndExp::Eq(eq_exp) => {
                // 解释相等表达式
                self.interpret_eq_exp(eq_exp, env)
            }
            _ => unimplemented!(),
        }
    }

    fn interpret_eq_exp(eq_exp: EqExp, env: &mut Environment) -> i32 {
        // 解释相等表达式
        match eq_exp {
            EqExp::Rel(rel_exp) => {
                // 解释关系表达式
                self.interpret_rel_exp(rel_exp, env)
            }
            EqExp::Eq(eq, op, rel) => {
                // 解释相等操作
                let lhs = self.interpret_eq_exp(*eq, env);
                let rhs = self.interpret_rel_exp(rel, env);
                match op {
                    BinaryOp::Eq => (lhs == rhs) as i32,
                    BinaryOp::Neq => (lhs != rhs) as i32,
                    _ => unimplemented!(),
                }
            }
        }
    }

    // 实现其他表达式解释方法以及其他节点类型的解释方法
}

// 环境结构体，用于存储变量和其值
struct Environment {
    variables: Vec<(String, Option<i32>)>,
}

impl Environment {
    fn new() -> Self {
        Environment { variables: Vec::new() }
    }

    fn add_variable(&mut self, name: String, value: Option<i32>) {
        self.variables.push((name, value));
    }
}
