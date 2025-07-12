use crate::ast::Expr;
use crate::msk_value::MskValue;
#[derive(Debug)]
pub enum ControlFlow {
    Break,
    Continue,
    Return(MskValue), // Return 可以携带一个可选的返回值
}