use cilly::ast::{Block, FuncDef, FuncFParam, FuncFParams};
use cilly::bytecode_translation::TransByteCode;
use cilly::error::{Error, Result};
use cilly::interpreter::environment::Environment;
use cilly::interpreter::Execute;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::{self, Write};

// 引用 lalrpop 生成的解析器
lalrpop_mod!(pub cy);

fn main() -> Result<()> {
    // 解析命令行参数
    let mut args = args();
    args.next();

    let mode = args.next().unwrap(); // inter or static

    // 内置函数声明
    let printfunc = FuncDef {
        ident: "print".to_string(),
        btype: None,
        funcfparams: Some(FuncFParams{params:vec![FuncFParam {ident: String::from("content"), btype: cilly::ast::BType::I32}]}),
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

    match mode.as_str() {
        "--inter" => {
            loop {
                print!(">>> ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                match input.trim() {
                    "exit" => break,
                    ""     => continue,
                    _      => {
                        let mut env = Environment::new();
                        env.new_func("print", &printfunc)?;
                        env.new_func("getint", &getintfunc)?;
                        match cy::BlockItemParser::new().parse(&input) {
                            Ok(ast) => {
                                if let Err(e) = ast.run(&mut env) {
                                    println!("Error {:?}", e);
                                    continue;
                                }
                            }
                            Err(e) => {
                                println!("Error: {:?}", e);
                                continue;
                            }
                        }
                    }
                }
            }
        },
        "--static" => {
            let input = args.next().unwrap();
            // 读取输入文件
            let input = read_to_string(input)?;
            // 调用 lalrpop 生成的 parser 解析输入文件
            let ast = cy::CompUnitParser::new().parse(&input).unwrap();
            ast.run(&mut env)?;
        },
        "--translate" => {
            let mut ast = cy::CompUnitParser::new().parse(testcode2()).unwrap();
            let res = ast.translate_byte(&mut cilly::bytecode_translation::environment::Environment::new(), 0)?;
            println!("{:?}", res);
        }
        _ => return Err(Error::UnExpectArgs),
    };

    // args.next(); // -o
    // let output = args.next().unwrap();  


    // let input = testcode();
    
    // // 输出解析得到的 AST
    // println!("{:?}", ast);

    Ok(())
}

fn testcode1() -> &'static str {
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

fn testcode2() -> &'static str {
    r#"
var a:i32 = 1 + 2;
    "#
}