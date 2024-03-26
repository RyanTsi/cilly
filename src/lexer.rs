#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Operator(String),
    Number(f64),
    Semicolon(String),
    String(String),
    EOF,
}

#[derive(Debug)]
enum Tag {
    CurChar(char),
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

    fn next(&mut self) -> Tag {
        self.position += 1;
        self.peek()
    }
    
    fn peek(&self) -> Tag {
        if self.position >= self.input.len() {
            Tag::EOF
        } else {
            Tag::CurChar(self.input.chars().nth(self.position).unwrap())
        }
    }

    fn skip_whitespace(&mut self) {
        while let Tag::CurChar(c) = self.peek() {
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

    pub fn next_token(&mut self) -> Token {

        self.skip_whitespace();

        match self.peek() {
            Tag::CurChar(c) => {
                match c {
                    // operator
                    '+'|'-'|'*'|'/'|'('|')'|'{'|'}'|'['|']'|'='|'<'|'>'|'!'|'^' => {
                        let mut op = c.to_string();
                        if c == '=' {
                            if let Tag::CurChar(nc) = self.next() {
                                if matches!(nc, '=' | '>' | '<') {
                                    op += &nc.to_string();
                                    self.next();
                                }
                            }
                        } else if c == '!' {
                            if let Tag::CurChar(nc) = self.next() {
                                if matches!(nc, '=') {
                                    op += &nc.to_string();
                                    self.next();
                                }
                            }
                        } else {
                            self.next();
                        }
                        Token::Operator(op)
                    }
                    // semicolon
                    ','|';' => {
                        self.next();
                        Token::Semicolon(c.to_string())
                    }
                    // identifier or keyword
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let start = self.position;
                        let mut end = start + 1;
                        while let Tag::CurChar(nc) = self.next() {
                            if Self::isletter_or_digit(nc) {
                                end += 1;
                            } else {
                                break;
                            }
                        }
                        let word = &self.input[start..end];
                        if Self::iskeywords(word) {
                            Token::Keyword(word.to_string())
                        } else {
                            Token::Identifier(word.to_string())
                        }
                    }
                    // number
                    '0'..='9' => {
                        let start = self.position;
                        let mut end = start + 1;
                        let mut num_type = 0;
                        while let Tag::CurChar(nc) = self.next() {
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
                        Token::Number(num)
                    }
                    // string
                    '\"' => {
                        let start = self.position;
                        let mut end = start + 1;
                        while let Tag::CurChar(nc) = self.next() {
                            if nc != '\"' {
                                end += 1;
                            } else {
                                break;
                            }
                        }
                        let string = &self.input[start..end];
                        self.next();
                        Token::String(string.to_string())
                    }
                    _ => {
                        panic!("error in make token");
                    }
                }
            }
            Tag::EOF => {
                Token::EOF
            }
        }
    }
}