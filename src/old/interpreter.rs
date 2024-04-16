use std::collections::HashMap;

use crate::{error::{Error, Result}};

use super::parser::{self, Atom, Expr, Node, Statement};

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

pub enum Value {
    Num(f64),
    None,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new()
        }
    }

    pub fn eval(&mut self, node: &Node) -> Result<()> {
        match node {
            Node::Program(statements) => {
                for node in statements {
                    match node {
                        Node::Statement(s) => self.eval_statement(s)?,
                        _ => return Err(Error::InterpreterError("in fn eval".to_string())),
                    }
                }
                Ok(())
            }
            _ => Err(Error::InterpreterError("in fn eval".to_string()))
        }
    }

    fn eval_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::RetStat(expr) => {
                if let Some(expr) = expr {
                    let _ = self.eval_expr(expr)?;
                }
            }
            Statement::ExprStat(expr) => {
                let _ = self.eval_expr(expr)?;
            }
            Statement::IfStat(condition, then_branch, else_branch) => {
                if self.eval_expr_boolean(condition)? {
                    self.eval_statement(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.eval_statement(else_branch)?;
                }
            }
            Statement::WhileStat(condition, codebody) => {
                while self.eval_expr_boolean(&condition)? {
                    self.eval_statement(&*codebody)?;
                }
            }
            Statement::BlockStat(statements) => {
                for statement in statements {
                    self.eval_statement(statement)?;
                }
            }
            Statement::PrintStat(content) => {
                
            }
            Statement::Args(exprs) => {
                for expr in exprs {

                }
            }
            _ => return Err(Error::InterpreterError("in fn eval_statement".to_string())),
        }
        Ok(())
    }

    fn eval_expr_boolean(&mut self, expr: &Expr) -> Result<bool> {
        Ok(self.eval_expr(expr)?.abs() < 1e-9)
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<f64> {
        match expr {
            Expr::Atom(atom) => self.eval_atom(atom),
            // 在这里处理其他类型的表达式
            _ => Err(Error::InterpreterError("in fn eval_expr".to_string())),
        }
    }

    fn eval_atom(&mut self, atom: &Atom) -> Result<f64> {
        match atom {
            Atom::Num(num) => Ok(*num),
            // Atom::String(s) => Ok(s.clone()),

            // 在这里处理其他类型的原子
            _ => Err(Error::InterpreterError("in fn atom".to_string())),
        }
    }

}
