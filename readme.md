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