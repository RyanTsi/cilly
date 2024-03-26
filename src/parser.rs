use core::panic;
use std::clone;
use std::fmt::format;
use std::os::linux::raw;
use std::ptr::eq;
use std::result;

use crate::error::*;
use crate::lexer::*;

#[derive(Debug, PartialEq)]
pub enum Node {
    Program(Vec<Node>),
    Statement(Statement),
    Expr(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Atom(Atom),
    Pow(Atom, Box<Expr>),
    Unary(Vec<String>, Box<Expr>),
    Factor(Vec<(Option<String>, Expr)>),
    Term(Vec<(Option<String>, Expr)>),
    Comparison(Vec<(Option<String>, Expr)>),
    Equality(Vec<(Option<String>, Expr)>),
    LogicAnd(Vec<Expr>),
    LogicOr(Vec<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    RetStat(Option<Expr>),
    ExprStat(Expr),
    IfStat(Expr, Box<Statement>, Option<Box<Statement>>),
    WhileStat(Expr, Box<Statement>),
    VarStat(Atom, Expr),
    AssignStat(Atom, Expr),
    BlockStat(Vec<Statement>),
    FuncStat(Atom, Option<Box<Statement>>, Box<Statement>),
    PrintStat(Option<Box<Statement>>),
    Args(Vec<Expr>),
    Params(Vec<Atom>),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Call(Box<Atom>, Box<Statement>),
    Identifier(String),
    Num(f64),
    True,
    False,
    Expr(Box<Expr>),
}

// 定义 Parser 结构体
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    // 构造函数
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    // 获取当前位置 token, 并移动到下一个 token
    fn next(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            self.position += 1;
            self.tokens.get(self.position - 1)
        } else {
            None
        }
    }

    // 检查是否和所给 token 匹配，若匹配则移动到下一个 token
    fn match_token(&mut self, token_type: Token) -> Result<(), error> {
        match self.next() {
            Some(tk) => {
                if *tk == token_type {
                    Ok(())
                } else {
                    Err(error::ParserError(format!("Expected {:?}, found {:?}", token_type, self.peek())))
                }
            }
            None => Err(error::ParserError("Unexpected end of input".to_string()))
        }
    }

    // 检查是否和所给的 keyword 匹配，若匹配则移动到下一个 token
    fn match_keyword(&mut self, keyword: &str) -> Result<(), error> {
        match self.next() {
            Some(Token::Keyword(ref kw)) if kw == keyword => Ok(()),
            Some(tk) => Err(error::ParserError(format!("Expected keyword '{}', found {:?}", keyword, tk))),
            None => Err(error::ParserError("Unexpected end of input".to_string()))
        }
    }

    // 得到 id, 并移动到下一个 token
    fn consume_identifier(&mut self) -> Result<Atom, error> {
        match self.next() {
            Some(Token::Identifier(id)) => Ok(Atom::Identifier(id.clone())),
            Some(tk) => Err(error::ParserError(format!("Expected identifier, found {:?}", tk))),
            None => Err(error::ParserError("Unexpected end of input".to_string())),
        }
    }

    // 查看当前位置 token
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn is_at_end(&self) -> Result<bool, error> {
        match self.peek() {
            Some(Token::EOF) => Ok(true),
            Some(tk) => Ok(false),
            None => Err(error::ParserError("Unexpected end of input".to_string()))
        }
    }

    // 解析入口函数
    pub fn parse(&mut self) -> Result<Node, error> {
        self.program()
    }

    // 具体的解析方法，按照文法规则逐步实现
    fn program(&mut self) -> Result<Node, error> {
        let mut statements = Vec::new();
        while !self.is_at_end()? {
            statements.push(Node::Statement(self.statement()?));
        }
        Ok(Node::Program(statements))
    }

