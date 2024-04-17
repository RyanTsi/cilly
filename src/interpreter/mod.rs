use crate::error::Result;

use self::environment::Environment;

mod eval;
mod environment;
mod values;
mod func;
pub mod run;

pub trait Execute {
    fn run(&self, env: &mut Environment) -> Result<()>;
}