use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use log::info;
use crate::ast::Stmt;
use crate::callable::Callable;
use crate::environment::Environment;
use crate::interpreter::{Interpreter, RuntimeError, ScopeGuard};
use crate::msk_value::MskValue;
use crate::token::Token;

pub struct UserFunction {
    pub name: String,
    pub params: Vec<Token>,
    pub body: Rc<Stmt>,
    pub closure: Rc<RefCell<Environment>>,
}
impl Callable for UserFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<MskValue>) -> Result<MskValue,RuntimeError> {
        if args.len() != self.arity() {
            return Err(format!("Expected {} arguments but got {}.", self.arity(), args.len()).into());
        }

        let guard = ScopeGuard::new(interpreter);
        for (param, arg) in self.params.iter().zip(args) {
            (*guard.interpreter.env).borrow_mut().define(&param.lexeme, arg);

        }
        if let Stmt::Block {statements} = &*self.body {
            Ok(guard.interpreter.interpret(statements.as_slice())?)
        } else {
            Err("Function body must be a block statement.".to_string().into())
        }
    }
}