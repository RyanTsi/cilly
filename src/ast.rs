
#[derive(Debug)]
pub struct CompUnit {
    pub globaldefs: Vec<GlobalDef>,
}

#[derive(Debug)]
pub enum GlobalDef {
    FuncDef(FuncDef),
    Decl(Decl),
}

#[derive(Debug, Clone)]
pub enum Decl {
    VarDecl(VarDecl),
    ValDecl(ValDecl),
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub ident: String,
    pub btype: BType,
    pub initval: InitVal,
}

#[derive(Debug, Clone)]
pub struct ValDecl {
    pub ident: String,
    pub btype: BType,
    pub initval: InitVal,
}

#[derive(Debug, Clone)]
pub struct InitVal {
    pub exp: Exp,
}

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub ident: String,
    pub btype: Option<BType>,
    pub funcfparams: Option<FuncFParams>,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct FuncFParams {
    pub params: Vec<FuncFParam>,    
}

#[derive(Debug, Clone)]
pub struct FuncFParam {
    pub ident: String,
    pub btype: BType, 
}

#[derive(Debug, Clone)]
pub struct FuncRParams {
    pub exps: Vec<Exp>,
}

#[derive(Debug, Clone)]
pub enum BType {
    I32,
}


#[derive(Debug, Clone)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Debug, Clone)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(LVal, Exp),
    Block(Block),
    Exp(Option<Exp>),
    Ret(Option<Exp>),
    If { condition: Exp, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { condition: Exp, loopbody: Box<Stmt> },
    FuncDef(FuncDef),
    Continue,
    Break,
}

#[derive(Debug, Clone)]
pub struct LVal {
    pub ident: String,
}

#[derive(Debug, Clone)]
pub struct Exp {
    pub lor_exp: LOrExp,
}


#[derive(Debug, Clone)]
pub enum LOrExp {
    And(LAndExp),
    Or(Box<LOrExp>, LAndExp),
}

#[derive(Debug, Clone)]
pub enum LAndExp {
    Eq(EqExp),
    And(Box<LAndExp>, EqExp),
}


#[derive(Debug, Clone)]
pub enum EqExp {
    Rel(RelExp),
    Eq(Box<EqExp>, BinaryOp, RelExp),
}

#[derive(Debug, Clone)]
pub enum RelExp {
    Add(AddExp),
    Rel(Box<RelExp>, BinaryOp, AddExp),
}

#[derive(Debug, Clone)]
pub enum AddExp {
    Mul(MulExp),
    Add(Box<AddExp>, BinaryOp, MulExp),
}


#[derive(Debug, Clone)]
pub enum MulExp {
    Unary(UnaryExp),
    Mul(Box<MulExp>, BinaryOp, UnaryExp)
}


#[derive(Debug, Clone)]
pub enum UnaryExp {
    Pri(PrimaryExp),
    Unary(UnaryOp, Box<UnaryExp>),
    FuncCall {ident: String, funcrparams: Option<FuncRParams> },
}


#[derive(Debug, Clone)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    Number(i32),
    LVal(LVal),
}


#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
    Lt,  // <
    Gt,  // >
    Leq, // <=
    Geq, // >=
    Eq,  // ==
    Neq, // !=
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg, // -
    Not, // !
}