use crate::error::Result;

use self::{environment::Environment, run::Label};

mod eval;
pub mod environment;
mod values;
mod func;
pub mod run;


pub trait Execute<'ast> {
    fn run(&'ast self, env: &mut Environment<'ast>) -> Result<Option<Label>>;
}