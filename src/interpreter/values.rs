#[derive(Debug)]
pub enum Value {
    Const(Type),
    Var(Type),
}

#[derive(Debug)]
pub enum Type {
    I32(i32),
}

impl Value {
    pub fn new(is_const: bool, val: Type) -> Self {
        if is_const {
            Value::Const(val)
        } else {
            Value::Var(val)
        }
    }
}

impl From<Option<i32>> for Type {
    fn from(value: Option<i32>) -> Self {
        if let Some(v) = value {
            Self::I32(v)
        } else {
            Self::I32(0)
        }
    }
}

