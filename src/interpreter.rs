use crate::ast::{Expr, Stmt};
use crate::msk_value::MskValue;
use crate::token::{Literal, Token, TokenType};

pub struct Interpreter {

}

impl Interpreter {
    pub fn interpret(&self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", value);
                Ok(())
            }
        }
    }
}

impl Interpreter {
    /// 创建一个新的 Interpreter 实例。
    pub fn new() -> Self {
        Interpreter {}
    }

    /// 解释并执行给定的 AST 表达式。
    /// 返回一个 Result，包含执行结果或错误信息。
    pub fn evaluate(&self, expr: Expr) -> Result<MskValue, String> {
        match expr {
            Expr::Unary { operator, right } => {
                let value = self.evaluate(*right)?;
                self.evaluate_unary(operator, value)
            }
            Expr::Binary { left, operator, right } => {
                let left_value = self.evaluate(*left)?;
                let right_value = self.evaluate(*right)?;
                self.evaluate_binary(operator, left_value, right_value)
            }
            Expr::Grouping { expression } => self.evaluate(*expression),
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
    fn evaluate_binary(&self, operator: Token, left: MskValue, right: MskValue) -> Result<MskValue, String> {
        match operator.token_type {
            TokenType::Plus => match (left, right) {
                (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Float(l + r)),
                (MskValue::String(l), MskValue::String(r)) => Ok(MskValue::String(format!("{}{}", l, r))),
                _ => Err(format!("[line {}] Operands must be two numbers or two strings for '+' operator.", operator.line)),
            },
            TokenType::Minus => match (left, right) {
                (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Float(l - r)),
                _ => Err(format!("[line {}] Operands must be numbers for '-' operator.", operator.line)),
            },
            TokenType::Star => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Float(l * r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '*' operator.", operator.line))
                }
            },
            TokenType::Slash => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    if r == 0.0 {
                        Err(format!("[line {}] Division by zero is not allowed.", operator.line))
                    } else {
                        Ok(MskValue::Float(l / r))
                    }
                } else {
                    Err(format!("[line {}] Operands must be numbers for '/' operator.", operator.line))
                }
            },
            TokenType::Greater => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l > r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '>' operator.", operator.line))
                }
            },
            TokenType::GreaterEqual => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l >= r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '>=' operator.", operator.line))
                }
            },
            TokenType::Less => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l < r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '<' operator.", operator.line))
                }
            },
            TokenType::LessEqual => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l <= r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '<=' operator.", operator.line))
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
            _ => Err(format!("[line {}] Unsupported binary operator: {:?}", operator.line, operator)),
        }
    }
    fn evaluate_unary(&self, operator: Token, value: MskValue) -> Result<MskValue, String> {
        match operator.token_type {
            TokenType::Minus => {
                if let MskValue::Float(n) = value {
                    Ok(MskValue::Float(-n))
                } else {
                    Err(format!("[line {}] Operand must be a number.", operator.line))
                }
            }
            TokenType::Bang => {
                Ok(MskValue::Boolean(!value.is_true()))
            }
            _ => Err(format!("[line {}] Unsupported unary operator", operator.line))
        }
    }
    // 其他方法...
}