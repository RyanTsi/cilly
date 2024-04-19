use std::{io, result};


pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DuplicatedDef,      // 定义冲突
    SymbolNotFound,     // 缺少定义
    CallError(String),  // 函数调用错误
    MissingExpression,  // 缺少表达式
    UnExpectArgs,
    IoError(io::Error),
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}