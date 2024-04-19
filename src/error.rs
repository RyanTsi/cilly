use std::result;


pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    LexerError(String),
    ParserUnexpectedEnd,
    ParserUnexpectedMatch(String),
    InterpreterError(String),
    DuplicatedDef,      // 定义冲突
    SymbolNotFound,     // 缺少定义
    CallError(String),  // 函数调用错误
    MissingExpression,  // 缺少表达式
}
