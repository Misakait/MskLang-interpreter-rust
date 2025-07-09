use std::rc::Rc;
use crate::callable::Callable;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum MskValue {
    // 一个浮点数值。
    Float(f64),
    /// 一个布尔值，表示真或假。
    Boolean(bool),
    /// 一个字符串值。
    String(String),
    Callable(Rc<dyn Callable>),
    Nil,
}

impl Display for MskValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MskValue::Float(n) => write!(f, "{}", n),
            MskValue::Boolean(b) => write!(f, "{}", b),
            MskValue::String(s) => write!(f, "{}", s),
            MskValue::Callable(_) => write!(f, "<fn>"),
            MskValue::Nil => write!(f, "nil"),
        }
    }
}

impl MskValue {
    pub fn is_true(&self) -> bool {
        match self {
            MskValue::Boolean(b) => *b,
            MskValue::Nil => false,
            _ => true, // 非 Nil 和 Boolean 的值都视为 true
        }
    }
}