## cilly_grammar

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

call : ID '(' args? ')';
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


test
```
fun fact(n){
    if(n==0)
        return 1;
    else
        return n * fact(n-1);
}
print(fact(10));
fun k(x){
    fun ky(y){
        return x + y;
    }
    return ky;
}
var ky = k(3);
print(ky(5));
fun fib0(n){
    if(n < 2)
        return n;
    else
        return fib0(n-1) + fib0(n-2);
}
fun fib(n){
    var f0 = 0;
    var f1 = 1;
    while(n > 0){
        var t = f1;
        f1 = f0 + f1;
        f0 = t;
        n = n - 1;
    }
    return f0;
}
print(fib(10),"hello world");

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
var dog = make_dog();
var eat = dog("eat");
eat(10);
print(dog("get"));
eat(20);
print(dog("get"));
var c1 = make_count(1);
var c2 = make_count(1);
print(c1(), c1(), c1(), c2());
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



## tokens

[Keyword("fun"), Identifier("fact"), Operator("("), Identifier("n"), Operator(")"), Operator("{"), Keyword("if"), Operator("("), Identifier("n"), Operator("=="), Number(0.0), Operator(")"), Keyword("return"), Number(1.0), Semicolon(";"), Keyword("else"), Keyword("return"), Identifier("n"), Operator("*"), Identifier("fact"), Operator("("), Identifier("n"), Operator("-"), Number(1.0), Operator(")"), Semicolon(";"), Operator("}"), Keyword("print"), Operator("("), Identifier("fact"), Operator("("), Number(10.0), Operator(")"), Operator(")"), Semicolon(";"), Keyword("fun"), Identifier("k"), Operator("("), Identifier("x"), Operator(")"), Operator("{"), Keyword("fun"), Identifier("ky"), Operator("("), Identifier("y"), Operator(")"), Operator("{"), Keyword("return"), Identifier("x"), Operator("+"), Identifier("y"), Semicolon(";"), Operator("}"), Keyword("return"), Identifier("ky"), Semicolon(";"), Operator("}"), Keyword("var"), Identifier("ky"), Operator("="), Identifier("k"), Operator("("), Number(3.0), Operator(")"), Semicolon(";"), Keyword("print"), Operator("("), Identifier("ky"), Operator("("), Number(5.0), Operator(")"), Operator(")"), Semicolon(";"), Keyword("fun"), Identifier("fib0"), Operator("("), Identifier("n"), Operator(")"), Operator("{"), Keyword("if"), Operator("("), Identifier("n"), Operator("<"), Number(2.0), Operator(")"), Keyword("return"), Identifier("n"), Semicolon(";"), Keyword("else"), Keyword("return"), Identifier("fib0"), Operator("("), Identifier("n"), Operator("-"), Number(1.0), Operator(")"), Operator("+"), Identifier("fib0"), Operator("("), Identifier("n"), Operator("-"), Number(2.0), Operator(")"), Semicolon(";"), Operator("}"), Keyword("fun"), Identifier("fib"), Operator("("), Identifier("n"), Operator(")"), Operator("{"), Keyword("var"), Identifier("f0"), Operator("="), Number(0.0), Semicolon(";"), Keyword("var"), Identifier("f1"), Operator("="), Number(1.0), Semicolon(";"), Keyword("while"), Operator("("), Identifier("n"), Operator(">"), Number(0.0), Operator(")"), Operator("{"), Keyword("var"), Identifier("t"), Operator("="), Identifier("f1"), Semicolon(";"), Identifier("f1"), Operator("="), Identifier("f0"), Operator("+"), Identifier("f1"), Semicolon(";"), Identifier("f0"), Operator("="), Identifier("t"), Semicolon(";"), Identifier("n"), Operator("="), Identifier("n"), Operator("-"), Number(1.0), Semicolon(";"), Operator("}"), Keyword("return"), Identifier("f0"), Semicolon(";"), Operator("}"), Keyword("print"), Operator("("), Identifier("fib"), Operator("("), Number(10.0), Operator(")"), Semicolon(","), String("hello world"), Operator(")"), Semicolon(";"), Keyword("fun"), Identifier("make_count"), Operator("("), Identifier("n"), Operator(")"), Operator("{"), Keyword("fun"), Identifier("inc"), Operator("("), Operator(")"), Operator("{"), Identifier("n"), Operator("="), Identifier("n"), Operator("+"), Number(1.0), Semicolon(";"), Keyword("return"), Identifier("n"), Semicolon(";"), Operator("}"), Keyword("return"), Identifier("inc"), Semicolon(";"), Operator("}"), Keyword("fun"), Identifier("make_dog"), Operator("("), Operator(")"), Operator("{"), Keyword("var"), Identifier("weight"), Operator("="), Number(10.0), Semicolon(";"), Keyword("fun"), Identifier("eat"), Operator("("), Identifier("m"), Operator(")"), Operator("{"), Identifier("weight"), Operator("="), Identifier("m"), Operator("+"), Identifier("weight"), Semicolon(";"), Operator("}"), Keyword("fun"), Identifier("get"), Operator("("), Operator(")"), Operator("{"), Keyword("return"), Identifier("weight"), Semicolon(";"), Operator("}"), Keyword("fun"), Identifier("dispatch"), Operator("("), Identifier("m"), Operator(")"), Operator("{"), Keyword("if"), Operator("("), Identifier("m"), Operator("=="), String("eat"), Operator(")"), Operator("{"), Keyword("return"), Identifier("eat"), Semicolon(";"), Operator("}"), Keyword("else"), Keyword("if"), Operator("("), Identifier("m"), Operator("=="), String("get"), Operator(")"), Operator("{"), Keyword("return"), Identifier("get"), Operator("("), Operator(")"), Semicolon(";"), Operator("}"), Operator("}"), Keyword("return"), Identifier("dispatch"), Semicolon(";"), Operator("}"), Keyword("var"), Identifier("dog"), Operator("="), Identifier("make_dog"), Operator("("), Operator(")"), Semicolon(";"), Keyword("var"), Identifier("eat"), Operator("="), Identifier("dog"), Operator("("), String("eat"), Operator(")"), Semicolon(";"), Identifier("eat"), Operator("("), Number(10.0), Operator(")"), Semicolon(";"), Keyword("print"), Operator("("), Identifier("dog"), Operator("("), String("get"), Operator(")"), Operator(")"), Semicolon(";"), Identifier("eat"), Operator("("), Number(20.0), Operator(")"), Semicolon(";"), Keyword("print"), Operator("("), Identifier("dog"), Operator("("), String("get"), Operator(")"), Operator(")"), Semicolon(";"), Keyword("var"), Identifier("c1"), Operator("="), Identifier("make_count"), Operator("("), Number(1.0), Operator(")"), Semicolon(";"), Keyword("var"), Identifier("c2"), Operator("="), Identifier("make_count"), Operator("("), Number(1.0), Operator(")"), Semicolon(";"), Keyword("print"), Operator("("), Identifier("c1"), Operator("("), Operator(")"), Semicolon(","), Identifier("c1"), Operator("("), Operator(")"), Semicolon(","), Identifier("c1"), Operator("("), Operator(")"), Semicolon(","), Identifier("c2"), Operator("("), Operator(")"), Operator(")"), Semicolon(";"), Keyword("print"), Operator("("), Number(2.0), Operator("*"), Number(10.0), Operator("!"), Operator(")"), Semicolon(";")]


## res

Program([Statement(FuncStat(Identifier("fact"), Some(Params([Identifier("n")])), BlockStat([IfStat(Equality([(None, Atom(Identifier("n"))), (Some("=="), Atom(Num(0.0)))]), RetStat(Some(Atom(Num(1.0)))), Some(RetStat(Some(Factor([(None, Atom(Identifier("n"))), (Some("*"), Atom(Call(Identifier("fact"), Some(Args([Term([(None, Atom(Identifier("n"))), (Some("-"), Atom(Num(1.0)))])])))))])))))]))), Statement(PrintStat(Some(Args([Atom(Call(Identifier("fact"), Some(Args([Atom(Num(10.0))]))))])))), Statement(FuncStat(Identifier("k"), Some(Params([Identifier("x")])), BlockStat([FuncStat(Identifier("ky"), Some(Params([Identifier("y")])), BlockStat([RetStat(Some(Term([(None, Atom(Identifier("x"))), (Some("+"), Atom(Identifier("y")))])))])), RetStat(Some(Atom(Identifier("ky"))))]))), Statement(VarStat(Identifier("ky"), Atom(Call(Identifier("k"), Some(Args([Atom(Num(3.0))])))))), Statement(PrintStat(Some(Args([Atom(Call(Identifier("ky"), Some(Args([Atom(Num(5.0))]))))])))), Statement(FuncStat(Identifier("fib0"), Some(Params([Identifier("n")])), BlockStat([IfStat(Comparison([(None, Atom(Identifier("n"))), (Some("<"), Atom(Num(2.0)))]), RetStat(Some(Atom(Identifier("n")))), Some(RetStat(Some(Term([(None, Atom(Call(Identifier("fib0"), Some(Args([Term([(None, Atom(Identifier("n"))), (Some("-"), Atom(Num(1.0)))])]))))), (Some("+"), Atom(Call(Identifier("fib0"), Some(Args([Term([(None, Atom(Identifier("n"))), (Some("-"), Atom(Num(2.0)))])])))))])))))]))), Statement(FuncStat(Identifier("fib"), Some(Params([Identifier("n")])), BlockStat([VarStat(Identifier("f0"), Atom(Num(0.0))), VarStat(Identifier("f1"), Atom(Num(1.0))), WhileStat(Comparison([(None, Atom(Identifier("n"))), (Some(">"), Atom(Num(0.0)))]), BlockStat([VarStat(Identifier("t"), Atom(Identifier("f1"))), AssignStat(Identifier("f1"), Term([(None, Atom(Identifier("f0"))), (Some("+"), Atom(Identifier("f1")))])), AssignStat(Identifier("f0"), Atom(Identifier("t"))), AssignStat(Identifier("n"), Term([(None, Atom(Identifier("n"))), (Some("-"), Atom(Num(1.0)))]))])), RetStat(Some(Atom(Identifier("f0"))))]))), Statement(PrintStat(Some(Args([Atom(Call(Identifier("fib"), Some(Args([Atom(Num(10.0))])))), Atom(String("hello world"))])))), Statement(FuncStat(Identifier("make_count"), Some(Params([Identifier("n")])), BlockStat([FuncStat(Identifier("inc"), None, BlockStat([AssignStat(Identifier("n"), Term([(None, Atom(Identifier("n"))), (Some("+"), Atom(Num(1.0)))])), RetStat(Some(Atom(Identifier("n"))))])), RetStat(Some(Atom(Identifier("inc"))))]))), Statement(FuncStat(Identifier("make_dog"), None, BlockStat([VarStat(Identifier("weight"), Atom(Num(10.0))), FuncStat(Identifier("eat"), Some(Params([Identifier("m")])), BlockStat([AssignStat(Identifier("weight"), Term([(None, Atom(Identifier("m"))), (Some("+"), Atom(Identifier("weight")))]))])), FuncStat(Identifier("get"), None, BlockStat([RetStat(Some(Atom(Identifier("weight"))))])), FuncStat(Identifier("dispatch"), Some(Params([Identifier("m")])), BlockStat([IfStat(Equality([(None, Atom(Identifier("m"))), (Some("=="), Atom(String("eat")))]), BlockStat([RetStat(Some(Atom(Identifier("eat"))))]), Some(IfStat(Equality([(None, Atom(Identifier("m"))), (Some("=="), Atom(String("get")))]), BlockStat([RetStat(Some(Atom(Call(Identifier("get"), None))))]), None)))])), RetStat(Some(Atom(Identifier("dispatch"))))]))), Statement(VarStat(Identifier("dog"), Atom(Call(Identifier("make_dog"), None)))), Statement(VarStat(Identifier("eat"), Atom(Call(Identifier("dog"), Some(Args([Atom(String("eat"))])))))), Statement(ExprStat(Atom(Call(Identifier("eat"), Some(Args([Atom(Num(10.0))])))))), Statement(PrintStat(Some(Args([Atom(Call(Identifier("dog"), Some(Args([Atom(String("get"))]))))])))), Statement(ExprStat(Atom(Call(Identifier("eat"), Some(Args([Atom(Num(20.0))])))))), Statement(PrintStat(Some(Args([Atom(Call(Identifier("dog"), Some(Args([Atom(String("get"))]))))])))), Statement(VarStat(Identifier("c1"), Atom(Call(Identifier("make_count"), Some(Args([Atom(Num(1.0))])))))), Statement(VarStat(Identifier("c2"), Atom(Call(Identifier("make_count"), Some(Args([Atom(Num(1.0))])))))), Statement(PrintStat(Some(Args([Atom(Call(Identifier("c1"), None)), Atom(Call(Identifier("c1"), None)), Atom(Call(Identifier("c1"), None)), Atom(Call(Identifier("c2"), None))]))))])