    fn statement(&mut self) -> Result<Statement, error> {
        match self.peek() {
            Some(Token::Keyword(ref kw)) => {
                if      kw == "return" {  Ok(self.ret_stat()?)   }
                else if kw == "if"     {  Ok(self.if_stat()?)    }
                else if kw == "while"  {  Ok(self.while_stat()?) }
                else if kw == "var"    {  Ok(self.var_stat()?)   }
                else if kw == "fun"    {  Ok(self.func_stat()?)  }
                else if kw == "print"  {  Ok(self.print_stat()?) }
                else { Err(error::ParserError(format!("Unexpected keyword {}", kw)))}
            }
            Some(Token::Identifier(_)) => {
                let id = self.consume_identifier()?;
                match self.peek() {
                    Some(Token::Operator(ref sym)) if sym == "=" => Ok(self.assign_stat(id)?),
                    None => Err(error::ParserError("Unexpected end of input".to_string())),
                    _ => Ok(self.expr_stat2(id)?),
                }
            }
            Some(Token::Operator(ref sym)) if sym == "{" => Ok(self.block_stat()?),
            _ => Ok(self.expr_stat()?),
        }
    }

    fn ret_stat(&mut self) -> Result<Statement, error> {
        self.match_keyword("return");
        let expr = match self.peek() {
            Some(Token::Semicolon(ref sem)) if sem == ";" => { Ok(None) }
            None => Err(error::ParserError("Unexpected end of input".to_string())),
            _ => { Ok(Some(self.expr()?)) }
        }?;
        self.match_token(Token::Semicolon(";".to_string()));

        Ok(Statement::RetStat(expr))
    }

    fn expr_stat(&mut self) -> Result<Statement, error> {
        let expr = self.expr()?;
        self.match_token(Token::Semicolon(";".to_string()));
        Ok(Statement::ExprStat(expr))
    }
    fn expr_stat2(&mut self, id: Atom) -> Result<Statement, error> {
        todo!()
    }

    fn if_stat(&mut self) -> Result<Statement, error> {
        self.match_keyword("if");
        self.match_token(Token::Operator("(".to_string()));
        let condition = self.expr()?;
        self.match_token(Token::Operator(")".to_string()));
        let then_branch = Box::new(self.statement()?);
        let else_branch = match self.peek() {
            Some(Token::Keyword(ref kw)) if kw == "else" => {
                self.match_keyword("else");
                Ok(Some(Box::new(self.statement()?)))
            }
            None => { Err(error::ParserError("Unexpected end of input".to_string())) }
            _ => { Ok(None) }
        }?;
        Ok(Statement::IfStat(condition, then_branch, else_branch))
    }

    fn while_stat(&mut self) -> Result<Statement, error> {
        self.match_keyword("while");
        self.match_token(Token::Operator("(".to_string()));
        let condition = self.expr()?;
        self.match_token(Token::Operator(")".to_string()));
        let body = Box::new(self.statement()?);
        Ok(Statement::WhileStat(condition, body))
    }

    fn var_stat(&mut self) -> Result<Statement, error> {
        self.match_keyword("var");
        let id = self.consume_identifier()?;
        self.match_token(Token::Operator("=".to_string()));
        let expr = self.expr()?;
        self.match_token(Token::Semicolon(";".to_string()));
        Ok(Statement::VarStat(id, expr))
    }

    fn assign_stat(&mut self, name: Atom) -> Result<Statement, error> {
        self.match_token(Token::Operator("=".to_string()));
        let expr = self.expr()?;
        self.match_token(Token::Semicolon(";".to_string()));
        Ok(Statement::AssignStat(name, expr))
    }

    fn block_stat(&mut self) -> Result<Statement, error> {
        self.match_token(Token::Operator("{".to_string()));
        let mut statements = Vec::new();
        loop {
            match self.peek() {
                Some(Token::Operator(ref op)) if op == "}" => break,
                None => return Err(error::ParserError("Unexpected end of input".to_string())),
                _ => statements.push(self.statement()?),
            }
        }
        self.match_token(Token::Operator("}".to_string()));
        Ok(Statement::BlockStat(statements))
    }

    fn func_stat(&mut self) -> Result<Statement, error> {
        self.match_keyword("fun");
        let id = self.consume_identifier()?;
        self.match_token(Token::Operator("(".to_string()));
        let params = match self.peek() {
            Some(Token::Operator(ref op)) if op == ")" => Ok(Some(Box::new(self.params()?))),
            None => Err(error::ParserError("Unexpected end of input".to_string())),
            _ => Ok(None),
        }?;
    
        self.match_token(Token::Operator(")".to_string()));
        let body = self.block_stat()?;

        Ok(Statement::FuncStat(id, params, Box::new(body)))
    }

