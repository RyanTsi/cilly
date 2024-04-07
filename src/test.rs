use crate::error::*;

use crate::parser::*;

use super::lexer::*;


#[test]
fn test_lexer() -> Result<()>{
    let inp = "return ;";
    let mut lexer = Lexer::new(inp);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token()?;
        tokens.push(token.clone());
        if token == Token::EOF {
            break;
        }
    }
    println!("{:?}", tokens);
    Ok(())
}

#[test]
fn test_parser() -> Result<()>{
    
    let test = 
    r#"
    fun make_dog(){
        var weight = 10;
        fun eat(m){
            weight = m + weight;        
        }
        fun get(){
            return weight;
        }
        fun dispatch(m){
            if(m == "eat"){
                return eat;
            } else if (m == "get"){
                return get();
            }
        }
        return dispatch;
    }
    "#;
    
    
    let mut lexer = Lexer::new(test);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token()?;
        tokens.push(token.clone());
        if token == Token::EOF {
            break;
        }
    }
    // println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);

    let res = parser.parse()?;
    println!("{:?}",res);

    Ok(())
}




/*
fun make_count(n){
    fun inc(){
        n = n + 1;
        return n;
    }
    return inc;
}
fun make_dog(){
    var weight = 10;
    fun eat(m){
        weight = m + weight;        
    }
    fun get(){
        return weight;
    }
    fun dispatch(m){
        if(m == "eat"){
            return eat;
        } else if (m == "get"){
            return get();
        }
    }
    return dispatch;
}

print(2*10!);

*/