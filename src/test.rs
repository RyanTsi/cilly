use crate::vm::{OpCode, VM};

#[test]
fn vm() {
    let opcode = vec![
        OpCode::LoadConst(1),
        OpCode::LoadConst(2),
        OpCode::BinOpAdd,
        OpCode::PrintItem,
    ];
    let mut vm = VM::new(opcode);
    vm.run();
    // let x = 1..3 + 2;
    // for i in x {
    //     println!("{i}");
    // }
}