    fn print_stat(&mut self) -> Result<Statement, error> {
        self.match_keyword("print");
        self.match_token(Token::Operator("(".to_string()));
        let args = match self.peek() {
            Some(Token::Operator(ref op)) if op == ")" => Ok(Some(Box::new(self.args()?))),
            None => Err(error::ParserError("Unexpected end of input".to_string())),
            _ => Ok(None)
        }?;
        self.match_token(Token::Operator(")".to_string()));
        self.match_token(Token::Semicolon(";".to_string()));
       
        Ok(Statement::PrintStat(args))
    }

    fn args(&mut self) -> Result<Statement, error> {
        let mut args = vec![self.expr()?];
        loop {
            match self.peek() {
                Some(Token::Semicolon(ref sem)) if sem == "," => {
                    self.match_token(Token::Semicolon(",".to_string()));
                    args.push(self.expr()?);
                }
                None => return Err(error::ParserError("Unexpected end of input".to_string())),
                _ => break,
            }
        }
        Ok(Statement::Args(args))
    }

    fn params(&mut self) -> Result<Statement, error> {
        let mut params = vec![self.consume_identifier()?];
        loop {
            match self.peek() {
                Some(Token::Semicolon(ref sem)) if sem == "," => {
                    self.match_token(Token::Semicolon(",".to_string()));
                    params.push(self.consume_identifier()?);
                }
                None => return Err(error::ParserError("Unexpected end of input".to_string())),
                _ => break,
            }
        }
        Ok(Statement::Params(params))
    }

    fn call(&mut self, id: Atom) -> Result<Atom, error> {
        self.match_token(Token::Operator("(".to_string()));
        let args = self.args()?;
        self.match_token(Token::Operator(")".to_string()));
        Ok(Atom::Call(Box::new(id), Box::new(args)))
    }

    fn atom(&mut self) -> Result<Atom, error> {
        match self.peek() {
            Some(Token::Identifier(_)) => {
                let id = self.consume_identifier()?;
                match self.peek() {
                    Some(Token::Operator(ref op)) if op == "(" => self.call(id),
                    None => return Err(error::ParserError("Unexpected end of input".to_string())),
                    _ => Ok(id),
                }
            }
            Some(Token::Number(num)) => {
                let res = num.clone();
                self.next();
                Ok(Atom::Num(res))
            }
            Some(Token::Keyword(ref kw)) if kw == "true"  => {
                self.match_keyword("true");
                Ok(Atom::True)
            }
            Some(Token::Keyword(ref kw)) if kw == "false" => {
                self.match_keyword("false");
                Ok(Atom::False)
            }
            Some(Token::Operator(op)) if op == "(" => {
                self.match_token(Token::Operator("(".to_string()));
                let  res = self.expr()?;
                self.match_token(Token::Operator(")".to_string()));
                Ok(Atom::Expr(Box::new(res)))
            }
            _ => Err(error::ParserError("Unexpected input".to_string()))
        }
    }

    fn pow(&mut self) -> Result<Expr, error> {
        let atom = self.atom()?;
        match self.peek() {
            Some(Token::Operator(ref op)) if op == "^" => {
                self.match_token(Token::Operator("^".to_string()));
                Ok(Expr::Pow(atom, Box::new(self.pow()?)))
            }
            _ => Ok(Expr::Atom(atom)),
        }
    }
    
    fn unary(&mut self) -> Result<Expr, error> {
        let mut ops = Vec::new();
        loop {
            match self.peek() {
                Some(Token::Operator(ref op)) if op == "!" => {
                    self.match_token(Token::Operator(op.clone()));
                    ops.push("!".to_string());
                }
                Some(Token::Operator(ref op)) if op == "-" => {
                    self.match_token(Token::Operator(op.clone()));
                    ops.push("-".to_string());
                }
                _ => break,
            }
        }
        if ops.is_empty() {
            self.pow()
        } else {
            Ok(Expr::Unary(ops, Box::new(self.pow()?)))
        }
    }

