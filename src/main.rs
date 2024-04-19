use cilly::ast::{Block, FuncDef};
use cilly::error::Result;
use cilly::interpreter::environment::Environment;
use cilly::interpreter::Execute;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(pub cy);

fn main() -> Result<()> {
    // 解析命令行参数
    let mut args = args();
    args.next();
    let input = args.next().unwrap();
    // args.next(); // -o
    // let output = args.next().unwrap();  

    // 读取输入文件
    let input = read_to_string(input).unwrap();

    // let input = testcode();


    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = cy::CompUnitParser::new().parse(&input).unwrap();
    
    // 输出解析得到的 AST
    println!("{:?}", ast);


    // 内置函数声明
    let printfunc = FuncDef {
        ident: "print".to_string(),
        btype: None,
        funcfparams: None,
        block: Block { items: vec![] },
    };
    let getintfunc = FuncDef {
        ident: "getint".to_string(),
        btype: None,
        funcfparams: None,
        block: Block{ items: vec![] },
    };
    let mut env = Environment::new();
    env.new_func("print", &printfunc)?;
    env.new_func("getint", &getintfunc)?;
    

    // ast.run(&mut env)?;
    
    Ok(())
}

fn testcode() -> &'static str {
    r#"
fn feb(n: i32) -> i32 {
    if(n < 2) {
        return 1;
    } else {
        return feb(n - 1) + feb(n - 2);
    }
}
fn main () {
    val m: i32 = getint();
    val res: i32 = feb(m);
    print(res);
}
    "#
}