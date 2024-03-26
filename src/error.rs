#[derive(Debug)]
pub enum error {
    ParserError(String),
    LexerError(String)
}