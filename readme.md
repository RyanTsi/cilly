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

ValDecl         ::= "val" IDENT "=" InitVal ";";
VarDecl         ::= "var" IDENT "=" InitVal ";";
InitVal         ::= Exp;

FuncDef         ::= FuncType IDENT "(" [FuncFParams] ")" Block;
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
fn abs(a: i32) {
    fn b(x : i32) {
        return a + b;
    }
    return b;
}

fn feb(n: i32) {
    if(n == 0) {
        return 1;
    }
    return n * feb(n - 1);
}

fn main () {
    val a = abs(1);
    val b = a(1);
    val res = feb(10);
    return 0;
}
```

**AST**

CompUnit { globaldefs: [FuncDef(FuncDef { ident: "abs", funcfparams: Some(FuncFParams { params: [FuncFParam { ident: "a", btype: I32 }] }), block: Block { items: [Stmt(FuncDef(FuncDef { ident: "b", funcfparams: Some(FuncFParams { params: [FuncFParam { ident: "x", btype: I32 }] }), block: Block { items: [Stmt(Ret(Some(Exp { lor_exp: And(Eq(Rel(Add(Add(Mul(Unary(Pri(LVal(LVal { ident: "a" })))), Add, Unary(Pri(LVal(LVal { ident: "b" })))))))) })))] } })), Stmt(Ret(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(LVal(LVal { ident: "b" })))))))) })))] } }), FuncDef(FuncDef { ident: "feb", funcfparams: Some(FuncFParams { params: [FuncFParam { ident: "n", btype: I32 }] }), block: Block { items: [Stmt(If { condition: Exp { lor_exp: And(Eq(Eq(Rel(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" })))))), Eq, Add(Mul(Unary(Pri(Number(0)))))))) }, then_branch: Block(Block { items: [Stmt(Ret(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(Number(1)))))))) })))] }), else_branch: None }), Stmt(Ret(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Mul(Unary(Pri(LVal(LVal { ident: "n" }))), Mul, FuncCall { ident: "feb", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Add(Mul(Unary(Pri(LVal(LVal { ident: "n" })))), Sub, Unary(Pri(Number(1)))))))) }] }) })))))) })))] } }), FuncDef(FuncDef { ident: "main", funcfparams: None, block: Block { items: [Decl(ValDecl(ValDecl { ident: "a", initval: InitVal { exp: Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "abs", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(Number(1)))))))) }] }) })))))) } } })), Decl(ValDecl(ValDecl { ident: "b", initval: InitVal { exp: Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "a", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(Number(1)))))))) }] }) })))))) } } })), Decl(ValDecl(ValDecl { ident: "res", initval: InitVal { exp: Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(FuncCall { ident: "feb", funcrparams: Some(FuncRParams { exps: [Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(Number(10)))))))) }] }) })))))) } } })), Stmt(Ret(Some(Exp { lor_exp: And(Eq(Rel(Add(Mul(Unary(Pri(Number(0)))))))) })))] } })] }
