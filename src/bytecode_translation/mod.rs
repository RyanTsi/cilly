pub mod translate;
pub mod environment;

use environment::Environment;
use crate::vm::OpCode;
use crate::error::Result;

pub trait TransByteCode {
    // extension 扩展 (第16位记录PC位置)
    fn translate_byte(&mut self, env: &mut Environment, extension: usize) -> Result<Vec<OpCode>>;
}