use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    String(String),
    Number(f64),
    Operator(String),
    Delimiters(String),
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {

    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer {
            input,
            position: 0,
        };
        return lexer;
    }

    fn next(&mut self) -> Option<char> {
        self.position += 1;
        self.peek()
    }
    
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.next();
        }
    }

    fn isdigit(c: char) -> bool {
        matches!(c, '0'..='9')
    }
    
    fn isletter_(c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '_')
    }

    fn isletter_or_digit(c: char) -> bool {
        Self::isdigit(c) || Self::isletter_(c)
    }

    fn iskeywords(word: &str) -> bool {
        matches!(word, "return" | "var" | "if" | "else" | "while" | "break" | "for" | "fun" | "print" | "true" | "false" | "and" | "or" | "print")
    }

    pub fn next_token(&mut self) -> Result<Token> {

        self.skip_whitespace();

        match self.peek() {
            Some(c) => {
                match c {
                    // operator
                    '+'|'-'|'*'|'/'|'='|'<'|'>'|'!'|'^'|';'|',' => {
                        let mut op = c.to_string();
                        if c == '=' {
                            if let Some(nc) = self.next() {
                                if matches!(nc, '=' | '>' | '<') {
                                    op += &nc.to_string();
                                    self.next();
                                }
                            }
                        } else if c == '!' {
                            if let Some(nc) = self.next() {
                                if matches!(nc, '=') {
                                    op += &nc.to_string();
                                    self.next();
                                }
                            }
                        } else {
                            self.next();
                        }
                        Ok(Token::Operator(op))
                    }
                    // delimiters
                    '('|')' | '{'|'}' | '['|']' => {
                        self.next();
                        Ok(Token::Delimiters(c.to_string()))
                    }
                    // identifier or keyword
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let start = self.position;
                        let mut end = start + 1;
                        while let Some(nc) = self.next() {
                            if Self::isletter_or_digit(nc) {
                                end += 1;
                            } else {
                                break;
                            }
                        }
                        let word = &self.input[start..end];
                        if Self::iskeywords(word) {
                            Ok(Token::Keyword(word.to_string()))
                        } else {
                            Ok(Token::Identifier(word.to_string()))
                        }
                    }
                    // number
                    '0'..='9' => {
                        let start = self.position;
                        let mut end = start + 1;
                        let mut num_type = 0;
                        while let Some(nc) = self.next() {
                            if Self::isdigit(nc) {
                                end += 1;
                            } else if nc == '.' && (num_type & 1) == 0 {
                                num_type |= 1;
                                end += 1;
                            } else {
                                break;
                            }
                        }
                        let num = self.input[start..end].parse().unwrap();
                        Ok(Token::Number(num))
                    }
                    // string
                    '\"' => {
                        self.next();
                        let start = self.position;
                        let mut end = start + 1;
                        while let Some(nc) = self.next() {
                            if nc != '\"' {
                                end += 1;
                            } else {
                                break;
                            }
                        }
                        let string = &self.input[start..end];
                        self.next();
                        Ok(Token::String(string.to_string()))
                    }
                    _ => {
                        Err(Error::LexerError("error in make token".to_owned()))
                    }
                }
            }
            None => Ok(Token::EOF)
        }
    }
}