/** 
 * 执行 Expression 将其转化为 i32
*/

use crate::ast::{AddExp, BinaryOp, EqExp, Exp, InitVal, LAndExp, LOrExp, LVal, MulExp, PrimaryExp, RelExp, UnaryExp, UnaryOp};

use super::{environment::Environment, values::{Type, Value}};

pub trait Evaluate {
    fn eval(&self, env: &Environment) -> Option<i32>;
}

impl Evaluate for Exp {
    fn eval(&self, env: &Environment) -> Option<i32> {
        self.lor_exp.eval(env)
    }
}

impl Evaluate for InitVal {
    fn eval(&self, env: &Environment) -> Option<i32> {
        self.exp.eval(env)
    }
}

impl Evaluate for LOrExp {
    fn eval(&self, env: &Environment) -> Option<i32> {
        match &self {
            LOrExp::And(and) => and.eval(env),
            LOrExp::Or(lhs, rhs) => {
                match (lhs.eval(env), rhs.eval(env)) {
                    (Some(lhs), Some(rhs)) => {
                        Some((lhs != 0 || rhs != 0) as i32)
                    }
                    _ => None
                }
            }
        }
    }
}

impl Evaluate for LAndExp {
    fn eval(&self, env: &Environment) -> Option<i32> {
        match &self {
            LAndExp::Eq(eq) => eq.eval(env),
            LAndExp::And(lhs, rhs) => {
                match (lhs.eval(env), rhs.eval(env)) {
                    (Some(lhs), Some(rhs)) => {
                        Some((lhs != 0 && rhs != 0) as i32)
                    }
                    _ => None
                }
            }
        }
    }
}

impl Evaluate for EqExp {
    fn eval(&self, env: &Environment) -> Option<i32> {
        match &self {
            EqExp::Rel(rel) => rel.eval(env),
            EqExp::Eq(lhs, op, rhs) => {
                match (lhs.eval(env), rhs.eval(env)) {
                    (Some(lhs), Some(rhs)) => {
                        match op {
                            BinaryOp::Eq =>  Some((lhs == rhs) as i32),
                            BinaryOp::Neq => Some((lhs != rhs) as i32),
                            _ => None,
                        }
                    }
                    _ => None
                }
            }
        }
    }
}

impl Evaluate for RelExp {
    fn eval(&self, env: &Environment) -> Option<i32> {
        match &self {
            RelExp::Add(add) => add.eval(env),
            RelExp::Rel(lhs, op, rhs) => {
                match (lhs.eval(env), rhs.eval(env)) {
                    (Some(lhs), Some(rhs)) => {
                        match op {
                            BinaryOp::Lt  => Some((lhs <  rhs) as i32),
                            BinaryOp::Leq => Some((lhs <= rhs) as i32),
                            BinaryOp::Gt  => Some((lhs >  rhs) as i32),
                            BinaryOp::Geq => Some((lhs >= rhs) as i32),
                            _ => None,
                        }
                    }
                    _ => None
                }
            }
        }
    }
}

impl Evaluate for AddExp {
    fn eval(&self, env: &Environment) -> Option<i32> {
        match &self {
            AddExp::Mul(mul) => mul.eval(env),
            AddExp::Add(lhs, op, rhs) => {
                match (lhs.eval(env), rhs.eval(env)) {
                    (Some(lhs), Some(rhs)) => {
                        match op {
                            BinaryOp::Add => Some(lhs + rhs),
                            BinaryOp::Sub => Some(lhs - rhs),
                            _ => None,
                        }
                    }
                    _ => None
                }
            }
        }
    }
}

impl Evaluate for MulExp {
    fn eval(&self, env: &Environment) -> Option<i32> {
        match &self {
            MulExp::Unary(unary) => unary.eval(env),
            MulExp::Mul(lhs, op, rhs) => {
                match (lhs.eval(env), rhs.eval(env)) {
                    (Some(lhs), Some(rhs)) => {
                        match op {
                            BinaryOp::Mul => Some(lhs * rhs),
                            BinaryOp::Div => Some(lhs / rhs),
                            BinaryOp::Mod => Some(lhs % rhs),
                            _ => None,
                        }
                    }
                    _ => None
                }
            }
        }
    }
}

impl Evaluate for UnaryExp {
    fn eval(&self, env: &Environment) -> Option<i32> {
        match &self {
            UnaryExp::Pri(pri) => pri.eval(env),
            UnaryExp::Unary(op, unary) => 
                unary.eval(env).map(|exp| match op {
                    UnaryOp::Neg => -exp,
                    UnaryOp::Not => (exp == 0) as i32,
                }),
            UnaryExp::FuncCall { ident, funcrparams } => None,
        }
    }
}

impl Evaluate for PrimaryExp {
    fn eval(&self, env: &Environment) -> Option<i32> {
        match &self {
            PrimaryExp::Exp(exp) => exp.eval(env),
            PrimaryExp::Number(num) => Some(num.to_owned()) ,
            PrimaryExp::LVal(id) => id.eval(env),
        }
    }
}

impl Evaluate for LVal {
    fn eval(&self, env: &Environment) -> Option<i32> {
        let val = env.value(&self.ident).ok()?;
        match val {
            Value::Const(Type::I32(v)) => Some(v.to_owned()),
            Value::Var(Type::I32(v)) => Some(v.to_owned()),
        }
    }
}