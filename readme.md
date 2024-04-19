## cilly_grammar

### 词法规范

#### 标识符 

```
identifier ::= identifier-nondigit
             | identifier identifier-nondigit
             | identifier digit;
```

其中, `identifier-nondigit` 为下划线, 小写英文字母或大写英文字母; `digit` 为数字 0 到 9.

#### 数值常量

```
integer-const       ::= decimal-const
                      | octal-const
                      | hexadecimal-const;
decimal-const       ::= nonzero-digit
                      | decimal-const digit;
octal-const         ::= "0"
                      | octal-const octal-digit;
hexadecimal-const   ::= hexadecimal-prefix hexadecimal-digit
                      | hexadecimal-const hexadecimal-digit;
hexadecimal-prefix  ::= "0x" | "0X";
```

其中, `nonzero-digit` 为数字 1 到 9; `octal-digit` 为数字 0 到 7; `hexadecimal-digit` 为数字 0 到 9, 或大写/小写字母 a 到 f.

### 语法规范

```
CompUnit        ::= [CompUnit] (FuncDef | Decl);

Decl            ::= ValDecl | VarDecl;

ValDecl         ::= "val" IDENT ":" BType "=" InitVal ";";
VarDecl         ::= "var" IDENT ":" BType "=" InitVal ";";
InitVal         ::= Exp;

FuncDef         ::= FuncType IDENT "(" [FuncFParams] ")" ["->" BType] Block;
FuncFParams     ::= FuncFParam {"," FuncFParam};
FuncFParam      ::= IDENT ":" BType;
FuncRParams     ::= Exp {"," Exp};
BType           ::= "i32";

Block           ::= "{" {BlockItem} "}";
BlockItem       ::= Decl | Stmt;
Stmt            ::= LVal "=" Exp ";"
                | Block
                | [Exp] ";"
                | "return" [Exp] ";";
                | "if" "(" Exp ")" Stmt ["else" Stmt]
                | "while" "(" Exp ")" Stmt
                | "continue"
                | "break"
                | FuncDef;

LVal            ::= IDENT;
    
Exp             ::= LOrExp;
PrimaryExp      ::= "(" Exp ")" | Number | LVal;
Number          ::= INT_CONST;
UnaryExp        ::= PrimaryExp
                  | UnaryOp UnaryExp
                  | IDENT "(" [FuncRParams] ")"; 
UnaryOp         ::= "+" | "-" | "!";
MulExp          ::= UnaryExp | MulExp ("*" | "/" | "%") UnaryExp;
AddExp          ::= MulExp | AddExp ("+" | "-") MulExp;
RelExp          ::= AddExp | RelExp ("<" | ">" | "<=" | ">=") AddExp;
EqExp           ::= RelExp | EqExp ("==" | "!=") RelExp;
LAndExp         ::= EqExp | LAndExp "&&" EqExp;
LOrExp          ::= LAndExp | LOrExp "||" LAndExp;

```

### AST 样例

**测试代码**

```
fn fact(n: i32) -> i32 {
    if(n == 0) return 1;
    return n * fact(n - 1);
}

fn feb(n: i32) -> i32 {
    if(n < 2) {
        return 1;
    } else {
        return feb(n - 1) + feb(n - 2);
    }
}

fn while_test() -> i32 {
    var n: i32 = 0;
    while(n < 10) {
        print(n);
        n = n + 1;
    }
    return n;
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main () {
    while_test();
    val n: i32 = getint();
    val res: i32 = fact(n);
    print(res);
    
    val m: i32 = getint();
    print(feb(m));
}
```

**AST**

CompUnit { globaldefs: [FuncDef(FuncDef { ident: "fact", btype: Some(I32), funcfparams: Some(FuncFParams { params: [FuncFParam { ident: "n", btype: I32 }] }), block: Block { items: [Stmt(If { condition: Exp { lor_exp: And(Eq(Eq(Rel(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" })))))), Eq, Add(Mul(Unary(Pri(Number(0)))))))) }, then_branch: Ret(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(Number(1)))))))) })), else_branch: None }), Stmt(Ret(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Mul(Unary(Pri(LVal(LVal { ident: "n" }))), Mul, FuncCall { ident: "fact", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" })))), Sub, Unary(Pri(Number(1)))))))) }] }) })))))) })))] } }), FuncDef(FuncDef { ident: "feb", btype: Some(I32), funcfparams: Some(FuncFParams { params: [FuncFParam { ident: "n", btype: I32 }] }), block: Block { items: [Stmt(If { condition: Exp { lor_exp: And(Eq(Rel(Rel(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" }))))), Lt, Mul(Unary(Pri(Number(2)))))))) }, then_branch: Block(Block { items: [Stmt(Ret(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(Number(1)))))))) })))] }), else_branch: Some(Block(Block { items: [Stmt(Ret(Some(Exp { lor_exp: And(Eq(Rel(Add(Add(Mul(Unary(FuncCall { ident: "feb", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" })))), Sub, Unary(Pri(Number(1)))))))) }] }) })), Add, Unary(FuncCall { ident: "feb", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" })))), Sub, Unary(Pri(Number(2)))))))) }] }) })))))) })))] })) })] } }), FuncDef(FuncDef { ident: "while_test", btype: Some(I32), funcfparams: None, block: Block { items: [Decl(VarDecl(VarDecl { ident: "n", btype: I32, initval: InitVal { exp: Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(Number(0)))))))) } } })), Stmt(While { condition: Exp { lor_exp: And(Eq(Rel(Rel(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" }))))), Lt, Mul(Unary(Pri(Number(10)))))))) }, loopbody: Block(Block { items: [Stmt(Exp(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "print", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" })))))))) }] }) })))))) }))), Stmt(Assign(LVal { ident: "n" }, Exp { lor_exp: And(Eq(Rel(Add(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" })))), Add, Unary(Pri(Number(1)))))))) }))] }) }), Stmt(Ret(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" })))))))) })))] } }), FuncDef(FuncDef { ident: "add", btype: Some(I32), funcfparams: Some(FuncFParams { params: [FuncFParam { ident: "a", btype: I32 }, FuncFParam { ident: "b", btype: I32 }] }), block: Block { items: [Stmt(Ret(Some(Exp { lor_exp: And(Eq(Rel(Add(Add(Mul(Unary(Pri(LVal(LVal { ident: "a" })))), Add, Unary(Pri(LVal(LVal { ident: "b" })))))))) })))] } }), FuncDef(FuncDef { ident: "main", btype: None, funcfparams: None, block: Block { items: [Stmt(Exp(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "print", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "add", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(Number(100)))))))) }, Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(Number(100)))))))) }] }) })))))) }] }) })))))) }))), Stmt(Exp(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "while_test", funcrparams: None })))))) }))), Decl(ValDecl(ValDecl { ident: "n", btype: I32, initval: InitVal { exp: Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "getint", funcrparams: None })))))) } } })), Decl(ValDecl(ValDecl { ident: "res", btype: I32, initval: InitVal { exp: Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "fact", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" })))))))) }] }) })))))) } } })), Stmt(Exp(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "print", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(LVal(LVal { ident: "res" })))))))) }] }) })))))) }))), Decl(ValDecl(ValDecl { ident: "m", btype: I32, initval: InitVal { exp: Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "getint", funcrparams: None })))))) } } })), Stmt(Exp(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "print", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "feb", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(LVal(LVal { ident: "m" })))))))) }] }) })))))) }] }) })))))) })))] } })] }


**解释器结果**

input
```
10
10
```

output
```
200
0
1
2
3
4
5
6
7
8
9
3628800
89
```