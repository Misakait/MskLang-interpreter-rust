use crate::interpreter::{Interpreter, RuntimeError};
use crate::msk_value::MskValue;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<MskValue>) -> Result<MskValue, RuntimeError>;
}
