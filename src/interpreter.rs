use crate::ast::Expr;
use crate::msk_value::MskValue;
use crate::token::{Literal, Token, TokenType};

pub struct Interpreter {

}
impl Interpreter {
    /// 创建一个新的 Interpreter 实例。
    pub fn new() -> Self {
        Interpreter {}
    }

    /// 解释并执行给定的 AST 表达式。
    /// 返回一个 Result，包含执行结果或错误信息。
    pub fn interpret(&self, expr: Expr) -> Result<MskValue, String> {
        match expr {
            Expr::Unary { operator, right } => {
                let value = self.interpret(*right)?;
                self.evaluate_unary(operator, value)
            }
            Expr::Binary { left, operator, right } => {
                let left_value = self.interpret(*left)?;
                let right_value = self.interpret(*right)?;
                self.evaluate_binary(operator.token_type, left_value, right_value)
            }
            Expr::Grouping { expression } => self.interpret(*expression),
            Expr::Literal { value } => {
                match value.token_type {
                    TokenType::String => Ok(MskValue::String(value.literal.unwrap().to_string())),
                    TokenType::Number => {
                        match value.literal.unwrap() {
                            Literal::Number(n) => Ok(MskValue::Float(n)),
                            _ => Err(format!("Unexpected number type for token: {}", value.lexeme)),
                        }
                    }
                    TokenType::True => Ok(MskValue::Boolean(true)),
                    TokenType::False => Ok(MskValue::Boolean(false)),
                    TokenType::Nil => Ok(MskValue::Nil),
                    _ => {
                        Err(format!("Unexpected token type: {:?}", value.token_type))
                    }
                }
            },
        }
    }
    fn evaluate_binary(&self, operator: TokenType, left: MskValue, right: MskValue) -> Result<MskValue, String> {
        match operator {
            TokenType::Plus => match (left, right) {
                (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Float(l + r)),
                (MskValue::String(l), MskValue::String(r)) => Ok(MskValue::String(format!("{}{}", l, r))),
                _ => Err("Operands must be two numbers or two strings for '+' operator.".to_string()),
            },
            TokenType::Minus => match (left, right) {
                (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Float(l - r)),
                _ => Err("Operands must be numbers for '-' operator.".to_string()),
            },
            TokenType::Star => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Float(l * r))
                } else {
                    Err("Operands must be numbers for '*' operator.".to_string())
                }
            }
            TokenType::Slash => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    if r == 0.0 {
                        Err("Division by zero is not allowed.".to_string())
                    } else {
                        Ok(MskValue::Float(l / r))
                    }
                } else {
                    Err("Operands must be numbers for '/' operator.".to_string())
                }
            }
            TokenType::Greater => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l > r))
                } else {
                    Err("Operands must be numbers for '>' operator.".to_string())
                }
            }
            TokenType::GreaterEqual => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l >= r))
                } else {
                    Err("Operands must be numbers for '>=' operator.".to_string())
                }
            }
            TokenType::Less => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l < r))
                } else {
                    Err("Operands must be numbers for '<' operator.".to_string())
                }
            }
            TokenType::LessEqual => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l <= r))
                } else {
                    Err("Operands must be numbers for '<=' operator.".to_string())
                }
            }
            TokenType::EqualEqual => {
                match (left,right) { 
                    (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Boolean(l == r)),
                    (MskValue::String(l), MskValue::String(r)) => Ok(MskValue::Boolean(l == r)),
                    (MskValue::Boolean(l), MskValue::Boolean(r)) => Ok(MskValue::Boolean(l == r)),
                    // (MskValue::Nil, MskValue::Nil) => Ok(MskValue::Boolean(true)),
                    _ => Ok(MskValue::Boolean(false)), // 不同类型的比较返回 false
                }
            }
            TokenType::BangEqual => {
                match (left,right) {
                    (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Boolean(l != r)),
                    (MskValue::String(l), MskValue::String(r)) => Ok(MskValue::Boolean(l != r)),
                    (MskValue::Boolean(l), MskValue::Boolean(r)) => Ok(MskValue::Boolean(l != r)),
                    // (MskValue::Nil, MskValue::Nil) => Ok(MskValue::Boolean(true)),
                    _ => Ok(MskValue::Boolean(true)), 
                }
            }
            _ => Err(format!("Unsupported binary operator: {:?}", operator)),
        }
    }
    fn evaluate_unary(&self, operator: Token, value: MskValue) -> Result<MskValue, String> {
        match operator.token_type {
            TokenType::Minus => {
                if let MskValue::Float(n) = value {
                    Ok(MskValue::Float(-n))
                } else {
                    Err(format!("Operand must be a number.\n[line {}]", operator.line))
                }
            }
            TokenType::Bang => {
                Ok(MskValue::Boolean(!value.is_true()))
            }
            _ => Err(format!("Unsupported unary operator: {:?}", operator)),
        }
    }
    // 其他方法...
}