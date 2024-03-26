cilly_grammar

```
program : statement* EOF;

statement : ret_stat | if_stat | while_stat | var_stat | func_stat
          | assign_stat | block_stat | expr_stat;

ret_stat: 'return' expr? ';' ;
expr_stat: expr ';' ;
if_stat : 'if' '(' expr ')' statement ('else' statement)? ;
while_stat : 'while' '(' expr ')' statement ;
var_stat : 'var' ID '=' expr ';' ;
assign_stat: ID '=' expr ';' ;
block_stat: '{' statement* '}'
func_stat: 'fun' ID '(' params? ')' block_stat;
print_stat: 'print' '(' args? ')' ';' ;
args : expr (',' expr)* ;
params : ID (',' ID)* ;

call : ID '(' args ')';
atom : call | ID | NUM | 'true' | 'false' | '(' expr ')';
pow : atom ^ pow | atom;
unary : ('-' | '!')* pow;
factor: unary ( ('*' | '/') unary)* ;
term: factor ( ('+' | '-') factor)* ;
comparison: term ( ('>' | '>=' | '<' | '<=') term)* ;
equality :comparison ( ('=='|'!=') comparison)*;
logic_and : equality ( 'and' equality)*;
logic_or : logic_and ( 'or' logic_and)*;
expr : logic_or;
```

```rust
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
```