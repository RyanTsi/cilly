use core::panic;

use crate::error::*;
use crate::lexer::*;

#[derive(Debug, PartialEq)]
pub enum Node {
    Program(Vec<Node>),
    Statement(Statement),
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Atom(Atom),
    Pow(Atom, Option<Box<Expr>>),
    Unary(Vec<String>, Box<Expr>),
    Factor(Vec<(Option<String>, Expr)>),
    Term(Vec<(Option<String>, Expr)>),
    Comparison(Vec<(Option<String>, Expr)>),
    Equality(Vec<(Option<String>, Expr)>),
    LogicAnd(Vec<Expr>),
    LogicOr(Vec<Expr>),
}

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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

    // 辅助方法
    fn next(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            self.position += 1;
            self.tokens.get(self.position - 1)
        } else {
            None
        }
    }

    fn match_token(&mut self, token_type: Token) -> Result<&Token, error> {
        if *self.peek() == token_type {
            Ok(self.next())
        } else {
            Err(error::ParserError)
        }
    }

    fn match_keyword(&mut self, keyword: &str) -> Result<&Token, error>{
        if *self.peek() == Token::Keyword(keyword.to_string()) {
            Ok(self.next())
        } else {
            Err(error::ParserError)
        }
    }

    fn consume_identifier(&mut self) -> Atom {
        match self.peek() {
            Token::Identifier(ref id) => {
                let id = id.clone();
                self.next();
                Atom::Identifier(id)
            }
            _ => panic!("Expected identifier, found {:?}", self.peek()),
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len() || *self.peek() == Token::EOF
    }

    // 解析入口函数
    pub fn parse(&mut self) -> Node {
        self.program()
    }

    // 具体的解析方法，按照文法规则逐步实现
    fn program(&mut self) -> Node {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(Node::Statement(self.statement()));
        }
        Node::Program(statements)
    }

    fn statement(&mut self) -> Statement {
        match self.peek() {
            Token::Keyword(ref kw) if kw == "return" => self.ret_stat(),
            Token::Keyword(ref kw) if kw == "if" => self.if_stat(),
            Token::Keyword(ref kw) if kw == "while" => self.while_stat(),
            Token::Keyword(ref kw) if kw == "var" => self.var_stat(),
            Token::Keyword(ref kw) if kw == "fun" => self.func_stat(),
            Token::Keyword(ref kw) if kw == "print" => self.print_stat(),
            Token::Identifier(_) => {
                let name = self.consume_identifier();
                match self.peek() {
                    Token::Operator(ref sym) if sym == "=" => self.assign_stat(name),
                    _ => self.expr_stat(),
                }
            }
            Token::Operator(ref sym) if sym == "{" => self.block_stat(),
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }

    fn ret_stat(&mut self) -> Statement {
        self.match_keyword("return");
        let expr = if *self.peek() != Token::Semicolon(";".to_string()) {
            Some(self.expr())
        } else {
            None
        };
        self.match_token(Token::Semicolon(";".to_string()));

        Statement::RetStat(expr)
    }

    fn expr_stat(&mut self) -> Statement {
        let expr = self.expr();
        self.match_token(Token::Semicolon(";".to_string()));
        Statement::ExprStat(expr)
    }

    fn if_stat(&mut self) -> Statement {
        self.match_keyword("if");
        self.match_token(Token::Operator("(".to_string()));
        let condition = self.expr();
        self.match_token(Token::Operator(")".to_string()));
        let then_branch = Box::new(self.statement());
        let else_branch = if *self.peek() == Token::Keyword("else".to_string()) {
            self.next();
            Some(Box::new(self.statement()))
        } else {
            None
        };
        Statement::IfStat(condition, then_branch, else_branch)
    }

    fn while_stat(&mut self) -> Statement {
        self.match_keyword("while");
        self.match_token(Token::Operator("(".to_string()));
        let condition = self.expr();
        self.match_token(Token::Operator(")".to_string()));
        let body = Box::new(self.statement());
        Statement::WhileStat(condition, body)
    }

    fn var_stat(&mut self) -> Statement {
        self.match_keyword("var");
        let name = self.consume_identifier();
        self.match_token(Token::Operator("=".to_string()));
        let expr = self.expr();
        self.match_token(Token::Semicolon(";".to_string()));
        Statement::VarStat(name, expr)
    }

    fn assign_stat(&mut self, name: Atom) -> Statement {
        self.match_token(Token::Operator("=".to_string()));
        let expr = self.expr();
        self.match_token(Token::Semicolon(";".to_string()));
        Statement::AssignStat(name, expr)
    }

    fn block_stat(&mut self) -> Statement {
        self.match_token(Token::Operator("{".to_string()));
        let mut statements = Vec::new();
        while *self.peek() != Token::Operator("}".to_string()) {
            statements.push(self.statement());
        }
        self.match_token(Token::Operator("}".to_string()));
        Statement::BlockStat(statements)
    }

    fn func_stat(&mut self) -> Statement {
        self.match_keyword("fun");
        let name = self.consume_identifier();
        self.match_token(Token::Operator("(".to_string()));
        let params = if *self.peek() != Token::Operator(")".to_string()) {
            Some(Box::new(self.params()))
        } else {
            None
        };
        self.match_token(Token::Operator(")".to_string()));
        let body = self.block_stat();
        Statement::FuncStat(name, params, Box::new(body))
    }

    fn print_stat(&mut self) -> Statement {
        self.match_keyword("print");
        self.match_token(Token::Operator("(".to_string()));
        let args = if *self.peek() != Token::Operator(")".to_string()) {
            Some(Box::new(self.args()))
        } else {
            None
        };
        self.match_token(Token::Operator(")".to_string()));
        self.match_token(Token::Semicolon(";".to_string()));
        Statement::PrintStat(args)
    }

    fn args(&mut self) -> Statement {
        let mut args = vec![self.expr()];
        while *self.peek() == Token::Operator(",".to_string()) {
            self.next();
            args.push(self.expr());
        }
        Statement::Args(args)
    }

    fn params(&mut self) -> Statement {
        let mut params = vec![self.consume_identifier()];
        while *self.peek() == Token::Operator(",".to_string()) {
            self.next();
            params.push(self.consume_identifier());
        }
        Statement::Params(params)
    }

    fn call(&mut self, name: Atom) -> Atom {
        self.match_token(Token::Operator("(".to_string()));
        let args = if *self.peek() != Token::Operator(")".to_string()) {
            self.args()
        } else {
            panic!("Expected Call, found None");
        };
        self.match_token(Token::Operator(")".to_string()));
        Atom::Call(Box::new(name), Box::new(args))
    }

    fn atom(&mut self) -> Atom {
        match self.peek() {
            Token::Identifier(_) => {
                let name = self.consume_identifier();
                if *self.peek() != Token::Operator("(".to_string()) {
                    name
                } else {
                    self.call(name)
                }
            }
            Token::Number(num) => {
                Atom::Num(num.clone())
            }
            Token::Keyword(kw) => {
                if *kw == "true" {
                    Atom::True
                } else if *kw == "false" {
                    Atom::False
                } else {
                    panic!("Expected atom, found {:?}", self.peek())
                }
            }
            Token::Operator(op) => {
                if *op == "(" {
                    self.next();
                    let res = self.expr();
                    self.next();
                    Atom::Expr(Box::new(res))
                } else {
                    panic!("Expected atom, found {:?}", self.peek())
                }
            }
            _ => panic!("Expected atom, found {:?}", self.peek())
        }
    }

    fn pow(&mut self) -> Expr {
        let atom = self.atom();
        let mut pow = None;
        if *self.peek() == Token::Operator("^".to_string()) {
            self.next();
            pow = Some(Box::new(self.pow()));
        }
        Expr::Pow(atom, pow)
    }
    
    fn unary(&mut self) -> Expr {
        let mut ops = Vec::new();
        
        while let Token::Operator(op) = self.next() {
            if op == "!" || op == "-" {
                ops.push(op.clone());
            } else {
                panic!("Expected operator -or!, found {:?}", op)
            }
        }
        Expr::Unary(ops, Box::new(self.pow()))
    }

    fn factor(&mut self) -> Expr {
        let mut unarys = Vec::new();
        unarys.push((None,self.unary()));
        
        while let Token::Operator(op) = self.next() {
            if op == "*" || op == "/" {
                unarys.push((Some(op.clone()), self.unary()));
            } else {
                panic!("Expected operator *or/, found {:?}", op)
            }
        }
        Expr::Factor(unarys)
    }
    
    fn term(&mut self) -> Expr {
        let mut factors = Vec::new();
        factors.push((None,self.factor()));
        
        while let Token::Operator(op) = self.next() {
            if op == "+" || op == "-" {
                factors.push((Some(op.clone()), self.factor()));
            } else {
                panic!("Expected operator +or-, found {:?}", op)
            }
        }
        Expr::Term(factors)
    }

    fn comparison(&mut self) -> Expr{
        let mut terms = Vec::new();
        terms.push((None,self.term()));
        
        while let Token::Operator(op) = self.next() {
            if op == ">" || op == ">=" || op == "<" || op == "<=" {
                terms.push((Some(op.clone()), self.term()));
            } else {
                panic!("Expected operator >or<or>=or<=, found {:?}", op)
            }
        }
        Expr::Comparison(terms)
    }

    fn equality(&mut self) -> Expr{
        let mut comparisons = Vec::new();
        comparisons.push((None, self.comparison()));
        
        while let Token::Operator(op) = self.next() {
            if op == "==" || op == "!=" {
                comparisons.push((Some(op.clone()), self.comparison()));
            } else {
                panic!("Expected operator !=or==, found {:?}", op)
            }
        }
        Expr::Equality(comparisons)
    }
    fn logic_and(&mut self) -> Expr{
        let mut equalitys = Vec::new();
        equalitys.push(self.equality());
        while let Token::Keyword(kw) = self.next() {
            if kw == "and" {
                equalitys.push(self.equality());
            } else {
                panic!("Expected keyword and, found {:?}", kw)
            }
        }
        Expr::LogicAnd(equalitys)
    }
    fn logic_or(&mut self) -> Expr {
        let mut logic_ors = Vec::new();
        logic_ors.push(self.logic_and());

        while let Token::Keyword(kw) = self.next() {
            if kw == "or" {
                logic_ors.push(self.logic_and());
            } else {
                panic!("Expected keyword or, found {:?}", kw)
            }
        }
        Expr::LogicOr(logic_ors)
    }
    fn expr(&mut self) -> Expr {
        self.logic_or()
    }
}