    fn factor(&mut self) -> Result<Expr, error> {
        let mut unarys = Vec::new();
        unarys.push((None, self.unary()?));
        
        loop {
            match self.peek() {
                Some(Token::Operator(ref op)) if op == "*" => {
                    self.match_token(Token::Operator(op.clone()));
                    unarys.push((Some("*".to_string()), self.unary()?));
                }
                Some(Token::Operator(ref op)) if op == "/" => {
                    self.match_token(Token::Operator(op.clone()));
                    unarys.push((Some("/".to_string()), self.unary()?));
                }
                _ => break,
            }
        }
        if unarys.len() == 1 {
            Ok(unarys[0].1.clone())
        } else {
            Ok(Expr::Factor(unarys))
        }
    }
    
    fn term(&mut self) -> Result<Expr, error> {
        let mut factors = Vec::new();
        factors.push((None, self.factor()?));
        
        loop {
            match self.peek() {
                Some(Token::Operator(ref op)) if op == "+" => {
                    self.match_token(Token::Operator(op.clone()));
                    factors.push((Some("+".to_string()), self.factor()?));
                }
                Some(Token::Operator(ref op)) if op == "-" => {
                    self.match_token(Token::Operator(op.clone()));
                    factors.push((Some("-".to_string()), self.factor()?));
                }
                _ => break,
            }
        }
        if factors.len() == 1 {
            Ok(factors[0].1.clone())
        } else {
            Ok(Expr::Term(factors))
        }
    }

    fn comparison(&mut self) -> Result<Expr, error> {
        let mut terms = Vec::new();

        terms.push((None, self.term()?));
        
        loop {
            match self.peek() {
                Some(Token::Operator(ref op)) if op == ">" => {
                    self.match_token(Token::Operator(op.clone()));
                    terms.push((Some(">".to_string()), self.term()?));
                }
                Some(Token::Operator(ref op)) if op == "<" => {
                    self.match_token(Token::Operator(op.clone()));
                    terms.push((Some("<".to_string()), self.term()?));
                }
                Some(Token::Operator(ref op)) if op == ">=" => {
                    self.match_token(Token::Operator(op.clone()));
                    terms.push((Some(">=".to_string()), self.term()?));
                }
                Some(Token::Operator(ref op)) if op == "<=" => {
                    self.match_token(Token::Operator(op.clone()));
                    terms.push((Some("<=".to_string()), self.term()?));
                }
                _ => break,
            }
        }
        if terms.len() == 1 {
            Ok(terms[0].1.clone())
        } else {
            Ok(Expr::Comparison(terms))
        }
    }

    fn equality(&mut self) -> Result<Expr, error> {
        let mut comparisons = Vec::new();
        comparisons.push((None, self.comparison()?));

        loop {
            match self.peek() {
                Some(Token::Operator(ref op)) if op == "==" => {
                    self.match_token(Token::Operator(op.clone()));
                    comparisons.push((Some("==".to_string()), self.comparison()?));
                }
                Some(Token::Operator(ref op)) if op == "!=" => {
                    self.match_token(Token::Operator(op.clone()));
                    comparisons.push((Some("!=".to_string()), self.comparison()?));
                }
                _ => break,
            }
        }
        if comparisons.len() == 1 {
            Ok(comparisons[0].1.clone())
        } else {
            Ok(Expr::Equality(comparisons))
        }
    }


    fn logic_and(&mut self) -> Result<Expr, error>{
        let mut equalitys = Vec::new();
        equalitys.push(self.equality()?);

        loop {
            match self.peek() {
                Some(Token::Keyword(ref kw)) if kw == "and" => {
                    self.match_keyword("and");
                    equalitys.push(self.equality()?);
                }
                _ => break,
            }
        }
        if equalitys.len() == 1 {
            Ok(equalitys[0].clone())
        } else {
            Ok(Expr::LogicAnd(equalitys))
        }
    }
    fn logic_or(&mut self) -> Result<Expr, error> {
        let mut logic_ors = Vec::new();
        logic_ors.push(self.logic_and()?);

        loop {
            match self.peek() {
                Some(Token::Keyword(ref kw)) if kw == "or" => {
                    self.match_keyword("or");
                    logic_ors.push(self.logic_and()?);
                }
                _ => break,
            }
        }
        if logic_ors.len() == 1 {
            Ok(logic_ors[0].clone())
        } else {
            Ok(Expr::LogicAnd(logic_ors))
        }
    }
    fn expr(&mut self) -> Result<Expr, error> {
        self.logic_or()
    }


}