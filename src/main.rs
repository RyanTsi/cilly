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
    args.next();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input).unwrap();
    // let input = testcode();
    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = cy::CompUnitParser::new().parse(&input).unwrap();
    // println!("{:?}", ast);
    ast.run(&mut Environment::new())?;
    // 输出解析得到的 AST
    Ok(())
}

fn testcode() -> &'static str {
    r#"
fn feb(n: i32) -> i32 {
    if(n == 0) return 1;
    return feb(n - 1) * n;
}
fn main() {
    val x: i32 = feb(2);
}
    "#
}