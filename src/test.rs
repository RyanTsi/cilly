use crate::error::*;

use crate::parser::*;

use super::lexer::*;

#[test]
fn test_lexer() {
    let inp = "return ;";
    let mut lexer = Lexer::new(inp);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());
        if token == Token::EOF {
            break;
        }
    }
    println!("{:?}", tokens);
}

#[test]
fn test_parser() -> Result<(), error>{
    let input = "if(1 == 2) x = 3;";
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());
        if token == Token::EOF {
            break;
        }
    }
    println!("{:?}", tokens.clone());
    let mut parser = Parser::new(tokens);

    let res = parser.parse()?;
    println!("{:?}",res);
    Ok(())
}