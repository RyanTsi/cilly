use crate::parser::*;

use super::lexer::*;

#[test]
fn test_lexer() {
    let inp = "if (n == 1) y = 1";
    let mut lexer = Lexer::new(inp);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }
    println!("{:?}", tokens);
}

#[test]
fn test_parser() {
    let input = "if (n == 1) y = 1";
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);

    let res = parser.parse();
    println!("{:?}",res);
}