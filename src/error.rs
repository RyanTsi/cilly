use std::result;


pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    LexerError(String),
    ParserUnexpectedEnd,
    ParserUnexpectedMatch(String),
    InterpreterError(String),
}
