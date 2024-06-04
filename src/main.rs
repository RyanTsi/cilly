use cilly::ast::{Block, FuncDef, FuncFParam, FuncFParams};
use cilly::bytecode_translation::translate::{translate_from, translate_to};
use cilly::bytecode_translation::TransByteCode;
use cilly::error::{Error, Result};
use cilly::interpreter::environment::Environment;
use cilly::interpreter::Execute;
use cilly::vm::VM;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::{read_to_string, File};
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
        "--static" => {
            let input = args.next().unwrap();
            // 读取输入文件
            let input = read_to_string(input)?;
            // 调用 lalrpop 生成的 parser 解析输入文件
            let ast = cy::CompUnitParser::new().parse(&input).unwrap();
            ast.run(&mut env)?;
        },
        "--translate" => {
            let filename = args.next().unwrap();
            // 读取输入文件
            let input = read_to_string(&filename)?;
            // 调用 lalrpop 生成的 parser 解析输入文件
            let mut ast = cy::CompUnitParser::new().parse(&input).unwrap();
            let res = ast.translate_byte(&mut cilly::bytecode_translation::environment::Environment::new(), 0)?;
            let res = translate_to(res);
            let filename = filename.replace(".cil", ".class");
            let mut file = File::create(&filename)?;
            
            for i in &res {
                file.write(format!("{} ",i).as_bytes()).unwrap();
            }
        },
        "--vmrun" => {
            let input = args.next().unwrap();
            // 读取输入文件
            let input = read_to_string(input)?;
            let code: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
            let code = translate_from(code);
            let mut vm = VM::new(code);
            vm.run()?;
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
    var a: i32 = 10;
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
    
    fn while_test(n: i32) -> i32 {
        var i: i32 = 0;
        var res: i32 = 0;
        while(i < 10) {
            res = res + i;
            i = i + 1;
        }
        return res;
    }
    
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
    
    fn main () {
        print(a);
        print(add(100, 100));
        
        val x: i32 = getint();
        print(while_test(x));
        
        val n: i32 = getint();
        val res: i32 = fact(n);
        print(res);
        
        val m: i32 = getint();
        print(feb(m));
    }
    "#
}

fn testcode2() -> &'static str {
    r#"
var a: i32 = 10;
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



fn main () {
    print(a);
    var n: i32 = getint();
    var res: i32 = fact(n);
    print(res);
    print(while_test(n));
    val m: i32 = getint();
    print(feb(m));
}
    "#
}